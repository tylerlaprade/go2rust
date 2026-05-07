use std::cell::{RefCell};
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

pub(crate) static value: GoGlobal<i32> = GoGlobal::new();

pub(crate) static allValues: GoGlobal<i32> = GoGlobal::new();

pub(crate) static copiedValue: GoGlobal<i32> = GoGlobal::new();


pub(crate) fn __go_init_globals() {
    *value.borrow_mut() = Some(0);
    *allValues.borrow_mut() = Some(0);
    *copiedValue.borrow_mut() = Some(0);
    *allValues.borrow_mut() = Some(5);
}


pub fn print_value(value_local: Rc<RefCell<Option<i32>>>) {
    println!("{}", { let __v = (*value_local.borrow().as_ref().unwrap()).clone(); __v });
}

fn main() {
    __go_init_all();
    { let new_val = 3; *value.borrow_mut() = Some(new_val); };
    { let new_val = allValues.borrow().as_ref().unwrap().clone(); *copiedValue.borrow_mut() = Some(new_val); };
    print_value(Rc::new(RefCell::new(Some(7))));
    println!("{}", { let __v = (*value.borrow().as_ref().unwrap()).clone(); __v });
    println!("{}", { let __v = (*copiedValue.borrow().as_ref().unwrap()).clone(); __v });
}

pub(crate) fn __go_init_all() {
    __go_init_globals();
}
