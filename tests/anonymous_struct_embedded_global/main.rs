use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Payload {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl Payload {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for Payload {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
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

pub(crate) static holder: GoGlobal<AnonymousStruct1> = GoGlobal::new();


fn __go_init_globals() {
    *holder.borrow_mut() = Some(Default::default());
}


fn main() {
    __go_init_all();
    println!("{} {}", format!("{}", (*(*(*holder.borrow().as_ref().unwrap()).payload.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*holder.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap())));
    { let new_val = 7; *(*(*holder.borrow_mut().as_mut().unwrap()).payload.borrow_mut().as_mut().unwrap()).n.borrow_mut() = Some(new_val); };
    { let new_val = 2; *(*holder.borrow().as_ref().unwrap()).version.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", (*(*(*holder.borrow().as_ref().unwrap()).payload.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*holder.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap())));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub payload: Rc<RefCell<Option<Payload>>>,
    pub version: Rc<RefCell<Option<i64>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.payload.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.version.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            payload: __go_clone_0_0,
            version: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Payload::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            payload: __go_default_0_0,
            version: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.payload.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.version.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) type holder = AnonymousStruct1;


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
