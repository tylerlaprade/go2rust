use std::cell::{RefCell};
use std::rc::{Rc};

pub fn get_hello() -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some("Hello".to_string())));
}

pub fn get_world() -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some("World".to_string())));
}

pub fn get_magic_number() -> i32 {

    return 42 as i32;
}