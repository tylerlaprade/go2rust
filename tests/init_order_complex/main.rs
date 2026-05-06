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

pub(crate) static a: GoGlobal<i32> = GoGlobal::new();

pub(crate) static b: GoGlobal<i32> = GoGlobal::new();

pub(crate) static c: GoGlobal<i32> = GoGlobal::new();

pub(crate) static d: GoGlobal<i32> = GoGlobal::new();


pub(crate) fn __go_init_globals() {
    *a.borrow_mut() = Some(0);
    *b.borrow_mut() = Some(0);
    *c.borrow_mut() = Some(0);
    *d.borrow_mut() = Some(0);
    *d.borrow_mut() = Some(3);
    *b.borrow_mut() = Some((*f().borrow().as_ref().unwrap()).clone());
    *c.borrow_mut() = Some((*f().borrow().as_ref().unwrap()).clone());
    *a.borrow_mut() = Some((*c.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap()));
}


pub fn f() -> Rc<RefCell<Option<i32>>> {

    { let mut guard = d.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return d.clone();
}

pub fn __go_init_0() {
    println!("{}", "First init".to_string());
    { let mut guard = d.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
}

pub fn __go_init_1() {
    println!("{}", "Second init".to_string());
    print!("a={}, b={}, c={}, d={}\n", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*d.borrow().as_ref().unwrap()).clone(); __v });
}

fn main() {
    __go_init_all();
    print!("In main: a={}, b={}, c={}, d={}\n", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*d.borrow().as_ref().unwrap()).clone(); __v });
}

pub(crate) fn __go_init_all() {
    __go_init_globals();
    __go_init_0();
    __go_init_1();
}
