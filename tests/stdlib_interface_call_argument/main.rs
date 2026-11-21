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

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
#[derive(Debug, Clone, Default)]
pub struct Walker {
}

impl Walker {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for Walker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Walker {
    pub fn has(&self, t: Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>) -> bool {
        true
    }
}

pub fn has(t: Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>) -> bool {
    true
}

fn main() {
    ::bytes::__go_init_all();
    ::cmp::__go_init_all();
    ::container_heap::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_constant::__go_init_all();
    ::go_token::__go_init_all();
    ::go_types::__go_init_all();
    ::go_version::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_buildcfg::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_goexperiment::__go_init_all();
    ::internal_gover::__go_init_all();
    ::internal_goversion::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_types_errors::__go_init_all();
    ::math::__go_init_all();
    ::math_big::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::slices::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut w: Arc<Mutex<Option<Walker>>> = Arc::new(Mutex::new(Some(Default::default())));

    println!("{}", format!("{}", has(Arc::new(Mutex::new(Some(Box::new(go_types::tuple::TuplePtr(go_types::new_tuple(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None))])))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))))));
    println!("{}", format!("{}", (*w.lock().unwrap().as_ref().unwrap()).has(Arc::new(Mutex::new(Some(Box::new(go_types::tuple::TuplePtr(go_types::new_tuple(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None))])))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))))));
}

impl GoValueClone for Walker {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
