use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct exporter {
    pub indent: Rc<RefCell<Option<i32>>>,
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
    }) as Box<dyn Fn() -> ()>))); let __f_guard = __f_holder.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
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