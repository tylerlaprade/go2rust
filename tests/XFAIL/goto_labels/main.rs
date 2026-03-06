use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(0)));

    if (*i.borrow().as_ref().unwrap()) < 5 {
        print!("i = {}\n", (*i.borrow().as_ref().unwrap()));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        // TODO: goto not supported
    }

    println!("{}", "First loop done".to_string());

        // Goto to skip code
    let mut x = Rc::new(RefCell::new(Some(1)));
    if (*x.borrow().as_ref().unwrap()) > 0 {
        // TODO: goto not supported
    }
    println!("{}", "This won't print".to_string());

    println!("{}", "Skipped to here".to_string());

        // More complex goto pattern
    let mut j = Rc::new(RefCell::new(Some(0)));
    while (*j.borrow().as_ref().unwrap()) < 3 {
        let mut k = Rc::new(RefCell::new(Some(0)));
    while (*k.borrow().as_ref().unwrap()) < 3 {
        if (*j.borrow().as_ref().unwrap()) == 1 && (*k.borrow().as_ref().unwrap()) == 1 {
        // TODO: goto not supported
    }
        print!("j={}, k={}\n", (*j.borrow().as_ref().unwrap()), (*k.borrow().as_ref().unwrap()));
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    println!("{}", "All done".to_string());
}