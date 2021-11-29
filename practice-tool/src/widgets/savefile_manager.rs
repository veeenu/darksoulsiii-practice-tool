use std::sync::Arc;

use imgui::{ListBox, Selectable};
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
        SavefileManager {
            label: format!("Savefile manager ({})", hotkey),
            hotkey,
            inner: Arc::new(Mutex::new(Box::new(SavefileManagerInner::new()) as _)),
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
pub(crate) struct SavefileManagerInner {
    key_enter: KeyState,
    index: usize,
    open: bool,
    open_toggle: bool,
}

impl SavefileManagerInner {
    fn new() -> Self {
        SavefileManagerInner {
            key_enter: KeyState::new(get_key_code("x").unwrap()),
            index: 0,
            open: true,
            open_toggle: true,
        }
    }
}

impl Widget for SavefileManagerInner {
    fn render(&self, ui: &imgui::Ui) {
        let vals = ["one", "two", "three", "four", "five"];

        ui.text("I am internal hehe");

        let listbox = ListBox::new("##savefile-manager").size([100., 100.]);

        if let Some(_token) = listbox.begin(ui) {
            for (idx, i) in vals.iter().enumerate() {
                let is_selected = idx == self.index;
                Selectable::new(i).selected(is_selected).build(ui);
            }

            // if !self.open {
            //     ui.close_current_popup();
            // }
        };
    }

    fn interact(&mut self) {
        self.open_toggle = false;

        if self.key_enter.keyup() {
            self.open = !self.open;
            self.open_toggle = true;
        }
    }

    fn cursor_down(&mut self) {
        self.index = usize::min(self.index + 1, 2);
    }

    fn cursor_up(&mut self) {
        self.index = self.index.saturating_sub(1);
    }
}