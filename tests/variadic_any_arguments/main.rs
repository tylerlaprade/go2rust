use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn label() -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some("value".to_string())));
}

pub fn fail() -> Rc<RefCell<Option<Box<dyn StdError>>>> {

    return Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("bad".to_string()))));
}

pub fn count(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
}

fn main() {
    let mut err = fail();
    println!("{}", format!("{}", (*count(Rc::new(RefCell::new(Some(vec![Box::new({ let __v = label(); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new(format!("{}", (*err.borrow().as_ref().unwrap()))) as Box<dyn Any>, Box::new("literal".to_string()) as Box<dyn Any>, Box::new(3) as Box<dyn Any>])))).borrow().as_ref().unwrap())));
}