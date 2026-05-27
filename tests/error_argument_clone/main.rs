use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn check(err: Rc<RefCell<Option<Box<dyn StdError>>>>) {
    if (*err.borrow()).is_some() {
        println!("{}", format!("{}", "bad".to_string()));
    }
}

fn main() {
    let (_, mut err) = { let __atoi_input = "x".to_string().clone(); match __atoi_input.parse::<i32>() { Ok(n) => (n, Rc::new(RefCell::new(None))), Err(_) => (0 as i32, Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    check(err.clone());
    println!("{}", format!("{}", "ok".to_string()));
}