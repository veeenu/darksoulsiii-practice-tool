//! In-game editor for the configuration file.
//!
//! The TOML document is edited in place, guided by a static schema describing
//! how each value is presented. Only edited values are rewritten, so comments
//! and formatting are preserved. On save, the document is validated by parsing
//! it as a [`Config`].

use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs, slice, thread};

use imgui::internal::DataTypeKind;
use imgui::sys::{igSetNextWindowPos, igSetNextWindowSize, ImVec2};
use imgui::{
    Condition, DragDropFlags, SelectableFlags, StyleColor, TableColumnFlags, TableColumnSetup,
    TableFlags, Ui,
};
use libds3::prelude::*;
use practice_tool_core::controller::ControllerCombination;
use practice_tool_core::crossbeam_channel::{self, Receiver, TryRecvError};
use practice_tool_core::key::Key;
use toml_edit::visit_mut::{self, VisitMut};
use toml_edit::{Array, Decor, DocumentMut, InlineTable, Item, RawString, TableLike};

use crate::config::{config_path, Config, FLAGS, INDICATORS};
use crate::icons::{Icon, Icons};

const POPUP: &str = "##config_editor";
const CONFIRM_SAVE: &str = "##config_editor_save";
const CONFIRM_CLOSE: &str = "##config_editor_close";
const ERROR_COLOR: [f32; 4] = [1., 0.3, 0.3, 1.];
const INT_FORMAT: &str = "%lld";
const FLOAT_FORMAT: &str = "%g";

/// How a configuration value is presented, and its default for new entries.
#[derive(Clone, Copy)]
enum Field {
    /// Hotkey.
    Key,
    /// Hotkey, omitted when empty.
    OptKey,
    /// Hotkey, `true` when empty.
    KeyOrTrue,
    /// Controller button combination, omitted when empty.
    OptCombo,
    Text(&'static str),
    Bool(bool),
    Int(i64),
    Float(f64),
    Ints(&'static [i64]),
    Floats(&'static [f64]),
    /// One of the given values, the first being the default.
    Choice(&'static [&'static str]),
    Flag,
    Widgets,
    Indicators,
}

/// A kind of configuration table. For list entries, the first field identifies
/// the kind.
struct Kind {
    name: &'static str,
    fields: &'static [(&'static str, Field)],
}

#[rustfmt::skip]
const WIDGETS: &[Kind] = &[
    Kind { name: "Flag", fields: &[("flag", Field::Flag), ("hotkey", Field::OptKey)] },
    Kind { name: "Label", fields: &[("label", Field::Text(""))] },
    Kind { name: "Group", fields: &[("group", Field::Text("Group")), ("commands", Field::Widgets)] },
    Kind { name: "Savefile manager", fields: &[("savefile_manager", Field::KeyOrTrue)] },
    Kind { name: "Item spawner", fields: &[("item_spawner", Field::KeyOrTrue)] },
    Kind { name: "Character stats", fields: &[("character_stats", Field::KeyOrTrue)] },
    Kind { name: "Position", fields: &[("position", Field::KeyOrTrue), ("save", Field::OptKey)] },
    Kind { name: "Nudge position", fields: &[("nudge", Field::Float(1.)), ("nudge_up", Field::OptKey), ("nudge_down", Field::OptKey)] },
    Kind { name: "Cycle speed", fields: &[("cycle_speed", Field::Floats(&[0.5, 1., 2.])), ("hotkey", Field::OptKey)] },
    Kind { name: "Cycle color", fields: &[("cycle_color", Field::Ints(&[0, 1, 2, 3])), ("hotkey", Field::OptKey)] },
    Kind { name: "Souls", fields: &[("souls", Field::Int(10000)), ("hotkey", Field::OptKey)] },
    Kind { name: "Open menu", fields: &[("open_menu", Field::Choice(&["travel", "attune"])), ("hotkey", Field::OptKey)] },
    Kind { name: "Quitout", fields: &[("quitout", Field::KeyOrTrue)] },
    Kind { name: "Target", fields: &[("target", Field::KeyOrTrue)] },
    Kind { name: "Input viewer", fields: &[("input_viewer", Field::KeyOrTrue), ("seconds", Field::Int(5))] },
];

const MENU_ITEM: Kind =
    Kind { name: "Menu item", fields: &[("label", Field::Text("New item")), ("key", Field::Key)] };

const SETTINGS: Kind = Kind {
    name: "Settings",
    fields: &[
        ("log_level", Field::Choice(&["DEBUG", "TRACE", "INFO", "WARN", "ERROR", "OFF"])),
        ("display", Field::Key),
        ("hide", Field::OptKey),
        ("show_console", Field::Bool(false)),
        ("radial_menu_open", Field::OptCombo),
        ("indicators", Field::Indicators),
    ],
};

#[derive(Default)]
pub(crate) struct ConfigEditor {
    document: Option<Result<DocumentMut, String>>,
    /// Buffer for text inputs, allocated while the editor is open.
    scratch: String,
    dirty: bool,
    status: Option<Result<String, String>>,
    saving: Option<Receiver<Result<Config, String>>>,
}

