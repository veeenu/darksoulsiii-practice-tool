use hudhook::*;

use std::collections::HashMap;
use std::str::FromStr;

use log::*;
use serde::{Deserialize, Serialize};
use winapi::um::winuser::*;

pub use crate::commands::CommandSettings;

#[derive(Debug, PartialEq)]
pub struct Config {
  pub settings: ConfigSettings,
  pub command: Vec<CommandSettings>,
}

#[derive(Debug, PartialEq)]
pub struct ConfigSettings {
  pub log_level: log::Level,
  pub display: i32,
  pub down: i32,
  pub up: i32,
  pub left: i32,
  pub right: i32,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      settings: ConfigSettings::default(),
      command: vec![],
    }
  }
}

impl Default for ConfigSettings {
  fn default() -> ConfigSettings {
    ConfigSettings {
      log_level: log::Level::Debug,
      display: '0' as _,
      down: VK_DOWN,
      up: VK_UP,
      left: VK_LEFT,
      right: VK_RIGHT,
    }
  }
}

impl Config {
  pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), String> {
    let toml = toml::to_string(&LocalConfig::from(self))
      .map_err(|e| format!("Could not serialize config: {}", e))?;
    std::fs::write(path, toml).map_err(|e| format!("Could not save config: {}.", e))
  }

  pub fn load_from_file(path: &std::path::Path) -> Result<Config, String> {
    std::fs::read_to_string(path)
      .map_err(|e| format!("Could not load config: {:?}. Using default.", e))
      .and_then(|s| Config::load(&s))
  }

  pub(crate) fn load(config_toml: &str) -> Result<Config, String> {
    match toml::from_str::<LocalConfig>(&config_toml) {
      Ok(conf) => Ok(Config::from(conf)),
      Err(e) => Err(format!("Could not parse config: {}. Using default.", e)),
    }
  }
}

pub(crate) fn get_symbol(hotkey: i32) -> Option<String> {
  VK_INV_SYMBOL_MAP.get(&hotkey).map(String::clone)
}

pub(crate) fn get_keycode(hotkey: &str) -> Option<i32> {
  VK_SYMBOL_MAP.get(hotkey).copied()
}

#[derive(Debug, Deserialize, Serialize)]
struct LocalConfig {
  settings: LocalConfigSettings,
  command: Vec<CommandSettings>,
}
#[derive(Debug, Deserialize, Serialize)]
struct LocalConfigSettings {
  log_level: String,
  display: Option<String>,
  down: Option<String>,
  up: Option<String>,
  left: Option<String>,
  right: Option<String>,
}

impl From<LocalConfig> for Config {
  fn from(local_conf: LocalConfig) -> Config {
    Config {
      settings: ConfigSettings::from(local_conf.settings),
      command: local_conf.command,
    }
  }
}

impl From<&Config> for LocalConfig {
  fn from(conf: &Config) -> LocalConfig {
    LocalConfig {
      settings: LocalConfigSettings::from(&conf.settings),
      command: conf.command.clone(),
    }
  }
}

impl From<LocalConfigSettings> for ConfigSettings {
  fn from(local_conf_settings: LocalConfigSettings) -> ConfigSettings {
    let log_level = match log::Level::from_str(&local_conf_settings.log_level) {
      Ok(log_level) => log_level,
      Err(e) => {
        error!("Could not parse log level: {:?}. Using default.", e);
        log::Level::Info
      }
    };

    fn symmap_or(item: Option<String>, default: i32) -> i32 {
      *item
        .map(|k| VK_SYMBOL_MAP.get(&k))
        .unwrap_or(Some(&default))
        .unwrap_or(&default)
    }

    ConfigSettings {
      log_level,
      display: symmap_or(local_conf_settings.display, '0' as _),
      down: symmap_or(local_conf_settings.down, VK_DOWN),
      up: symmap_or(local_conf_settings.up, VK_UP),
      left: symmap_or(local_conf_settings.left, VK_LEFT),
      right: symmap_or(local_conf_settings.right, VK_RIGHT),
    }
  }
}

