#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod helpers;
use helpers::*;

use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Part {
    pub a: Rc<RefCell<Option<i64>>>,
}

impl Part {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
        }
    }
}


impl Default for Part {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            a: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.borrow().as_ref().unwrap()));
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

pub(crate) struct GoGlobalMap<T> {
    value: std::cell::UnsafeCell<Option<std::rc::Rc<std::cell::RefCell<Option<T>>>>>,
}
unsafe impl<T> Sync for GoGlobalMap<T> {}
impl<T> GoGlobalMap<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    fn handle(&'static self) -> &'static std::rc::Rc<std::cell::RefCell<Option<T>>> {
        unsafe { (*self.value.get()).get_or_insert_with(|| std::rc::Rc::new(std::cell::RefCell::new(None))) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.handle().as_ptr() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.handle().as_ptr() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> {
        self.handle().clone()
    }
    // Assigning a map to the variable makes it that map's handle.
    pub(crate) fn set(&'static self, handle: std::rc::Rc<std::cell::RefCell<Option<T>>>) {
        unsafe { *self.value.get() = Some(handle) }
    }
}

pub(crate) static seen: GoGlobalMap<BTreeMap<Part, Rc<RefCell<Option<bool>>>>> = GoGlobalMap::new();


fn __go_init_globals() {
    self::__go_zero_globals();
    self::__go_init_order_0();
}


pub(crate) fn __go_zero_globals() {
    *seen.borrow_mut() = Some(BTreeMap::new());
}


pub(crate) fn __go_init_order_0() {
    {
        let mut __go_map = BTreeMap::<Part, Rc<RefCell<Option<bool>>>>::new();
        *seen.borrow_mut() = Some(__go_map);
    }
}


fn main() {
    go_install_panic_exit();
    __go_init_all();
    let mut keys = Rc::new(RefCell::new(Some(BTreeMap::<AnonymousStruct1, Rc<RefCell<Option<bool>>>>::from([]))));
    { let __map_key = { let __v = origin(); let __guard = __v.borrow(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned }; let __map_value = Rc::new(RefCell::new(Some(true))); (*keys.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = AnonymousStruct1 { x: Rc::new(RefCell::new(Some(1))) }; let __map_value = Rc::new(RefCell::new(Some(false))); (*keys.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = Part { a: Rc::new(RefCell::new(Some(1))), ..Default::default() }; let __map_value = Rc::new(RefCell::new(Some(true))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {}", format!("{}", ({ let __len = (*keys.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map_holder = keys.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&{ let __v = origin(); let __guard = __v.borrow(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned })).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false) }), format!("{}", ({ let __len = (*seen.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_zero_globals();
        self::__go_init_order_0();
    });
}
