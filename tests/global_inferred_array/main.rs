use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
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

pub(crate) static items: GoGlobal<[item; 2]> = GoGlobal::new();


fn __go_init_globals() {
    *items.borrow_mut() = Some(std::array::from_fn(|_| Default::default()));
    *items.borrow_mut() = Some((*Rc::new(RefCell::new(Some([item { name: Rc::new(RefCell::new(Some("first".to_string()))), ..Default::default() }, item { name: Rc::new(RefCell::new(Some("second".to_string()))), ..Default::default() }]))).borrow().as_ref().unwrap()).clone());
}


fn main() {
    __go_init_all();
    println!("{} {} {}", (*(*items.borrow().as_ref().unwrap())[(0) as usize].clone().name.borrow().as_ref().unwrap()), (*(*items.borrow().as_ref().unwrap())[(1) as usize].clone().name.borrow().as_ref().unwrap()), (*items.borrow().as_ref().unwrap()).len());
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
