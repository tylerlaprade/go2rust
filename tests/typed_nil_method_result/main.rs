use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Checker {
    pub cleaned: Rc<RefCell<Option<bool>>>,
}

impl Checker {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cleaned.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            cleaned: __go_clone_0_0,
        }
    }
}


impl Default for Checker {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            cleaned: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.cleaned.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Alias {
    pub id: Rc<RefCell<Option<i32>>>,
}

impl Alias {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.id.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            id: __go_clone_0_0,
        }
    }
}


impl Default for Alias {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            id: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.id.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Checker {
    pub fn new_alias(&mut self) -> Rc<RefCell<Option<Alias>>> {
        let mut a = Rc::new(RefCell::new(Some(Alias { id: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
        if true {
        { let new_val = true; *self.cleaned.borrow_mut() = Some(new_val); };
    }
        return a.clone();
    }
}

fn main() {
    println!("{}", format!("{}", (*(*__go_nil_recv_checker_new_alias(Rc::new(RefCell::new(None::<Checker>))).borrow().as_ref().unwrap()).id.borrow().as_ref().unwrap())));
    let mut check = Rc::new(RefCell::new(Some(Checker { cleaned: Rc::new(RefCell::new(Some(false))) })));
    println!("{}", format!("{}", (*(*__go_nil_recv_checker_new_alias({ let __recv = check.clone(); __recv }).borrow().as_ref().unwrap()).id.borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*(*check.borrow().as_ref().unwrap()).cleaned.borrow().as_ref().unwrap())));
}

pub fn __go_nil_recv_checker_new_alias(check: Rc<RefCell<Option<Checker>>>) -> Rc<RefCell<Option<Alias>>> {
    let mut a = Rc::new(RefCell::new(Some(Alias { id: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    if { let __nil_result = (*check.borrow()).is_some(); __nil_result } {
        { let new_val = true; *(*check.borrow().as_ref().unwrap()).cleaned.borrow_mut() = Some(new_val); };
    }
    return a.clone();
}