use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut s = Rc::new(RefCell::new(Some("hello".to_string())));
    println!("{}", (*s.borrow().as_ref().unwrap()).len());

    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow_mut().as_mut().unwrap()) < (*s.borrow().as_ref().unwrap()).len() {
        print!("{} ", (*(*s.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()).as_bytes()[(*i.borrow_mut().as_mut().unwrap()) as usize]);
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!();

    for r in &"go".to_string() {
        print!("{} ", r);
    }
    println!();
}