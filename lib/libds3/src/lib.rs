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
