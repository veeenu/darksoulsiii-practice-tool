use std::mem;

use practice_tool_core::key::Key;
use practice_tool_core::widgets::store_value::{ReadWrite, StoreValue};
use practice_tool_core::widgets::Widget;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub(crate) enum OpenMenuKind {
    #[serde(rename = "travel")]
    Travel,
    #[serde(rename = "attune")]
    Attune,
}

#[derive(Debug)]
struct OpenMenu {
    kind: OpenMenuKind,
    travel_ptr: usize,
    attune_ptr: usize,
}

impl OpenMenu {
    pub(crate) fn new(kind: OpenMenuKind, travel_ptr: usize, attune_ptr: usize) -> Self {
        Self { kind, travel_ptr, attune_ptr }
    }

    fn call(&self) {
        let ptr = match self.kind {
            OpenMenuKind::Travel => self.travel_ptr,
            OpenMenuKind::Attune => self.attune_ptr,
        };

        let stack_space = [0u8; 0x48];

        let func: extern "system" fn(*const u8) = unsafe { mem::transmute(ptr) };
        func(stack_space.as_ptr());
    }
}

impl ReadWrite for OpenMenu {
    fn read(&mut self) -> bool {
        true
    }

    fn write(&mut self) {
        self.call()
    }

    fn label(&self) -> &str {
        match self.kind {
            OpenMenuKind::Travel => "Warp menu",
            OpenMenuKind::Attune => "Attune menu",
        }
    }
}

pub(crate) fn open_menu(
    kind: OpenMenuKind,
    travel_ptr: usize,
    attune_ptr: usize,
    key: Option<Key>,
) -> Box<dyn Widget> {
    Box::new(StoreValue::new(OpenMenu::new(kind, travel_ptr, attune_ptr), key))
}
