use std::path::PathBuf;
use std::str::FromStr;

use libds3::prelude::*;
use practice_tool_core::controller::ControllerCombination;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::Widget;
use practice_tool_memedit::widgets::flag_widget;
use serde::Deserialize;
use tracing_subscriber::filter::LevelFilter;

use crate::util;
use crate::widgets::character_stats::character_stats_edit;
use crate::widgets::cycle_color::cycle_color;
use crate::widgets::cycle_speed::cycle_speed;
use crate::widgets::group::group;
use crate::widgets::input_viewer::InputViewer;
use crate::widgets::item_spawn::ItemSpawner;
use crate::widgets::label::label_widget;
use crate::widgets::nudge_pos::nudge_position;
use crate::widgets::open_menu::{open_menu, OpenMenuKind};
use crate::widgets::position::save_position;
use crate::widgets::quitout::quitout;
use crate::widgets::savefile_manager::savefile_manager;
use crate::widgets::souls::souls;
use crate::widgets::target::Target;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) settings: Settings,
    #[serde(rename = "radial-menu")]
    pub(crate) radial_menu: Vec<RadialMenu>,
    commands: Vec<CfgCommand>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Settings {
    pub(crate) log_level: LevelFilterSerde,
    pub(crate) display: Key,
    pub(crate) hide: Option<Key>,
    #[serde(default)]
    pub(crate) show_console: bool,
    #[serde(default = "Indicator::default_set")]
    pub(crate) indicators: Vec<Indicator>,
    pub(crate) radial_menu_open: Option<ControllerCombination>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RadialMenu {
    // pub index: usize,
    pub key: Key,
    pub label: String,
}

impl AsRef<str> for RadialMenu {
    fn as_ref(&self) -> &str {
        &self.label
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub(crate) enum IndicatorType {
    Igt,
    Position,
    PositionChange,
    GameVersion,
    ImguiDebug,
    Fps,
    FrameCount,
    Animation,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "IndicatorConfig")]
pub(crate) struct Indicator {
    pub(crate) indicator: IndicatorType,
    pub(crate) enabled: bool,
}

/// Indicator specifiers, their types, and whether they're enabled by default.
pub(crate) const INDICATORS: &[(&str, IndicatorType, bool)] = &[
    ("game_version", IndicatorType::GameVersion, true),
    ("igt", IndicatorType::Igt, true),
    ("position", IndicatorType::Position, false),
    ("position_change", IndicatorType::PositionChange, false),
    ("animation", IndicatorType::Animation, false),
    ("fps", IndicatorType::Fps, false),
    ("framecount", IndicatorType::FrameCount, false),
    ("imgui_debug", IndicatorType::ImguiDebug, false),
];

impl Indicator {
    fn default_set() -> Vec<Indicator> {
        INDICATORS.iter().map(|&(_, indicator, enabled)| Indicator { indicator, enabled }).collect()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct IndicatorConfig {
    indicator: String,
    enabled: bool,
}

impl TryFrom<IndicatorConfig> for Indicator {
    type Error = String;

    fn try_from(indicator: IndicatorConfig) -> Result<Self, Self::Error> {
        INDICATORS
            .iter()
            .find(|(id, ..)| *id == indicator.indicator)
            .map(|&(_, kind, _)| Indicator { indicator: kind, enabled: indicator.enabled })
            .ok_or_else(|| format!("Unrecognized indicator: {}", indicator.indicator))
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum PlaceholderOption<T> {
    Data(T),
    #[allow(dead_code)]
    Placeholder(bool),
}

impl<T> PlaceholderOption<T> {
    fn into_option(self) -> Option<T> {
        match self {
            PlaceholderOption::Data(d) => Some(d),
            PlaceholderOption::Placeholder(_) => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CfgCommand {
    SavefileManager {
        #[serde(rename = "savefile_manager")]
        hotkey_load: PlaceholderOption<Key>,
    },
    ItemSpawner {
        #[serde(rename = "item_spawner")]
        hotkey_load: PlaceholderOption<Key>,
    },
    Flag {
        flag: FlagSpec,
        hotkey: Option<Key>,
    },
    Label {
        #[serde(rename = "label")]
        label: String,
    },
    Position {
        position: PlaceholderOption<Key>,
        save: Option<Key>,
    },
    CycleSpeed {
        #[serde(rename = "cycle_speed")]
        values: Vec<f32>,
        hotkey: Option<Key>,
    },
    CycleColor {
        #[serde(rename = "cycle_color")]
        values: Vec<i32>,
        hotkey: Option<Key>,
    },
    CharacterStats {
        #[serde(rename = "character_stats")]
        value: PlaceholderOption<Key>,
    },
    Souls {
        #[serde(rename = "souls")]
        amount: u32,
        hotkey: Option<Key>,
    },
    OpenMenu {
        #[serde(rename = "open_menu")]
        kind: OpenMenuKind,
        hotkey: Option<Key>,
    },
    Quitout {
        #[serde(rename = "quitout")]
        hotkey: PlaceholderOption<Key>,
    },
    Target {
        #[serde(rename = "target")]
        hotkey: PlaceholderOption<Key>,
    },
    InputViewer {
        #[serde(rename = "input_viewer")]
        hotkey: PlaceholderOption<Key>,
        #[serde(default = "default_input_viewer_seconds")]
        seconds: usize,
    },
    NudgePosition {
        nudge: f32,
        nudge_up: Option<Key>,
        nudge_down: Option<Key>,
    },
    Group {
        #[serde(rename = "group")]
        label: String,
        commands: Vec<CfgCommand>,
    },
}

impl CfgCommand {
    fn into_widget(self, settings: &Settings, chains: &'static PointerChains) -> Box<dyn Widget> {
        match self {
            CfgCommand::Flag { flag, hotkey: key } => {
                flag_widget(&flag.label, (flag.getter)(chains), key)
            },
            CfgCommand::Label { label } => label_widget(label.as_str()),
            CfgCommand::SavefileManager { hotkey_load: key_load } => {
                savefile_manager(key_load.into_option(), settings.display)
            },
            CfgCommand::ItemSpawner { hotkey_load: key_load } => Box::new(ItemSpawner::new(
                chains.spawn_item_func_ptr as usize,
                chains.map_item_man as usize,
                &chains.gravity,
                key_load.into_option(),
                settings.display,
            )),
            CfgCommand::Position { position, save } => {
                save_position(&chains.position, position.into_option(), save)
            },
            CfgCommand::NudgePosition { nudge, nudge_up, nudge_down } => {
                nudge_position(&chains.position, nudge, nudge_up, nudge_down)
            },
            CfgCommand::CharacterStats { value } => {
                character_stats_edit(&chains.character_stats, value.into_option(), settings.display)
            },
            CfgCommand::CycleSpeed { values, hotkey } => {
                cycle_speed(values.as_slice(), &chains.speed, hotkey)
            },
            CfgCommand::CycleColor { values, hotkey } => {
                cycle_color(values.as_slice(), &chains.mesh_color, hotkey)
            },
            CfgCommand::Souls { amount, hotkey } => souls(amount, &chains.souls, hotkey),
            CfgCommand::Quitout { hotkey } => quitout(&chains.quitout, hotkey.into_option()),
            CfgCommand::OpenMenu { hotkey, kind } => {
                open_menu(kind, chains.travel_ptr, chains.attune_ptr, hotkey)
            },
            CfgCommand::Target { hotkey } => {
                Box::new(Target::new(&chains.current_target, chains.xa, hotkey.into_option()))
            },
            CfgCommand::InputViewer { hotkey, seconds } => {
                Box::new(InputViewer::new(seconds, hotkey.into_option()))
            },
            CfgCommand::Group { label, commands } => group(
                label.as_str(),
                commands.into_iter().map(|c| c.into_widget(settings, chains)).collect(),
                settings.display,
            ),
        }
    }
}

fn default_input_viewer_seconds() -> usize {
    5
}

#[derive(Deserialize, Debug, Clone)]
#[serde(try_from = "String")]
pub(crate) struct LevelFilterSerde(LevelFilter);

impl LevelFilterSerde {
    pub(crate) fn inner(&self) -> LevelFilter {
        self.0
    }
}

impl TryFrom<String> for LevelFilterSerde {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(LevelFilterSerde(
            LevelFilter::from_str(&value)
                .map_err(|e| format!("Couldn't parse log level filter: {}", e))?,
        ))
    }
}

impl Config {
    pub(crate) fn parse(cfg: &str) -> Result<Self, String> {
        toml::from_str::<Config>(cfg).map_err(|e| format!("TOML configuration parse error: {}", e))
    }

    pub(crate) fn make_commands(self, chains: &'static PointerChains) -> Vec<Box<dyn Widget>> {
        self.commands.into_iter().map(|c| c.into_widget(&self.settings, chains)).collect()
    }
}

type FlagGetter = fn(&PointerChains) -> &Bitflag<u8>;

#[derive(Deserialize)]
#[serde(try_from = "String")]
struct FlagSpec {
    label: String,
    getter: FlagGetter,
}

impl std::fmt::Debug for FlagSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlagSpec {{ label: {:?} }}", self.label)
    }
}

impl FlagSpec {
    fn new(label: &str, getter: FlagGetter) -> FlagSpec {
        FlagSpec { label: label.to_string(), getter }
    }
}

/// Valid flag specifiers, their labels, and the pointer chains they toggle.
#[rustfmt::skip]
pub(crate) const FLAGS: &[(&str, &str, FlagGetter)] = &[
    ("all_no_damage", "All no damage", |c| &c.all_no_damage),
    ("inf_stamina", "Inf Stamina", |c| &c.inf_stamina),
    ("inf_focus", "Inf Focus", |c| &c.inf_focus),
    ("inf_consumables", "Inf Consumables", |c| &c.inf_consumables),
    ("deathcam", "Deathcam", |c| &c.deathcam),
    ("no_death", "No death", |c| &c.no_death),
    ("one_shot", "One shot", |c| &c.one_shot),
    ("evt_draw", "Event draw", |c| &c.evt_draw),
    ("bloodstain_draw", "Stable/Bloodstain draw", |c| &c.bloodstain_draw),
    ("evt_disable", "Event disable", |c| &c.evt_disable),
    ("ai_disable", "AI disable", |c| &c.ai_disable),
    ("ember", "Ember", |c| &c.ember),
    ("rend_chr", "Render characters", |c| &c.rend_chr),
    ("rend_obj", "Render objects", |c| &c.rend_obj),
    ("rend_map", "Render map", |c| &c.rend_map),
    ("rend_mesh_hi", "Collision mesh hi", |c| &c.rend_mesh_hi),
    ("rend_mesh_lo", "Collision mesh lo", |c| &c.rend_mesh_lo),
    ("rend_mesh_hit", "Collision mesh hit", |c| &c.rend_mesh_hit),
    ("debug_draw", "Debug draw", |c| &c.debug_draw),
    ("hurtbox", "Hurtbox", |c| &c.rend_hurtbox),
    ("all_draw_hit", "All draw hit", |c| &c.all_draw_hit),
    ("ik_foot_ray", "IK foot ray", |c| &c.ik_foot_ray),
    ("debug_sphere_1", "Debug sphere 1", |c| &c.debug_sphere_1),
    ("debug_sphere_2", "Debug sphere 2", |c| &c.debug_sphere_2),
    ("gravity", "No Gravity", |c| &c.gravity),
    ("collision", "No Collision", |c| &c.collision),
];

impl TryFrom<String> for FlagSpec {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        FLAGS
            .iter()
            .find(|(id, ..)| *id == value)
            .map(|&(_, label, getter)| FlagSpec::new(label, getter))
            .ok_or_else(|| format!("\"{}\" is not a valid flag specifier", value))
    }
}

/// The bundled configuration, written next to the DLL when the file is missing.
pub(crate) const DEFAULT_CONFIG: &str = include_str!("../../jdsd_dsiii_practice_tool.toml");

/// Path of the configuration file, next to the DLL.
pub(crate) fn config_path() -> Option<PathBuf> {
    util::get_dll_path().map(|mut path| {
        path.pop();
        path.push("jdsd_dsiii_practice_tool.toml");
        path
    })
}

#[cfg(test)]
mod tests {
    use super::{Config, DEFAULT_CONFIG};

    #[test]
    fn test_parse_ok() {
        println!("{:#?}", toml::from_str::<toml::Value>(DEFAULT_CONFIG));
        println!("{:#?}", Config::parse(DEFAULT_CONFIG));
    }

    #[test]
    fn test_parse_errors() {
        println!(
            "{:#?}",
            Config::parse(
                r#"commands = [ { boh = 3 } ]
                [settings]
                log_level = "DEBUG"
                "#
            )
        );
    }
}
