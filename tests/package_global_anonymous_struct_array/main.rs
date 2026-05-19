use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
struct AnonymousStruct1 {
    value: Rc<RefCell<Option<i32>>>,
    name: Rc<RefCell<Option<String>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))), name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.value.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()))
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

pub(crate) static modes: GoGlobal<[AnonymousStruct1; 2]> = GoGlobal::new();


fn __go_init_globals() {
    *modes.borrow_mut() = Some(std::array::from_fn(|_| Default::default()));
    *modes.borrow_mut() = Some((*Rc::new(RefCell::new(Some([AnonymousStruct1 { value: Rc::new(RefCell::new(Some(1))), name: Rc::new(RefCell::new(Some("one".to_string()))), ..Default::default() }, AnonymousStruct1 { value: Rc::new(RefCell::new(Some(2))), name: Rc::new(RefCell::new(Some("two".to_string()))), ..Default::default() }]))).borrow().as_ref().unwrap()).clone());
}


fn main() {
    __go_init_all();
    { let __range_holder = modes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for item in __range_values.iter() {
        println!("{} {}", (*item.name.borrow().as_ref().unwrap()).clone(), (*item.value.borrow().as_ref().unwrap()));
    } }
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
