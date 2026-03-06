use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(0)));
    'outer: while (*i.borrow().as_ref().unwrap()) < 3 {
        let mut j = Rc::new(RefCell::new(Some(0)));
    while (*j.borrow().as_ref().unwrap()) < 3 {
        if (*i.borrow().as_ref().unwrap()) == 1 && (*j.borrow().as_ref().unwrap()) == 1 {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'outer
    }
        if (*i.borrow().as_ref().unwrap()) == 2 && (*j.borrow().as_ref().unwrap()) == 1 {
        break 'outer
    }
        print!("({}, {}) ", (*i.borrow().as_ref().unwrap()), (*j.borrow().as_ref().unwrap()));
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        println!();
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", "Done".to_string());
}