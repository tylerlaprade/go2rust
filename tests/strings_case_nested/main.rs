use std::cell::{RefCell};
use std::rc::{Rc};

pub fn clean(name: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some({ let __s = (*Rc::new(RefCell::new(Some({ let __s = (*Rc::new(RefCell::new(Some({ let __s = (*name.borrow().as_ref().unwrap()).clone(); __s.to_lowercase() }))).borrow().as_ref().unwrap()).clone(); let __prefix = "r#".to_string(); __s.strip_prefix(&__prefix).unwrap_or(&__s).to_string() }))).borrow().as_ref().unwrap()).clone(); __s.to_uppercase() })))
}

pub fn clean_suffix(name: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some({ let __s = (*Rc::new(RefCell::new(Some({ let __s = (*name.borrow().as_ref().unwrap()).clone(); let __suffix = ".RS".to_string(); __s.strip_suffix(&__suffix).unwrap_or(&__s).to_string() }))).borrow().as_ref().unwrap()).clone(); __s.to_lowercase() })))
}

fn main() {
    println!("{}", format!("{}", (*clean(Rc::new(RefCell::new(Some("R#Go2Rust".to_string())))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*clean_suffix(Rc::new(RefCell::new(Some("Go2Rust.RS".to_string())))).borrow().as_ref().unwrap())));
}