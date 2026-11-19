use std::cell::{RefCell};
use std::rc::{Rc};

pub fn trim_parens(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some({ let __s = &((*s.borrow().as_ref().unwrap()).clone()); let __low = ("(".len()) as usize; let __high = (((*s.borrow().as_ref().unwrap()).len() as i32) - (")".len() as i32)) as usize; __s[__low..__high].to_string() })))
}

fn main() {
    println!("{}", format!("{}", (*trim_parens(Rc::new(RefCell::new(Some("(go2rust)".to_string())))).borrow().as_ref().unwrap())));
}