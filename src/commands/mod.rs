mod flag;
mod item_ids;
mod item_spawn;
mod position;
mod quitout;
mod savefiles;
mod souls;
mod speed;

use std::path::PathBuf;



use log::*;
use serde::{self, Deserialize, Serialize};

use crate::Context;
use crate::config::get_keycode;
use crate::memory::PointerChains;

pub(crate) use flag::*;
pub(crate) use position::*;
pub(crate) use quitout::*;
pub(crate) use savefiles::*;
pub(crate) use souls::*;
pub(crate) use speed::*;

// const BUTTON_WIDTH: f32 = 128.;
// const BUTTON_HEIGHT: f32 = 18.;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "cmd")]
pub enum CommandSettings {
  #[serde(rename = "toggle")]
  Toggle { flag: String, hotkey: String },
  #[serde(rename = "quitout")]
  Quitout { hotkey: String },
  #[serde(rename = "position")]
  Position {
    hotkey_save: String,
    hotkey_load: String,
  },
  #[serde(rename = "souls")]
  Souls { quantity: i32, hotkey: String },
  #[serde(rename = "cycle_speed")]
  CycleSpeed {
    values: Option<Vec<f32>>,
    hotkey: String,
  },
  #[serde(rename = "item_spawn")]
  SpawnItem {
    item_id: i64,
    infusion: Option<i64>,
    upgrade: Option<i64>,
    hotkey: String,
  },
  #[serde(rename = "savefile_manager")]
  SavefileManager { path: Option<PathBuf>, hotkey: String },
}

impl std::fmt::Display for CommandSettings {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CommandSettings::Toggle { .. } => write!(f, "Toggle"),
      CommandSettings::Quitout { .. } => write!(f, "Quitout"),
      CommandSettings::Position { .. } => write!(f, "Position"),
      CommandSettings::Souls { .. } => write!(f, "Souls"),
      CommandSettings::CycleSpeed { .. } => write!(f, "Cycle Speed"),
      CommandSettings::SpawnItem { .. } => write!(f, "Spawn Item"),
      CommandSettings::SavefileManager { .. } => write!(f, "Savefile manager"),
    }
  }
}

pub(crate) trait Command {
  fn display(&mut self, ctx: &Context) -> bool;
  fn interact(&mut self, ctx: &Context, is_interacting: bool);
  fn is_valid(&self) -> bool;
}

