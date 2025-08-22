use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut x = Rc::new(RefCell::new(Some(42)));
    let mut p = x.clone();
    println!("{} {}", "x:".to_string(), (*x.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "p points to:".to_string(), (*p.borrow().as_ref().unwrap()));

    { let new_val = 100; *p.borrow_mut() = Some(new_val); };
    println!("{} {}", "x after change:".to_string(), (*x.borrow_mut().as_mut().unwrap()));
}