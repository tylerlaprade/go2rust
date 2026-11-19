use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub mod io {
    use super::*;
    pub fn EOF() -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }
}


pub fn same(err: Rc<RefCell<Option<Box<dyn StdError>>>>) -> bool {
    { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq }
}

fn main() {
    println!("{}", format!("{}", same({ let __field = io::EOF().clone(); __field })));
}