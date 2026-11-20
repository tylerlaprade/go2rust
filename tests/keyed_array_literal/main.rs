use std::cell::{RefCell};
use std::rc::{Rc};

pub const FIRST: i64 = 0;
pub const SECOND: i64 = 1;
pub const THIRD: i64 = 2;


fn main() {
    let mut names = Rc::new(RefCell::new(Some(["one".to_string(), "two".to_string(), "three".to_string()])));
    println!("{} {}", format!("{}", (*names.borrow().as_ref().unwrap())[(FIRST) as usize].clone()), format!("{}", (*names.borrow().as_ref().unwrap())[(THIRD) as usize].clone()));
}