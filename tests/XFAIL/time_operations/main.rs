use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut now = (*time.borrow_mut().as_mut().unwrap())::now();
    println!("{} {}", "Current time:".to_string(), (*now.borrow_mut().as_mut().unwrap()));

    let mut future = (*now.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(24 * (*(*time.borrow_mut().as_mut().unwrap())::hour.borrow().as_ref().unwrap())))));
    println!("{} {}", "Tomorrow:".to_string(), (*future.borrow_mut().as_mut().unwrap()));

    println!("{} {}", "Unix timestamp:".to_string(), (*(*now.borrow_mut().as_mut().unwrap()).unix().borrow().as_ref().unwrap()));
}