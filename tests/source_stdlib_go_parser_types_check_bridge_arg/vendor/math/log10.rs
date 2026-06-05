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

/// Log2 returns the binary logarithm of x.
/// The special cases are the same as for [Log].
pub fn log2(x: Arc<Mutex<Option<f64>>>) -> f64 {
    if HAVE_ARCH_LOG2 {
        return arch_log2(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    log2_1(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn log2_1(x: Arc<Mutex<Option<f64>>>) -> f64 {
    let (mut frac, mut exp) = frexp(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Make sure exact powers of two give an exact answer.
        // Don't depend on Log(0.5)*(1/Ln2)+exp being exactly exp-1.
    if { let __tmp_x = frac; let __tmp_y = 0.5; __tmp_x == __tmp_y } {
        return (*Arc::new(Mutex::new(Some(({ let __tmp_x = exp; let __tmp_y = 1; __tmp_x - __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap());
    }
    return { let __tmp_x = { let __tmp_x = log(Arc::new(Mutex::new(Some(frac)))); let __tmp_y = 1.4426950408889634; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(exp as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
}