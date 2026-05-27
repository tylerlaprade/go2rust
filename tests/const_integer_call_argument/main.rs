use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub(crate) const DELTA_NEW_FILE: i32 = -64;


pub(crate) const BUNDLE_VERSION: i32 = 1;


#[derive(Debug, Clone, Default)]
pub struct writer {
}

impl writer {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl writer {
    pub fn int64(&self, x: Rc<RefCell<Option<i64>>>) -> i64 {
        return (*x.borrow().as_ref().unwrap());
    }
}

pub fn take_int64(x: Rc<RefCell<Option<i64>>>) -> i64 {

    return (*x.borrow().as_ref().unwrap());
}

pub fn take_uint64(x: Rc<RefCell<Option<u64>>>) -> u64 {

    return (*x.borrow().as_ref().unwrap());
}

fn main() {
    let mut w: Rc<RefCell<Option<writer>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{} {} {}", format!("{}", take_int64(Rc::new(RefCell::new(Some(DELTA_NEW_FILE as i64))))), format!("{}", (*w.borrow().as_ref().unwrap()).int64(Rc::new(RefCell::new(Some(DELTA_NEW_FILE as i64))))), format!("{}", take_uint64(Rc::new(RefCell::new(Some(BUNDLE_VERSION as u64))))));
}