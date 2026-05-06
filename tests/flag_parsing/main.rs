use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut name = Rc::new(RefCell::new(Some("World".to_string())));
    ();
    print!("Hello {}!\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v });
}