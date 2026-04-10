use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut now = Rc::new(RefCell::new(Some(std::time::SystemTime::now())));
    println!("{} {}", "Current time:".to_string(), (*now.borrow().as_ref().unwrap()));

    let mut future = (*now.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(24 * std::time::Duration::from_secs(3600)))));
    println!("{} {}", "Tomorrow:".to_string(), (*future.borrow().as_ref().unwrap()));

    println!("{} {}", "Unix timestamp:".to_string(), (*(*now.borrow().as_ref().unwrap()).unix().borrow().as_ref().unwrap()));
}