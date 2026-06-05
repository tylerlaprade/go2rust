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

/// Exp returns e**x, the base-e exponential of x.
///
/// Special cases are:
///
///	Exp(+Inf) = +Inf
///	Exp(NaN) = NaN
///
/// Very large values overflow to 0 or +Inf.
/// Very small values underflow to 1.
pub fn exp(x: Arc<Mutex<Option<f64>>>) -> f64 {
    if HAVE_ARCH_EXP {
        return arch_exp(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    exp_1(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn exp_1(x: Arc<Mutex<Option<f64>>>) -> f64 {
    const Ln2Hi: f64 = 6.93147180369123816490e-01;
const Ln2Lo: f64 = 1.90821492927058770002e-10;
const Log2e: f64 = 1.44269504088896338700e+00;
const Overflow: f64 = 7.09782712893383973096e+02;
const Underflow: f64 = -7.45133219101941108420e+02;
const NearZero: f64 = 1.0 / 2.68435456e+08;


        // 2**-28
        // special cases
    if is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1)))) {
            return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
        } else if is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(-1)))) {
            return 0.0;
        } else if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = Overflow as f64; __tmp_x > __tmp_y } {
            return inf(Arc::new(Mutex::new(Some(1))));
        } else if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = Underflow as f64; __tmp_x < __tmp_y } {
            return 0.0;
        } else if { let __tmp_x = -3.725290298461914e-09; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = NearZero as f64; __tmp_x < __tmp_y } {
            return { let __tmp_x = 1.0; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
        }

        // reduce; computed as r = hi - lo for extra precision.
    let mut k: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = Log2e as f64; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 0.5; __tmp_x - __tmp_y }) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *k.lock().unwrap() = __moved_val; };
        } else if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x > __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = Log2e as f64; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 0.5; __tmp_x + __tmp_y }) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *k.lock().unwrap() = __moved_val; };
        }
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*k.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = Ln2Hi as f64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
    let mut lo = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*k.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = Ln2Lo as f64; __tmp_x * __tmp_y })));

        // compute
    return expmulti(Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// exp1 returns e**r × 2**k where r = hi - lo and |r| ≤ ln(2)/2.
pub fn expmulti(hi: Arc<Mutex<Option<f64>>>, lo: Arc<Mutex<Option<f64>>>, k: Arc<Mutex<Option<i32>>>) -> f64 {
    const P1: f64 = 1.66666666666666657415e-01;
const P2: f64 = -2.77777777770155933842e-03;
const P3: f64 = 6.61375632143793436117e-05;
const P4: f64 = -1.65339022054652515390e-06;
const P5: f64 = 4.13813679705723846039e-08;


        /* 0x3FC55555; 0x55555555 */
        /* 0xBF66C16C; 0x16BEBD93 */
        /* 0x3F11566A; 0xAF25DE2C */
        /* 0xBEBBBD41; 0xC5D26BF1 */
        /* 0x3E663769; 0x72BEA4D0 */
    let mut r = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut t = Arc::new(Mutex::new(Some({ let __bin_r = (*r.lock().unwrap().as_ref().unwrap()).clone(); __bin_r * __bin_r })));
    let mut c = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = P1 as f64; let __tmp_y = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = P2 as f64; let __tmp_y = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = P3 as f64; let __tmp_y = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = P4 as f64; let __tmp_y = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = P5 as f64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
    let mut y = Arc::new(Mutex::new(Some({ let __tmp_x = 1.0; let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }); let __tmp_y = ({ let __tmp_x = 2.0; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x / __tmp_y }; __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y })));

        // TODO(rsc): make sure Ldexp can handle boundary k
    return ldexp(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}