impl ConfigEditor {
    pub(crate) fn is_open(&self) -> bool {
        self.document.is_some()
    }

    /// Renders the button opening the editor, and the editor itself. Returns
    /// the new configuration once it has been saved.
    pub(crate) fn render(&mut self, ui: &Ui, icons: &Icons) -> Option<Config> {
        if icons.small_button(ui, "##config_editor_open", Icon::Settings) {
            self.document = Some(load_document());
            self.scratch = String::with_capacity(64);
            self.dirty = false;
            self.status = None;
            ui.open_popup(POPUP);
        }

        let config = self.poll_saving();

        let [w, h] = ui.io().display_size;
        unsafe {
            igSetNextWindowPos(
                ImVec2::new(w * 0.5, h * 0.5),
                Condition::Always as _,
                ImVec2::new(0.5, 0.5),
            );
            igSetNextWindowSize(ImVec2::new(w * 0.5, h * 0.8), Condition::Always as _);
        }

        ui.modal_popup_config(POPUP)
            .resizable(false)
            .movable(false)
            .title_bar(false)
            .scroll_bar(false)
            .build(|| {
                POINTER_CHAINS.cursor_show.set(true);

                let footer_height = ui.frame_height_with_spacing() * 3.;
                match &mut self.document {
                    Some(Ok(document)) => {
                        if render_document(ui, &mut self.scratch, document, icons, footer_height) {
                            self.dirty = true;
                            self.status = None;
                        }
                    },
                    Some(Err(e)) => ui.text_colored(ERROR_COLOR, e),
                    None => {},
                }

                self.render_footer(ui);
            });

        config
    }

    /// Buttons, followed by the status.
    fn render_footer(&mut self, ui: &Ui) {
        let can_save = self.dirty && self.saving.is_none() && matches!(self.document, Some(Ok(_)));
        ui.disabled(!can_save, || {
            if ui.button("Save") {
                self.validate(ui);
            }
        });
        ui.same_line();
        // Closing mid-save could apply the result at an unexpected time, or let
        // the old file be reopened and saved over the new one.
        ui.disabled(self.saving.is_some(), || {
            if ui.button("Close") {
                if self.dirty {
                    ui.open_popup(CONFIRM_CLOSE);
                } else {
                    self.close(ui);
                }
            }
        });

        ui.same_line();
        let _wrap = ui.push_text_wrap_pos();
        match &self.status {
            Some(Ok(status)) => ui.text(status),
            Some(Err(e)) => ui.text_colored(ERROR_COLOR, e),
            None if self.dirty => ui.text_disabled("Unsaved changes"),
            None => {},
        }

        if confirm(
            ui,
            CONFIRM_SAVE,
            "Overwrite the configuration file?\nWidgets will be reloaded, losing their state \
             (e.g. saved positions).",
        ) {
            self.save();
        }
        if confirm(ui, CONFIRM_CLOSE, "Discard unsaved changes?") {
            self.close(ui);
        }
    }

    fn close(&mut self, ui: &Ui) {
        ui.close_current_popup();
        POINTER_CHAINS.cursor_show.set(false);
        self.document = None;
        self.scratch = String::new();
    }

    /// Asks for confirmation if the edited configuration is valid.
    fn validate(&mut self, ui: &Ui) {
        let Some(Ok(document)) = &self.document else { return };

        match Config::parse(&document.to_string()) {
            Ok(_) => ui.open_popup(CONFIRM_SAVE),
            Err(e) => self.status = Some(Err(e)),
        }
    }

    fn save(&mut self) {
        let (Some(Ok(document)), Some(path)) = (&self.document, config_path()) else {
            self.status = Some(Err("Couldn't find config file".to_string()));
            return;
        };

        let content = document.to_string();
        let (tx, rx) = crossbeam_channel::bounded(1);
        thread::spawn(move || tx.send(replace_file(&path, &content)));
        self.saving = Some(rx);
        self.status = Some(Ok("Saving...".to_string()));
    }

    fn poll_saving(&mut self) -> Option<Config> {
        let result = match self.saving.as_ref()?.try_recv() {
            Ok(result) => result,
            Err(TryRecvError::Empty) => return None,
            Err(TryRecvError::Disconnected) => {
                Err("Saving thread stopped unexpectedly".to_string())
            },
        };
        self.saving = None;

        match result {
            Ok(config) => {
                self.dirty = false;
                self.status = Some(Ok("Configuration saved".to_string()));
                Some(config)
            },
            Err(e) => {
                self.status = Some(Err(e));
                None
            },
        }
    }
}

/// Replaces the file at `path` with `content`, verified in a temporary file
/// first and then moved over it in one step.
fn replace_file(path: &Path, content: &str) -> Result<Config, String> {
    let temp = path.with_extension("tmp.toml");
    let result = write_verified(&temp, content).and_then(|config| {
        fs::rename(&temp, path).map_err(|e| format!("Couldn't replace config file: {e}"))?;
        Ok(config)
    });
    if result.is_err() {
        fs::remove_file(&temp).ok();
    }
    result
}

