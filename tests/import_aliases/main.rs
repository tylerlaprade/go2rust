use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    let (mut n, _) = { let __atoi_input = "42".to_string().clone(); match __atoi_input.parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some({ let __s = "go".to_string(); __s.to_uppercase() }))).borrow().as_ref().unwrap())), format!("{}", n));
}