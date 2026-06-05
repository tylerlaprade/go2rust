use go2rust_stdlib_stubs::*;

use crate::{format_any, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values};

use crate::r#mod::*;
use crate::commentmap::*;
use crate::filter::*;
use crate::import::*;
use crate::resolve::*;
use crate::scope::*;
use crate::walk::*;

use std::sync::{Arc, Mutex};

pub(crate) static indent: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *indent.lock().unwrap() = Some(vec![]);
    *indent.lock().unwrap() = Some((*Arc::new(Mutex::new(Some((".  ".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *indent.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_1() {
    *indent.lock().unwrap() = Some((*Arc::new(Mutex::new(Some((".  ".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
