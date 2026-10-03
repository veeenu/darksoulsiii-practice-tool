use libds3::memedit::Bitflag as BitflagInner;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::flag::{Flag, FlagWidget};
use practice_tool_core::widgets::Widget;

struct Bitflag(&'static BitflagInner<u8>);

impl Flag for Bitflag {
    fn set(&mut self, value: bool) {
        self.0.set(value);
    }

    fn get(&self) -> Option<bool> {
        self.0.get()
    }
}

pub(crate) fn flag_widget(
    label: &str,
    bitflag: &'static BitflagInner<u8>,
    key: Option<Key>,
) -> Box<dyn Widget> {
    Box::new(FlagWidget::new(label, Bitflag(bitflag), key))
}
