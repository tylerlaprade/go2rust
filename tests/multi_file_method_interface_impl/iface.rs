use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use std::any::Any;
use std::fmt::{Display};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// Stepper is implemented by Counter, whose methods are split across two files.
pub trait Stepper: std::fmt::Display + Any {
    fn __go_clone_box_stepper(&self) -> Box<dyn Stepper>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_stepper(&self, other: &dyn Stepper) -> bool;
    fn step(&mut self);
    fn value(&self) -> i64;
}

impl Clone for Box<dyn Stepper> {
    fn clone(&self) -> Self {
        Stepper::__go_clone_box_stepper(self.as_ref())
    }
}