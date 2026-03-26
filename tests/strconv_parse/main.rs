use std::cell::{RefCell};
use std::error::Error;
use std::rc::{Rc};

fn main() {
    let mut str = Rc::new(RefCell::new(Some("42".to_string())));
    let (mut num, mut err) = { let __atoi_input = (*str.borrow().as_ref().unwrap()).clone(); match __atoi_input.parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn Error + Send + Sync>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    if (*err.borrow()).is_some() {
        println!("{} {}", "Error:".to_string(), format!("{}", (*err.borrow().as_ref().unwrap())));
        return;
    }
    println!("{} {}", "Parsed number:".to_string(), (*num.borrow().as_ref().unwrap()));
}