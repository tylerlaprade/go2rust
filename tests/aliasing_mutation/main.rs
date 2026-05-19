use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // This is the simplest case that breaks Rust's ownership
    let mut x = Rc::new(RefCell::new(Some(42)));
    let mut p = x.clone();
    let mut q = x.clone();

    println!("{} {}", format!("{}", "Initial: x =".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "Initial: *p =".to_string()), format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "Initial: *q =".to_string()), format!("{}", { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }));

    { let new_val = 100; *p.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "After *p = 100: x =".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "After *p = 100: *p =".to_string()), format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "After *p = 100: *q =".to_string()), format!("{}", { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }));

    { let new_val = 200; *q.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "After *q = 200: x =".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "After *q = 200: *p =".to_string()), format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "After *q = 200: *q =".to_string()), format!("{}", { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }));

    { let new_val = 300; *x.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "After x = 300: x =".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "After x = 300: *p =".to_string()), format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "After x = 300: *q =".to_string()), format!("{}", { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }));
}