use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn check(err: Rc<RefCell<Option<Box<dyn StdError>>>>) {
    if (*err.borrow()).is_some() {
        println!("{}", "bad".to_string());
    }
}

fn main() {
    let (_, mut err) = { let __atoi_input = "x".to_string().clone(); match __atoi_input.parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    check(err.clone());
    println!("{}", "ok".to_string());
}