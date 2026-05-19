use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct counter {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for counter {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl counter {
    pub fn inc(&mut self) {
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn start(&mut self) {
        let mut c_closure_clone = self.clone(); run(Rc::new(RefCell::new(Some(Box::new(move || {
        c_closure_clone.inc();
    }) as Box<dyn FnMut() -> ()>))));
    }
}

pub fn run(f: Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>) {
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(counter { n: Rc::new(RefCell::new(Some(0))) })));
    (*c.borrow_mut().as_mut().unwrap()).start();
    println!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));
}