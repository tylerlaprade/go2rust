#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct item {
    pub index: Arc<Mutex<Option<i64>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            index: __go_clone_0_0,
        }
    }
}


impl Default for item {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            index: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.index.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl item {
    pub fn index(&self) -> i64 {
        return (*self.index.lock().unwrap().as_ref().unwrap());
    }
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut v = Arc::new(Mutex::new(Some(item { index: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    println!("{}", format!("{}", (*strconv::itoa(Arc::new(Mutex::new(Some({ let __arg_value = (*v.lock().unwrap().as_ref().unwrap()).index(); __arg_value })))).lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for item {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
