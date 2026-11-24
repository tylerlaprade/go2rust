use go2rust_stdlib_stubs::*;

use crate::{go_float_string};

use crate::{types::{Counter, Point}};

use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
impl crate::types::Counter {
    /// Methods for Counter type
    pub fn increment(&mut self) {
        { let __target = self.value.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Arc<Mutex<Option<i64>>>) {
        { let __target = self.value.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    pub fn value(&self) -> i64 {
        return (*self.value.lock().unwrap().as_ref().unwrap());
    }
}

impl crate::types::Point {
    /// Methods for Point type
    pub fn distance(&self, other: Arc<Mutex<Option<Point>>>) -> f64 {
        let mut dx = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.x.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*other.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        let mut dy = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.y.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*other.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        return math::sqrt(Arc::new(Mutex::new(Some({ let __tmp_x = { let __bin_dx = ({ let __v = (*dx.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_dx * __bin_dx }; let __tmp_y = { let __bin_dy = ({ let __v = (*dy.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_dy * __bin_dy }; __tmp_x + __tmp_y }))));
    }

    pub fn r#move(&mut self, dx: Arc<Mutex<Option<f64>>>, dy: Arc<Mutex<Option<f64>>>) {
        { let __target = self.x.clone(); let __rhs = (*dx.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.y.clone(); let __rhs = (*dy.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}