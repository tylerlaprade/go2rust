use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut now = (*time.borrow().as_ref().unwrap())::now();
    println!("{} {}", "Current time:".to_string(), (*now.borrow().as_ref().unwrap()));

    let mut future = (*now.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(24 * (*time.borrow().as_ref().unwrap())::hour))));
    println!("{} {}", "Tomorrow:".to_string(), (*future.borrow().as_ref().unwrap()));

    println!("{} {}", "Unix timestamp:".to_string(), (*(*now.borrow().as_ref().unwrap()).unix().borrow().as_ref().unwrap()));
}