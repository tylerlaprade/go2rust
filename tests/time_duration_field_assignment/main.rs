use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut delay = Rc::new(RefCell::new(Some(std::time::Duration::from_nanos(0))));
    { let new_val = std::time::Duration::from_secs(30); *delay.borrow_mut() = Some(new_val); };
    println!("{}", (*delay.borrow().as_ref().unwrap()) == std::time::Duration::from_secs(30));
}