use std::cell::{RefCell};
use std::error::Error;
use std::rc::{Rc};

fn main() {
    let mut str = Rc::new(RefCell::new(Some("42".to_string())));
    let (mut num, mut err) = match (*str.borrow_mut().as_mut().unwrap()).parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::new(e) as Box<dyn Error + Send + Sync>)))) };
    if (*err.borrow()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.borrow_mut().as_mut().unwrap()));
        return;
    }
    println!("{} {}", "Parsed number:".to_string(), (*num.borrow_mut().as_mut().unwrap()));
}