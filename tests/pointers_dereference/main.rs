use std::cell::{RefCell};
use std::rc::{Rc};

pub fn zeroval(mut ival: Rc<RefCell<Option<i32>>>) {
    { let new_val = 0; *ival.borrow_mut() = Some(new_val); };
}

pub fn zeroptr(iptr: Rc<RefCell<Option<i32>>>) {
    { let new_val = 0; *iptr.borrow_mut() = Some(new_val); };
}

fn main() {
    let mut i = Rc::new(RefCell::new(Some(1)));
    println!("{} {}", format!("{}", "initial:".to_string()), format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }));

    zeroval(Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()).clone()))));
    println!("{} {}", format!("{}", "zeroval:".to_string()), format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }));

    zeroptr(i.clone());
    println!("{} {}", format!("{}", "zeroptr:".to_string()), format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }));

    let mut p = i.clone();
    println!("{} {}", format!("{}", "pointer is non-nil:".to_string()), format!("{}", (*p.borrow()).is_some()));
}