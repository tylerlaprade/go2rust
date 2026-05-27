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

pub(crate) static enabled: GoGlobal<Rc<RefCell<Option<bool>>>> = GoGlobal::new();

pub(crate) static label: GoGlobal<Rc<RefCell<Option<String>>>> = GoGlobal::new();


fn __go_init_globals() {
    *enabled.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *label.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *enabled.borrow_mut() = Some(bool_ptr(Rc::new(RefCell::new(Some(true)))));
    *label.borrow_mut() = Some(string_ptr(Rc::new(RefCell::new(Some("ready".to_string())))));
}


pub fn bool_ptr(v: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<bool>>> {

    return v.clone();
}

pub fn string_ptr(v: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    return v.clone();
}

pub fn invert(v: Rc<RefCell<Option<bool>>>) -> bool {

    return !(*v.borrow().as_ref().unwrap());
}

pub fn suffix(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some(format!("{}{}", (*s.borrow().as_ref().unwrap()), "!".to_string()))));
}

fn main() {
    __go_init_all();
    println!("{}", format!("{}", (*(*enabled.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*(*label.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", invert(Rc::new(RefCell::new(Some((*(*enabled.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()).clone()))))));
    println!("{}", format!("{}", (*suffix(Rc::new(RefCell::new(Some((*(*label.borrow().as_ref().unwrap()).borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
