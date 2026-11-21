use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, GoSliceView, Vec};

use crate::{shape::{Shape}};

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
pub struct Circle {
    pub r: Rc<RefCell<Option<i64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
        }
    }
}


impl Default for Circle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            r: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Circle {
    pub fn area(&self) -> i64 {
        return (*self.r.borrow().as_ref().unwrap()) * (*self.r.borrow().as_ref().unwrap());
    }
}

impl Shape for Circle {
    fn area(&self) -> i64 {
        Circle::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CirclePtr(pub Rc<RefCell<Option<Circle>>>);

impl std::fmt::Display for CirclePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for CirclePtr {
    fn area(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CirclePtr>() {
            let __go_left_nil = self.0.borrow().is_none();
            let __go_right_nil = __other.0.borrow().is_none();
            if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&self.0, &__other.0) }
        } else {
            false
        }
    }
}

pub fn new_circle(r: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Box<dyn Shape>>>> {
    Rc::new(RefCell::new(Some(Box::new(Circle { r: Rc::new(RefCell::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }) as Box<dyn Shape>)))
}