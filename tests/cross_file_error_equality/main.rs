#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod methods;
mod types;
use methods::*;
use types::*;

use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
fn main() {
    let mut pointer = Rc::new(RefCell::new(Some(splitError { code: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_splitError(pointer.clone())) as Rc<dyn StdError>)));
    let (mut asserted, mut ok) = ({
        let val = err.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<__go_error_ptr_splitError>() {
                (typed_val.0.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<splitError>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<splitError>)), false)
        }
    });
    println!("{} {} {} {}", format!("{}", { let __left = err.clone(); let __right = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_splitError(pointer.clone())) as Rc<dyn StdError>))); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_splitError>(), __r.downcast_ref::<__go_error_ptr_splitError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", ok), format!("{}", { let __left = asserted.clone(); let __right = pointer.clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right); __eq }), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
}