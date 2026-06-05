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
use crate::tan::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

pub(crate) static tanhP: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 3]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static tanhQ: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 3]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *tanhP.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *tanhQ.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *tanhP.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(9.64399179425052238628e-1), -(9.92877231001918586564e1), -(1.61468768441708447952e3)]))).lock().unwrap().as_ref().unwrap()).clone());
    *tanhQ.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.12811678491632931402e2, 2.23548839060100448583e3, 4.84406305325125486048e3]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *tanhP.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *tanhQ.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_49() {
    *tanhP.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(9.64399179425052238628e-1), -(9.92877231001918586564e1), -(1.61468768441708447952e3)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_50() {
    *tanhQ.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.12811678491632931402e2, 2.23548839060100448583e3, 4.84406305325125486048e3]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
