use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct lexer {
    pub pos: Rc<RefCell<Option<i32>>>,
}

impl lexer {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for lexer {
    fn default() -> Self {
        Self { pos: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.pos.borrow().as_ref().unwrap()))
    }
}


impl lexer {
    pub fn next(&mut self) -> i32 {
        { let __target = self.pos.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*self.pos.borrow().as_ref().unwrap());
    }

    pub fn scan_with(&self, f: Rc<RefCell<Option<Box<dyn FnMut() -> i32>>>>) -> i32 {
        { let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        { let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }
    }

    pub fn scan(&self) -> i32 {
        { let __method_arg0 = Rc::new(RefCell::new(Some({ let mut __recv = self.clone(); Box::new(move || -> i32 { __recv.next() }) as Box<dyn FnMut() -> i32> }))); self.scan_with(__method_arg0) }
    }
}

fn main() {
    let mut l = Rc::new(RefCell::new(Some(lexer { pos: Rc::new(RefCell::new(Some(0))) })));
    let mut r = (*l.borrow().as_ref().unwrap()).scan();
    eprintln!("{}", format!("{}", r));
    eprintln!("{}", format!("{}", (*(*l.borrow().as_ref().unwrap()).pos.borrow().as_ref().unwrap())));
}