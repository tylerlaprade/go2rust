use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn describe(err: Rc<RefCell<Option<Box<dyn StdError>>>>) -> Rc<RefCell<Option<String>>> {
    if (*err.borrow()).is_none() {
        return Rc::new(RefCell::new(Some("nil".to_string())));
    }
    Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap()))))
}

fn main() {
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("boom".to_string()))))).borrow().as_ref().unwrap())));
}