fn write_verified(path: &Path, content: &str) -> Result<Config, String> {
    fs::write(path, content).map_err(|e| format!("Couldn't write config file: {e}"))?;
    let written =
        fs::read_to_string(path).map_err(|e| format!("Couldn't read back config file: {e}"))?;
    if written != content {
        return Err("Config file content doesn't match what was written".to_string());
    }
    Config::parse(&written)
}

fn load_document() -> Result<DocumentMut, String> {
    let path = config_path().ok_or("Couldn't find config file")?;
    let content =
        fs::read_to_string(path).map_err(|e| format!("Couldn't read config file: {e}"))?;
    parse_document(&content)
}

fn parse_document(content: &str) -> Result<DocumentMut, String> {
    let mut document: DocumentMut =
        content.parse().map_err(|e| format!("Couldn't parse config file: {e}"))?;

    // Lists written as arrays of tables (`[[commands]]`) become arrays of
    // inline tables, which is what the editor works with.
    for key in ["commands", "radial-menu"] {
        if let Some(item) = document.get_mut(key).filter(|item| item.is_array_of_tables()) {
            item.make_value();
            if let Some(array) = item.as_array_mut() {
                one_per_line(array);
            }
            if let Some(mut key) = document.key_mut(key) {
                key.leaf_decor_mut().clear();
            }
        }
    }
    DetachTrailing.visit_document_mut(&mut document);

    if !document.get("commands").is_some_and(Item::is_array) {
        return Err("Missing commands".to_string());
    }
    let settings =
        document.get_mut("settings").and_then(Item::as_table_like_mut).ok_or("Missing settings")?;
    add_missing_indicators(settings);
    document.entry("radial-menu").or_insert(toml_edit::value(Array::new()));

    Ok(document)
}

/// Lists all indicators, so that they can be toggled. Indicators missing from
/// the list are hidden, unless the whole list is, which shows the default ones.
fn add_missing_indicators(settings: &mut dyn TableLike) {
    let defaults = !settings.contains_key("indicators");
    let indicators = settings.entry("indicators").or_insert(toml_edit::value(Array::new()));
    let Some(indicators) = indicators.as_array_mut() else { return };

    for &(name, _, enabled) in INDICATORS {
        let present = indicators.iter().any(|indicator| {
            let indicator = indicator.as_inline_table().and_then(|i| i.get("indicator"));
            indicator.and_then(toml_edit::Value::as_str) == Some(name)
        });
        if !present {
            let indicator = [("indicator", name.into()), ("enabled", (defaults && enabled).into())];
            push_entry(
                indicators,
                toml_edit::Value::from_iter::<[(_, toml_edit::Value); 2]>(indicator),
            );
        }
    }

    if defaults {
        one_per_line(indicators);
    }
}

/// Lays out the array with one item per line.
fn one_per_line(array: &mut Array) {
    for item in array.iter_mut() {
        item.decor_mut().set_prefix("\n  ");
    }
    array.set_trailing_comma(true);
    array.set_trailing("\n");
}

/// Moves the whitespace and comments before each array's closing bracket out
/// of its last item, which owns them when there's no trailing comma, so that
/// items can be reordered without moving them along.
struct DetachTrailing;

impl VisitMut for DetachTrailing {
    fn visit_array_mut(&mut self, array: &mut Array) {
        if !array.trailing_comma() {
            if let Some(last) = array.iter_mut().last() {
                let suffix = last.decor().suffix().and_then(RawString::as_str);
                let suffix = suffix.unwrap_or_default().to_string();
                last.decor_mut().set_suffix("");
                let trailing = array.trailing().as_str().unwrap_or_default();
                array.set_trailing(format!("{suffix}{trailing}"));
            }
        }
        visit_mut::visit_array_mut(self, array);
    }
}

/// Sets `table[name]` to `value`, keeping the formatting around the value it
/// replaces, or removes it if `None`.
fn set(table: &mut dyn TableLike, name: &str, value: Option<toml_edit::Value>) {
    let Some(mut value) = value else {
        remove(table, name);
        return;
    };

    if let Some(Item::Value(old)) = table.get_mut(name) {
        *value.decor_mut() = old.decor().clone();
        *old = value;
    } else {
        table.insert(name, Item::Value(value));
    }
}

/// Removes `table[name]`, keeping the comments above it, unless it's the last
/// key.
fn remove(table: &mut dyn TableLike, name: &str) {
    let comments = table.key(name).and_then(|key| comments(key.leaf_decor())).map(str::to_string);
    let next = table.iter().map(|(key, _)| key).skip_while(|&key| key != name).nth(1);
    let next = next.map(str::to_string);
    table.remove(name);

    let Some((comments, next)) = comments.zip(next) else { return };
    if let Some(mut key) = table.key_mut(&next) {
        let decor = key.leaf_decor_mut();
        let prefix =
            prepend(&comments, decor.prefix().and_then(RawString::as_str).unwrap_or_default());
        decor.set_prefix(prefix);
    }
}

