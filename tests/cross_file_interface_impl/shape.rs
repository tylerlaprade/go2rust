use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// Shape is declared here; its only implementor (Circle) lives in circle.go.
/// The `impl Shape for Circle` must be generated even though they are in
/// different files of the same package.
pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool;
    fn area(&self) -> i64;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

pub fn total_area(shapes: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Shape>>>>>>>>) -> i64 {
    let mut sum = Rc::new(RefCell::new(Some(0i64)));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        { let __rhs = (*s.borrow().as_ref().unwrap()).area(); let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*sum.borrow().as_ref().unwrap());
}