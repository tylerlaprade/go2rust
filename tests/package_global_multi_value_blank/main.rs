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

pub(crate) static enabled: GoGlobal<bool> = GoGlobal::new();


fn __go_init_globals() {
    *enabled.borrow_mut() = Some(false);
    let (__go_pkg_init_0, _) = parse_enabled();
    *enabled.borrow_mut() = Some(__go_pkg_init_0);
}


pub fn parse_enabled() -> (bool, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    (true, Rc::new(RefCell::new(None)))
}

fn main() {
    __go_init_all();
    println!("{}", format!("{}", { let __v = (*enabled.borrow().as_ref().unwrap()).clone(); __v }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
