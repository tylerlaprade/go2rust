use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    println!("{} {} {}", 1, !3, 7 & ! 3);
    println!("{}", !(*Rc::new(RefCell::new(Some(0 as u64))).borrow().as_ref().unwrap()));
}