impl CommandSettings {
  pub(crate) fn try_to_command(
    &self,
    pc: &PointerChains,
  ) -> Option<Box<dyn Command + Send + Sync>> {
    info!("{:#?}", self);
    match self {
      CommandSettings::SpawnItem {
        item_id,
        infusion,
        upgrade,
        hotkey,
      } => {
        if cfg!(item_spawn) {
          pub(crate) use item_spawn::*;
          Some(Box::new(ItemSpawn::new(
            "Item Spawn",
            *item_id as _,
            infusion.unwrap_or(0) as _,
            upgrade.unwrap_or(0) as _,
            1,
            0xffffffff,
            pc.item_spawn.0,
            pc.item_spawn.1 as _,
            pc.item_spawn.2 as _,
            get_keycode(hotkey),
          )))
        } else {
          None
        }
      }
      CommandSettings::Position {
        hotkey_save,
        hotkey_load,
      } => Some(Box::new(PositionPointer::new(
        pc.position.0.clone(),
        pc.position.1.clone(),
        pc.position.2.clone(),
        pc.position.3.clone(),
        get_keycode(hotkey_load),
        get_keycode(hotkey_save),
      ))),
      CommandSettings::Quitout { hotkey } => Some(Box::new(QuitoutPointer::new(
        pc.quitout.clone(),
        get_keycode(hotkey),
      ))),
      CommandSettings::Souls { quantity, hotkey } => Some(Box::new(SoulsPointer::new(
        pc.souls.clone(),
        *quantity,
        get_keycode(hotkey),
      ))),
      CommandSettings::CycleSpeed { values, hotkey } => Some(Box::new(
        CycleSpeedPointer::new(pc.speed.clone(), get_keycode(hotkey), values.clone()),
      )),
      CommandSettings::SavefileManager { hotkey, .. } => Some(
        SavefileManager::new(get_keycode(hotkey))
      ),
      CommandSettings::Toggle { flag, hotkey } => match flag.as_str() {
        "all_no_damage" => Some(Box::new(FlagPointer::new(
          "All no damage",
          pc.all_no_damage.0.clone(),
          pc.all_no_damage.1,
          get_keycode(&hotkey),
        ))),
        "no_death" => Some(Box::new(FlagPointer::new(
          "No death",
          pc.no_death.0.clone(),
          pc.no_death.1,
          get_keycode(&hotkey),
        ))),
        "one_shot" => Some(Box::new(FlagPointer::new(
          "One shot",
          pc.one_shot.0.clone(),
          pc.one_shot.1,
          get_keycode(&hotkey),
        ))),
        "inf_stamina" => Some(Box::new(FlagPointer::new(
          "Inf stamina",
          pc.inf_stamina.0.clone(),
          pc.inf_stamina.1,
          get_keycode(&hotkey),
        ))),
        "inf_focus" => Some(Box::new(FlagPointer::new(
          "Inf focus",
          pc.inf_focus.0.clone(),
          pc.inf_focus.1,
          get_keycode(&hotkey),
        ))),
        "inf_consumables" => Some(Box::new(FlagPointer::new(
          "Inf consumables",
          pc.inf_consumables.0.clone(),
          pc.inf_consumables.1,
          get_keycode(&hotkey),
        ))),
        "deathcam" => Some(Box::new(FlagPointer::new(
          "Deathcam",
          pc.deathcam.0.clone(),
          pc.deathcam.1,
          get_keycode(&hotkey),
        ))),
        "evt_draw" => Some(Box::new(FlagPointer::new(
          "Event draw",
          pc.evt_draw.0.clone(),
          pc.evt_draw.1,
          get_keycode(&hotkey),
        ))),
        "evt_disable" => Some(Box::new(FlagPointer::new(
          "Event disable",
          pc.evt_disable.0.clone(),
          pc.evt_disable.1,
          get_keycode(&hotkey),
        ))),
        "ai_disable" => Some(Box::new(FlagPointer::new(
          "AI disable",
          pc.ai_disable.0.clone(),
          pc.ai_disable.1,
          get_keycode(&hotkey),
        ))),
        "rend_chr" => Some(Box::new(FlagPointer::new(
          "Render character",
          pc.rend_chr.0.clone(),
          pc.rend_chr.1,
          get_keycode(&hotkey),
        ))),
        "rend_obj" => Some(Box::new(FlagPointer::new(
          "Render objects",
          pc.rend_obj.0.clone(),
          pc.rend_obj.1,
          get_keycode(&hotkey),
        ))),
        "rend_map" => Some(Box::new(FlagPointer::new(
          "Render map",
          pc.rend_map.0.clone(),
          pc.rend_map.1,
          get_keycode(&hotkey),
        ))),
        "rend_mesh_hi" => Some(Box::new(FlagPointer::new(
          "Render mesh (high)",
          pc.rend_mesh_hi.0.clone(),
          pc.rend_mesh_hi.1,
          get_keycode(&hotkey),
        ))),
        "rend_mesh_lo" => Some(Box::new(FlagPointer::new(
          "Render mesh (low)",
          pc.rend_mesh_lo.0.clone(),
          pc.rend_mesh_lo.1,
          get_keycode(&hotkey),
        ))),
        "all_draw_hit" => Some(Box::new(FlagPointer::new(
          "Draw hits",
          pc.all_draw_hit.clone(),
          0,
          get_keycode(&hotkey),
        ))),
        "ik_foot_ray" => Some(Box::new(FlagPointer::new(
          "IK foot ray",
          pc.ik_foot_ray.clone(),
          0,
          get_keycode(&hotkey),
        ))),
        "debug_sphere_1" => Some(Box::new(FlagPointer::new(
          "Hitboxes (#1)",
          pc.debug_sphere_1.clone(),
          0,
          get_keycode(&hotkey),
        ))),
        "debug_sphere_2" => Some(Box::new(FlagPointer::new(
          "Hitboxes (#2)",
          pc.debug_sphere_2.clone(),
          0,
          get_keycode(&hotkey),
        ))),
        "gravity" => Some(Box::new(FlagPointer::new(
          "Gravity",
          pc.gravity.0.clone(),
          pc.gravity.1,
          get_keycode(&hotkey),
        ))),
        other => {
          error!("Unrecognized flag: {}", other);
          None
        }
      },
    }
  }
}
