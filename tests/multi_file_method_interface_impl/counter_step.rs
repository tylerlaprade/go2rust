use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use crate::{iface::{Stepper}};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
#[derive(Debug, Clone)]
pub struct Counter {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Counter {
    pub fn step(&mut self) {
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl Stepper for Counter {
    fn step(&mut self) {
        Counter::step(self)
    }
    fn value(&self) -> i64 {
        Counter::value(self)
    }
    fn __go_clone_box_stepper(&self) -> Box<dyn Stepper> {
        Box::new(self.clone()) as Box<dyn Stepper>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stepper(&self, other: &dyn Stepper) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Counter>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CounterPtr(pub Rc<RefCell<Option<Counter>>>);

impl std::fmt::Display for CounterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Stepper for CounterPtr {
    fn step(&mut self) {
        let mut __recv_guard = self.0.borrow_mut();
        let __recv = __recv_guard.as_mut().unwrap();
        Counter::step(__recv)
    }
    fn value(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Counter::value(__recv)
    }
    fn __go_clone_box_stepper(&self) -> Box<dyn Stepper> {
        Box::new(self.clone()) as Box<dyn Stepper>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stepper(&self, other: &dyn Stepper) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CounterPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}