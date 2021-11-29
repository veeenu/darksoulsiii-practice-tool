use std::cmp::Ordering;
use std::path::PathBuf;
use std::sync::Arc;

use imgui::{ComboBox, ListBox, Selectable};
use parking_lot::Mutex;

use crate::util::{get_key_code, KeyState};

use super::Widget;

#[derive(Debug)]
pub(crate) struct SavefileManager {
    label: String,
    hotkey: KeyState,
    inner: Arc<Mutex<Box<dyn Widget>>>,
}

impl SavefileManager {
    pub(crate) fn new(hotkey: KeyState) -> Self {
        let inner = match SavefileManagerInner::new(hotkey.clone()) {
            Ok(i) => Arc::new(Mutex::new(Box::new(i) as _)),
            Err(i) => Arc::new(Mutex::new(Box::new(i) as _)),
        };

        SavefileManager {
            label: format!("Savefile manager ({})", hotkey),
            hotkey,
            inner,
        }
    }
}

impl Widget for SavefileManager {
    fn render(&self, ui: &imgui::Ui) {
        ui.text(&self.label);
    }

    fn interact(&mut self) {}

    fn enter(&self) -> Option<Arc<Mutex<Box<(dyn Widget + 'static)>>>> {
        Some(self.inner.clone())
    }
}

#[derive(Debug)]
pub(crate) struct ErroredSavefileManagerInner {
    error: String,
}

impl ErroredSavefileManagerInner {
    pub fn new(error: String) -> Self {
        ErroredSavefileManagerInner { error }
    }
}

impl Widget for ErroredSavefileManagerInner {
    fn render(&self, ui: &imgui::Ui) {
        ui.text(&self.error);
    }
}

#[derive(Debug)]
pub(crate) struct SavefileManagerInner {
    key_enter: KeyState,
    key_exit: KeyState,
    key_load: KeyState,
    dir_stack: DirStack,
    savefile_path: PathBuf,
}

impl SavefileManagerInner {
    fn new(key_load: KeyState) -> Result<Self, ErroredSavefileManagerInner> {
        let mut savefile_path = get_savefile_path().map_err(|e| {
            ErroredSavefileManagerInner::new(format!("Could not find savefile path: {}", e))
        })?;

        let dir_stack = DirStack::new(&savefile_path).map_err(|e| {
            ErroredSavefileManagerInner::new(format!("Couldn't construct file browser: {}", e))
        })?;

        savefile_path.push("DS30000.sl2");

        Ok(SavefileManagerInner {
            key_enter: KeyState::new(get_key_code("return").unwrap()),
            key_exit: KeyState::new(get_key_code("q").unwrap()),
            key_load,
            dir_stack,
            savefile_path,
        })
    }
}

impl Widget for SavefileManagerInner {
    fn render(&self, ui: &imgui::Ui) {
        const TAG: &'static str = "##savefile-manager";

        ui.text(format!("Enter directory: {}", self.key_enter));
        ui.text(format!("Exit directory:  {}", self.key_exit));
        ui.text(format!("Load savefile:   {}", self.key_load));

        ListBox::new(TAG).size([0f32, 100.]).build(ui, || {
            for (is_selected, i) in self.dir_stack.values() {
                Selectable::new(i).selected(is_selected).build(ui);
                if is_selected {
                    ui.set_scroll_here_y();
                }
            }
        });
    }

    fn interact(&mut self) {
        if self.key_enter.keyup() {
            self.dir_stack.enter();
        } else if self.key_exit.keyup() {
            self.dir_stack.exit();
        } else if self.key_load.keyup() {
            // TODO error checking and reporting, now just fails silently
            let src_path = self.dir_stack.current();
            println!("Current {:?}", src_path);
            if src_path.is_file() {
                println!("Loading savefile {:?}", self.savefile_path);
                load_savefile(src_path, &self.savefile_path).ok();
            }
        }
    }

