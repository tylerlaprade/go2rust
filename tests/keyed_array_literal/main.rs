use std::cell::{RefCell};
use std::rc::{Rc};

const FIRST: i32 = 0;
const SECOND: i32 = 1;
const THIRD: i32 = 2;


fn main() {
    let mut names = Rc::new(RefCell::new(Some(["one".to_string(), "two".to_string(), "three".to_string()])));
    println!("{} {}", (*names.borrow().as_ref().unwrap())[FIRST as usize].clone(), (*names.borrow().as_ref().unwrap())[THIRD as usize].clone());
}