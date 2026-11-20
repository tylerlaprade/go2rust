use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
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

pub(crate) static modes: GoGlobal<[AnonymousStruct1; 2]> = GoGlobal::new();


fn __go_init_globals() {
    *modes.borrow_mut() = Some(std::array::from_fn::<_, 2, _>(|_| Default::default()));
    {
        let mut __go_array = Vec::<AnonymousStruct1>::with_capacity(2);
        __go_array.push(AnonymousStruct1 { value: Rc::new(RefCell::new(Some(1))), name: Rc::new(RefCell::new(Some("one".to_string()))), ..Default::default() });
        __go_array.push(AnonymousStruct1 { value: Rc::new(RefCell::new(Some(2))), name: Rc::new(RefCell::new(Some("two".to_string()))), ..Default::default() });
        let __go_array: [AnonymousStruct1; 2] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *modes.borrow_mut() = Some(__go_array);
    }
}


fn main() {
    __go_init_all();
    { let __range_holder = modes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        let item = &item.__go_value_clone();
        println!("{} {}", format!("{}", (*item.name.borrow().as_ref().unwrap()).clone()), format!("{}", (*item.value.borrow().as_ref().unwrap())));
    } }
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub value: Rc<RefCell<Option<i32>>>,
    pub name: Rc<RefCell<Option<String>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
            name: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            value: __go_default_0_0,
            name: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
