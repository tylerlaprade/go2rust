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
use crate::tanh::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

/// Frexp breaks f into a normalized fraction
/// and an integral power of two.
/// It returns frac and exp satisfying f == frac × 2**exp,
/// with the absolute value of frac in the interval [½, 1).
///
/// Special cases are:
///
///	Frexp(±0) = ±0, 0
///	Frexp(±Inf) = ±Inf, 0
///	Frexp(NaN) = NaN, 0
pub fn frexp(f: Arc<Mutex<Option<f64>>>) -> (f64, i32) {
    let mut frac: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut exp: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    if HAVE_ARCH_FREXP {
        return arch_frexp(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    frexp_1(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn frexp_1(mut f: Arc<Mutex<Option<f64>>>) -> (f64, i32) {
    let mut frac: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut exp: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        // special cases
    if { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } {
            return ({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }, 0);
        } else if is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) || is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return ({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }, 0);
        }
        // correctly return -0
    { let (__tmp_0, __tmp_1) = normalize(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *f.lock().unwrap() = Some(__tmp_0); *exp.lock().unwrap() = Some(__tmp_1); };
    let mut x = float64bits(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let __rhs = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = x; let __tmp_y = SHIFT; __tmp_x >> __tmp_y }); let __tmp_y = MASK as u64; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1023; __tmp_x - __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __rhs = ((MASK as u64) << (SHIFT as u64)) as u64; x = x & ! __rhs; };
    { let __rhs = ((-((1 as i128)) + (BIAS as i128)) << (SHIFT as i128)) as u64; x = x | __rhs; };
    { let new_val = float64frombits(Arc::new(Mutex::new(Some(x)))); *frac.lock().unwrap() = Some(new_val); };
    return ((*frac.lock().unwrap().as_ref().unwrap()), (*exp.lock().unwrap().as_ref().unwrap()));
}