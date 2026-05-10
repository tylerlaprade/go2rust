mod flags;
use flags::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn set_flag() -> Rc<RefCell<Option<u32>>> {

    let mut flags: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(Some(0)));
    { let mut guard = flags.borrow_mut(); *guard = Some(guard.as_ref().unwrap() | FLAG_SYNC_MARKERS as u32); };
    return flags.clone();
}

fn main() {
    println!("{}", (*set_flag().borrow().as_ref().unwrap()));
}