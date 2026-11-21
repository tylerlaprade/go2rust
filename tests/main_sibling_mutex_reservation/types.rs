use crate::{GoChannel, GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, GoSliceView, Vec};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
#[derive(Debug, Clone)]
pub struct Mutex {
    pub value: Arc<StdMutex<Option<i64>>>,
}

impl Mutex {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for Mutex {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Mutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn new_mutex() -> Arc<StdMutex<Option<Mutex>>> {
    Arc::new(StdMutex::new(Some(Mutex { value: Arc::new(StdMutex::new(Some(7))), ..Default::default() })))
}