use std::str::FromStr;

use libds3::prelude::*;
use serde::Deserialize;
use tracing_subscriber::filter::LevelFilter;

use crate::util;
use crate::util::KeyState;
use crate::widgets::character_stats::CharacterStatsEdit;
use crate::widgets::cycle_speed::CycleSpeed;
use crate::widgets::flag::Flag;
use crate::widgets::group::Group;
use crate::widgets::item_spawn::ItemSpawner;
use crate::widgets::nudge_pos::NudgePosition;
use crate::widgets::open_menu::{OpenMenu, OpenMenuKind};
use crate::widgets::position::SavePosition;
use crate::widgets::quitout::Quitout;
use crate::widgets::savefile_manager::SavefileManager;
use crate::widgets::souls::Souls;
use crate::widgets::target::Target;
use crate::widgets::Widget;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) settings: Settings,
    commands: Vec<CfgCommand>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub(crate) log_level: LevelFilterSerde,
    pub(crate) display: KeyState,
    #[serde(default)]
    pub(crate) show_console: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CfgCommand {
    SavefileManager {
        #[serde(rename = "savefile_manager")]
        hotkey_load: KeyState,
        hotkey_open: Option<KeyState>,
    },
    ItemSpawner {
        #[serde(rename = "item_spawner")]
        hotkey_load: KeyState,
    },
    Flag {
        flag: FlagSpec,
        hotkey: Option<KeyState>,
    },
    Position {
        #[serde(rename = "position")]
        hotkey: KeyState,
        modifier: KeyState,
    },
    CycleSpeed {
        #[serde(rename = "cycle_speed")]
        cycle_speed: Vec<f32>,
        hotkey: KeyState,
    },
    CharacterStats {
        #[serde(rename = "character_stats")]
        hotkey_open: KeyState,
    },
    Souls {
        #[serde(rename = "souls")]
        amount: u32,
        hotkey: KeyState,
    },
    OpenMenu {
        #[serde(rename = "open_menu")]
        kind: OpenMenuKind,
        hotkey: Option<KeyState>,
    },
    Quitout {
        #[serde(rename = "quitout")]
        hotkey: KeyState,
    },
    Target {
        #[serde(rename = "target")]
        hotkey: KeyState,
    },
    NudgePosition {
        nudge: f32,
        nudge_up: KeyState,
        nudge_down: KeyState,
    },
    Group {
        #[serde(rename = "group")]
        label: String,
        commands: Vec<CfgCommand>,
    },
}

#[derive(Deserialize, Debug)]
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

    fn make_commands_inner(
        commands: &[CfgCommand],
        settings: &Settings,
        chains: &PointerChains,
    ) -> Vec<Box<dyn Widget>> {
        commands
            .iter()
            .map(|cmd| match cmd {
                CfgCommand::Flag { flag, hotkey } => {
                    Box::new(Flag::new(&flag.label, (flag.getter)(chains).clone(), *hotkey))
                        as Box<dyn Widget>
                },
                CfgCommand::SavefileManager { hotkey_load, hotkey_open } => {
                    SavefileManager::new_widget(*hotkey_load, *hotkey_open, settings.display)
                },
                CfgCommand::ItemSpawner { hotkey_load } => Box::new(ItemSpawner::new(
                    chains.spawn_item_func_ptr as usize,
                    chains.map_item_man as usize,
                    chains.gravity.clone(),
                    *hotkey_load,
                    settings.display,
                )),
                CfgCommand::Position { hotkey, modifier } => {
                    Box::new(SavePosition::new(chains.position.clone(), *hotkey, *modifier))
                },
                CfgCommand::NudgePosition { nudge, nudge_up, nudge_down } => Box::new(
                    NudgePosition::new(chains.position.clone().1, *nudge, *nudge_up, *nudge_down),
                ),
                CfgCommand::CharacterStats { hotkey_open } => Box::new(CharacterStatsEdit::new(
                    *hotkey_open,
                    settings.display,
                    chains.character_stats.clone(),
                )),
                CfgCommand::CycleSpeed { cycle_speed, hotkey } => {
                    Box::new(CycleSpeed::new(cycle_speed.as_slice(), chains.speed.clone(), *hotkey))
                },
                CfgCommand::Souls { amount, hotkey } => {
                    Box::new(Souls::new(*amount, chains.souls.clone(), *hotkey))
                },
                CfgCommand::Quitout { hotkey } => {
                    Box::new(Quitout::new(chains.quitout.clone(), *hotkey))
                },
                CfgCommand::OpenMenu { hotkey, kind } => {
                    Box::new(OpenMenu::new(*kind, chains.travel_ptr, chains.attune_ptr, *hotkey))
                },
                CfgCommand::Target { hotkey } => {
                    Box::new(Target::new(chains.current_target.clone(), chains.xa, *hotkey))
                },
                CfgCommand::Group { label, commands } => Box::new(Group::new(
                    label.as_str(),
                    settings.display,
                    Self::make_commands_inner(commands.as_slice(), settings, chains),
                )),
            })
            .collect()
    }

    pub(crate) fn make_commands(&self, chains: &PointerChains) -> Vec<Box<dyn Widget>> {
        Self::make_commands_inner(&self.commands, &self.settings, chains)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                log_level: LevelFilterSerde(LevelFilter::DEBUG),
                display: KeyState::new(util::get_key_code("0").unwrap(), None),
                show_console: false,
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
            "evt_disable" => Ok(FlagSpec::new("Event disable", |c| &c.evt_disable)),
            "ai_disable" => Ok(FlagSpec::new("AI disable", |c| &c.ai_disable)),
            "rend_chr" => Ok(FlagSpec::new("Render characters", |c| &c.rend_chr)),
            "rend_obj" => Ok(FlagSpec::new("Render objects", |c| &c.rend_obj)),
            "rend_map" => Ok(FlagSpec::new("Render map", |c| &c.rend_map)),
            "rend_mesh_hi" => Ok(FlagSpec::new("Collision mesh hi", |c| &c.rend_mesh_hi)),
            "rend_mesh_lo" => Ok(FlagSpec::new("Collision mesh lo", |c| &c.rend_mesh_lo)),
            "all_draw_hit" => Ok(FlagSpec::new("All draw hit", |c| &c.all_draw_hit)),
            "ik_foot_ray" => Ok(FlagSpec::new("IK foot ray", |c| &c.ik_foot_ray)),
            "debug_sphere_1" => Ok(FlagSpec::new("Debug sphere 1", |c| &c.debug_sphere_1)),
            "debug_sphere_2" => Ok(FlagSpec::new("Debug sphere 2", |c| &c.debug_sphere_2)),
            "gravity" => Ok(FlagSpec::new("Gravity", |c| &c.gravity)),
            e => Err(format!("\"{}\" is not a valid flag specifier", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_parse() {
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