/// Removes `array[index]`, keeping the comments above it.
fn remove_entry(array: &mut Array, index: usize) {
    let removed = array.remove(index);
    let Some(comments) = comments(removed.decor()) else { return };

    match array.get_mut(index) {
        Some(next) => {
            let prefix = next.decor().prefix().and_then(RawString::as_str).unwrap_or_default();
            let prefix = prepend(comments, prefix);
            next.decor_mut().set_prefix(prefix);
        },
        None => {
            let trailing = prepend(comments, array.trailing().as_str().unwrap_or_default());
            array.set_trailing(trailing);
        },
    }
}

/// The comment lines in an item's prefix, if any, up to the item's own line.
fn comments(decor: &Decor) -> Option<&str> {
    let prefix = decor.prefix()?.as_str()?;
    let comments = &prefix[..prefix.rfind('\n')? + 1];
    comments.contains('#').then_some(comments)
}

/// Puts comment lines in front of an item's `prefix`.
fn prepend(comments: &str, prefix: &str) -> String {
    format!("{comments}{}", prefix.strip_prefix('\n').unwrap_or(prefix))
}

fn new_entry(kind: &Kind) -> toml_edit::Value {
    kind.fields
        .iter()
        .filter_map(|&(name, field)| {
            let value: toml_edit::Value = match field {
                Field::KeyOrTrue => true.into(),
                Field::Text(text) => text.into(),
                Field::Bool(b) => b.into(),
                Field::Int(i) => i.into(),
                Field::Float(f) => f.into(),
                Field::Ints(values) => values.iter().copied().collect(),
                Field::Floats(values) => values.iter().copied().collect(),
                Field::Choice(options) => options[0].into(),
                Field::Flag => FLAGS[0].0.into(),
                Field::Widgets => Array::new().into(),
                Field::Key | Field::OptKey | Field::OptCombo | Field::Indicators => return None,
            };
            Some((name, value))
        })
        .collect()
}

/// Appends `entry`, on its own line if the array spans multiple lines.
fn push_entry(array: &mut Array, mut entry: toml_edit::Value) {
    let indent = array
        .iter()
        .filter_map(|item| {
            let prefix = item.decor().prefix()?.as_str()?;
            Some(prefix[prefix.rfind('\n')?..].to_string())
        })
        .last();

    match indent {
        Some(indent) => {
            entry.decor_mut().set_prefix(indent);
            array.push_formatted(entry);
        },
        None => array.push(entry),
    }
}

/// Renders the editor tabs. Returns whether anything was changed.
fn render_document(
    ui: &Ui,
    scratch: &mut String,
    document: &mut DocumentMut,
    icons: &Icons,
    footer_height: f32,
) -> bool {
    let Some(_tabs) = ui.tab_bar("##config_editor_tabs") else { return false };

    let columns = [("Widget", 14.), ("Value", 0.), ("Hotkey", 0.)];
    let mut changed = tab(ui, "Widgets", &columns, footer_height, || {
        let commands = document.get_mut("commands")?.as_array_mut()?;
        Some(render_entries(ui, scratch, commands, WIDGETS, icons))
    });

    let columns = [("Item", 9.), ("Label", 0.), ("Hotkey", 0.)];
    changed |= tab(ui, "Radial menu", &columns, footer_height, || {
        let items = document.get_mut("radial-menu")?.as_array_mut()?;
        Some(render_entries(ui, scratch, items, slice::from_ref(&MENU_ITEM), icons))
    });

    let columns = [("Setting", 18.), ("Value", 0.)];
    changed |= tab(ui, "Settings", &columns, footer_height, || {
        let settings = document.get_mut("settings")?.as_table_like_mut()?;
        Some(render_settings(ui, scratch, settings))
    });

    changed
}

/// Tab holding a scrolling table with a header row, filling the window except
/// for the footer. Column widths are in font sizes, zero stretching to fill the
/// remaining space. Returns whether the rows changed anything.
fn tab(
    ui: &Ui,
    label: &str,
    columns: &[(&str, f32)],
    footer_height: f32,
    rows: impl FnOnce() -> Option<bool>,
) -> bool {
    let Some(_tab) = ui.tab_item(label) else { return false };
    let flags = TableFlags::ROW_BG | TableFlags::SCROLL_Y | TableFlags::PAD_OUTER_X;
    let Some(_table) =
        ui.begin_table_with_sizing(label, columns.len(), flags, [0., -footer_height], 0.)
    else {
        return false;
    };

    for &(name, width) in columns {
        ui.table_setup_column_with(TableColumnSetup {
            flags: if width > 0. {
                TableColumnFlags::WIDTH_FIXED
            } else {
                TableColumnFlags::WIDTH_STRETCH
            },
            init_width_or_weight: width * ui.current_font_size(),
            ..TableColumnSetup::new(name)
        });
    }
    ui.table_setup_scroll_freeze(0, 1);
    ui.table_headers_row();

    rows().unwrap_or_default()
}

