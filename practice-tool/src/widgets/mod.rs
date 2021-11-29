use std::sync::Arc;

use parking_lot::Mutex;

use crate::style::StyleState;

pub(crate) mod cycle_speed;
pub(crate) mod flag;
pub(crate) mod position;
pub(crate) mod quitout;
pub(crate) mod savefile_manager;
pub(crate) mod souls;

pub(crate) trait Widget: Send + Sync + std::fmt::Debug {
    fn render(&self, ui: &imgui::Ui);
    fn interact(&mut self) {}
    fn interact_ui(&mut self) {}

    fn enter(&self) -> Option<Arc<Mutex<Box<dyn Widget>>>> { None }
    fn cursor_down(&mut self) {}
    fn cursor_up(&mut self) {}
}

#[derive(Debug)]
pub(crate) struct WidgetList {
    widgets: Vec<Box<dyn Widget>>,
    cursor: usize,
}

impl WidgetList {
    pub(crate) fn new(widgets: Vec<Box<dyn Widget>>) -> Self {
        WidgetList { widgets, cursor: 0 }
    }
}

impl Widget for WidgetList {
    fn render(&self, ui: &imgui::Ui) {
        for (i, w) in self.widgets.iter().enumerate() {
            let mut token = if i == self.cursor {
                StyleState::Active.get_style_token(ui)
            } else {
                StyleState::Inactive.get_style_token(ui)
            };
            w.render(ui);
            token.pop();
        }
    }

    fn interact(&mut self) {
        for w in self.widgets.iter_mut() {
            w.interact();
        }
    }

    fn interact_ui(&mut self) {
        self.widgets[self.cursor].interact_ui();
    }

    fn enter(&self) -> Option<Arc<Mutex<Box<(dyn Widget + 'static)>>>> {
        self.widgets[self.cursor].enter()
    }

    fn cursor_down(&mut self) {
        self.cursor = usize::min(self.cursor + 1, self.widgets.len() - 1);
    }

    fn cursor_up(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
}
