use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub mod io {
    use super::*;
    pub fn EOF() -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }
}


pub fn same(err: Rc<RefCell<Option<Box<dyn StdError>>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some((*err.borrow()).is_none() == (*io::EOF().borrow()).is_none())));
}

fn main() {
    println!("{}", (*same(io::EOF()).borrow().as_ref().unwrap()));
}