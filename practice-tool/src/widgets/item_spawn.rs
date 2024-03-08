use std::borrow::Cow;
use std::ffi::c_void;
use std::fmt::Display;

use imgui::sys::{
    igGetCursorPosX, igGetCursorPosY, igGetTreeNodeToLabelSpacing, igGetWindowPos, igIndent,
    igSetNextWindowPos, igUnindent, ImVec2,
};
use imgui::{Condition, InputText, TreeNodeFlags};
use libds3::memedit::Bitflag;
use once_cell::sync::Lazy;
use practice_tool_core::crossbeam_channel::Sender;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::{scaling_factor, Widget, BUTTON_HEIGHT, BUTTON_WIDTH};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

const DEFAULT_ITEM: u32 = 0x007A1200;

static INFUSION_TYPES: [(u32, &str); 16] = [
    (0, "Normal"),
    (100, "Heavy"),
    (200, "Sharp"),
    (300, "Refined"),
    (400, "Simple"),
    (500, "Crystal"),
    (600, "Fire"),
    (700, "Chaos"),
    (800, "Lightning"),
    (900, "Deep"),
    (1000, "Dark"),
    (1100, "Poison"),
    (1200, "Blood"),
    (1300, "Raw"),
    (1400, "Blessed"),
    (1500, "Hollow"),
];

static UPGRADES: [(u32, &str); 11] = [
    (0, "+0"),
    (1, "+1"),
    (2, "+2"),
    (3, "+3"),
    (4, "+4"),
    (5, "+5"),
    (6, "+6"),
    (7, "+7"),
    (8, "+8"),
    (9, "+9"),
    (10, "+10"),
];

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ItemIDNode {
    Leaf { id: HexU32, desc: String },
    Node { node: String, children: Vec<ItemIDNode> },
}

#[derive(Debug)]
enum ItemIDNodeRef<'a> {
    Leaf { node: &'a str, value: u32 },
    Node { node: &'a str, children: Vec<ItemIDNodeRef<'a>> },
}

impl<'a> ItemIDNodeRef<'a> {
    fn render(&self, ui: &imgui::Ui, current: &mut u32, filtered: bool) {
        match self {
            ItemIDNodeRef::Leaf { node, value } => {
                unsafe { igUnindent(igGetTreeNodeToLabelSpacing()) };
                ui.tree_node_config(*node)
                    .label::<&str, &str>(node)
                    .flags(if current == value {
                        TreeNodeFlags::LEAF
                            | TreeNodeFlags::SELECTED
                            | TreeNodeFlags::NO_TREE_PUSH_ON_OPEN
                            | TreeNodeFlags::SPAN_AVAIL_WIDTH
                    } else {
                        TreeNodeFlags::LEAF
                            | TreeNodeFlags::NO_TREE_PUSH_ON_OPEN
                            | TreeNodeFlags::SPAN_AVAIL_WIDTH
                    })
                    .build(|| {});
                unsafe { igIndent(igGetTreeNodeToLabelSpacing()) };
                if ui.is_item_clicked() {
                    *current = *value;
                }
            },
            ItemIDNodeRef::Node { node, children } => {
                let n = ui.tree_node_config(*node).label::<&str, &str>(node);

                let n = if filtered { n.opened(filtered, Condition::Always) } else { n };

                n.flags(TreeNodeFlags::SPAN_AVAIL_WIDTH).build(|| {
                    for node in children {
                        node.render(ui, current, filtered);
                    }
                });
            },
        }
    }
}

impl<'a> From<&'a ItemIDNode> for ItemIDNodeRef<'a> {
    fn from(v: &'a ItemIDNode) -> Self {
        match v {
            ItemIDNode::Leaf { id, desc } => ItemIDNodeRef::Leaf { node: desc, value: id.0 },
            ItemIDNode::Node { node, children } => ItemIDNodeRef::Node {
                node,
                children: children.iter().map(ItemIDNodeRef::from).collect(),
            },
        }
    }
}

impl ItemIDNode {
    fn filter(&self, filter: &str) -> Option<ItemIDNodeRef> {
        if filter.is_empty() {
            Some(ItemIDNodeRef::from(self))
        } else {
            match self {
                ItemIDNode::Leaf { id, desc } => {
                    if string_match(filter, desc) {
                        Some(ItemIDNodeRef::Leaf { node: desc, value: id.0 })
                    } else {
                        None
                    }
                },
                ItemIDNode::Node { node, children } => {
                    let children: Vec<_> = children
                        .iter()
                        .filter_map(|c| c.filter(filter).map(ItemIDNodeRef::from))
                        .collect();
                    if children.is_empty() {
                        None
                    } else {
                        Some(ItemIDNodeRef::Node { node, children })
                    }
                },
            }
        }
    }
}

