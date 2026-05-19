use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    println!("{} {} {}", format!("{}", 1), format!("{}", !3), format!("{}", 7 & ! 3));
    println!("{}", format!("{}", !(*Rc::new(RefCell::new(Some(0 as u64))).borrow().as_ref().unwrap())));
}