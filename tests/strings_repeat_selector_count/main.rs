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
pub struct printer {
    pub indent: Arc<Mutex<Option<i64>>>,
}

impl printer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.indent.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            indent: __go_clone_0_0,
        }
    }
}


impl Default for printer {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            indent: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for printer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.indent.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut p = Arc::new(Mutex::new(Some(printer { indent: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    println!("{}", format!("{}", format!("{}{}", (*strings::repeat(Arc::new(Mutex::new(Some("..".to_string()))), Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).indent.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()), "x".to_string())));
}

impl GoValueClone for printer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