fn string_match(needle: &str, haystack: &str) -> bool {
    let needle = needle.chars().flat_map(char::to_lowercase);
    let mut haystack = haystack.chars().flat_map(char::to_lowercase);

    'o: for c in needle {
        for d in &mut haystack {
            if c == d {
                continue 'o;
            }
        }
        return false;
    }
    true
}

const ISP_TAG: &str = "##item-spawn";
static ITEM_ID_TREE: Lazy<Vec<ItemIDNode>> =
    Lazy::new(|| serde_json::from_str(include_str!("item_ids.json")).unwrap());

#[derive(Debug)]
pub(crate) struct ItemSpawner<'a> {
    func_ptr: usize,
    map_item_man: usize,
    hotkey_load: Option<Key>,
    hotkey_close: Key,
    sentinel: Bitflag<u8>,

    label_load: String,
    label_close: String,

    qty: u32,
    item_id: u32,
    durability: u32,
    upgrade: usize,
    infusion_type: usize,

    filter_string: String,
    logs: Vec<String>,
    item_id_tree: Vec<ItemIDNodeRef<'a>>,
}

impl ItemSpawner<'_> {
    pub(crate) fn new(
        func_ptr: usize,
        map_item_man: usize,
        sentinel: Bitflag<u8>,
        hotkey_load: Option<Key>,
        hotkey_close: Key,
    ) -> Self {
        let label_load = if let Some(hotkey_load) = hotkey_load {
            format!("Spawn item ({hotkey_load})")
        } else {
            "Spawn item".to_string()
        };
        let label_close = format!("Close ({hotkey_close})");
        ItemSpawner {
            func_ptr,
            map_item_man,
            hotkey_load,
            hotkey_close,
            label_load,
            label_close,
            sentinel,
            qty: 1,
            durability: 100,
            item_id: DEFAULT_ITEM,
            upgrade: 0,
            infusion_type: 0,
            filter_string: String::new(),
            logs: Vec::new(),
            item_id_tree: ITEM_ID_TREE.iter().map(ItemIDNodeRef::from).collect(),
        }
    }

    fn spawn(&mut self) {
        if self.sentinel.get().is_none() {
            self.write_log("Not spawning item when not in game".into());
            return;
        }

        let upgrade = UPGRADES[self.upgrade].0;
        let infusion = INFUSION_TYPES[self.infusion_type].0;

        let i = ItemSpawnInstance {
            spawn_item_func_ptr: self.func_ptr as _,
            map_item_man: self.map_item_man as _,
            qty: self.qty,
            durability: self.durability,
            upgrade,
            infusion,
            item_id: self.item_id,
        };

        self.write_log(format!(
            "Spawning {} #{} {} {}",
            i.qty, self.item_id, UPGRADES[self.upgrade].1, INFUSION_TYPES[self.infusion_type].1,
        ));

        unsafe {
            i.spawn();
        }
    }

    fn write_log(&mut self, log: String) {
        self.logs.push(log);
    }
}

