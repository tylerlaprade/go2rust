use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut data = Rc::new(RefCell::new(Some(("abcdef".to_string()).as_bytes().to_vec())));
    let mut limit = Rc::new(RefCell::new(Some(std::cmp::min(((*data.borrow().as_ref().unwrap()).len() as i32), (3 as i32)))));
    println!("{}", { let __v = (*limit.borrow().as_ref().unwrap()).clone(); __v });
    println!("{}", std::cmp::max((2 as i32), ((*limit.borrow().as_ref().unwrap()) as i32)));
    println!("{}", std::cmp::min("beta".to_string(), "alpha".to_string()));
}