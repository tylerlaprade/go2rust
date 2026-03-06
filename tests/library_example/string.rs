use std::cell::{RefCell};
use std::rc::{Rc};

/// Repeat repeats a string n times
pub fn repeat(s: Rc<RefCell<Option<String>>>, n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {

    let mut result = Rc::new(RefCell::new(Some("".to_string())));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < (*n.borrow().as_ref().unwrap()) {
        (*result.borrow_mut().as_mut().unwrap()).push_str(&(*s.borrow().as_ref().unwrap()));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return result.clone();
}