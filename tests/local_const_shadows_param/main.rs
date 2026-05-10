use std::cell::{RefCell};
use std::rc::{Rc};

pub fn seed() {
    const flag: bool = true;

    let _ = flag;
}

pub fn check(flag: Rc<RefCell<Option<bool>>>) {
    if !(*flag.borrow().as_ref().unwrap()) {
        println!("{}", "off".to_string());
    }
}

fn main() {
    seed();
    check(Rc::new(RefCell::new(Some(false))));
}