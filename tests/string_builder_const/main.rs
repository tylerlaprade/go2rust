use std::cell::{RefCell};
use std::rc::{Rc};

pub(crate) const SEPARATOR: &'static str = " | ";


fn main() {
    let mut b: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*b.borrow_mut().as_mut().unwrap()).push_str("left");
    (*b.borrow_mut().as_mut().unwrap()).push_str(SEPARATOR);
    (*b.borrow_mut().as_mut().unwrap()).push_str("right");
    println!("{}", (*Rc::new(RefCell::new(Some({ let __builder = b.clone(); let __guard = __builder.borrow(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value }))).borrow().as_ref().unwrap()));
}