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
use crate::tanh::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

pub(crate) static _gamP: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 7]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _gamQ: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 8]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _gamS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_gamP.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_gamQ.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_gamS.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_gamP.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.60119522476751861407e-04, 1.19135147006586384913e-03, 1.04213797561761569935e-02, 4.76367800457137231464e-02, 2.07448227648435975150e-01, 4.94214826801497100753e-01, 9.99999999999999996796e-01]))).lock().unwrap().as_ref().unwrap()).clone());
    *_gamQ.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(2.31581873324120129819e-05), 5.39605580493303397842e-04, -(4.45641913851797240494e-03), 1.18139785222060435552e-02, 3.58236398605498653373e-02, -(2.34591795718243348568e-01), 7.14304917030273074085e-02, 1.00000000000000000320e+00]))).lock().unwrap().as_ref().unwrap()).clone());
    *_gamS.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([7.87311395793093628397e-04, -(2.29549961613378126380e-04), -(2.68132617805781232825e-03), 3.47222221605458667310e-03, 8.33333333333482257126e-02]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *_gamP.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_gamQ.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_gamS.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_0() {
    *_gamP.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.60119522476751861407e-04, 1.19135147006586384913e-03, 1.04213797561761569935e-02, 4.76367800457137231464e-02, 2.07448227648435975150e-01, 4.94214826801497100753e-01, 9.99999999999999996796e-01]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_1() {
    *_gamQ.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(2.31581873324120129819e-05), 5.39605580493303397842e-04, -(4.45641913851797240494e-03), 1.18139785222060435552e-02, 3.58236398605498653373e-02, -(2.34591795718243348568e-01), 7.14304917030273074085e-02, 1.00000000000000000320e+00]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_2() {
    *_gamS.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([7.87311395793093628397e-04, -(2.29549961613378126380e-04), -(2.68132617805781232825e-03), 3.47222221605458667310e-03, 8.33333333333482257126e-02]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
