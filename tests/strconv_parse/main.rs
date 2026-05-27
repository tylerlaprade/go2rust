use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    let mut str = Rc::new(RefCell::new(Some("42".to_string())));
    let (mut num, mut err) = { let __atoi_input = (*str.borrow().as_ref().unwrap()).clone(); match __atoi_input.parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
        return;
    }
    println!("{} {}", format!("{}", "Parsed number:".to_string()), format!("{}", num));
}