/// List rows with drag and drop reordering, deletion, and addition of entries
/// of the given `kinds`.
fn render_entries(
    ui: &Ui,
    scratch: &mut String,
    entries: &mut Array,
    kinds: &'static [Kind],
    icons: &Icons,
) -> bool {
    // Payloads are tagged with the list they belong to, so that entries can
    // only be dropped within the same list.
    let payload_type = format!("cfg{:x}", entries as *const _ as usize);
    let mut changed = false;
    let mut moved = None;
    let mut deleted = None;

    for i in 0..entries.len() {
        // Entries of unknown kinds can only be moved or deleted.
        let entry = entries.get_mut(i).and_then(toml_edit::Value::as_inline_table_mut);
        let entry = entry.and_then(|entry| {
            Some((kinds.iter().find(|kind| entry.contains_key(kind.fields[0].0))?, entry))
        });
        let name = entry.as_ref().map_or("Unknown", |(kind, _)| kind.name);

        let _id = ui.push_id_usize(i);
        ui.table_next_row();
        ui.table_next_column();

        // Drop target spanning the row, behind its controls.
        let start = ui.cursor_pos();
        ui.selectable_config("##row")
            .span_all_columns(true)
            .flags(SelectableFlags::ALLOW_ITEM_OVERLAP)
            .size([0., ui.frame_height()])
            .build();
        if let Some(from) = drop_target(ui, &payload_type, i) {
            moved = Some((from, i));
        }
        ui.set_cursor_pos(start);

        icons.button(ui, "##move", Icon::Grip);
        if let Some(_tooltip) = ui.drag_drop_source_config(&payload_type).begin_payload(i) {
            ui.text(name);
        }
        ui.same_line();
        if icons.button(ui, "##delete", Icon::Trash) {
            ui.open_popup("##confirm_delete");
        }
        if confirm(ui, "##confirm_delete", format_args!("Delete {name}?")) {
            deleted = Some(i);
        }
        ui.same_line();

        let children = entry.as_ref().and_then(|(kind, _)| {
            kind.fields.iter().find(|&&(_, field)| Column::of(field) == Column::Children)
        });
        let group = match children {
            Some(_) => ui.tree_node_config(name).default_open(true).frame_padding(true).push(),
            None => {
                ui.align_text_to_frame_padding();
                ui.text(name);
                None
            },
        };

        let Some((kind, entry)) = entry else { continue };
        let len = entry.len();
        ui.table_next_column();
        changed |= render_column(ui, scratch, entry, kind, Column::Value);
        ui.table_next_column();
        changed |= render_column(ui, scratch, entry, kind, Column::Hotkey);

        // Adding or removing keys leaves odd spacing around them. Inline
        // tables can't hold comments, so reformatting them loses nothing.
        if entry.len() != len {
            entry.fmt();
        }

        let children = children.and_then(|&(name, _)| entry.get_mut(name)?.as_array_mut());
        if let (Some(_group), Some(children)) = (group, children) {
            changed |= render_entries(ui, scratch, children, WIDGETS, icons);
        }
    }

    if let Some((from, to)) = moved.filter(|(from, to)| from != to) {
        let entry = entries.remove(from);
        entries.insert_formatted(to, entry);
        changed = true;
    }

    if let Some(i) = deleted {
        remove_entry(entries, i);
        changed = true;
    }

    ui.table_next_row();
    ui.table_next_column();
    if ui.button_with_size("+ Add", [ui.content_region_avail()[0], 0.]) {
        match kinds {
            [kind] => {
                push_entry(entries, new_entry(kind));
                changed = true;
            },
            _ => ui.open_popup("##add"),
        }
    }
    ui.popup("##add", || {
        for kind in kinds {
            if ui.selectable(kind.name) {
                push_entry(entries, new_entry(kind));
                changed = true;
            }
        }
    });

    changed
}

/// Makes the last item a target for dropping list items of `payload_type`,
/// showing where the dropped item would land. Returns the index of the item
/// dropped.
fn drop_target(ui: &Ui, payload_type: &str, index: usize) -> Option<usize> {
    let target = ui.drag_drop_target()?;
    let flags = DragDropFlags::ACCEPT_BEFORE_DELIVERY | DragDropFlags::ACCEPT_NO_DRAW_DEFAULT_RECT;
    let payload = target.accept_payload::<usize, _>(payload_type, flags)?.ok()?;

    // Items moving down land after the target, items moving up before it.
    let [min, max] = [ui.item_rect_min(), ui.item_rect_max()];
    let y = if payload.data < index { max[1] } else { min[1] };
    ui.get_foreground_draw_list()
        .add_line([min[0], y], [max[0], y], ui.style_color(StyleColor::DragDropTarget))
        .thickness(2.)
        .build();

    payload.delivery.then_some(payload.data)
}

