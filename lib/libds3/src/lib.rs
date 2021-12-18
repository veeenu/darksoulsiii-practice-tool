#![feature(once_cell)]

mod params;
pub mod version;

use std::time::Duration;

pub use params::*;

pub fn wait_option<T, F: Fn() -> Option<T>>(f: F) -> T {
    loop {
        if let Some(t) = f() {
            return t;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

pub trait ParamVisitor {
    fn visit_u8(&mut self, name: &str, v: &mut u8);
    fn visit_u16(&mut self, name: &str, v: &mut u16);
    fn visit_u32(&mut self, name: &str, v: &mut u32);
    fn visit_i8(&mut self, name: &str, v: &mut i8);
    fn visit_i16(&mut self, name: &str, v: &mut i16);
    fn visit_i32(&mut self, name: &str, v: &mut i32);
    fn visit_f32(&mut self, name: &str, v: &mut f32);
    fn visit_bool(&mut self, name: &str, v: &mut bool);
}

pub trait ParamStruct {
    fn visit<T: ParamVisitor + ?Sized>(&mut self, t: &mut T);
}
