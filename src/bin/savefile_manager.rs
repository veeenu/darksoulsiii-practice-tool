#![windows_subsystem = "windows"]
use libjdsd_dsiii_practice_tool::utils::imgui_loop;

use chrono::prelude::*;
use imgui::*;
use winapi::um::winuser::GetAsyncKeyState;

const TITLE: &str = "Practice Tool Savefiles Manager";

mod database {
  use std::io::Write;
  use std::path::PathBuf;

  use rusqlite::{params, Connection, DatabaseName};

  pub(super) struct Database {
    savefile_path: PathBuf,
    database_path: PathBuf,
  }

  impl Database {
    pub(super) fn new() -> Result<Database, String> {
      let re = regex::Regex::new(r"^[a-f0-9]+$").unwrap();
      let savefile_path: PathBuf = [std::env::var("APPDATA").unwrap().as_str(), "DarkSoulsIII"]
        .iter()
        .collect();
      let savefile_path = std::fs::read_dir(&savefile_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .find(|e| re.is_match(&e.file_name().to_string_lossy()) && e.path().is_dir())
        .map(|e| e.path())
        .map(|mut e| {
          e.push("DS30000.sl2");
          e
        })
        .map(PathBuf::from)
        .ok_or_else(|| "Could not find savefile position".to_string())?;

      Ok(Database {
        savefile_path: savefile_path,
        database_path: PathBuf::from("./savefiles.db"),
      })
    }

    pub(super) fn save(&self) -> Result<(), String> {
      let savefile_content = std::fs::read(&self.savefile_path)
        .map_err(|e| format!("Could not read savefile: {}", e))?;
      let conn = Connection::open(&self.database_path)
        .map_err(|e| format!("Could not open savefile database: {}", e))?;
      conn
        .execute(
          r"CREATE TABLE IF NOT EXISTS savefiles (data BLOB)",
          params![],
        )
        .map_err(|e| format!("Savefile database error, CREATE: {}", e))?;
      conn
        .execute(
          r"INSERT INTO savefiles (data) VALUES (ZEROBLOB(?))",
          params![savefile_content.len() as isize],
        )
        .map_err(|e| format!("Savefile database error, INSERT: {}", e))?;

      let rowid = conn.last_insert_rowid();
      let mut blob = conn
        .blob_open(DatabaseName::Main, "savefiles", "data", rowid, false)
        .map_err(|e| format!("Savefile database error, blob open: {}", e))?;
      let bytes_written = blob
        .write(&savefile_content)
        .map_err(|e| format!("Savefile database error, blob write: {}", e))?;
      if bytes_written != savefile_content.len() {
        Err(format!(
          "Savefile database error, blob write result: written {} bytes instead of {}",
          bytes_written,
          savefile_content.len()
        ))
      } else {
        Ok(())
      }
    }

    pub(super) fn load(&self) -> Result<(), String> {
      let conn = rusqlite::Connection::open(&self.database_path)
        .map_err(|e| format!("Could not open savefile database: {}", e))?;
      let rowid = conn
        .query_row(r"SELECT MAX(rowid) FROM savefiles", params![], |r| r.get(0))
        .map_err(|e| format!("Savefile database error, SELECT: {}", e))?;
      let blob = conn
        .blob_open(DatabaseName::Main, "savefiles", "data", rowid, false)
        .map_err(|e| format!("Savefile database error, blob open: {}", e))?;
      let mut savefile_content = vec![0u8; blob.size() as usize];
      blob
        .read_at(&mut savefile_content, 0)
        .map_err(|e| format!("Savefile database error, blob read: {}", e))?;

      std::fs::write(&self.savefile_path, &savefile_content)
        .map_err(|e| format!("Could not write savefile: {}", e))?;

      Ok(())
    }
  }
}

fn main() {
  let db = database::Database::new();

  let mut was_save_pressed: bool = false;
  let mut was_load_pressed: bool = false;
  let mut events: Vec<ImString> = Vec::new();

  match db {
    Ok(db) => imgui_loop(TITLE, move |_, ui, display| {
      let (width, height) = {
        let s = display.gl_window().window().inner_size();
        (s.width as f32, s.height as f32)
      };

      let stack_token = ui.push_style_vars({
        &[
          StyleVar::WindowRounding(0.),
          StyleVar::FrameBorderSize(0.),
          StyleVar::WindowBorderSize(0.),
        ]
      });

      let save_key_state = unsafe { GetAsyncKeyState('5' as _) & 0x01 } != 0;
      let load_key_state = unsafe { GetAsyncKeyState('9' as _) & 0x01 } != 0;

      if save_key_state && !was_save_pressed {
        match db.save() {
          Ok(_) => {
            let dt = Local::now();
            events.push(ImString::from(format!(
              "{} State saved",
              dt.format("%T").to_string()
            )));
          }
          Err(e) => {
            let dt = Local::now();
            events.push(ImString::from(format!(
              "{} {}",
              dt.format("%T").to_string(),
              e
            )));
          }
        }
      }

      if load_key_state && !was_load_pressed {
        match db.load() {
          Ok(_) => {
            let dt = Local::now();
            events.push(ImString::from(format!(
              "{} State loaded",
              dt.format("%T").to_string()
            )));
          }
          Err(e) => {
            let dt = Local::now();
            events.push(ImString::from(format!(
              "{} {}",
              dt.format("%T").to_string(),
              e
            )));
          }
        }
      }

      was_save_pressed = save_key_state;
      was_load_pressed = load_key_state;

      Window::new(im_str!("window1"))
        .position([0., 0.], Condition::Always)
        .size([width, height], Condition::Always)
        .flags({
          WindowFlags::NO_DECORATION
            | WindowFlags::NO_COLLAPSE
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
        })
        .build(ui, || {
          let messages = events.iter().rev().take(50);
          ui.text(im_str!("Save: 5 / Load: 9"));
          ui.separator();

          let tok = ui.push_item_width(-1.);
          for label in messages {
            ui.text(&label);
          }
          tok.pop(ui);
        });

      stack_token.pop(ui);
    }),
    Err(e) => imgui_loop(TITLE, move |_, ui, display| {
      let (width, height) = {
        let s = display.gl_window().window().inner_size();
        (s.width as f32, s.height as f32)
      };

      ui.text(im_str!("{}", e));
    }),
  };
}
