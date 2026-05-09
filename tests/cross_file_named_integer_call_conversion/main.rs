mod types;
use types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn raw_marker() -> Rc<RefCell<Option<u64>>> {

    return Rc::new(RefCell::new(Some(5 as u64)));
}

fn main() {
    let mut marker = Rc::new(RefCell::new(Some(Marker(Rc::new(RefCell::new(Some((*raw_marker().borrow().as_ref().unwrap()) as i32)))))));
    println!("{}", (*Rc::new(RefCell::new(Some((*(*marker.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap()));
}