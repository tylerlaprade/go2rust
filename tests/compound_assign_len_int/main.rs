use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    let mut marker = Rc::new(RefCell::new(Some("xx".to_string())));
    let mut n = Rc::new(RefCell::new(Some(10)));
    { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - (*items.borrow().as_ref().unwrap()).len() as i32); };
    { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - (*marker.borrow().as_ref().unwrap()).len() as i32); };
    println!("{}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));
}