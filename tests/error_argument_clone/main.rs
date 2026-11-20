use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn check(err: Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{}", format!("{}", "bad".to_string()));
    }
}

fn main() {
    let (_, mut err) = { let __atoi_input = "x".to_string(); match __atoi_input.parse::<i64>() { Ok(n) => (n, Rc::new(RefCell::new(None))), Err(_) => (0 as i64, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input))))))) } };
    check({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });
    println!("{}", format!("{}", "ok".to_string()));
}