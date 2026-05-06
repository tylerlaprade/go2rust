use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut x = Rc::new(RefCell::new(Some(42)));
    let mut p = x.clone();
    println!("{} {}", "x:".to_string(), { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });
    println!("{} {}", "p points to:".to_string(), { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v });

    { let new_val = 100; *p.borrow_mut() = Some(new_val); };
    println!("{} {}", "x after change:".to_string(), { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });
}