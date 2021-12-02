use std::cmp::Ordering;
use std::path::PathBuf;

use imgui::{ChildWindow, Condition, ListBox, PopupModal, Selectable, WindowFlags};

use crate::util::KeyState;

use super::Widget;

const SFM_TAG: &'static str = "##savefile-manager";

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
    fn render(&mut self, ui: &imgui::Ui) {
        ui.text(&self.error);
    }
}

#[derive(Debug)]
pub(crate) struct SavefileManager {
    label: String,
    key_back: KeyState,
    key_close: KeyState,
    key_load: KeyState,
    dir_stack: DirStack,
    savefile_path: PathBuf,
    breadcrumbs: String,
    log: Option<String>,
}

impl SavefileManager {
    pub(crate) fn new(
        key_load: KeyState,
        key_back: KeyState,
        key_close: KeyState,
    ) -> Box<dyn Widget> {
        match SavefileManager::new_inner(key_load, key_back, key_close) {
            Ok(i) => Box::new(i) as _,
            Err(i) => Box::new(i) as _,
        }
    }

    fn new_inner(
        key_load: KeyState,
        key_back: KeyState,
        key_close: KeyState,
    ) -> Result<Self, ErroredSavefileManagerInner> {
        let label = format!("Savefiles (load with {})", key_load);
        let mut savefile_path = get_savefile_path().map_err(|e| {
            ErroredSavefileManagerInner::new(format!("Could not find savefile path: {}", e))
        })?;

        let dir_stack = DirStack::new(&savefile_path).map_err(|e| {
            ErroredSavefileManagerInner::new(format!("Couldn't construct file browser: {}", e))
        })?;

        savefile_path.push("DS30000.sl2");

        Ok(SavefileManager {
            label,
            key_back,
            key_close,
            key_load,
            dir_stack,
            savefile_path,
            log: None,
            breadcrumbs: " /".to_string(),
        })
    }

    fn load_savefile(&mut self) {
        let src_path = self.dir_stack.current();
        if src_path.is_file() {
            self.log = match load_savefile(src_path, &self.savefile_path) {
                Ok(()) => Some(format!(
                    "Loaded {} / {}",
                    self.breadcrumbs,
                    src_path.file_name().unwrap().to_str().unwrap()
                )),
                Err(e) => Some(format!("Error loading savefile: {}", e)),
            };
        }
    }
}

impl Widget for SavefileManager {
    fn render(&mut self, ui: &imgui::Ui) {
        if ui.button_with_size(&self.label, [super::BUTTON_WIDTH, super::BUTTON_HEIGHT]) {
            ui.open_popup(SFM_TAG);
        }
        let [cx, cy] = ui.cursor_pos();
        let [wx, wy] = ui.window_pos();
        let [x, y] = [cx + wx, cy + wy];
        unsafe {
            imgui_sys::igSetNextWindowPos(
                imgui_sys::ImVec2 { x, y },
                Condition::Always as _,
                imgui_sys::ImVec2 { x: 0., y: 0. },
            )
        };

        let style_tokens =
            [ui.push_style_color(imgui::StyleColor::ModalWindowDimBg, [0., 0., 0., 0.])];

        if let Some(_token) = PopupModal::new(SFM_TAG)
            .flags(
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE,
            )
            .begin_popup(ui)
        {
            ChildWindow::new("##savefile-manager-breadcrumbs")
                .size([240., 14.])
                .build(ui, || {
                    ui.text(&self.breadcrumbs);
                    ui.set_scroll_x(ui.scroll_max_x());
                });

            ListBox::new(SFM_TAG).size([240., 100.]).build(ui, || {
                if Selectable::new(format!(".. Up one dir ({})", self.key_back)).build(ui) {
                    self.dir_stack.exit();
                }

                let mut goto: Option<usize> = None;
                for (idx, is_selected, i) in self.dir_stack.values() {
                    if Selectable::new(i).selected(is_selected).build(ui) {
                        goto = Some(idx);
                    }
                }

                if let Some(idx) = goto {
                    self.dir_stack.goto(idx);
                    self.dir_stack.enter();
                    self.breadcrumbs = self.dir_stack.breadcrumbs();
                }
            });

            if ui.button_with_size(format!("Load savefile ({})", self.key_load), [240., 20.]) {
                self.load_savefile();
            }

            if ui.button_with_size(format!("Close ({})", self.key_close), [240., 20.])
                || self.key_close.keyup()
            {
                ui.close_current_popup();
            }
        }

        style_tokens.into_iter().rev().for_each(|t| t.pop());
    }

    fn interact(&mut self) {
        if self.key_back.keyup() {
            self.dir_stack.exit();
            self.breadcrumbs = self.dir_stack.breadcrumbs();
        } else if self.key_load.keyup() {
            self.load_savefile();
        }
    }

    fn log(&mut self) -> Option<Vec<String>> {
        let log_entry = self.log.take();
        log_entry.map(|e| vec![e])
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
                    format!("+  {}", a.file_name().unwrap().to_str().unwrap())
                } else {
                    format!("   {}", a.file_name().unwrap().to_str().unwrap())
                };
                (a, repr)
            })
            .collect();

        DirEntry { list, cursor: 0 }
    }

    fn values(&self) -> impl IntoIterator<Item = (usize, bool, &str)> {
        self.list
            .iter()
            .enumerate()
            .map(|(i, f)| (i, i == self.cursor, f.1.as_str()))
    }

    fn current(&self) -> &PathBuf {
        &self.list[self.cursor].0
    }

    fn goto(&mut self, idx: usize) {
        if idx < self.list.len() {
            self.cursor = idx;
        }
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

    fn exit(&mut self) -> bool {
        if self.stack.len() <= 1 {
            true
        } else {
            self.stack.pop().unwrap();
            false
        }
    }

    fn breadcrumbs(&self) -> String {
        if self.stack.len() == 1 {
            String::from(" / ")
        } else {
            let mut breadcrumbs = String::new();
            for e in self.stack[..self.stack.len() - 1].iter() {
                breadcrumbs.extend(" / ".chars());
                breadcrumbs.extend(e.current().file_name().unwrap().to_str().unwrap().chars());
            }
            breadcrumbs
        }
    }

    fn values(&self) -> impl IntoIterator<Item = (usize, bool, &str)> {
        self.stack.last().unwrap().values()
    }

    fn current(&self) -> &PathBuf {
        self.stack.last().unwrap().current()
    }

    fn goto(&mut self, idx: usize) {
        self.stack.last_mut().unwrap().goto(idx);
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
