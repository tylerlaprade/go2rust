use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let (mut n, _) = (*conv.borrow().as_ref().unwrap())::atoi(Rc::new(RefCell::new(Some("42".to_string()))));
    println!("{} {}", (*(*str.borrow().as_ref().unwrap())::to_upper(Rc::new(RefCell::new(Some("go".to_string())))).borrow().as_ref().unwrap()), (*n.borrow().as_ref().unwrap()));
}