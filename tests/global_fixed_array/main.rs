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

pub(crate) static labels: GoGlobal<[String; 40]> = GoGlobal::new();


pub(crate) fn __go_init_globals() {
    *labels.borrow_mut() = Some(std::array::from_fn(|_| String::new()));
}


pub fn __go_init_0() {
    { let __range_holder = labels.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for i in 0..__range_values.len() {
        (*labels.borrow_mut().as_mut().unwrap())[i] = (*Rc::new(RefCell::new(Some(format!("label-{}", i)))).borrow().as_ref().unwrap()).clone();
    } }
}

fn main() {
    __go_init_all();
    println!("{} {} {}", (*labels.borrow().as_ref().unwrap())[(0) as usize].clone(), (*labels.borrow().as_ref().unwrap())[(1) as usize].clone(), (*labels.borrow().as_ref().unwrap())[(39) as usize].clone());
}

pub(crate) fn __go_init_all() {
    __go_init_globals();
    __go_init_0();
}
