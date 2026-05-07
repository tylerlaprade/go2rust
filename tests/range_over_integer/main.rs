use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut literalSum = Rc::new(RefCell::new(Some(0)));
    for i in 0..(5) {
        { let mut guard = literalSum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + i); };
    }

    let mut count = Rc::new(RefCell::new(Some(0)));
    for _ in 0..(3) {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut n = Rc::new(RefCell::new(Some(4)));
    let mut variableSum = Rc::new(RefCell::new(Some(0)));
    for i in 0..((*n.borrow().as_ref().unwrap())) {
        { let mut guard = variableSum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + i); };
    }

    println!("{} {} {}", { let __v = (*literalSum.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*variableSum.borrow().as_ref().unwrap()).clone(); __v });
}