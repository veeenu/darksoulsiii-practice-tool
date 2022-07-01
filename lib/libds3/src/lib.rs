#![feature(once_cell)]

pub mod memedit;
mod params;
pub mod pointers;
pub mod version;

use std::time::Duration;

pub use params::*;

pub fn wait_option<T, F: FnMut() -> Option<T>>(mut f: F) -> T {
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

pub fn print_hex<T: Sized>(ptr: *const T) {
    let ptr = ptr as *const u8;

    let bytes: Vec<u8> = (0..std::mem::size_of::<T>()).map(|i| unsafe { *ptr.add(i) }).collect();

    bytes.chunks(16).for_each(|bs| {
        for i in bs {
            print!("{:02x} ", i);
        }

        print!("  ");

        for _ in bs.len()..16 {
            print!("  ");
        }

        for i in bs {
            let c = *i as char;
            print!("{}", if c.is_ascii() { c } else { '.' });
        }

        println!();
    });
}
