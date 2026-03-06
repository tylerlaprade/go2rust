mod math;
mod string;
use math::*;
use string::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Test math functions
    let mut sum = add(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(3))));
    print!("5 + 3 = {}\n", (*sum.borrow().as_ref().unwrap()));

    let mut product = multiply(Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(7))));
    print!("4 * 7 = {}\n", (*product.borrow().as_ref().unwrap()));

        // Test string function
    let mut repeated = repeat(Rc::new(RefCell::new(Some("Go".to_string()))), Rc::new(RefCell::new(Some(3))));
    print!("Repeat(\"Go\", 3) = {}\n", (*repeated.borrow().as_ref().unwrap()));
}