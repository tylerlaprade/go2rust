use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Counter {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Counter {
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

pub(crate) static shared: GoGlobal<Rc<RefCell<Option<Counter>>>> = GoGlobal::new();


fn __go_init_globals() {
    *shared.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *shared.borrow_mut() = Some(Rc::new(RefCell::new(Some(Counter { value: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() }))));
}


impl Counter {
    pub fn value(&self) -> i32 {
        return (*(*(*shared.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()) + (*self.value.borrow().as_ref().unwrap());
    }
}

fn main() {
    __go_init_all();
    println!("{}", format!("{}", { let __recv_holder = (*shared.borrow().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.borrow().as_ref().unwrap()).clone(); let __result = __recv_value.value(); __result }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
