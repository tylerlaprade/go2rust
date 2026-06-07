use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub const ENABLED: bool = false;


pub fn read(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
}

pub fn write(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
}