impl Widget for ItemSpawner<'_> {
    fn render(&mut self, ui: &imgui::Ui) {
        let scale = scaling_factor(ui);
        let button_width = BUTTON_WIDTH * scale;
        let button_height = BUTTON_HEIGHT;

        let (x, y) = unsafe {
            let mut wnd_pos = ImVec2::default();
            igGetWindowPos(&mut wnd_pos);
            (igGetCursorPosX() + wnd_pos.x, igGetCursorPosY() + wnd_pos.y)
        };

        if ui.button_with_size(&self.label_load, [button_width, button_height]) {
            ui.open_popup(ISP_TAG);
        }

        unsafe {
            igSetNextWindowPos(
                ImVec2::new(x + 200. * scale, y),
                Condition::Always as i8 as _,
                ImVec2::new(0., 0.),
            )
        };

        if let Some(_token) = ui
            .modal_popup_config(ISP_TAG)
            .resizable(false)
            .movable(false)
            .title_bar(false)
            .scroll_bar(false)
            .begin_popup()
        {
            let button_height = button_height * scale;

            {
                let _tok = ui.push_item_width(-1.);
                if InputText::new(ui, "##item-spawn-filter", &mut self.filter_string)
                    .hint("Filter...")
                    .build()
                {
                    self.item_id_tree =
                        ITEM_ID_TREE.iter().filter_map(|n| n.filter(&self.filter_string)).collect();
                }
            }
            ui.child_window("##item-spawn-list").size([400., 200.]).build(|| {
                for node in &self.item_id_tree {
                    node.render(ui, &mut self.item_id, !self.filter_string.is_empty());
                }
            });

            ui.set_next_item_width(195.);
            ui.combo(
                "##item-spawn-infusion-type",
                &mut self.infusion_type,
                &INFUSION_TYPES,
                |(_, label)| Cow::Borrowed(label),
            );

            ui.same_line();
            ui.set_next_item_width(195.);
            ui.combo("##item-spawn-upgrade", &mut self.upgrade, &UPGRADES, |(_, label)| {
                Cow::Borrowed(label)
            });

            ui.slider_config("Qty", 1, 99).build(&mut self.qty);
            ui.slider_config("Dur", 0, 9999).build(&mut self.durability);
            if ui.button_with_size(&self.label_load, [400., button_height]) {
                self.spawn();
            }

            if ui.button_with_size("Clear", [400., button_height]) {
                self.filter_string.clear();
                self.qty = 1;
                self.durability = 100;
                self.item_id = DEFAULT_ITEM;
                self.upgrade = 0;
                self.infusion_type = 0;
                self.item_id_tree = ITEM_ID_TREE.iter().map(ItemIDNodeRef::from).collect();
            }

            if ui.button_with_size(&self.label_close, [400., button_height])
                || (self.hotkey_close.is_pressed(ui) && !ui.is_any_item_active())
            {
                ui.close_current_popup();
            }
        }
    }

    fn interact(&mut self, ui: &imgui::Ui) {
        if ui.is_any_item_active() || ui.io().want_capture_keyboard {
            return;
        }

        if self.hotkey_load.map(|k| k.is_pressed(ui)).unwrap_or(false) {
            self.spawn();
        }
    }

    fn log(&mut self, tx: Sender<String>) {
        for x in self.logs.drain(..) {
            tx.send(x).ok();
        }
    }
}

#[derive(Debug)]
struct HexU32(u32);

impl Display for HexU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl<'de> Deserialize<'de> for HexU32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HexVisitor;

        impl<'de> Visitor<'de> for HexVisitor {
            type Value = HexU32;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a 4-byte (8 chars) hexadecimal string")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() != 8 {
                    return Err(E::custom(format!("Invalid hex string length {}: {}", v.len(), v)));
                }

                let mut bytes = [0u8; 4];
                hex::decode_to_slice(v, &mut bytes[..])
                    .map_err(|e| E::custom(format!("Hex decode error for {}: {}", v, e)))?;
                Ok(HexU32(u32::from_be_bytes(bytes)))
            }
        }

        deserializer.deserialize_any(HexVisitor)
    }
}

#[derive(Debug)]
struct ItemSpawnInstance {
    spawn_item_func_ptr: u64,
    map_item_man: u64,
    qty: u32,
    durability: u32,
    item_id: u32,
    infusion: u32,
    upgrade: u32,
}

impl Display for ItemSpawnInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x} (qty={}, dur={}, infusion={}, upgrade={})",
            self.item_id, self.qty, self.durability, self.infusion, self.upgrade
        )
    }
}

impl ItemSpawnInstance {
    unsafe fn spawn(&self) {
        #[repr(C)]
        struct SpawnRequest {
            unknown: u32,
            item_id: u32,
            qty: u32,
            durability: u32,
        }

        type SpawnItemFn = extern "system" fn(*const c_void, *mut SpawnRequest, *mut [u32; 4]);
        let spawn_fn_ptr = std::mem::transmute::<_, SpawnItemFn>(self.spawn_item_func_ptr);
        let pp_map_item_man = self.map_item_man as *const *const c_void;

        let qty = self.qty;
        let durability = self.durability;
        let item_id = self.item_id + self.infusion + self.upgrade;

        let mut spawn_request = SpawnRequest { qty, item_id, durability, unknown: 1 };

        spawn_fn_ptr(*pp_map_item_man, &mut spawn_request as *mut _, &mut [0u32; 4] as *mut _);
    }
}
