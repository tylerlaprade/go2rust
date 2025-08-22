use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Every variable should be wrapped
    let mut x = Rc::new(RefCell::new(Some(42)));
    let mut y = Rc::new(RefCell::new(Some((*x.borrow_mut().as_mut().unwrap()) + 1)));

        // Taking address should work naturally
    let mut p = x.clone();
    { let new_val = 100; *p.borrow_mut() = Some(new_val); };

        // x should reflect the change
    println!("{} {}", "x =".to_string(), (*x.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "y =".to_string(), (*y.borrow_mut().as_mut().unwrap()));
}