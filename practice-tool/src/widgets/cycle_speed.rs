use std::cmp::Ordering;

use libds3::memedit::PointerChain;
use practice_tool_core::{
    key::Key,
    widgets::{store_value::ReadWrite, Widget},
};

#[derive(Debug)]
pub(crate) struct CycleSpeed {
    ptr: PointerChain<f32>,
    key: Key,
    values: Vec<f32>,
    current: Option<f32>,
    label: String,
}

impl CycleSpeed {
    pub(crate) fn new(values: &[f32], ptr: PointerChain<f32>, key: Key) -> Self {
        let mut values = values.to_vec();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        CycleSpeed { ptr, key, values, current: None, label: String::new() }
    }

    fn cycle(&self) -> Option<f32> {
        let next = self.ptr.read().map(|speed| {
            *self
                .values
                .iter()
                .find(|&&x| x > speed)
                .unwrap_or_else(|| self.values.first().unwrap_or(&1.0))
        });
        next.map(|speed| self.ptr.write(speed));
        next
    }
}

impl ReadWrite for CycleSpeed {
    fn read(&mut self) -> bool {
        self.current = self.ptr.read();
    }

    fn write(&mut self) {
        let next = *self
            .values
            .iter()
            .find(|&&x| x > self.current)
            .unwrap_or_else(|| self.values.first().unwrap_or(&1.0));

        self.ptr.write(next);
    }

    fn label(&self) -> &str {
        self.label.clear();

        match self.current {
            Some(c) => write!(self.label, "Speed [{:.1}x]", c),
            None => write!(self.label, "Speed"),
        };

        &self.label
    }
}

pub(crate) fn cycle_speed(values: &[f32], ptr: PointerChain<f32>, key: Key) -> Box<dyn Widget> {
    Box::new(CycleSpeed::new(values, ptr, key))
}
