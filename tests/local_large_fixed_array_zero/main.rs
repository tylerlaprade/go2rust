use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut buf: Rc<RefCell<Option<[u8; 128]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0))));
    println!("{} {}", (*buf.borrow().as_ref().unwrap()).len(), (*buf.borrow().as_ref().unwrap())[(0) as usize].clone());
}