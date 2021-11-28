pub(crate) mod flag;
pub(crate) mod position;

pub(crate) trait Command: Send + Sync + std::fmt::Debug {
    fn render(&self, ui: &imgui::Ui);
}