/// Where a field is shown in a list row.
#[derive(PartialEq)]
enum Column {
    Value,
    Hotkey,
    /// Rows below the entry's own.
    Children,
}

impl Column {
    fn of(field: Field) -> Self {
        match field {
            Field::Key | Field::OptKey | Field::KeyOrTrue => Column::Hotkey,
            Field::Widgets => Column::Children,
            _ => Column::Value,
        }
    }
}

/// Renders the fields of `entry` belonging to `column` side by side, filling
/// it.
fn render_column(
    ui: &Ui,
    scratch: &mut String,
    entry: &mut InlineTable,
    kind: &Kind,
    column: Column,
) -> bool {
    let fields = || kind.fields.iter().filter(|&&(_, field)| Column::of(field) == column);
    let count = fields().count() as f32;
    if count == 0. {
        return false;
    }

    let spacing = ui.clone_style().item_spacing[0];
    let _width =
        ui.push_item_width((ui.content_region_avail()[0] - spacing * (count - 1.)) / count);

    let mut changed = false;
    for (i, &(name, field)) in fields().enumerate() {
        if i > 0 {
            ui.same_line();
        }
        changed |= render_value(ui, scratch, entry, name, field);
    }
    changed
}

/// Renders one row per setting, with its name and value.
fn render_settings(ui: &Ui, scratch: &mut String, settings: &mut dyn TableLike) -> bool {
    let mut changed = false;
    for &(name, field) in SETTINGS.fields {
        ui.table_next_row();
        ui.table_next_column();
        ui.align_text_to_frame_padding();
        ui.text(name);
        // Only applied at startup.
        if matches!(name, "log_level" | "show_console") {
            ui.same_line();
            ui.text_disabled("(requires restart)");
        }
        ui.table_next_column();
        let _width = ui.push_item_width(-f32::MIN_POSITIVE);
        changed |= render_value(ui, scratch, settings, name, field);
    }
    changed
}

/// Renders an input for `table[name]`, filling the item width. Returns whether
/// it was changed.
fn render_value(
    ui: &Ui,
    scratch: &mut String,
    table: &mut dyn TableLike,
    name: &str,
    field: Field,
) -> bool {
    let _id = ui.push_id(name);
    let value = table.get(name).and_then(Item::as_value);
    let text = value.and_then(toml_edit::Value::as_str).unwrap_or_default();

    // The new value, if edited: `None` removes it.
    let edit: Option<Option<toml_edit::Value>> = ui.group(|| match field {
        Field::Flag => combo(ui, text, || FLAGS.iter().map(|&(id, label, _)| (id, label)))
            .map(|id| Some(id.into())),
        Field::Choice(options) => {
            combo(ui, text, || options.iter().map(|&o| (o, o))).map(|o| Some(o.into()))
        },
        Field::Key | Field::OptKey | Field::KeyOrTrue => hotkey(ui, scratch, name, field, text)
            .map(|key| match field {
                Field::KeyOrTrue if key.is_empty() => Some(true.into()),
                _ if key.is_empty() => None,
                _ => Some(key.into()),
            }),
        Field::Text(_) | Field::OptCombo => {
            scratch.clear();
            scratch.push_str(text);
            let _color = (!is_valid(field, text))
                .then(|| ui.push_style_color(StyleColor::Text, ERROR_COLOR));
            ui.input_text("##value", scratch).hint(name).build().then(|| match field {
                Field::OptCombo if scratch.is_empty() => None,
                _ => Some(scratch.as_str().into()),
            })
        },
        Field::Bool(default) => {
            let mut b = value.and_then(toml_edit::Value::as_bool).unwrap_or(default);
            ui.checkbox("##value", &mut b).then(|| Some(b.into()))
        },
        Field::Int(default) => {
            let mut i = value.and_then(toml_edit::Value::as_integer).unwrap_or(default);
            ui.input_scalar("##value", &mut i).build().then(|| Some(i.into()))
        },
        Field::Float(default) => {
            let mut f = value.and_then(as_float).unwrap_or(default);
            let input = ui.input_scalar("##value", &mut f).display_format(FLOAT_FORMAT);
            input.build().then(|| Some(f.into()))
        },
        Field::Ints(_) => {
            let values = value.and_then(toml_edit::Value::as_array);
            scalars(ui, values, toml_edit::Value::as_integer, INT_FORMAT).map(|a| Some(a.into()))
        },
        Field::Floats(_) => {
            let values = value.and_then(toml_edit::Value::as_array);
            scalars(ui, values, as_float, FLOAT_FORMAT).map(|a| Some(a.into()))
        },
        Field::Indicators => {
            indicators(ui, value.and_then(toml_edit::Value::as_array)).map(|a| Some(a.into()))
        },
        // Rendered as rows instead.
        Field::Widgets => None,
    });

    if ui.is_item_hovered() {
        ui.tooltip_text(name);
    }

    let Some(value) = edit else { return false };
    set(table, name, value);
    true
}

