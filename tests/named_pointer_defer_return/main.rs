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
        let __go_clone_0_0 = { let __guard = self.complete.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            complete: __go_clone_0_0,
        }
    }
}


impl Default for Package {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            complete: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.complete.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Package {
    pub fn complete(&self) -> bool {
        return (*self.complete.borrow().as_ref().unwrap());
    }
}

pub fn import(path: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<Package>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut pkg: Rc<RefCell<Option<Package>>> = Rc::new(RefCell::new(None));
    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    let mut unsafePkg = Rc::new(RefCell::new(Some(Package { complete: Rc::new(RefCell::new(Some(true))), ..Default::default() })));
    if (*path.borrow().as_ref().unwrap()).clone() == "unsafe".to_string() {
        {
        { let new_val = unsafePkg.clone(); pkg = new_val; };
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (pkg.clone(), err.clone());
    }
    }
    {
        *pkg.borrow_mut() = None;;
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (pkg.clone(), err.clone());
    }
}

fn main() {
    let (mut pkg, mut err) = import(Rc::new(RefCell::new(Some("unsafe".to_string()))));
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        panic!("{}", (*err.borrow().as_ref().unwrap()));
    }
    {
            let __go_print_arg_0 = format!("{}", (*pkg.borrow().as_ref().unwrap()).complete());
            eprintln!("{}", __go_print_arg_0)
        };
}