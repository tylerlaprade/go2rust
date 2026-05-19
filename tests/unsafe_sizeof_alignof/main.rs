use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut x: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(7)));
    let mut y: Rc<RefCell<Option<u64>>> = Rc::new(RefCell::new(Some(9)));

    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(std::mem::size_of::<i32>()))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(std::mem::align_of::<i32>()))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(std::mem::size_of::<u64>()))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(std::mem::align_of::<u64>()))).borrow().as_ref().unwrap())));
}