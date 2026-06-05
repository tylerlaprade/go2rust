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
use crate::sincos::*;
use crate::sinh::*;
use crate::sqrt::*;
use crate::stubs::*;
use crate::tan::*;
use crate::tanh::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

pub(crate) static _sin: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cos: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_sin.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_cos.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_sin.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.58962301576546568060e-10, -(2.50507477628578072866e-8), 2.75573136213857245213e-6, -(1.98412698295895385996e-4), 8.33333333332211858878e-3, -(1.66666666666666307295e-1)]))).lock().unwrap().as_ref().unwrap()).clone());
    *_cos.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(1.13585365213876817300e-11), 2.08757008419747316778e-9, -(2.75573141792967388112e-7), 2.48015872888517045348e-5, -(1.38888888888730564116e-3), 4.16666666666665929218e-2]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *_sin.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_cos.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_45() {
    *_sin.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.58962301576546568060e-10, -(2.50507477628578072866e-8), 2.75573136213857245213e-6, -(1.98412698295895385996e-4), 8.33333333332211858878e-3, -(1.66666666666666307295e-1)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_46() {
    *_cos.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(1.13585365213876817300e-11), 2.08757008419747316778e-9, -(2.75573141792967388112e-7), 2.48015872888517045348e-5, -(1.38888888888730564116e-3), 4.16666666666665929218e-2]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
