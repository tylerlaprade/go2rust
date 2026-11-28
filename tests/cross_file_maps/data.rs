use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
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

pub static Users: GoGlobalMap<BTreeMap<String, Rc<RefCell<Option<i64>>>>> = GoGlobalMap::new();

pub static Numbers: GoGlobal<Vec<i64>> = GoGlobal::new();

pub static Groups: GoGlobalMap<BTreeMap<String, Rc<RefCell<Option<Vec<String>>>>>> = GoGlobalMap::new();

pub static Records: GoGlobal<Vec<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Box<dyn Any>>>>>>>>>> = GoGlobal::new();


fn __go_init_globals() {
    self::__go_zero_globals();
    self::__go_init_order_0();
    self::__go_init_order_1();
    self::__go_init_order_2();
    self::__go_init_order_3();
}


pub(crate) fn __go_zero_globals() {
    *Users.borrow_mut() = Some(BTreeMap::new());
    *Numbers.borrow_mut() = Some(Vec::nil());
    *Groups.borrow_mut() = Some(BTreeMap::new());
    *Records.borrow_mut() = Some(Vec::nil());
}


pub(crate) fn __go_init_order_0() {
    {
        let mut __go_map = BTreeMap::<String, Rc<RefCell<Option<i64>>>>::new();
        __go_map.insert("alice".to_string(), Rc::new(RefCell::new(Some(1))));
        __go_map.insert("bob".to_string(), Rc::new(RefCell::new(Some(2))));
        __go_map.insert("carol".to_string(), Rc::new(RefCell::new(Some(3))));
        *Users.borrow_mut() = Some(__go_map);
    }
}


pub(crate) fn __go_init_order_1() {
    {
        let mut __go_slice = Vec::<i64>::with_capacity(5);
        __go_slice.push(10i64);
        __go_slice.push(20i64);
        __go_slice.push(30i64);
        __go_slice.push(40i64);
        __go_slice.push(50i64);
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *Numbers.borrow_mut() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_order_2() {
    {
        let mut __go_map = BTreeMap::<String, Rc<RefCell<Option<Vec<String>>>>>::new();
        let __go_map_key_372 = "admins".to_string();
        let mut __go_map_value_372 = Vec::<String>::new();
        __go_map_value_372.push("alice".to_string());
        __go_map_value_372.push("bob".to_string());
        __go_map.insert(__go_map_key_372, Rc::new(RefCell::new(Some(__go_map_value_372))));
        let __go_map_key_401 = "users".to_string();
        let mut __go_map_value_401 = Vec::<String>::new();
        __go_map_value_401.push("carol".to_string());
        __go_map_value_401.push("dave".to_string());
        __go_map_value_401.push("eve".to_string());
        __go_map.insert(__go_map_key_401, Rc::new(RefCell::new(Some(__go_map_value_401))));
        *Groups.borrow_mut() = Some(__go_map);
    }
}


pub(crate) fn __go_init_order_3() {
    {
        let mut __go_slice = Vec::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Box<dyn Any>>>>>>>>>::with_capacity(2);
        __go_slice.push(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("name".to_string(), Rc::new(RefCell::new(Some(Box::new("Alice".to_string()) as Box<dyn Any>)))), ("age".to_string(), Rc::new(RefCell::new(Some(Box::new(30i64) as Box<dyn Any>))))])))));
        __go_slice.push(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("name".to_string(), Rc::new(RefCell::new(Some(Box::new("Bob".to_string()) as Box<dyn Any>)))), ("age".to_string(), Rc::new(RefCell::new(Some(Box::new(25i64) as Box<dyn Any>))))])))));
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *Records.borrow_mut() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all_data() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
    });
}
