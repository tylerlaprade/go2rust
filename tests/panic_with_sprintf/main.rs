use std::cell::{RefCell};
use std::rc::{Rc};

pub fn check(n: Rc<RefCell<Option<i32>>>) {
    if (*n.borrow().as_ref().unwrap()) < 0 {
        panic!("invalid n {} (should be >= 0)", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v });
    }
    println!("{} {}", format!("{}", "ok".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));
}

fn main() {
    check(Rc::new(RefCell::new(Some(5))));
}