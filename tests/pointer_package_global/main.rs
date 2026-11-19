use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Regression: a pointer-typed package global used as a map value must clone the
/// stored pointer handle, not the package-global slot that stores that handle.
#[derive(Debug, Clone)]
pub struct RangeTable {
    pub lo: Rc<RefCell<Option<i32>>>,
    pub hi: Rc<RefCell<Option<i32>>>,
}

impl RangeTable {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lo.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.hi.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            lo: __go_clone_0_0,
            hi: __go_clone_0_1,
        }
    }
}


impl Default for RangeTable {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            lo: __go_default_0_0,
            hi: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for RangeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lo.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.hi.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
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

pub(crate) static _C: GoGlobal<Rc<RefCell<Option<RangeTable>>>> = GoGlobal::new();

pub static C: GoGlobal<Rc<RefCell<Option<RangeTable>>>> = GoGlobal::new();

pub static Tables: GoGlobal<BTreeMap<String, Rc<RefCell<Option<RangeTable>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *_C.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *C.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *Tables.borrow_mut() = Some(BTreeMap::new());
    *_C.borrow_mut() = Some(Rc::new(RefCell::new(Some(RangeTable { lo: Rc::new(RefCell::new(Some(1))), hi: Rc::new(RefCell::new(Some(2))), ..Default::default() }))));
    *C.borrow_mut() = Some((*_C.borrow().as_ref().unwrap()).clone());
    {
        let mut __go_map = BTreeMap::<String, Rc<RefCell<Option<RangeTable>>>>::new();
        __go_map.insert("C".to_string(), (*C.borrow().as_ref().unwrap()).clone());
        *Tables.borrow_mut() = Some(__go_map);
    }
}


fn main() {
    __go_init_all();
    println!("{} {}", format!("{}", (*(*{ let __map_holder = Tables.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"C".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).lo.borrow().as_ref().unwrap())), format!("{}", (*(*{ let __map_holder = Tables.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"C".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).hi.borrow().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
