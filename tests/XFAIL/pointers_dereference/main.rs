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
    println!("{} {}", "initial:".to_string(), (*i.borrow_mut().as_mut().unwrap()));

    zeroval(i.clone());
    println!("{} {}", "zeroval:".to_string(), (*i.borrow_mut().as_mut().unwrap()));

    zeroptr(Rc::new(RefCell::new(Some(i.clone()))));
    println!("{} {}", "zeroptr:".to_string(), (*i.borrow_mut().as_mut().unwrap()));

    println!("{} {}", "pointer:".to_string(), i.clone());
}