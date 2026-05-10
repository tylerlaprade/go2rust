use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut builder: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*builder.borrow_mut().as_mut().unwrap()).push_str("Hello");
    (*builder.borrow_mut().as_mut().unwrap()).push_str(" ");
    (*builder.borrow_mut().as_mut().unwrap()).push_str("World");
    let mut result = Rc::new(RefCell::new(Some((*builder.borrow().as_ref().unwrap()).clone())));
    println!("{}", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });
}