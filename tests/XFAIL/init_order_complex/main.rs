use std::cell::{RefCell};
use std::rc::{Rc};

pub fn f() -> Rc<RefCell<Option<i32>>> {

    { let mut guard = d.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return d.clone();
}

pub fn init() {
    println!("{}", "First init".to_string());
    { let mut guard = d.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
}

pub fn init() {
    println!("{}", "Second init".to_string());
    print!("a={}, b={}, c={}, d={}\n", (*a.borrow_mut().as_mut().unwrap()), (*b.borrow_mut().as_mut().unwrap()), (*c.borrow_mut().as_mut().unwrap()), (*d.borrow_mut().as_mut().unwrap()));
}

fn main() {
    print!("In main: a={}, b={}, c={}, d={}\n", (*a.borrow_mut().as_mut().unwrap()), (*b.borrow_mut().as_mut().unwrap()), (*c.borrow_mut().as_mut().unwrap()), (*d.borrow_mut().as_mut().unwrap()));
}