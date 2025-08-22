use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Basic variable declarations that currently fail
    let mut x: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(42)));
    let mut y: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some("hello".to_string())));
    let mut z: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(3.14)));

        // Short variable declarations
    let mut a = Rc::new(RefCell::new(Some(100)));
    let mut b = Rc::new(RefCell::new(Some("world".to_string())));
    let mut c = Rc::new(RefCell::new(Some(2.71)));

    println!("{} {} {} {}", "Variables:".to_string(), (*x.borrow_mut().as_mut().unwrap()), (*y.borrow_mut().as_mut().unwrap()), (*z.borrow_mut().as_mut().unwrap()));
    println!("{} {} {} {}", "Short vars:".to_string(), (*a.borrow_mut().as_mut().unwrap()), (*b.borrow_mut().as_mut().unwrap()), (*c.borrow_mut().as_mut().unwrap()));
}