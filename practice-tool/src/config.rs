use std::str::FromStr;

use log::LevelFilter;
use serde::Deserialize;

use crate::memedit::*;
use crate::pointers::PointerChains;
use crate::util;
use crate::util::KeyState;
use crate::widgets::cycle_speed::CycleSpeed;
use crate::widgets::flag::Flag;
use crate::widgets::item_spawn::ItemSpawner;
use crate::widgets::position::SavePosition;
use crate::widgets::quitout::Quitout;
use crate::widgets::savefile_manager::SavefileManager;
use crate::widgets::souls::Souls;
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
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CfgCommand {
    SavefileManager {
        #[serde(rename = "savefile_manager")]
        hotkey_load: KeyState,
        hotkey_back: KeyState,
        hotkey_close: KeyState,
    },
    ItemSpawner {
        #[serde(rename = "item_spawner")]
        hotkey_load: KeyState,
        hotkey_back: KeyState,
        hotkey_close: KeyState,
    },
    Flag {
        flag: FlagSpec,
        hotkey: KeyState,
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
    Souls {
        #[serde(rename = "souls")]
        amount: u32,
        hotkey: KeyState,
    },
    Quitout {
        #[serde(rename = "quitout")]
        hotkey: KeyState,
    },
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub(crate) struct LevelFilterSerde(log::LevelFilter);

impl LevelFilterSerde {
    pub(crate) fn inner(&self) -> log::LevelFilter {
        self.0
    }
}

impl TryFrom<String> for LevelFilterSerde {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(LevelFilterSerde(
            log::LevelFilter::from_str(&value)
                .map_err(|e| format!("Couldn't parse log level filter: {}", e))?,
        ))
    }
}

impl Config {
    pub(crate) fn parse(cfg: &str) -> Result<Self, String> {
        toml::from_str::<Config>(cfg).map_err(|e| format!("TOML configuration parse error: {}", e))
    }

    pub(crate) fn make_commands(&self, chains: &PointerChains) -> Vec<Box<dyn Widget>> {
        self.commands
            .iter()
            .map(|cmd| match cmd {
                CfgCommand::Flag { flag, hotkey } => Box::new(Flag::new(
                    &flag.label,
                    (flag.getter)(chains).clone(),
                    hotkey.clone(),
                )) as Box<dyn Widget>,
                CfgCommand::SavefileManager {
                    hotkey_load,
                    hotkey_back,
                    hotkey_close,
                } => SavefileManager::new_widget(
                    hotkey_load.clone(),
                    hotkey_back.clone(),
                    hotkey_close.clone(),
                ),
                CfgCommand::ItemSpawner {
                    hotkey_load,
                    hotkey_back,
                    hotkey_close,
                } => Box::new(ItemSpawner::new(
                    chains.spawn_item_func_ptr,
                    chains.map_item_man,
                    chains.gravity.clone(),
                    hotkey_load.clone(),
                    hotkey_back.clone(),
                    hotkey_close.clone(),
                )),
                CfgCommand::Position { hotkey, modifier } => Box::new(SavePosition::new(
                    chains.position.clone(),
                    hotkey.clone(),
                    modifier.clone(),
                )),
                CfgCommand::CycleSpeed {
                    cycle_speed,
                    hotkey,
                } => Box::new(CycleSpeed::new(
                    cycle_speed,
                    chains.speed.clone(),
                    hotkey.clone(),
                )),
                CfgCommand::Souls { amount, hotkey } => {
                    Box::new(Souls::new(*amount, chains.souls.clone(), hotkey.clone()))
                }
                CfgCommand::Quitout { hotkey } => {
                    Box::new(Quitout::new(chains.quitout.clone(), hotkey.clone()))
                }
            })
            .collect()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                log_level: LevelFilterSerde(LevelFilter::Debug),
                display: KeyState::new(util::get_key_code("0").unwrap()),
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
        FlagSpec {
            label: label.to_string(),
            getter,
        }
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
        println!(
            "{:#?}",
            Config::parse(include_str!("../../jdsd_dsiii_practice_tool.toml"))
        );
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
