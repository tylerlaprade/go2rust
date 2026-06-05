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

/// Ldexp is the inverse of [Frexp].
/// It returns frac × 2**exp.
///
/// Special cases are:
///
///	Ldexp(±0, exp) = ±0
///	Ldexp(±Inf, exp) = ±Inf
///	Ldexp(NaN, exp) = NaN
pub fn ldexp(frac: Arc<Mutex<Option<f64>>>, exp: Arc<Mutex<Option<i32>>>) -> f64 {
    if HAVE_ARCH_LDEXP {
        return arch_ldexp(Arc::new(Mutex::new(Some({ let __arg_holder = frac.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = exp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    ldexp_1(Arc::new(Mutex::new(Some({ let __arg_holder = frac.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = exp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn ldexp_1(mut frac: Arc<Mutex<Option<f64>>>, mut exp: Arc<Mutex<Option<i32>>>) -> f64 {
        // special cases
    if { let __tmp_x = { let __v = (*frac.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } {
            return { let __v = (*frac.lock().unwrap().as_ref().unwrap()).clone(); __v };
        } else if is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = frac.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) || is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = frac.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return { let __v = (*frac.lock().unwrap().as_ref().unwrap()).clone(); __v };
        }
        // correctly return -0
    let (__tmp_0, mut e) = normalize(Arc::new(Mutex::new(Some({ let __arg_holder = frac.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *frac.lock().unwrap() = Some(__tmp_0);;
    { let __rhs = e; let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    let mut x = float64bits(Arc::new(Mutex::new(Some({ let __arg_holder = frac.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let __rhs = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = x; let __tmp_y = SHIFT; __tmp_x >> __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2047; __tmp_x & __tmp_y }; let __tmp_y = 1023; __tmp_x - __tmp_y }; let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    if { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1075; __tmp_x < __tmp_y } {
        return copysign(Arc::new(Mutex::new(Some(0.0))), Arc::new(Mutex::new(Some({ let __arg_holder = frac.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // underflow
    if { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1023; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __v = (*frac.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
        return inf(Arc::new(Mutex::new(Some(-1))));
    }
        return inf(Arc::new(Mutex::new(Some(1))));
    }
    let mut m: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(1.0)));
    if { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1022; __tmp_x < __tmp_y } {
        { let __rhs = 53; let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = 1.1102230246251565e-16; *m.lock().unwrap() = Some(new_val); };
    }
        // 2**-53
    { let __rhs = ((MASK as u64) << (SHIFT as u64)) as u64; x = x & ! __rhs; };
    { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1023; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = SHIFT; __tmp_x << __tmp_y }; x = x | __rhs; };
    return { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = float64frombits(Arc::new(Mutex::new(Some(x)))); __tmp_x * __tmp_y };
}