fn as_float(value: &toml_edit::Value) -> Option<f64> {
    value.as_float().or(value.as_integer().map(|i| i as f64))
}

fn is_valid(field: Field, s: &str) -> bool {
    match field {
        Field::OptKey | Field::KeyOrTrue | Field::OptCombo if s.is_empty() => true,
        Field::Key | Field::OptKey | Field::KeyOrTrue => Key::from_str(s).is_ok(),
        Field::OptCombo => ControllerCombination::try_from(s).is_ok(),
        _ => true,
    }
}

/// Button showing a hotkey, which is replaced by the next key combination
/// pressed after clicking it. Returns the new hotkey, empty if cleared.
fn hotkey(
    ui: &Ui,
    scratch: &mut String,
    name: &str,
    field: Field,
    current: &str,
) -> Option<String> {
    {
        let color = if !is_valid(field, current) {
            Some(ERROR_COLOR)
        } else if current.is_empty() {
            Some(ui.style_color(StyleColor::TextDisabled))
        } else {
            None
        };
        let _color = color.map(|color| ui.push_style_color(StyleColor::Text, color));

        scratch.clear();
        scratch.push_str(if current.is_empty() { name } else { current });
        scratch.push_str("###hotkey");
        if ui.button_with_size(&scratch, [ui.calc_item_width(), 0.]) {
            ui.open_popup("##capture");
        }
    }

    let mut new = None;
    ui.modal_popup_config("##capture").resizable(false).movable(false).title_bar(false).build(
        || {
            ui.text(format!("Press the new {name} hotkey..."));

            if ui.is_key_pressed(imgui::Key::Escape) || ui.button("Cancel") {
                ui.close_current_popup();
            } else if let Some(key) = Key::pressed(ui) {
                new = Some(key.to_string());
                ui.close_current_popup();
            }

            if !matches!(field, Field::Key) {
                ui.same_line();
                if ui.button("Clear") {
                    new = Some(String::new());
                    ui.close_current_popup();
                }
            }
        },
    );

    new
}

/// Combo box over `(value, label)` options. Returns the selected value.
fn combo<I>(ui: &Ui, current: &str, options: impl Fn() -> I) -> Option<&'static str>
where
    I: Iterator<Item = (&'static str, &'static str)>,
{
    let preview = options().find(|&(id, _)| id == current).map_or(current, |(_, label)| label);
    let _combo = ui.begin_combo("##value", preview)?;

    let mut selected = None;
    for (id, label) in options() {
        if ui.selectable_config(label).selected(id == current).build() {
            selected = Some(id);
        }
    }
    selected
}

/// Row of numeric inputs with buttons for adding and removing values. Returns
/// the edited array.
fn scalars<T>(
    ui: &Ui,
    values: Option<&Array>,
    get: fn(&toml_edit::Value) -> Option<T>,
    format: &str,
) -> Option<Array>
where
    T: DataTypeKind + Default + Into<toml_edit::Value>,
{
    let empty = Array::new();
    let values = values.unwrap_or(&empty);
    let mut edited = None;

    {
        let _width = ui.push_item_width(ui.current_font_size() * 3.);
        for (i, value) in values.iter().enumerate() {
            let _id = ui.push_id_usize(i);
            let mut value = get(value).unwrap_or_default();
            if ui.input_scalar("##value", &mut value).display_format(format).build() {
                let mut values = values.clone();
                values.replace(i, value);
                edited = Some(values);
            }
            ui.same_line();
        }
    }

    if ui.button("+") {
        let mut values = values.clone();
        values.push(T::default());
        edited = Some(values);
    }
    ui.same_line();
    if ui.button("-") && !values.is_empty() {
        let mut values = values.clone();
        values.remove(values.len() - 1);
        edited = Some(values);
    }

    edited
}

/// Checkboxes for `{ indicator, enabled }` tables. Returns the edited array.
fn indicators(ui: &Ui, indicators: Option<&Array>) -> Option<Array> {
    let mut edited = None;

    for (i, indicator) in indicators.into_iter().flatten().enumerate() {
        let Some(indicator) = indicator.as_inline_table() else { continue };
        let name = indicator.get("indicator").and_then(toml_edit::Value::as_str);
        let mut enabled = indicator.get("enabled").and_then(toml_edit::Value::as_bool);

        if let (Some(name), Some(enabled)) = (name, enabled.as_mut()) {
            if ui.checkbox(name, enabled) {
                let mut indicators = indicators?.clone();
                let indicator = indicators.get_mut(i)?.as_inline_table_mut()?;
                set(indicator, "enabled", Some((*enabled).into()));
                edited = Some(indicators);
            }
        }
    }

    edited
}

