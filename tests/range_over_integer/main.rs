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
    pub fn len(&self) -> i32 {
        (*self.n.borrow().as_ref().unwrap())
    }
}

fn main() {
    let mut literalSum = Rc::new(RefCell::new(Some(0)));
    for i in 0..(5) {
        { let __rhs = i; let mut guard = literalSum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    let mut count = Rc::new(RefCell::new(Some(0)));
    for _ in 0..(3) {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut n = Rc::new(RefCell::new(Some(4)));
    let mut variableSum = Rc::new(RefCell::new(Some(0)));
    for i in 0..((*n.borrow().as_ref().unwrap())) {
        { let __rhs = i; let mut guard = variableSum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    let mut methodSum = Rc::new(RefCell::new(Some(0)));
    let mut c = Rc::new(RefCell::new(Some(counter { n: Rc::new(RefCell::new(Some(4 as i32))), ..Default::default() })));
    for i in 0..((*c.borrow().as_ref().unwrap()).len()) {
        { let __rhs = i; let mut guard = methodSum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    println!("{} {} {} {}", format!("{}", { let __v = (*literalSum.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*variableSum.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*methodSum.borrow().as_ref().unwrap()).clone(); __v }));
}