use std::cell::{RefCell};
use std::rc::{Rc};

pub fn zeroval(ival: Rc<RefCell<Option<i32>>>) {
    { let new_val = 0; *ival.borrow_mut() = Some(new_val); };
}

pub fn zeroptr(iptr: Rc<RefCell<Option<i32>>>) {
    { let new_val = 0; *iptr.borrow_mut() = Some(new_val); };
}

fn main() {
    let mut i = Rc::new(RefCell::new(Some(1)));
    println!("{} {}", "initial:".to_string(), (*i.borrow().as_ref().unwrap()));

    zeroval(Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()).clone()))));
    println!("{} {}", "zeroval:".to_string(), (*i.borrow().as_ref().unwrap()));

    zeroptr(i.clone());
    println!("{} {}", "zeroptr:".to_string(), (*i.borrow().as_ref().unwrap()));

    let mut p = i.clone();
    println!("{} {}", "pointer is non-nil:".to_string(), (*p.borrow()).is_some());
}