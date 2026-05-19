use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    eprintln!("{}", format!("{}", "This goes to stderr".to_string()));

    let mut s = Rc::new(RefCell::new(Some("hello".to_string())));
    eprintln!("{}", format!("{}", (*s.borrow().as_ref().unwrap()).len()));
}