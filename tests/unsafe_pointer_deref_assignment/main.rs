use std::cell::{RefCell};
use std::rc::{Rc};

pub fn mark_through_unsafe_pointer(addr: Rc<RefCell<Option<usize>>>) {
    { unimplemented!("unsafe.Pointer dereference assignment"); };
}

fn main() {
    println!("{}", format!("{}", "ok".to_string()));
}