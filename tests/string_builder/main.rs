use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut builder: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));
    (*builder.borrow_mut().as_mut().unwrap()).push_str("Hello");
    (*builder.borrow_mut().as_mut().unwrap()).push_str(" ");
    (*builder.borrow_mut().as_mut().unwrap()).push_str("World");
    let mut result = Rc::new(RefCell::new(Some((*builder.borrow().as_ref().unwrap()).clone())));
    println!("{}", (*result.borrow().as_ref().unwrap()));
}