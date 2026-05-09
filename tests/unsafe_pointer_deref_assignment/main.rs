use std::cell::{RefCell};
use std::rc::{Rc};

pub fn mark_through_unsafe_pointer(addr: Rc<RefCell<Option<usize>>>) {
    { let _ = true; };
}

fn main() {
    println!("{}", "ok".to_string());
}