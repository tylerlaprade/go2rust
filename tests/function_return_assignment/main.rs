use std::cell::{RefCell};
use std::rc::{Rc};

pub fn add_suffix(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some(format!("{}{}", (*s.borrow().as_ref().unwrap()), "!".to_string()))));
}

fn main() {
    let mut value = Rc::new(RefCell::new(Some("go".to_string())));
    { let new_val = add_suffix(Rc::new(RefCell::new(Some((*value.borrow().as_ref().unwrap()).clone())))); *value.borrow_mut() = new_val.borrow_mut().take(); };
    println!("{}", { let __v = (*value.borrow().as_ref().unwrap()).clone(); __v });
}