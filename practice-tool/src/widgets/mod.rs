pub(crate) mod config;
pub(crate) mod flag;

pub(crate) trait Command {
    fn render(&self, ui: &imgui::Ui);
}
