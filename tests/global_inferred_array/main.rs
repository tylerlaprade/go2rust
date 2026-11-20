use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
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

pub(crate) static items: GoGlobal<[item; 2]> = GoGlobal::new();


fn __go_init_globals() {
    *items.borrow_mut() = Some(std::array::from_fn::<_, 2, _>(|_| Default::default()));
    {
        let mut __go_array = Vec::<item>::with_capacity(2);
        __go_array.push(item { name: Rc::new(RefCell::new(Some("first".to_string()))), ..Default::default() });
        __go_array.push(item { name: Rc::new(RefCell::new(Some("second".to_string()))), ..Default::default() });
        let __go_array: [item; 2] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *items.borrow_mut() = Some(__go_array);
    }
}


fn main() {
    __go_init_all();
    println!("{} {} {}", format!("{}", (*(*items.borrow().as_ref().unwrap())[(0) as usize].clone().name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*items.borrow().as_ref().unwrap())[(1) as usize].clone().name.borrow().as_ref().unwrap()).clone()), format!("{}", (*items.borrow().as_ref().unwrap()).len()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
