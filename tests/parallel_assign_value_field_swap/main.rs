use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct pair {
    pub begin: Rc<RefCell<Option<i32>>>,
    pub end: Rc<RefCell<Option<i32>>>,
}

impl pair {
    pub fn __go_value_clone(&self) -> Self {
        Self { begin: { let __guard = self.begin.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, end: { let __guard = self.end.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for pair {
    fn default() -> Self {
        Self { begin: Rc::new(RefCell::new(Some(0))), end: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.begin.borrow().as_ref().unwrap()), (*self.end.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut p = Rc::new(RefCell::new(Some(pair { begin: Rc::new(RefCell::new(Some(1 as i32))), end: Rc::new(RefCell::new(Some(2 as i32))), ..Default::default() })));
    let mut x = Rc::new(RefCell::new(Some(9)));
    { let __tmp_0 = (*(*p.borrow().as_ref().unwrap()).end.borrow().as_ref().unwrap()); let __tmp_1 = (*x.borrow().as_ref().unwrap()); let __tmp_2 = (*(*p.borrow().as_ref().unwrap()).begin.borrow().as_ref().unwrap()); *x.borrow_mut() = Some(__tmp_0); *(*p.borrow().as_ref().unwrap()).begin.borrow_mut() = Some(__tmp_1); *(*p.borrow().as_ref().unwrap()).end.borrow_mut() = Some(__tmp_2); };
    eprintln!("{}", format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));
    eprintln!("{}", format!("{}", (*(*p.borrow().as_ref().unwrap()).begin.borrow().as_ref().unwrap())));
    eprintln!("{}", format!("{}", (*(*p.borrow().as_ref().unwrap()).end.borrow().as_ref().unwrap())));
}