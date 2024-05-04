use std::str::FromStr;

use libds3::prelude::*;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::Widget;
use serde::Deserialize;
use tracing_subscriber::filter::LevelFilter;

use crate::widgets::character_stats::character_stats_edit;
use crate::widgets::cycle_speed::cycle_speed;
use crate::widgets::flag::flag_widget;
use crate::widgets::group::group;
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
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) enum IndicatorType {
    Igt,
    Position,
    GameVersion,
    ImguiDebug,
    Fps,
    FrameCount,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "IndicatorConfig")]
pub(crate) struct Indicator {
    pub(crate) indicator: IndicatorType,
    pub(crate) enabled: bool,
}

impl Indicator {
    fn default_set() -> Vec<Indicator> {
        vec![
            Indicator { indicator: IndicatorType::GameVersion, enabled: true },
            Indicator { indicator: IndicatorType::Igt, enabled: true },
            Indicator { indicator: IndicatorType::Position, enabled: false },
            Indicator { indicator: IndicatorType::Fps, enabled: false },
            Indicator { indicator: IndicatorType::FrameCount, enabled: false },
            Indicator { indicator: IndicatorType::ImguiDebug, enabled: false },
        ]
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
        match indicator.indicator.as_str() {
            "igt" => Ok(Indicator { indicator: IndicatorType::Igt, enabled: indicator.enabled }),
            "position" => {
                Ok(Indicator { indicator: IndicatorType::Position, enabled: indicator.enabled })
            },
            "game_version" => {
                Ok(Indicator { indicator: IndicatorType::GameVersion, enabled: indicator.enabled })
            },
            "imgui_debug" => {
                Ok(Indicator { indicator: IndicatorType::ImguiDebug, enabled: indicator.enabled })
            },
            "fps" => Ok(Indicator { indicator: IndicatorType::Fps, enabled: indicator.enabled }),
            "framecount" => {
                Ok(Indicator { indicator: IndicatorType::FrameCount, enabled: indicator.enabled })
            },
            value => Err(format!("Unrecognized indicator: {value}")),
        }
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
    fn into_widget(self, settings: &Settings, chains: &PointerChains) -> Box<dyn Widget> {
        match self {
            CfgCommand::Flag { flag, hotkey: key } => {
                flag_widget(&flag.label, (flag.getter)(chains).clone(), key)
            },
            CfgCommand::Label { label } => label_widget(label.as_str()),
            CfgCommand::SavefileManager { hotkey_load: key_load } => {
                savefile_manager(key_load.into_option(), settings.display)
            },
            CfgCommand::ItemSpawner { hotkey_load: key_load } => Box::new(ItemSpawner::new(
                chains.spawn_item_func_ptr as usize,
                chains.map_item_man as usize,
                chains.gravity.clone(),
                key_load.into_option(),
                settings.display,
            )),
            CfgCommand::Position { position, save } => {
                save_position(chains.position.clone(), position.into_option(), save)
            },
            CfgCommand::NudgePosition { nudge, nudge_up, nudge_down } => {
                nudge_position(chains.position.clone(), nudge, nudge_up, nudge_down)
            },
            CfgCommand::CharacterStats { value } => character_stats_edit(
                chains.character_stats.clone(),
                value.into_option(),
                settings.display,
            ),
            CfgCommand::CycleSpeed { values, hotkey } => {
                cycle_speed(values.as_slice(), chains.speed.clone(), hotkey)
            },
            CfgCommand::Souls { amount, hotkey } => souls(amount, chains.souls.clone(), hotkey),
            CfgCommand::Quitout { hotkey } => quitout(chains.quitout.clone(), hotkey.into_option()),
            CfgCommand::OpenMenu { hotkey, kind } => {
                open_menu(kind, chains.travel_ptr, chains.attune_ptr, hotkey)
            },
            CfgCommand::Target { hotkey } => Box::new(Target::new(
                chains.current_target.clone(),
                chains.xa,
                hotkey.into_option(),
            )),
            CfgCommand::Group { label, commands } => group(
                label.as_str(),
                commands.into_iter().map(|c| c.into_widget(settings, chains)).collect(),
                settings.display,
            ),
        }
    }
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

    pub(crate) fn make_commands(self, chains: &PointerChains) -> Vec<Box<dyn Widget>> {
        self.commands.into_iter().map(|c| c.into_widget(&self.settings, chains)).collect()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                log_level: LevelFilterSerde(LevelFilter::DEBUG),
                display: "0".parse().unwrap(),
                hide: "rshift+0".parse().ok(),
                show_console: false,
                indicators: Indicator::default_set(),
            },
            commands: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
struct FlagSpec {
    label: String,
    getter: fn(&PointerChains) -> &Bitflag<u8>,
}

impl std::fmt::Debug for FlagSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlagSpec {{ label: {:?} }}", self.label)
    }
}