    fn cursor_down(&mut self) {
        self.dir_stack.next();
    }

    fn cursor_up(&mut self) {
        self.dir_stack.prev();
    }
}

#[derive(Debug)]
struct DirEntry {
    list: Vec<(PathBuf, String)>,
    cursor: usize,
}

impl DirEntry {
    fn new(path: &PathBuf) -> DirEntry {
        let mut list = DirStack::ls(path).unwrap();

        list.sort_by(|a, b| {
            let (ad, bd) = (a.is_dir(), b.is_dir());

            if ad == bd {
                a.cmp(b)
            } else if ad && !bd {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let list = list
            .into_iter()
            .map(|a| {
                let repr = if a.is_dir() {
                    format!("+ {}", a.file_name().unwrap().to_str().unwrap())
                } else {
                    format!("  {}", a.file_name().unwrap().to_str().unwrap())
                };
                (a, repr)
            })
            .collect();

        DirEntry { list, cursor: 0 }
    }

    fn values(&self) -> impl IntoIterator<Item = (bool, &str)> {
        self.list
            .iter()
            .enumerate()
            .map(|(i, f)| (i == self.cursor, f.1.as_str()))
    }

    fn current(&self) -> &PathBuf {
        &self.list[self.cursor].0
    }

    fn next(&mut self) {
        self.cursor = usize::min(self.cursor + 1, self.list.len() - 1);
    }

    fn prev(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
}

#[derive(Debug)]
struct DirStack {
    stack: Vec<DirEntry>,
}

impl DirStack {
    fn new(path: &PathBuf) -> Result<Self, String> {
        let stack = vec![DirEntry::new(path)];

        Ok(DirStack { stack })
    }

    fn enter(&mut self) {
        let new_entry = {
            let current_entry = self.stack.last().unwrap().current();
            if current_entry.is_dir() {
                Some(DirEntry::new(&current_entry))
            } else {
                None
            }
        };

        if let Some(e) = new_entry {
            self.stack.push(e);
        }
    }

    fn exit(&mut self) {
        if self.stack.len() <= 1 {
            return;
        }

        self.stack.pop().unwrap();
    }

    fn values(&self) -> impl IntoIterator<Item = (bool, &str)> {
        self.stack.last().unwrap().values()
    }

    fn current(&self) -> &PathBuf {
        self.stack.last().unwrap().current()
    }

    fn next(&mut self) {
        self.stack.last_mut().unwrap().next();
    }

    fn prev(&mut self) {
        self.stack.last_mut().unwrap().prev();
    }

    // TODO SAFETY
    // FS errors would be permission denied (which shouldn't happen but should be reported)
    // and not a directory (which doesn't happen because we checked for is_dir).
    // For the moment, I just unwrap.
    fn ls(path: &PathBuf) -> Result<Vec<PathBuf>, String> {
        Ok(std::fs::read_dir(path)
            .map_err(|e| format!("{}", e))?
            .filter_map(Result::ok)
            .map(|f| f.path())
            .collect())
    }
}

fn get_savefile_path() -> Result<PathBuf, String> {
    let re = regex::Regex::new(r"^[a-f0-9]+$").unwrap();
    let savefile_path: PathBuf = [
        std::env::var("APPDATA")
            .map_err(|e| format!("{}", e))?
            .as_str(),
        "DarkSoulsIII",
    ]
    .iter()
    .collect();
    std::fs::read_dir(&savefile_path)
        .map_err(|e| format!("{}", e))?
        .filter_map(|e| e.ok())
        .find(|e| re.is_match(&e.file_name().to_string_lossy()) && e.path().is_dir())
        .map(|e| e.path())
        .map(PathBuf::from)
        .ok_or_else(|| String::from("Couldn't find savefile path"))
}

fn load_savefile(src: &PathBuf, dest: &PathBuf) -> Result<(), std::io::Error> {
    let buf = std::fs::read(src)?;
    std::fs::write(dest, &buf)?;
    Ok(())
}
