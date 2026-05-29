use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct parser {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl parser {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for parser {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl parser {
    /// work defers a closure that captures the pointer receiver, then keeps using
    /// the receiver afterward. The deferred closure must capture a clone of the
    /// receiver handle, not move `self`, so the later `p.n = 42` still compiles.
    /// (go/parser's trace/un defers hit this on nearly every parse method.)
    pub fn work(&mut self) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut p_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{} {}", format!("{}", "deferred".to_string()), format!("{}", (*p_defer_captured.n.borrow().as_ref().unwrap())));
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        { let new_val = 42; *self.n.borrow_mut() = Some(new_val); };
        println!("{} {}", format!("{}", "body".to_string()), format!("{}", (*self.n.borrow().as_ref().unwrap())));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(parser { n: Rc::new(RefCell::new(Some(1 as i32))), ..Default::default() })));
    (*p.borrow_mut().as_mut().unwrap()).work();
}