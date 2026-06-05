use go2rust_stdlib_stubs::*;

use crate::abs::*;
use crate::acosh::*;
use crate::asin::*;
use crate::asinh::*;
use crate::atan::*;
use crate::atan2::*;
use crate::atanh::*;
use crate::bits::*;
use crate::cbrt::*;
use crate::r#const::*;
use crate::copysign::*;
use crate::dim::*;
use crate::dim_asm::*;
use crate::erf::*;
use crate::erfinv::*;
use crate::exp::*;
use crate::exp2_asm::*;
use crate::exp_asm::*;
use crate::expm1::*;
use crate::floor::*;
use crate::floor_asm::*;
use crate::fma::*;
use crate::frexp::*;
use crate::gamma::*;
use crate::hypot::*;
use crate::hypot_noasm::*;
use crate::j0::*;
use crate::j1::*;
use crate::jn::*;
use crate::ldexp::*;
use crate::lgamma::*;
use crate::log::*;
use crate::log10::*;
use crate::log1p::*;
use crate::log_stub::*;
use crate::logb::*;
use crate::r#mod::*;
use crate::modf::*;
use crate::modf_asm::*;
use crate::nextafter::*;
use crate::pow::*;
use crate::remainder::*;
use crate::signbit::*;
use crate::sin::*;
use crate::sincos::*;
use crate::sinh::*;
use crate::sqrt::*;
use crate::stubs::*;
use crate::tan::*;
use crate::tanh::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

pub(crate) static pow10tab: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 32]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pow10postab32: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 10]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pow10negtab32: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 11]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *pow10tab.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *pow10postab32.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *pow10negtab32.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *pow10tab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1e00, 1e01, 1e02, 1e03, 1e04, 1e05, 1e06, 1e07, 1e08, 1e09, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 1e23, 1e24, 1e25, 1e26, 1e27, 1e28, 1e29, 1e30, 1e31]))).lock().unwrap().as_ref().unwrap()).clone());
    *pow10postab32.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1e00, 1e32, 1e64, 1e96, 1e128, 1e160, 1e192, 1e224, 1e256, 1e288]))).lock().unwrap().as_ref().unwrap()).clone());
    *pow10negtab32.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1e-00, 1e-32, 1e-64, 1e-96, 1e-128, 1e-160, 1e-192, 1e-224, 1e-256, 1e-288, 1e-320]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *pow10tab.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *pow10postab32.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *pow10negtab32.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_42() {
    *pow10tab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1e00, 1e01, 1e02, 1e03, 1e04, 1e05, 1e06, 1e07, 1e08, 1e09, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 1e23, 1e24, 1e25, 1e26, 1e27, 1e28, 1e29, 1e30, 1e31]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_43() {
    *pow10postab32.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1e00, 1e32, 1e64, 1e96, 1e128, 1e160, 1e192, 1e224, 1e256, 1e288]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_44() {
    *pow10negtab32.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1e-00, 1e-32, 1e-64, 1e-96, 1e-128, 1e-160, 1e-192, 1e-224, 1e-256, 1e-288, 1e-320]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
