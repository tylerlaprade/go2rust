use std::cell::{RefCell};
use std::error::Error;
use std::rc::{Rc};

fn main() {
    let (mut n, _) = { let __atoi_input = "42".to_string().clone(); match __atoi_input.parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn Error>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    println!("{} {}", (*Rc::new(RefCell::new(Some("go".to_string().to_uppercase()))).borrow().as_ref().unwrap()), { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v });
}