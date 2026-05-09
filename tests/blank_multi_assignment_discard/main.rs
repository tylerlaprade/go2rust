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

pub(crate) static n: GoGlobal<i32> = GoGlobal::new();


fn __go_init_globals() {
    *n.borrow_mut() = Some(0);
}


pub fn next() -> Rc<RefCell<Option<i32>>> {

    { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return n.clone();
}

fn main() {
    __go_init_all();
    { let __tmp_0 = next(); let __tmp_1 = next(); let __tmp_2 = next(); };
    println!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v });
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
