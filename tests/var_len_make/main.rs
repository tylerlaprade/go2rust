use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut names = Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()])));
    let mut n = Rc::new(RefCell::new(Some((*names.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));let mut out = Rc::new(RefCell::new(Some(vec!["".to_string(); ((*n.borrow().as_ref().unwrap())) as usize])));
    (*out.borrow_mut().as_mut().unwrap())[(1) as usize] = "beta".to_string();
    println!("{} {}", format!("{}", (*out.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*out.borrow().as_ref().unwrap())[(1) as usize].clone()));
}