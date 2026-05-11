use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct termSet {
    pub complete: Rc<RefCell<Option<bool>>>,
}

impl termSet {
    pub fn __go_value_clone(&self) -> Self {
        Self { complete: { let __guard = self.complete.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for termSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.complete.borrow().as_ref().unwrap()))
    }
}


pub fn compute() -> Rc<RefCell<Option<bool>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();


    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<termSet>>>>::new())));
    let mut tset = Rc::new(RefCell::new(Some(termSet { complete: Rc::new(RefCell::new(Some(false))) })));
    let tset_defer_captured = tset.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        { let new_val = true; *(*tset_defer_captured.borrow().as_ref().unwrap()).complete.borrow_mut() = Some(new_val); };
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    { let __map_key = "x".to_string(); let __map_value = tset.clone(); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Rc::new(RefCell::new(Some(false)))
    }
}

fn main() {
    println!("{}", (*compute().borrow().as_ref().unwrap()));
}