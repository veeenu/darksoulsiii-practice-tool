use std::str::FromStr;

use log::LevelFilter;
use serde::Deserialize;

use crate::{memedit::*, util};
use crate::pointers::PointerChains;
use crate::util::KeyState;

#[derive(Debug, Deserialize)]
struct Config {
    settings: Settings,
    commands: Vec<CfgCommand>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    #[serde(deserialize_with = "deserialize_level_filter")]
    log_level: LevelFilter,
    display: KeyState,
    down: KeyState,
    up: KeyState,
    left: KeyState,
    right: KeyState,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CfgCommand {
    SavefileManager {
        #[serde(rename = "savefile_manager_hotkey")]
        hotkey: KeyState,
    },
    Flag {
        flag: FlagSpec,
        hotkey: KeyState,
    },
    Position {
        #[serde(rename = "position_hotkey")]
        hotkey: KeyState,
    },
    CycleSpeed {
        cycle_speed: Vec<f32>,
        hotkey: KeyState,
    },
    Souls {
        souls: u32,
        hotkey: KeyState,
    },
    Quitout {
        #[serde(rename = "quitout")]
        hotkey: KeyState,
    },
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                log_level: LevelFilter::Debug,
                display: KeyState::new(util::get_key_code("0").unwrap()),
                down: KeyState::new(util::get_key_code("down").unwrap()),
                up: KeyState::new(util::get_key_code("up").unwrap()),
                left: KeyState::new(util::get_key_code("left").unwrap()),
                right: KeyState::new(util::get_key_code("right").unwrap()),
            },
            commands: Vec::new()
        }
    }
}

struct FlagSpec {
    label: &'static str,
    getter: fn(&PointerChains) -> &Bitflag<u8>,
}

impl std::fmt::Debug for FlagSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlagSpec {{ label: {:?} }}", self.label)
    }
}

impl FlagSpec {
    fn new(label: &'static str, getter: fn(&PointerChains) -> &Bitflag<u8>) -> FlagSpec {
        FlagSpec { label, getter }
    }
}

impl<'de> Deserialize<'de> for FlagSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

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
            "rend_mesh_hi" => Ok(FlagSpec::new("Collision mesh (hi)", |c| &c.rend_mesh_hi)),
            "rend_mesh_lo" => Ok(FlagSpec::new("Collision mesh (lo)", |c| &c.rend_mesh_lo)),
            "all_draw_hit" => Ok(FlagSpec::new("All draw hit", |c| &c.all_draw_hit)),
            "ik_foot_ray" => Ok(FlagSpec::new("IK foot ray", |c| &c.ik_foot_ray)),
            "debug_sphere_1" => Ok(FlagSpec::new("Debug sphere 1", |c| &c.debug_sphere_1)),
            "debug_sphere_2" => Ok(FlagSpec::new("Debug sphere 2", |c| &c.debug_sphere_2)),
            "gravity" => Ok(FlagSpec::new("Gravity", |c| &c.gravity)),
            e => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(e),
                &"is not a valid flag specifier",
            )),
        }
    }
}

fn deserialize_level_filter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    LevelFilter::from_str(&value).map_err(|_| {
        serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&value),
            &"is not a valid level filter",
        )
    })
}

impl Config {
    fn parse(cfg: &str) -> Result<Config, String> {
        toml::from_str(cfg).map_err(|e| format!("TOML configuration parse error: {:?}", e))?
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_parse() {
        println!(
            "{:#?}",
            Config::parse(include_str!("../../../jdsd_dsiii_practice_tool.toml"))
        );
    }

    // TODO tests with errors
}
