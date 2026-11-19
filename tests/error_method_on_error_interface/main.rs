use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn describe(err: Rc<RefCell<Option<Rc<dyn StdError>>>>) -> Rc<RefCell<Option<String>>> {
    if { let __nil_result = (*err.borrow()).is_none(); __nil_result } {
        return Rc::new(RefCell::new(Some("nil".to_string())));
    }
    Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap()))))
}

fn main() {
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("boom".to_string())))))).borrow().as_ref().unwrap())));
}