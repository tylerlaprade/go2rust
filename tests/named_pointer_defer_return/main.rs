use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Package {
    pub complete: Rc<RefCell<Option<bool>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { complete: { let __guard = self.complete.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Package {
    fn default() -> Self {
        Self { complete: Rc::new(RefCell::new(Some(false))) }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.complete.borrow().as_ref().unwrap()))
    }
}


impl Package {
    pub fn complete(&self) -> bool {
        (*self.complete.borrow().as_ref().unwrap())
    }
}

pub fn import(path: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<Package>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut pkg: Rc<RefCell<Option<Package>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    let mut unsafePkg = Rc::new(RefCell::new(Some(Package { complete: Rc::new(RefCell::new(Some(true))), ..Default::default() })));
    if (*path.borrow().as_ref().unwrap()).clone() == "unsafe" {
        {
        { let new_val = unsafePkg.clone(); pkg = new_val; };
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (pkg, err);
    }
    }
    {
        *pkg.borrow_mut() = None;;
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (pkg, err);
    }
}

fn main() {
    let (mut pkg, mut err) = import(Rc::new(RefCell::new(Some("unsafe".to_string()))));
    if (*err.borrow()).is_some() {
        panic!("{}", (*err.borrow().as_ref().unwrap()));
    }
    eprintln!("{}", format!("{}", (*pkg.borrow().as_ref().unwrap()).complete()));
}