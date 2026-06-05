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
use crate::tan::*;
use crate::tanh::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

pub(crate) const HAVE_ARCH_ACOS: bool = false;


pub(crate) const HAVE_ARCH_ACOSH: bool = false;


pub(crate) const HAVE_ARCH_ASIN: bool = false;


pub(crate) const HAVE_ARCH_ASINH: bool = false;


pub(crate) const HAVE_ARCH_ATAN: bool = false;


pub(crate) const HAVE_ARCH_ATAN2: bool = false;


pub(crate) const HAVE_ARCH_ATANH: bool = false;


pub(crate) const HAVE_ARCH_CBRT: bool = false;


pub(crate) const HAVE_ARCH_COS: bool = false;


pub(crate) const HAVE_ARCH_COSH: bool = false;


pub(crate) const HAVE_ARCH_ERF: bool = false;


pub(crate) const HAVE_ARCH_ERFC: bool = false;


pub(crate) const HAVE_ARCH_EXPM1: bool = false;


pub(crate) const HAVE_ARCH_FREXP: bool = false;


pub(crate) const HAVE_ARCH_LDEXP: bool = false;


pub(crate) const HAVE_ARCH_LOG10: bool = false;


pub(crate) const HAVE_ARCH_LOG2: bool = false;


pub(crate) const HAVE_ARCH_LOG1P: bool = false;


pub(crate) const HAVE_ARCH_MOD: bool = false;


pub(crate) const HAVE_ARCH_POW: bool = false;


pub(crate) const HAVE_ARCH_REMAINDER: bool = false;


pub(crate) const HAVE_ARCH_SIN: bool = false;


pub(crate) const HAVE_ARCH_SINH: bool = false;


pub(crate) const HAVE_ARCH_TAN: bool = false;


pub(crate) const HAVE_ARCH_TANH: bool = false;


pub fn arch_frexp(x: Arc<Mutex<Option<f64>>>) -> (f64, i32) {
    std::panic::panic_any(Box::new("not implemented".to_string()) as Box<dyn Any + Send + Sync>);
}

pub fn arch_ldexp(frac: Arc<Mutex<Option<f64>>>, exp: Arc<Mutex<Option<i32>>>) -> f64 {
    std::panic::panic_any(Box::new("not implemented".to_string()) as Box<dyn Any + Send + Sync>);
}

pub fn arch_log2(x: Arc<Mutex<Option<f64>>>) -> f64 {
    std::panic::panic_any(Box::new("not implemented".to_string()) as Box<dyn Any + Send + Sync>);
}

pub fn arch_pow(x: Arc<Mutex<Option<f64>>>, y: Arc<Mutex<Option<f64>>>) -> f64 {
    std::panic::panic_any(Box::new("not implemented".to_string()) as Box<dyn Any + Send + Sync>);
}