impl FlagSpec {
    fn new(label: &str, getter: fn(&PointerChains) -> &Bitflag<u8>) -> FlagSpec {
        FlagSpec { label: label.to_string(), getter }
    }
}

impl TryFrom<String> for FlagSpec {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "all_no_damage" => Ok(FlagSpec::new("All no damage", |c| &c.all_no_damage)),
            "inf_stamina" => Ok(FlagSpec::new("Inf Stamina", |c| &c.inf_stamina)),
            "inf_focus" => Ok(FlagSpec::new("Inf Focus", |c| &c.inf_focus)),
            "inf_consumables" => Ok(FlagSpec::new("Inf Consumables", |c| &c.inf_consumables)),
            "deathcam" => Ok(FlagSpec::new("Deathcam", |c| &c.deathcam)),
            "no_death" => Ok(FlagSpec::new("No death", |c| &c.no_death)),
            "one_shot" => Ok(FlagSpec::new("One shot", |c| &c.one_shot)),
            "evt_draw" => Ok(FlagSpec::new("Event draw", |c| &c.evt_draw)),
            "bloodstain_draw" => {
                Ok(FlagSpec::new("Stable/Bloodstain draw", |c| &c.bloodstain_draw))
            },
            "evt_disable" => Ok(FlagSpec::new("Event disable", |c| &c.evt_disable)),
            "ai_disable" => Ok(FlagSpec::new("AI disable", |c| &c.ai_disable)),
            "ember" => Ok(FlagSpec::new("Ember", |c| &c.ember)),
            "rend_chr" => Ok(FlagSpec::new("Render characters", |c| &c.rend_chr)),
            "rend_obj" => Ok(FlagSpec::new("Render objects", |c| &c.rend_obj)),
            "rend_map" => Ok(FlagSpec::new("Render map", |c| &c.rend_map)),
            "rend_mesh_hi" => Ok(FlagSpec::new("Collision mesh hi", |c| &c.rend_mesh_hi)),
            "rend_mesh_lo" => Ok(FlagSpec::new("Collision mesh lo", |c| &c.rend_mesh_lo)),
            "rend_mesh_hit" => Ok(FlagSpec::new("Collision mesh hit", |c| &c.rend_mesh_hit)),
            "debug_draw" => Ok(FlagSpec::new("Debug draw", |c| &c.debug_draw)),
            "hurtbox" => Ok(FlagSpec::new("Hurtbox", |c| &c.rend_hurtbox)),
            "all_draw_hit" => Ok(FlagSpec::new("All draw hit", |c| &c.all_draw_hit)),
            "ik_foot_ray" => Ok(FlagSpec::new("IK foot ray", |c| &c.ik_foot_ray)),
            "debug_sphere_1" => Ok(FlagSpec::new("Debug sphere 1", |c| &c.debug_sphere_1)),
            "debug_sphere_2" => Ok(FlagSpec::new("Debug sphere 2", |c| &c.debug_sphere_2)),
            "gravity" => Ok(FlagSpec::new("No Gravity", |c| &c.gravity)),
            e => Err(format!("\"{}\" is not a valid flag specifier", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_parse_ok() {
        println!(
            "{:#?}",
            toml::from_str::<toml::Value>(include_str!("../../jdsd_dsiii_practice_tool.toml"))
        );
        println!("{:#?}", Config::parse(include_str!("../../jdsd_dsiii_practice_tool.toml")));
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
