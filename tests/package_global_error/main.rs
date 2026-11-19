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

pub static ErrUnset: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();

pub static ErrBoom: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();


fn __go_init_globals() {
    *ErrUnset.borrow_mut() = None;
    *ErrBoom.borrow_mut() = None;
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("boom".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *ErrBoom.borrow_mut() = new_val; }
}


fn main() {
    __go_init_all();
    if { let __nil_result = (*ErrUnset.borrow()).is_none(); __nil_result } {
        println!("{}", format!("{}", "unset".to_string()));
    }
    if { let __nil_result = (*ErrBoom.borrow()).is_some(); __nil_result } {
        println!("{}", format!("{}", { let __text = match ErrBoom.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