impl From<&ConfigSettings> for LocalConfigSettings {
  fn from(conf_settings: &ConfigSettings) -> LocalConfigSettings {
    /*let log_level = match log::Level::from_str(&local_conf_settings.log_level) {
      Ok(log_level) => log_level,
      Err(e) => {
        error!("Could not parse log level: {:?}. Using default.", e);
        log::Level::Info
      }
    };*/

    fn symmap_or(item: i32, default: String) -> String {
      VK_INV_SYMBOL_MAP.get(&item).cloned().unwrap_or(default)
    }

    LocalConfigSettings {
      log_level: format!("{}", conf_settings.log_level),
      // interact: Some(symmap_or(conf_settings.interact, String::from("VK_SPACE"))),
      // capture: Some(symmap_or(conf_settings.capture, String::from("VK_F1"))),
      display: Some(symmap_or(conf_settings.display, String::from("0"))),
      down: Some(symmap_or(conf_settings.down, String::from("VK_DOWN"))),
      up: Some(symmap_or(conf_settings.up, String::from("VK_UP"))),
      left: Some(symmap_or(conf_settings.left, String::from("VK_LEFT"))),
      right: Some(symmap_or(conf_settings.right, String::from("VK_RIGHT"))),
    }
  }
}

/*

This map would contain default keymappings but I decided to try a different
approach: an interactive menu like Dark Souls' debug exe with optional
global shortcuts. It'll be up to the user to add these mappings if they want.

[
("load_pos", VK_F1),
("inf_stamina", VK_F2),
("inf_focus", VK_F3),
("inf_consum", VK_F4),
("no_damage", VK_F5),
("no_death", VK_F6),
("save_pos", VK_F7),
("deathcam", VK_F8),
("one_shot", VK_F9),
("no_gravity", VK_F10),
("show", VK_F11),
("quitout", 'P' as u8 as _),
("all_no_damage", '2' as u8 as _),
("incr_souls", '3' as u8 as _),
("cycle_speed", '4' as u8 as _),
("event_draw", '5' as u8 as _),
("event_disable", '6' as u8 as _),
("ai_disable", '7' as u8 as _),
("rend_chr", '8' as u8 as _),
("rend_map", '9' as u8 as _),
("rend_obj", '0' as u8 as _),
]
.iter()
.map(|&(k, v)| (String::from(k), v))
.collect(),
*/

