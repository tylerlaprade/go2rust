use std::cell::{RefCell};
use std::rc::{Rc};

pub fn describe(ptr: Rc<RefCell<Option<i32>>>) {
    { let _switch_val = (*ptr.borrow().as_ref().unwrap());
    match _switch_val {
        None => {
            println!("{}", "nil pointer".to_string());
        }
        _ => {
            println!("{}", (*ptr.borrow().as_ref().unwrap()));
        }
    } }
}

fn main() {
    describe(Rc::new(RefCell::new(Some(None))));
    let mut x = Rc::new(RefCell::new(Some(42)));
    describe(x.clone());
}