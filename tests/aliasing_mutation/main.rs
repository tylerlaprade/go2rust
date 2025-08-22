use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // This is the simplest case that breaks Rust's ownership
    let mut x = Rc::new(RefCell::new(Some(42)));
    let mut p = x.clone();
    let mut q = x.clone();

    println!("{} {}", "Initial: x =".to_string(), (*x.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "Initial: *p =".to_string(), (*p.borrow().as_ref().unwrap()));
    println!("{} {}", "Initial: *q =".to_string(), (*q.borrow().as_ref().unwrap()));

    { let new_val = 100; *p.borrow_mut() = Some(new_val); };
    println!("{} {}", "After *p = 100: x =".to_string(), (*x.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "After *p = 100: *p =".to_string(), (*p.borrow().as_ref().unwrap()));
    println!("{} {}", "After *p = 100: *q =".to_string(), (*q.borrow().as_ref().unwrap()));

    { let new_val = 200; *q.borrow_mut() = Some(new_val); };
    println!("{} {}", "After *q = 200: x =".to_string(), (*x.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "After *q = 200: *p =".to_string(), (*p.borrow().as_ref().unwrap()));
    println!("{} {}", "After *q = 200: *q =".to_string(), (*q.borrow().as_ref().unwrap()));

    { let new_val = 300; *x.borrow_mut() = Some(new_val); };
    println!("{} {}", "After x = 300: x =".to_string(), (*x.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "After x = 300: *p =".to_string(), (*p.borrow().as_ref().unwrap()));
    println!("{} {}", "After x = 300: *q =".to_string(), (*q.borrow().as_ref().unwrap()));
}