// Now playing: Johnny Cash - Hurt
lazy_static::lazy_static! {
pub static ref VK_SYMBOL_MAP: HashMap<String, i32> = {
  use winapi::um::winuser::*;
  [
    ("VK_LBUTTON", VK_LBUTTON),
    ("VK_RBUTTON", VK_RBUTTON),
    ("VK_CANCEL", VK_CANCEL),
    ("VK_MBUTTON", VK_MBUTTON),
    ("VK_XBUTTON1", VK_XBUTTON1),
    ("VK_XBUTTON2", VK_XBUTTON2),
    ("VK_BACK", VK_BACK),
    ("VK_TAB", VK_TAB),
    ("VK_CLEAR", VK_CLEAR),
    ("VK_RETURN", VK_RETURN),
    ("VK_SHIFT", VK_SHIFT),
    ("VK_CONTROL", VK_CONTROL),
    ("VK_MENU", VK_MENU),
    ("VK_PAUSE", VK_PAUSE),
    ("VK_CAPITAL", VK_CAPITAL),
    ("VK_KANA", VK_KANA),
    ("VK_HANGUL", VK_HANGUL),
    ("VK_JUNJA", VK_JUNJA),
    ("VK_FINAL", VK_FINAL),
    ("VK_HANJA", VK_HANJA),
    ("VK_KANJI", VK_KANJI),
    ("VK_ESCAPE", VK_ESCAPE),
    ("VK_CONVERT", VK_CONVERT),
    ("VK_NONCONVERT", VK_NONCONVERT),
    ("VK_ACCEPT", VK_ACCEPT),
    ("VK_MODECHANGE", VK_MODECHANGE),
    ("VK_SPACE", VK_SPACE),
    ("VK_PRIOR", VK_PRIOR),
    ("VK_NEXT", VK_NEXT),
    ("VK_END", VK_END),
    ("VK_HOME", VK_HOME),
    ("VK_LEFT", VK_LEFT),
    ("VK_UP", VK_UP),
    ("VK_RIGHT", VK_RIGHT),
    ("VK_DOWN", VK_DOWN),
    ("VK_SELECT", VK_SELECT),
    ("VK_PRINT", VK_PRINT),
    ("VK_EXECUTE", VK_EXECUTE),
    ("VK_SNAPSHOT", VK_SNAPSHOT),
    ("VK_INSERT", VK_INSERT),
    ("VK_DELETE", VK_DELETE),
    ("VK_HELP", VK_HELP),
    ("0", '0' as i32),
    ("1", '1' as i32),
    ("2", '2' as i32),
    ("3", '3' as i32),
    ("4", '4' as i32),
    ("5", '5' as i32),
    ("6", '6' as i32),
    ("7", '7' as i32),
    ("8", '8' as i32),
    ("9", '9' as i32),
    ("A", 'A' as i32),
    ("B", 'B' as i32),
    ("C", 'C' as i32),
    ("D", 'D' as i32),
    ("E", 'E' as i32),
    ("F", 'F' as i32),
    ("G", 'G' as i32),
    ("H", 'H' as i32),
    ("I", 'I' as i32),
    ("J", 'J' as i32),
    ("K", 'K' as i32),
    ("L", 'L' as i32),
    ("M", 'M' as i32),
    ("N", 'N' as i32),
    ("O", 'O' as i32),
    ("P", 'P' as i32),
    ("Q", 'Q' as i32),
    ("R", 'R' as i32),
    ("S", 'S' as i32),
    ("T", 'T' as i32),
    ("U", 'U' as i32),
    ("V", 'V' as i32),
    ("W", 'W' as i32),
    ("X", 'X' as i32),
    ("Y", 'Y' as i32),
    ("Z", 'Z' as i32),
    ("VK_LWIN", VK_LWIN),
    ("VK_RWIN", VK_RWIN),
    ("VK_APPS", VK_APPS),
    ("VK_SLEEP", VK_SLEEP),
    ("VK_NUMPAD0", VK_NUMPAD0),
    ("VK_NUMPAD1", VK_NUMPAD1),
    ("VK_NUMPAD2", VK_NUMPAD2),
    ("VK_NUMPAD3", VK_NUMPAD3),
    ("VK_NUMPAD4", VK_NUMPAD4),
    ("VK_NUMPAD5", VK_NUMPAD5),
    ("VK_NUMPAD6", VK_NUMPAD6),
    ("VK_NUMPAD7", VK_NUMPAD7),
    ("VK_NUMPAD8", VK_NUMPAD8),
    ("VK_NUMPAD9", VK_NUMPAD9),
    ("VK_MULTIPLY", VK_MULTIPLY),
    ("VK_ADD", VK_ADD),
    ("VK_SEPARATOR", VK_SEPARATOR),
    ("VK_SUBTRACT", VK_SUBTRACT),
    ("VK_DECIMAL", VK_DECIMAL),
    ("VK_DIVIDE", VK_DIVIDE),
    ("VK_F1", VK_F1),
    ("VK_F2", VK_F2),
    ("VK_F3", VK_F3),
    ("VK_F4", VK_F4),
    ("VK_F5", VK_F5),
    ("VK_F6", VK_F6),
    ("VK_F7", VK_F7),
    ("VK_F8", VK_F8),
    ("VK_F9", VK_F9),
    ("VK_F10", VK_F10),
    ("VK_F11", VK_F11),
    ("VK_F12", VK_F12),
    ("VK_F13", VK_F13),
    ("VK_F14", VK_F14),
    ("VK_F15", VK_F15),
    ("VK_F16", VK_F16),
    ("VK_F17", VK_F17),
    ("VK_F18", VK_F18),
    ("VK_F19", VK_F19),
    ("VK_F20", VK_F20),
    ("VK_F21", VK_F21),
    ("VK_F22", VK_F22),
    ("VK_F23", VK_F23),
    ("VK_F24", VK_F24),
    ("VK_NUMLOCK", VK_NUMLOCK),
    ("VK_SCROLL", VK_SCROLL),
    ("VK_LSHIFT", VK_LSHIFT),
    ("VK_RSHIFT", VK_RSHIFT),
    ("VK_LCONTROL", VK_LCONTROL),
    ("VK_RCONTROL", VK_RCONTROL),
    ("VK_LMENU", VK_LMENU),
    ("VK_RMENU", VK_RMENU),
    ("VK_BROWSER_BACK", VK_BROWSER_BACK),
    ("VK_BROWSER_FORWARD", VK_BROWSER_FORWARD),
    ("VK_BROWSER_REFRESH", VK_BROWSER_REFRESH),
    ("VK_BROWSER_STOP", VK_BROWSER_STOP),
    ("VK_BROWSER_SEARCH", VK_BROWSER_SEARCH),
    ("VK_BROWSER_FAVORITES", VK_BROWSER_FAVORITES),
    ("VK_BROWSER_HOME", VK_BROWSER_HOME),
    ("VK_VOLUME_MUTE", VK_VOLUME_MUTE),
    ("VK_VOLUME_DOWN", VK_VOLUME_DOWN),
    ("VK_VOLUME_UP", VK_VOLUME_UP),
    ("VK_MEDIA_NEXT_TRACK", VK_MEDIA_NEXT_TRACK),
    ("VK_MEDIA_PREV_TRACK", VK_MEDIA_PREV_TRACK),
    ("VK_MEDIA_STOP", VK_MEDIA_STOP),
    ("VK_MEDIA_PLAY_PAUSE", VK_MEDIA_PLAY_PAUSE),
    ("VK_LAUNCH_MAIL", VK_LAUNCH_MAIL),
    ("VK_LAUNCH_MEDIA_SELECT", VK_LAUNCH_MEDIA_SELECT),
    ("VK_LAUNCH_APP1", VK_LAUNCH_APP1),
    ("VK_LAUNCH_APP2", VK_LAUNCH_APP2),
    ("VK_OEM_1", VK_OEM_1),
    ("VK_OEM_PLUS", VK_OEM_PLUS),
    ("VK_OEM_COMMA", VK_OEM_COMMA),
    ("VK_OEM_MINUS", VK_OEM_MINUS),
    ("VK_OEM_PERIOD", VK_OEM_PERIOD),
    ("VK_OEM_2", VK_OEM_2),
    ("VK_OEM_3", VK_OEM_3),
    ("VK_OEM_4", VK_OEM_4),
    ("VK_OEM_5", VK_OEM_5),
    ("VK_OEM_6", VK_OEM_6),
    ("VK_OEM_7", VK_OEM_7),
    ("VK_OEM_8", VK_OEM_8),
    ("VK_OEM_102", VK_OEM_102),
    ("VK_PROCESSKEY", VK_PROCESSKEY),
    ("VK_PACKET", VK_PACKET),
    ("VK_ATTN", VK_ATTN),
    ("VK_CRSEL", VK_CRSEL),
    ("VK_EXSEL", VK_EXSEL),
    ("VK_EREOF", VK_EREOF),
    ("VK_PLAY", VK_PLAY),
    ("VK_ZOOM", VK_ZOOM),
    ("VK_NONAME", VK_NONAME),
    ("VK_PA1", VK_PA1),
    ("VK_OEM_CLEAR", VK_OEM_CLEAR),
    ]
      .iter()
      .map(|&(k, v)| (String::from(k), v))
      .collect()
};

pub static ref VK_INV_SYMBOL_MAP: HashMap<i32, String> = VK_SYMBOL_MAP
  .iter()
  .map(|(k, &v)| (v, k.clone()))
  .collect();
 }

/*#[cfg(test)]
mod tests {
  // TODO
  // Config deserialization needs some in depth testing.

  use super::*;
  use hudhook::winapi::um::winuser::*;

  #[test]
  fn test_config_load() {
    let cfg = Config::load(
      r#"
      [mappings]
      quitout = "P"
      show = "VK_OEM_MINUS"
      [settings]
      log_level = "debug"
    "#,
    );

    assert_eq!(
      cfg,
      Config {
        mappings: [("quitout", 'P' as u8 as _), ("show", VK_OEM_MINUS),]
          .iter()
          .map(|&(k, v)| (String::from(k), v))
          .collect(),
        settings: ConfigSettings {
          log_level: log::Level::Debug
        }
      }
    );
  }
}*/
