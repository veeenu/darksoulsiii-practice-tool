use std::cmp::Ordering;
use std::path::PathBuf;
use std::sync::Arc;

use imgui::{ChildWindow, Condition, ListBox, PopupModal, Selectable, WindowFlags};
use parking_lot::Mutex;

use crate::style;
use crate::util::{get_key_code, GlobalKeys, KeyState};

use super::Widget;

const SFM_TAG: &'static str = "##savefile-manager";

#[derive(Debug)]
pub(crate) struct SavefileManager {
    label: String,
    inner: Arc<Mutex<Box<dyn Widget>>>,
    next_pos: Arc<Mutex<[f32; 2]>>,
}

impl SavefileManager {
    pub(crate) fn new(hotkey: KeyState) -> Self {
        let label = format!("Savefile manager ({})", hotkey);
        let next_pos = Arc::new(Mutex::new([0f32; 2]));
        let inner = match SavefileManagerInner::new(hotkey, next_pos.clone()) {
            Ok(i) => Arc::new(Mutex::new(Box::new(i) as _)),
            Err(i) => Arc::new(Mutex::new(Box::new(i) as _)),
        };

        SavefileManager {
            label,
            inner,
            next_pos,
        }
    }
}

impl Widget for SavefileManager {
    fn render(&mut self, ui: &imgui::Ui) {
        ui.button(&self.label);
        let [cx, cy] = ui.cursor_pos();
        let [wx, wy] = ui.window_pos();
        *self.next_pos.lock() = [cx + wx, cy + wy];
    }

    fn interact(&mut self) {
        self.inner.lock().interact();
    }

    fn interact_ui(&mut self) {
        self.inner.lock().interact_ui();
    }

    fn enter(&self, ui: &imgui::Ui) -> Option<Arc<Mutex<Box<(dyn Widget + 'static)>>>> {
        ui.open_popup(SFM_TAG);

        Some(self.inner.clone())
    }

    fn log(&mut self) -> Option<Vec<String>> {
        self.inner.lock().log()
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
    fn render(&mut self, ui: &imgui::Ui) {
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
    breadcrumbs: String,
    next_pos: Arc<Mutex<[f32; 2]>>,
    want_exit: bool,
    log: Option<String>,
}

impl SavefileManagerInner {
    fn new(
        key_load: KeyState,
        next_pos: Arc<Mutex<[f32; 2]>>,
    ) -> Result<Self, ErroredSavefileManagerInner> {
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
            next_pos,
            want_exit: false,
            log: None,
            breadcrumbs: " /".to_string(),
        })
    }
}

impl Widget for SavefileManagerInner {
    fn render(&mut self, ui: &imgui::Ui) {
        unsafe {
            let [x, y] = *self.next_pos.lock();
            imgui_sys::igSetNextWindowPos(
                imgui_sys::ImVec2 { x, y },
                Condition::Always as _,
                imgui_sys::ImVec2 { x: 0., y: 0. },
            )
        };

        let style_tokens = [
            ui.push_style_color(imgui::StyleColor::ChildBg, style::DARK_ORANGE),
            ui.push_style_color(imgui::StyleColor::ModalWindowDimBg, [0., 0., 0., 0.]),
        ];

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
            ui.text(format!("Enter directory:    {}", self.key_enter));
            ui.text(format!("Go up a directory:  {}", self.key_exit));
            ui.text(format!("Load savefile:      {}", self.key_load));
            ui.text(format!("Close popup:        {}", GlobalKeys::esc()));

            ChildWindow::new("##savefile-manager-breadcrumbs")
                .size([240., 14.])
                .build(ui, || {
                    ui.text(&self.breadcrumbs);
                    ui.set_scroll_x(ui.scroll_max_x());
                });

            ListBox::new(SFM_TAG).size([240., 100.]).build(ui, || {
                for (is_selected, i) in self.dir_stack.values() {
                    Selectable::new(i).selected(is_selected).build(ui);
                    if is_selected {
                        ui.set_scroll_here_y();
                    }
                }
            });
        }

        style_tokens.into_iter().rev().for_each(|t| t.pop());
    }

    fn interact_ui(&mut self) {
        self.dir_stack.enter();
        self.breadcrumbs = self.dir_stack.breadcrumbs();
    }

    fn interact(&mut self) {
        if self.key_enter.keyup() {
        } else if self.key_exit.keyup() {
            self.want_exit = self.dir_stack.exit();
            self.breadcrumbs = self.dir_stack.breadcrumbs();
        } else if self.key_load.keyup() {
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

    fn want_exit(&mut self) -> bool {
        let w = self.want_exit;
        self.want_exit = false;
        w
    }

    fn cursor_down(&mut self) {
        self.dir_stack.next();
    }

    fn cursor_up(&mut self) {
        self.dir_stack.prev();
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