/// Modal asking a yes/no question. Returns whether it was answered yes.
fn confirm(ui: &Ui, id: &str, question: impl fmt::Display) -> bool {
    let mut confirmed = false;

    ui.modal_popup_config(id).resizable(false).movable(false).title_bar(false).build(|| {
        ui.text(question.to_string());
        if ui.button("Yes") {
            confirmed = true;
            ui.close_current_popup();
        }
        ui.same_line();
        if ui.button("No") {
            ui.close_current_popup();
        }
    });

    confirmed
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG: &str = include_str!("../../jdsd_dsiii_practice_tool.toml");

    fn inline_table(array: &mut Array, i: usize) -> &mut InlineTable {
        array.get_mut(i).unwrap().as_inline_table_mut().unwrap()
    }

    #[test]
    fn test_round_trip() {
        assert_eq!(parse_document(CONFIG).unwrap().to_string(), CONFIG);
    }

    #[test]
    fn test_edits_preserve_comments() {
        let mut document = parse_document(CONFIG).unwrap();

        let commands = document["commands"].as_array_mut().unwrap();
        let first = commands.remove(0);
        commands.insert_formatted(commands.len(), first);

        let group = inline_table(commands, 8);
        let render_flags = group.get_mut("commands").unwrap().as_array_mut().unwrap();
        render_flags.remove(0);
        let ik_foot_ray = render_flags.remove(4);
        render_flags.insert_formatted(0, ik_foot_ray);
        let ik_foot_ray = inline_table(render_flags, 0);
        set(ik_foot_ray, "hotkey", Some("f10".into()));
        ik_foot_ray.fmt();
        push_entry(render_flags, new_entry(&WIDGETS[1]));

        let radial_menu = document["radial-menu"].as_array_mut().unwrap();
        let save_position = radial_menu.remove(6);
        radial_menu.insert_formatted(0, save_position);
        push_entry(radial_menu, new_entry(&MENU_ITEM));
        set(inline_table(radial_menu, 7), "key", Some("f12".into()));

        let written = document.to_string();
        println!("{written}");
        Config::parse(&written).unwrap();
        assert!(written.contains(
            "commands = [\n    # { flag = \"all_draw_hit\" }, # conflicts with debug_draw\n    { \
             flag = \"ik_foot_ray\", hotkey = \"f10\" },\n    { flag = \"rend_obj\", hotkey = \
             \"f5\" },"
        ));
        assert!(written.contains("{ flag = \"bloodstain_draw\" },\n    { label = \"\" },\n  ]},"));
        assert!(written.contains(
            "{ flag = \"evt_disable\", hotkey = \"f9\" },\n  { quitout = \"p\" },\n  { \
             savefile_manager = \"ctrl+o\" }\n]"
        ));
        assert!(written.contains(
            "radial-menu = [\n  { key = \"rshift+h\", label = \"Save position\" },\n  { key = \
             \"p\""
        ));
        assert!(written.contains(
            "{ key = \"ctrl+n\", label = \"Target Info\" },\n  { label = \"New item\", key = \
             \"f12\" },\n]"
        ));
    }

    #[test]
    fn test_new_widgets_are_valid() {
        let mut document = parse_document(CONFIG).unwrap();
        let commands = document["commands"].as_array_mut().unwrap();
        commands.clear();
        for kind in WIDGETS {
            push_entry(commands, new_entry(kind));
        }

        Config::parse(&document.to_string()).unwrap();
    }

    #[test]
    fn test_load_fixups_and_removals() {
        let mut document = parse_document(
            "commands = [\n  { quitout = \"p\" },\n  # About target\n  { target = \"t\" },\n  # \
             About label\n  { label = \"x\" }\n]\n\n[[radial-menu]]\nkey = \"p\"\nlabel = \
             \"Quitout\"\n\n[settings]\nlog_level = \"DEBUG\"\ndisplay = \"0\"\n# About \
             hide\nhide = \"rshift+0\"\nradial_menu_open = \"l3+r3\"\n",
        )
        .unwrap();

        let commands = document["commands"].as_array_mut().unwrap();
        remove_entry(commands, 1);
        remove_entry(commands, 1);
        set(document["settings"].as_table_like_mut().unwrap(), "hide", None);

        let written = document.to_string();
        println!("{written}");
        Config::parse(&written).unwrap();
        assert!(written.starts_with(
            "commands = [\n  { quitout = \"p\" }\n  # About target\n  # About label\n]\n"
        ));
        assert!(written.contains("radial-menu = [\n  { key = \"p\", label = \"Quitout\" },\n]"));
        assert!(written.contains("# About hide\nradial_menu_open = \"l3+r3\""));
        assert!(written.contains(
            "indicators = [\n  { indicator = \"game_version\", enabled = true },\n  { indicator = \
             \"igt\", enabled = true },\n  { indicator = \"position\", enabled = false },"
        ));
    }

    #[test]
    fn test_replace_file() {
        let dir = std::env::temp_dir().join(format!("config_editor_test_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");

        fs::write(&path, "old").unwrap();
        replace_file(&path, CONFIG).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), CONFIG);
        assert!(!path.with_extension("tmp.toml").exists());

        assert!(replace_file(&path, "invalid").is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), CONFIG);
        assert!(!path.with_extension("tmp.toml").exists());

        fs::remove_dir_all(&dir).unwrap();
    }
}
