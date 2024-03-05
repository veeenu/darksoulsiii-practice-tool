use std::cmp::Ordering;
use std::fmt::Write;

use libds3::memedit::PointerChain;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::store_value::{ReadWrite, StoreValue};
use practice_tool_core::widgets::Widget;

#[derive(Debug)]
struct CycleSpeed {
    ptr: PointerChain<f32>,
    values: Vec<f32>,
    current: Option<f32>,
    label: String,
}

impl CycleSpeed {
    fn new(values: &[f32], ptr: PointerChain<f32>) -> Self {
        let mut values = values.to_vec();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        CycleSpeed { ptr, values, current: None, label: String::new() }
    }

    // fn cycle(&self) -> Option<f32> {
    //     let next = self.ptr.read().map(|speed| {
    //         *self
    //             .values
    //             .iter()
    //             .find(|&&x| x > speed)
    //             .unwrap_or_else(|| self.values.first().unwrap_or(&1.0))
    //     });
    //     next.map(|speed| self.ptr.write(speed));
    //     next
    // }
}

impl ReadWrite for CycleSpeed {
    fn read(&mut self) -> bool {
        self.current = self.ptr.read();

        self.label.clear();

        match self.current {
            Some(c) => write!(self.label, "Speed [{:.1}x]", c).ok(),
            None => write!(self.label, "Speed").ok(),
        };

        self.current.is_some()
    }

    fn write(&mut self) {
        let next = *self
            .current
            .and_then(|current| self.values.iter().find(|&&x| x > current))
            .unwrap_or_else(|| self.values.first().unwrap_or(&1.0));

        self.ptr.write(next);
    }

    fn label(&self) -> &str {
        &self.label
    }
}

pub(crate) fn cycle_speed(values: &[f32], ptr: PointerChain<f32>, key: Key) -> Box<dyn Widget> {
    Box::new(StoreValue::new(CycleSpeed::new(values, ptr), Some(key)))
}
