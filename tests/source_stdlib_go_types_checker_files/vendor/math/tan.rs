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
use crate::pow10::*;
use crate::remainder::*;
use crate::signbit::*;
use crate::sin::*;
use crate::sincos::*;
use crate::sinh::*;
use crate::sqrt::*;
use crate::stubs::*;
use crate::tanh::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

pub(crate) static _tanP: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 3]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _tanQ: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_tanP.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_tanQ.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_tanP.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(1.30936939181383777646e4), 1.15351664838587416140e6, -(1.79565251976484877988e7)]))).lock().unwrap().as_ref().unwrap()).clone());
    *_tanQ.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.00000000000000000000e0, 1.36812963470692954678e4, -(1.32089234440210967447e6), 2.50083801823357915839e7, -(5.38695755929454629881e7)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *_tanP.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_tanQ.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_47() {
    *_tanP.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(1.30936939181383777646e4), 1.15351664838587416140e6, -(1.79565251976484877988e7)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_48() {
    *_tanQ.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.00000000000000000000e0, 1.36812963470692954678e4, -(1.32089234440210967447e6), 2.50083801823357915839e7, -(5.38695755929454629881e7)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
