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
use crate::stubs::*;
use crate::tan::*;
use crate::tanh::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

/// Sqrt returns the square root of x.
///
/// Special cases are:
///
///	Sqrt(+Inf) = +Inf
///	Sqrt(±0) = ±0
///	Sqrt(x < 0) = NaN
///	Sqrt(NaN) = NaN
pub fn sqrt(x: Arc<Mutex<Option<f64>>>) -> f64 {
    sqrt_1(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn sqrt_1(x: Arc<Mutex<Option<f64>>>) -> f64 {
        // special cases
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } || is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1)))) {
            return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
        } else if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
            return na_n();
        }
    let mut ix = float64bits(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // normalize x
    let mut exp = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = ix; let __tmp_y = SHIFT; __tmp_x >> __tmp_y }); let __tmp_y = MASK as u64; __tmp_x & __tmp_y }) as i32)));
    if { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        while { let __tmp_x = { let __tmp_x = ix; let __tmp_y = ((1 as u64) << (SHIFT as u64)) as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let __rhs = 1 as u64; ix = ix << __rhs; };
        { let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __rhs = 1023; let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    { let __rhs = ((MASK as u64) << (SHIFT as u64)) as u64; ix = ix & ! __rhs; };
    { let __rhs = ((1 as u64) << (SHIFT as u64)) as u64; ix = ix | __rhs; };
    if { let __tmp_x = { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x & __tmp_y }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let __rhs = 1 as u64; ix = ix << __rhs; };
    }
    { let __rhs = 1; let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };

        // generate sqrt(x) bit by bit
    { let __rhs = 1 as u64; ix = ix << __rhs; };
    let mut q: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut s: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut r = Arc::new(Mutex::new(Some(((1 as u64) << ((SHIFT as u64) + (1 as u64))) as u64)));
    while { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        let mut t = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ix; __tmp_x <= __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *s.lock().unwrap() = Some(new_val); };
        { let __rhs = (*t.lock().unwrap().as_ref().unwrap()); ix = ix - __rhs; };
        { let __rhs = (*r.lock().unwrap().as_ref().unwrap()); let mut guard = q.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let __rhs = 1 as u64; ix = ix << __rhs; };
        { let __rhs = 1 as u64; let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }

        // final rounding
    if { let __tmp_x = ix; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = { let __tmp_x = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x & __tmp_y }; let mut guard = q.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // round according to extra bit
    { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; let __tmp_y = 1023; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = SHIFT; __tmp_x << __tmp_y }; __tmp_x + __tmp_y }; ix = new_val; };
    float64frombits(Arc::new(Mutex::new(Some(ix))))
}