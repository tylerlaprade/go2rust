use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct entry {
    pub deps: Rc<RefCell<Option<String>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        Self { deps: { let __guard = self.deps.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for entry {
    fn default() -> Self {
        Self { deps: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.deps.borrow().as_ref().unwrap()))
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

pub(crate) static entries: GoGlobal<[entry; 1]> = GoGlobal::new();


fn __go_init_globals() {
    *entries.borrow_mut() = Some(std::array::from_fn(|_| Default::default()));
    *entries.borrow_mut() = Some((*Rc::new(RefCell::new(Some([entry { deps: Rc::new(RefCell::new(Some("abc".to_string()))), ..Default::default() }]))).borrow().as_ref().unwrap()).clone());
}


fn main() {
    __go_init_all();
    let mut data = Rc::new(RefCell::new(Some(((*(*entries.borrow().as_ref().unwrap())[(0) as usize].clone().deps.borrow().as_ref().unwrap())).as_bytes().to_vec())));
    println!("{} {}", format!("{}", (*data.borrow().as_ref().unwrap()).len()), format!("{}", (*data.borrow().as_ref().unwrap())[(1) as usize].clone()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
