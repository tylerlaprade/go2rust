use crate::circle::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

/// Shape is declared here; its only implementor (Circle) lives in circle.go.
/// The `impl Shape for Circle` must be generated even though they are in
/// different files of the same package.
pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool;
    fn area(&self) -> i32;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.__go_clone_box_shape()
    }
}

pub fn total_area(shapes: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Shape>>>>>>>>) -> i32 {
    let mut sum = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        { let __rhs = (*s.borrow().as_ref().unwrap()).area(); let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*sum.borrow().as_ref().unwrap());
}