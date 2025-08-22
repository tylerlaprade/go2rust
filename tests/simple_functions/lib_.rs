use std::cell::{RefCell};
use std::rc::{Rc};

pub fn get_hello() -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some("Hello".to_string())));
}

pub fn get_world() -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some("World".to_string())));
}

pub fn get_magic_number() -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some(42)));
}