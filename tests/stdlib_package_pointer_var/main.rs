use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Package;

impl std::fmt::Display for types_Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Package>")
    }
}


impl types_Package {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod types {
    use super::*;
    pub fn Unsafe() -> Rc<RefCell<Option<types_Package>>> {
        Rc::new(RefCell::new(Some::<types_Package>(Default::default())))
    }
}


fn main() {
    if false {
        let mut pkg: Rc<RefCell<Option<types_Package>>> = Rc::new(RefCell::new(None));
        { let new_val = types::Unsafe().clone(); pkg = new_val; };
        let _ = (*pkg.borrow().as_ref().unwrap());
    }

    println!("{}", "ok".to_string());
}