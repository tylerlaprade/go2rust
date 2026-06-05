use go2rust_stdlib_stubs::*;

use crate::abs::*;
use crate::acosh::*;
use crate::asin::*;
use crate::asinh::*;
use crate::atan::*;
use crate::atan2::*;
use crate::atanh::*;
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
use crate::tanh::*;
use crate::trig_reduce::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

pub(crate) const UVNAN: i64 = 0x7FF8000000000001;
pub(crate) const UVINF: i64 = 0x7FF0000000000000;
pub(crate) const UVNEGINF: u64 = 0xFFF0000000000000;
pub(crate) const UVONE: i64 = 0x3FF0000000000000;
pub(crate) const MASK: i32 = 0x7FF;
pub(crate) const SHIFT: i32 = 64 - 11 - 1;
pub(crate) const BIAS: i32 = 1023;
pub(crate) const SIGN_MASK: u64 = 1 << 63;
pub(crate) const FRAC_MASK: i64 = (1 << SHIFT) - 1;


/// Inf returns positive infinity if sign >= 0, negative infinity if sign < 0.
pub fn inf(sign: Arc<Mutex<Option<i32>>>) -> f64 {
    let mut v: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*sign.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = UVINF as u64; *v.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = UVNEGINF as u64; *v.lock().unwrap() = Some(new_val); };
    }
    return float64frombits(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// NaN returns an IEEE 754 “not-a-number” value.
pub fn na_n() -> f64 {
    float64frombits(Arc::new(Mutex::new(Some(UVNAN as u64))))
}

/// IsNaN reports whether f is an IEEE 754 “not-a-number” value.
pub fn is_na_n(f: Arc<Mutex<Option<f64>>>) -> bool {
    let mut is: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        // IEEE 754 says that only NaNs satisfy f != f.
        // To avoid the floating-point hardware, could use:
        //	x := Float64bits(f);
        //	return uint32(x>>shift)&mask == mask && x != uvinf && x != uvneginf
    return { let __bin_f = (*f.lock().unwrap().as_ref().unwrap()).clone(); __bin_f != __bin_f };
}

/// IsInf reports whether f is an infinity, according to sign.
/// If sign > 0, IsInf reports whether f is positive infinity.
/// If sign < 0, IsInf reports whether f is negative infinity.
/// If sign == 0, IsInf reports whether f is either infinity.
pub fn is_inf(f: Arc<Mutex<Option<f64>>>, sign: Arc<Mutex<Option<i32>>>) -> bool {
        // Test for infinity by comparing against maximum float.
        // To avoid the floating-point hardware, could use:
        //	x := Float64bits(f);
        //	return sign >= 0 && x == uvinf || sign <= 0 && x == uvneginf;
    return { let __tmp_x = { let __v = (*sign.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_FLOAT64 as f64; __tmp_x > __tmp_y } || { let __tmp_x = { let __v = (*sign.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1.7976931348623157e+308; __tmp_x < __tmp_y };
}

/// normalize returns a normal number y and exponent exp
/// satisfying x == y × 2**exp. It assumes x is finite and non-zero.
pub fn normalize(x: Arc<Mutex<Option<f64>>>) -> (f64, i32) {
    let mut y: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut exp: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    const SmallestNormal: f64 = 2.2250738585072014e-308;

    if { let __tmp_x = abs(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = SmallestNormal as f64; __tmp_x < __tmp_y } {
        return ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4.503599627370496e+15; __tmp_x * __tmp_y }, -(52));
    }
    return ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }, 0);
}