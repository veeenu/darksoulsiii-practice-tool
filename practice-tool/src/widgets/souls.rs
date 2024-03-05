use libds3::memedit::PointerChain;
use practice_tool_core::{
    key::Key,
    widgets::{
        store_value::{ReadWrite, StoreValue},
        Widget,
    },
};

struct Souls {
    ptr: PointerChain<u32>,
    current: u32,
    amount: u32,
    label: String,
}

impl Souls {
    fn new(amount: u32, ptr: PointerChain<u32>) -> Self {
        Self { ptr, current: 0, amount, label: format!("Add {amount} souls") }
    }
}

impl ReadWrite for Souls {
    fn read(&mut self) -> bool {
        if let Some(current) = self.ptr.read() {
            self.current = current;
            true
        } else {
            false
        }
    }

    fn write(&mut self) {
        self.ptr.write(self.current + self.amount);
    }

    fn label(&self) -> &str {
        &self.label
    }
}

pub(crate) fn souls(amount: u32, ptr: PointerChain<u32>, key: Key) -> Box<dyn Widget> {
    Box::new(StoreValue::new(Souls::new(amount, ptr), Some(key)))
}
