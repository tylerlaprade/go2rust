use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub next: Rc<RefCell<Option<node>>>,
    pub v: Rc<RefCell<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        Self { next: self.next.clone(), v: { let __guard = self.v.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for node {
    fn default() -> Self {
        Self { next: Rc::new(RefCell::new(None)), v: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.next.borrow().as_ref().unwrap()), (*self.v.borrow().as_ref().unwrap()))
    }
}


impl node {
    pub fn walk(&self) -> i32 {
        let mut y = Rc::new(RefCell::new(Some(self.clone())));
        let mut sum = Rc::new(RefCell::new(Some(0)));
        while (*y.borrow()).is_some() {
        { let __rhs = (*(*y.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()); let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = (*y.borrow().as_ref().unwrap()).next.clone(); y = new_val; };
    }
        return (*sum.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(node { v: Rc::new(RefCell::new(Some(3 as i32))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(node { v: Rc::new(RefCell::new(Some(2 as i32))), next: c.clone(), ..Default::default() })));
    let mut a = Rc::new(RefCell::new(Some(node { v: Rc::new(RefCell::new(Some(1 as i32))), next: b.clone(), ..Default::default() })));
    eprintln!("{}", format!("{}", (*a.borrow().as_ref().unwrap()).walk()));
}