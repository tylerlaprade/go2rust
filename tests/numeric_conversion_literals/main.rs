use std::cell::{RefCell};
use std::rc::{Rc};

pub fn byte_bit(i: Rc<RefCell<Option<i32>>>) -> u8 {
    (*Rc::new(RefCell::new(Some(1 as u8))).borrow().as_ref().unwrap()) as u8 << ((*i.borrow().as_ref().unwrap()) % 8)
}

pub fn uint64_mask(i: Rc<RefCell<Option<u32>>>) -> u64 {
    (*Rc::new(RefCell::new(Some(1 as u64))).borrow().as_ref().unwrap()) as u64 << (*i.borrow().as_ref().unwrap())
}

pub fn byte_from_expr(v: Rc<RefCell<Option<u8>>>) -> u8 {
    (*Rc::new(RefCell::new(Some(((*v.borrow().as_ref().unwrap()) + ('0' as u8)) as u8))).borrow().as_ref().unwrap())
}

fn main() {
    println!("{} {}", format!("{}", "byte literal bit:".to_string()), format!("{}", byte_bit(Rc::new(RefCell::new(Some(3))))));
    println!("{} {}", format!("{}", "uint64 literal mask:".to_string()), format!("{}", uint64_mask(Rc::new(RefCell::new(Some(5))))));
    println!("{} {}", format!("{}", "byte expression:".to_string()), format!("{}", byte_from_expr(Rc::new(RefCell::new(Some(4))))));
}