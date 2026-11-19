use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub mod io {
    use super::*;
    pub fn EOF() -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Rc<dyn StdError>>))
    }
}


pub fn same(err: Rc<RefCell<Option<Rc<dyn StdError>>>>) -> bool {
    { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }
}

fn main() {
    println!("{}", format!("{}", same({ let __err_source = { let __field = io::EOF().clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) })));
}