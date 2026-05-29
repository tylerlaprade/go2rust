use std::cell::{RefCell};
use std::rc::{Rc};

pub(crate) const SLASH: i32 = ('/' as i32);


fn main() {
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let __s = "a/b".to_string(); let __arg = (*Rc::new(RefCell::new(Some(char::from_u32((SLASH) as u32).unwrap().to_string()))).borrow().as_ref().unwrap()).clone(); __s.contains(&__arg) }))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let __s = "axb".to_string(); let __arg = (*Rc::new(RefCell::new(Some(char::from_u32((SLASH) as u32).unwrap().to_string()))).borrow().as_ref().unwrap()).clone(); __s.contains(&__arg) }))).borrow().as_ref().unwrap())));
}