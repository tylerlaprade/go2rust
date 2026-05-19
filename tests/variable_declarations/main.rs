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

    println!("{} {} {} {}", format!("{}", "Variables:".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*z.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {} {} {}", format!("{}", "Short vars:".to_string()), format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }));
}