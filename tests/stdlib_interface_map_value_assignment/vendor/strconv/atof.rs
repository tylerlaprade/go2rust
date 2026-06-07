use go2rust_stdlib_stubs::*;

use crate::{GoInteger, go_integer_add_one, go_integer_cast, go_integer_from_i128, go_integer_sub_one};

use crate::atob::*;
use crate::atoc::*;
use crate::atoi::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::itoa::*;
use crate::quote::*;

use std::sync::{Arc, Mutex};

pub(crate) const FN_PARSE_FLOAT: &'static str = "ParseFloat";


pub(crate) static optimize: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static powtab: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static float64pow10: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<f64>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static float32pow10: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<f32>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *optimize.lock().unwrap() = Some(false);
    *powtab.lock().unwrap() = Some(vec![]);
    *float64pow10.lock().unwrap() = Some(vec![]);
    *float32pow10.lock().unwrap() = Some(vec![]);
    *optimize.lock().unwrap() = Some(true);
    *powtab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![1, 3, 6, 9, 13, 16, 19, 23, 26]))).lock().unwrap().as_ref().unwrap()).clone());
    *float64pow10.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22]))).lock().unwrap().as_ref().unwrap()).clone());
    *float32pow10.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *optimize.lock().unwrap() = Some(false);
    *powtab.lock().unwrap() = Some(vec![]);
    *float64pow10.lock().unwrap() = Some(vec![]);
    *float32pow10.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_0() {
    *optimize.lock().unwrap() = Some(true);
}


pub(crate) fn __go_init_order_1() {
    *powtab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![1, 3, 6, 9, 13, 16, 19, 23, 26]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_2() {
    *float64pow10.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_3() {
    *float32pow10.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
