use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(0)));

    // TODO: Unhandled statement type: LabeledStmt

    println!("{}", "First loop done".to_string());

        // Goto to skip code
    let mut x = Rc::new(RefCell::new(Some(1)));
    if (*x.borrow_mut().as_mut().unwrap()) > 0 {
        // TODO: goto not supported
    }
    println!("{}", "This won't print".to_string());

    // TODO: Unhandled statement type: LabeledStmt

        // More complex goto pattern
    let mut j = Rc::new(RefCell::new(Some(0)));
    while (*j.borrow_mut().as_mut().unwrap()) < 3 {
        let mut k = Rc::new(RefCell::new(Some(0)));
    while (*k.borrow_mut().as_mut().unwrap()) < 3 {
        if (*j.borrow_mut().as_mut().unwrap()) == 1 && (*k.borrow_mut().as_mut().unwrap()) == 1 {
        // TODO: goto not supported
    }
        print!("j={}, k={}\n", (*j.borrow_mut().as_mut().unwrap()), (*k.borrow_mut().as_mut().unwrap()));
        { let mut guard = k.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    // TODO: Unhandled statement type: LabeledStmt
}