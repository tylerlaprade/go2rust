use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static ErrUnset: GoGlobal<Box<dyn StdError>> = GoGlobal::new();

pub(crate) static ErrBoom: GoGlobal<Box<dyn StdError>> = GoGlobal::new();


pub(crate) fn __go_init_globals() {
    *ErrUnset.borrow_mut() = None;
    *ErrBoom.borrow_mut() = None;
    { let __rhs_holder = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("boom".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *ErrBoom.borrow_mut() = new_val; }
}


fn main() {
    __go_init_all();
    if (*ErrUnset.borrow()).is_none() {
        println!("{}", "unset".to_string());
    }
    if (*ErrBoom.borrow()).is_some() {
        println!("{}", format!("{}", (*ErrBoom.borrow().as_ref().unwrap())));
    }
}

pub(crate) fn __go_init_all() {
    __go_init_globals();
}
