use go2rust_stdlib_stubs::*;

use crate::{go_float_string};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// Counter holds a numeric value
#[derive(Debug, Clone)]
pub struct Counter {
    pub value: Arc<Mutex<Option<i64>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// Point represents a 2D point
#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<f64>>>,
    pub y: Arc<Mutex<Option<f64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = go_float_string(((*self.x.lock().unwrap().as_ref().unwrap())) as f64, false);
        let __go_fmt_1 = go_float_string(((*self.y.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl GoValueClone for Counter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
