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

/// Modf returns integer and fractional floating-point numbers
/// that sum to f. Both values have the same sign as f.
///
/// Special cases are:
///
///	Modf(±Inf) = ±Inf, NaN
///	Modf(NaN) = NaN, NaN
pub fn modf(f: Arc<Mutex<Option<f64>>>) -> (f64, f64) {
    let mut int: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut frac: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));

    if HAVE_ARCH_MODF {
        return arch_modf(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    modf_1(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn modf_1(f: Arc<Mutex<Option<f64>>>) -> (f64, f64) {
    let mut int: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut frac: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));

    if { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1.0; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
            { let (__tmp_0, __tmp_1) = modf(Arc::new(Mutex::new(Some(-((*f.lock().unwrap().as_ref().unwrap())))))); *int.lock().unwrap() = Some(__tmp_0); *frac.lock().unwrap() = Some(__tmp_1); };
            return (-((*int.lock().unwrap().as_ref().unwrap())), -((*frac.lock().unwrap().as_ref().unwrap())));
        } else if { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } {
            return ({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v });
        }
                // Return -0, -0 when f == -0
        return (0.0, { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

        // Return -0, -0 when f == -0
    let mut x = float64bits(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut e = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = x; let __tmp_y = SHIFT; __tmp_x >> __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = BIAS as u64; __tmp_x - __tmp_y })));

        // Keep the top 12+e bits, the integer part; clear the rest.
    if { let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = 64; let __tmp_y = 12; __tmp_x - __tmp_y } as u64; __tmp_x < __tmp_y } {
        { let __rhs = { let __tmp_x = { let __tmp_x = (1 as u64); let __tmp_y = ({ let __tmp_x = { let __tmp_x = 64; let __tmp_y = 12; __tmp_x - __tmp_y } as u64; let __tmp_y = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }; x = x & ! __rhs; };
    }
    { let new_val = float64frombits(Arc::new(Mutex::new(Some(x)))); *int.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*int.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *frac.lock().unwrap() = Some(new_val); };
    return ((*int.lock().unwrap().as_ref().unwrap()), (*frac.lock().unwrap().as_ref().unwrap()));
}