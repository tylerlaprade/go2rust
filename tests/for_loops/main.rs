use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(1)));
    while (*i.borrow_mut().as_mut().unwrap()) <= 3 {
        println!("{}", (*i.borrow_mut().as_mut().unwrap()));
        { let new_val = (*i.borrow_mut().as_mut().unwrap()) + 1; *i.borrow_mut() = Some(new_val); };
    }

    let mut j = Rc::new(RefCell::new(Some(0)));
    while (*j.borrow_mut().as_mut().unwrap()) < 3 {
        println!("{}", (*j.borrow_mut().as_mut().unwrap()));
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    while true {
        println!("{}", "loop".to_string());
        break
    }
}