use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    let mut messageFormat = Rc::new(RefCell::new(Some("dynamic message".to_string())));
    let mut errorFormat = Rc::new(RefCell::new(Some("dynamic error".to_string())));

    println!("{}", (*Rc::new(RefCell::new(Some(format!("{}", (*messageFormat.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
    println!("{}", format!("{}", (*(Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("{}", (*errorFormat.borrow().as_ref().unwrap()).clone())))))).borrow().as_ref().unwrap())));
}