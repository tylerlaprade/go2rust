use std::cell::{RefCell};
use std::rc::{Rc};

pub fn fact(n: Rc<RefCell<Option<i32>>>) -> i32 {

    if (*n.borrow().as_ref().unwrap()) == 0 {
        return 1 as i32;
    }
    return (*n.borrow().as_ref().unwrap()) * fact(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) - 1))));
}

fn main() {
    println!("{}", format!("{}", fact(Rc::new(RefCell::new(Some(7))))));
}