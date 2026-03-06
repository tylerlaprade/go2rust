use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(0)));
    'outer: while (*i.borrow_mut().as_mut().unwrap()) < 3 {
        let mut j = Rc::new(RefCell::new(Some(0)));
    while (*j.borrow_mut().as_mut().unwrap()) < 3 {
        if (*i.borrow_mut().as_mut().unwrap()) == 1 && (*j.borrow_mut().as_mut().unwrap()) == 1 {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'outer
    }
        if (*i.borrow_mut().as_mut().unwrap()) == 2 && (*j.borrow_mut().as_mut().unwrap()) == 1 {
        break 'outer
    }
        print!("({}, {}) ", (*i.borrow_mut().as_mut().unwrap()), (*j.borrow_mut().as_mut().unwrap()));
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        println!();
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", "Done".to_string());
}