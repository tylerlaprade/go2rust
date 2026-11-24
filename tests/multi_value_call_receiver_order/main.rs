#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod collide;
use collide::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct acc {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl acc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for acc {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for acc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl acc {
    pub fn add(&mut self, x: Rc<RefCell<Option<i64>>>, y: Rc<RefCell<Option<i64>>>) -> i64 {
        println!("{} {} {}", format!("{}", "add".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }));
        { let __target = self.n.clone(); let __rhs = (*x.borrow().as_ref().unwrap()) + (*y.borrow().as_ref().unwrap()); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return (*self.n.borrow().as_ref().unwrap());
    }
}

pub fn pair() -> (i64, i64) {
    println!("{}", format!("{}", "pair".to_string()));
    (1, 2)
}

pub fn recv(a: Rc<RefCell<Option<acc>>>) -> Rc<RefCell<Option<acc>>> {
    println!("{}", format!("{}", "recv".to_string()));
    a.clone()
}

pub fn note() {
    println!("{}", format!("{}", "deferred".to_string()));
}

pub fn compute() -> i64 {
    println!("{}", format!("{}", "compute".to_string()));
    7
}

pub fn plain() -> i64 {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    __defer_stack.push(Box::new(move || {
        note();
    }));
    {
        let __return_value = compute();
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return __return_value;
    }
}

pub fn via_multi(a: Rc<RefCell<Option<acc>>>) -> i64 {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    __defer_stack.push(Box::new(move || {
        note();
    }));
    let (mut __go_mv_2_0, mut __go_mv_2_1) = pair();
    {
        let __return_value = (*a.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(__go_mv_2_0))), Rc::new(RefCell::new(Some(__go_mv_2_1))));
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return __return_value;
    }
}

fn main() {
    collide();
    let mut a = Rc::new(RefCell::new(Some(acc { n: Rc::new(RefCell::new(Some(0))) })));
    println!("{}", format!("{}", plain()));
    println!("{}", format!("{}", via_multi(a.clone())));
    let mut __go_mv_3_r = recv(a.clone());
    let (mut __go_mv_3_0, mut __go_mv_3_1) = pair();
    (*__go_mv_3_r.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(__go_mv_3_0))), Rc::new(RefCell::new(Some(__go_mv_3_1))));
    let mut __go_mv_1_0 = Rc::new(RefCell::new(Some(5i64)));
    let (mut __go_mv_4_0, mut __go_mv_4_1) = divmod(Rc::new(RefCell::new(Some(7))), Rc::new(RefCell::new(Some(3))));
    println!("{} {}", format!("{}", __go_mv_4_0), format!("{}", __go_mv_4_1));
    println!("{}", format!("{}", { let __v = (*__go_mv_1_0.borrow().as_ref().unwrap()).clone(); __v }));
}

pub fn divmod(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    ((*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()) % (*b.borrow().as_ref().unwrap()))
}