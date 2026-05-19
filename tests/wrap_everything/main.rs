use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Every variable should be wrapped
    let mut x = Rc::new(RefCell::new(Some(42)));
    let mut y = Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()) + 1)));

        // Taking address should work naturally
    let mut p = x.clone();
    { let new_val = 100; *p.borrow_mut() = Some(new_val); };

        // x should reflect the change
    println!("{} {}", format!("{}", "x =".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "y =".to_string()), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }));
}