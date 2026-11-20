use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub value: Rc<RefCell<Option<i64>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub type maker = Rc<RefCell<Option<Box<dyn FnMut() -> Rc<RefCell<Option<item>>>>>>>;


fn main() {
    let mut makeItem = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<item>>> {
        Rc::new(RefCell::new(Some(item { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })))
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<item>>>>)));
    let mut got = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<item>>>> = { let mut __f_guard = makeItem.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<item>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{}", format!("{}", (*(*got.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
}