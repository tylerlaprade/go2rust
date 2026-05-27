mod flags;
use flags::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn set_flag() -> u32 {
    let mut flags: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(Some(0)));
    { let __rhs = FLAG_SYNC_MARKERS as u32; let mut guard = flags.borrow_mut(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    return (*flags.borrow().as_ref().unwrap());
}

fn main() {
    println!("{}", format!("{}", set_flag()));
}