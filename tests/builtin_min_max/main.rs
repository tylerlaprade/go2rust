use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut data = Rc::new(RefCell::new(Some(("abcdef".to_string()).as_bytes().to_vec())));
    let mut limit = Rc::new(RefCell::new(Some(std::cmp::min(((*data.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32), (3 as i32)))));
    println!("{}", format!("{}", { let __v = (*limit.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", std::cmp::max((2 as i32), ((*limit.borrow().as_ref().unwrap()) as i32))));
    println!("{}", format!("{}", std::cmp::min("beta".to_string(), "alpha".to_string())));
}