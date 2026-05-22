use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct state {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl state {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for state {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for state {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


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

pub(crate) static current: GoGlobal<Rc<RefCell<Option<state>>>> = GoGlobal::new();


fn __go_init_globals() {
    *current.borrow_mut() = Some(Rc::new(RefCell::new(None)));
}


fn main() {
    __go_init_all();
    println!("{}", format!("{}", { let __slot_guard = current.borrow(); let __not_nil = __slot_guard.as_ref().map(|__ptr| (*__ptr.borrow()).is_some()).unwrap_or(false); !__not_nil }));
    { let new_val = Rc::new(RefCell::new(Some(state { value: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() }))).clone(); *current.borrow_mut() = Some(new_val); };
    println!("{}", format!("{}", { let __slot_guard = current.borrow(); let __not_nil = __slot_guard.as_ref().map(|__ptr| (*__ptr.borrow()).is_some()).unwrap_or(false); __not_nil }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
