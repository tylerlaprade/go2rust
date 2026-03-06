use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Simplest possible test
    let mut x = Rc::new(RefCell::new(Some(42)));
    println!("{}", (*x.borrow().as_ref().unwrap()));
}