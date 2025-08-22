use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // The killer case: taking address after declaration
    let mut x = Rc::new(RefCell::new(Some(5)));
    { let new_val = (*x.borrow_mut().as_mut().unwrap()) + 1; *x.borrow_mut() = Some(new_val); };
    let mut p = x.clone();
    { let new_val = 10; *p.borrow_mut() = Some(new_val); };

    println!("{} {}", "x =".to_string(), (*x.borrow_mut().as_mut().unwrap()));
}