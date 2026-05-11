use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct exporter {
    pub indent: Rc<RefCell<Option<i32>>>,
}

impl exporter {
    pub fn __go_value_clone(&self) -> Self {
        Self { indent: { let __guard = self.indent.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for exporter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.indent.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct writer {
    pub p: Rc<RefCell<Option<exporter>>>,
}

impl writer {
    pub fn __go_value_clone(&self) -> Self {
        Self { p: self.p.clone() }
    }
}

impl std::fmt::Display for writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.p.borrow().as_ref().unwrap()))
    }
}


impl writer {
    pub fn do_trace(&mut self) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        { let __target = (*self.p.borrow().as_ref().unwrap()).indent.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut w_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        { let __target = (*w_defer_captured.p.borrow().as_ref().unwrap()).indent.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }
}

fn main() {
    let mut w = Rc::new(RefCell::new(Some(writer { p: Rc::new(RefCell::new(Some(exporter { indent: Rc::new(RefCell::new(Some(0))) }))).clone(), ..Default::default() })));
    (*w.borrow_mut().as_mut().unwrap()).do_trace();
    println!("{}", (*(*(*w.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).indent.borrow().as_ref().unwrap()));
}