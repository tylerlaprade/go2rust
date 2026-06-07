use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub const ENABLED: bool = false;


pub fn acquire(addr: Arc<Mutex<Option<usize>>>) {
}

pub fn release(addr: Arc<Mutex<Option<usize>>>) {
}

pub fn release_merge(addr: Arc<Mutex<Option<usize>>>) {
}

pub fn disable() {
}

pub fn enable() {
}