use std::{
  error::Error,
  fmt::Display,
  path::{Path, PathBuf},
};

use log::{error, info};
use walkdir::WalkDir;

use hudhook::RenderContext;

use super::Command;
use crate::imgui::{im_str, ComboBox, ComboBoxHeight, ImString, Selectable};

struct SavefileEntry(PathBuf, ImString);

pub(crate) struct SavefileManager {
  savefile_path: PathBuf,
  paths: Vec<SavefileEntry>,
  cur_path: usize,
  open: bool,
  hotkey: Option<i32>,
}

pub(crate) struct ErroredSavefileManager(String);

impl<T: Error + Display> From<T> for ErroredSavefileManager {
  fn from(t: T) -> Self {
    ErroredSavefileManager(format!("{}", t))
  }
}

impl SavefileManager {
  fn new_inner(hotkey: Option<i32>) -> Result<SavefileManager, ErroredSavefileManager> {
    let re = regex::Regex::new(r"^[a-f0-9]+$").unwrap();
    let savefile_path: PathBuf = [std::env::var("APPDATA")?.as_str(), "DarkSoulsIII"]
      .iter()
      .collect();
    let mut savefile_path = std::fs::read_dir(&savefile_path)?
      .filter_map(|e| e.ok())
      .find(|e| re.is_match(&e.file_name().to_string_lossy()) && e.path().is_dir())
      .map(|e| e.path())
      .map(PathBuf::from)
      .ok_or_else(|| ErroredSavefileManager(String::from("Couldn't find savefile position")))?;

    let paths = WalkDir::new(&savefile_path)
      .into_iter()
      .filter_map(|entry| {
        if let Ok(entry) = entry {
          if entry.path().is_file() {
            println!("{:?}", entry.path());
            let pathbuf = PathBuf::from(entry.path());
            let pathbuf_str = pathbuf
              .strip_prefix(&savefile_path)
              .ok()
              .and_then(Path::to_str)
              .map(ImString::new);

            if let Some(pathbuf_str) = pathbuf_str {
              Some(SavefileEntry(pathbuf, pathbuf_str))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect();

    savefile_path.push("DS30000.sl2");

    Ok(SavefileManager {
      savefile_path,
      paths,
      cur_path: 0,
      open: false,
      hotkey
    })
  }
  pub(crate) fn new(hotkey: Option<i32>) -> Box<dyn Command + Send + Sync> {
    match SavefileManager::new_inner(hotkey) {
      Ok(e) => Box::new(e),
      Err(e) => Box::new(e),
    }
  }
}

fn load_savefile(src: &Path, dest: &Path) -> Result<(), std::io::Error> {
  let buf = std::fs::read(src)?;
  std::fs::write(dest, &buf)?;
  Ok(())
}

impl Command for SavefileManager {
  fn display(&mut self, ctx: &RenderContext) -> bool {
    if self.paths.len() > 0 {
      let preview = &self.paths[self.cur_path].1;
      let combo = ComboBox::new(im_str!("##item_spawn"))
        .preview_value(preview)
        .height(ComboBoxHeight::Large);
      {
        let mut cur_path = None;
        combo.build(ctx.frame, || {
          for (idx, SavefileEntry(_, path_str)) in self.paths.iter().enumerate() {
            let is_selected = idx == self.cur_path;
            if Selectable::new(path_str)
              .selected(is_selected)
              .build(ctx.frame)
            {
              cur_path = Some(idx);
            }
          }
        });
        if let Some(cur_path) = cur_path {
          self.cur_path = cur_path;
        }
      }
      ctx.frame.same_line();
      if ctx.frame.button(im_str!("Load savefile")) {
        self.interact(ctx, true)
      }
    }

    false
  }

  fn interact(&mut self, ctx: &RenderContext, is_interacting: bool) {
    if self
      .hotkey
      .map(|k| ctx.frame.is_key_index_released(k as _))
      .unwrap_or(false)
      || is_interacting
    {
      info!(
        "Loading {:?} -> {:?}",
        &self.paths[self.cur_path].0, &self.savefile_path
      );
      if let Err(e) = load_savefile(&self.paths[self.cur_path].0, &self.savefile_path) {
        error!("Couldn't load savefile: {:?}", e);
      }
    }
  }

  fn is_valid(&self) -> bool {
    true
  }
}

impl Command for ErroredSavefileManager {
  fn display(&mut self, ctx: &RenderContext) -> bool {
    ctx
      .frame
      .text(format!("Savefile manager failed: {:?}", self.0));
    false
  }

  fn interact(&mut self, _: &RenderContext, _: bool) {}

  fn is_valid(&self) -> bool {
    true
  }
}
