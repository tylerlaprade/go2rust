use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    let mut str = Rc::new(RefCell::new(Some("42".to_string())));
    let (mut num, mut err) = { let __atoi_input = (*str.borrow().as_ref().unwrap()).clone(); match __atoi_input.parse::<i64>() { Ok(n) => (n, Rc::new(RefCell::new(None))), Err(_) => (0 as i64, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input))))))) } };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        return;
    }
    println!("{} {}", format!("{}", "Parsed number:".to_string()), format!("{}", num));
}