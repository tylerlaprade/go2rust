use std::cell::{RefCell};
use std::rc::{Rc};

pub(crate) const MARKER: &'static str = "$";


fn main() {
    let mut name = Rc::new(RefCell::new(Some("$1".to_string())));
    println!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*name.borrow().as_ref().unwrap()).clone(); let __arg = MARKER; __s.starts_with(&__arg) }))).borrow().as_ref().unwrap()));
}