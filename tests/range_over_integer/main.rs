use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct counter {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for counter {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl counter {
    pub fn len(&self) -> i64 {
        return (*self.n.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut literalSum = Rc::new(RefCell::new(Some(0i64)));
    for i in 0..(5) {
        { let __rhs = i; let mut guard = literalSum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    let mut count = Rc::new(RefCell::new(Some(0i64)));
    for _ in 0..(3) {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut n = Rc::new(RefCell::new(Some(4i64)));
    let mut variableSum = Rc::new(RefCell::new(Some(0i64)));
    for i in 0..({ let __range_len = (*n.borrow().as_ref().unwrap()); __range_len }) {
        { let __rhs = i; let mut guard = variableSum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    let mut methodSum = Rc::new(RefCell::new(Some(0i64)));
    let mut c = Rc::new(RefCell::new(Some(counter { n: Rc::new(RefCell::new(Some(4))), ..Default::default() })));
    for i in 0..({ let __range_len = (*c.borrow().as_ref().unwrap()).len(); __range_len }) {
        { let __rhs = i; let mut guard = methodSum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    println!("{} {} {} {}", format!("{}", { let __v = (*literalSum.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*variableSum.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*methodSum.borrow().as_ref().unwrap()).clone(); __v }));
}