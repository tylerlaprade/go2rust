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

pub fn is_odd_int(x: Arc<Mutex<Option<f64>>>) -> bool {
    if { let __tmp_x = abs(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 9.007199254740992e+15; __tmp_x >= __tmp_y } {
                // 1 << 53 is the largest exact integer in the float64 format.
                // Any number outside this range will be truncated before the decimal point and therefore will always be
                // an even integer.
                // Without this check and if x overflows int64 the int64(xi) conversion below may produce incorrect results
                // on some architectures (and does so on arm64). See issue #57465.
        return false;
    }

        // 1 << 53 is the largest exact integer in the float64 format.
        // Any number outside this range will be truncated before the decimal point and therefore will always be
        // an even integer.
        // Without this check and if x overflows int64 the int64(xi) conversion below may produce incorrect results
        // on some architectures (and does so on arm64). See issue #57465.
    let (mut xi, mut xf) = modf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return { let __tmp_x = xf; let __tmp_y = 0.0; __tmp_x == __tmp_y } && { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(xi as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i64; __tmp_x & __tmp_y }; let __tmp_y = 1 as i64; __tmp_x == __tmp_y };
}

/// Pow returns x**y, the base-x exponential of y.
///
/// Special cases are (in order):
///
///	Pow(x, ±0) = 1 for any x
///	Pow(1, y) = 1 for any y
///	Pow(x, 1) = x for any x
///	Pow(NaN, y) = NaN
///	Pow(x, NaN) = NaN
///	Pow(±0, y) = ±Inf for y an odd integer < 0
///	Pow(±0, -Inf) = +Inf
///	Pow(±0, +Inf) = +0
///	Pow(±0, y) = +Inf for finite y < 0 and not an odd integer
///	Pow(±0, y) = ±0 for y an odd integer > 0
///	Pow(±0, y) = +0 for finite y > 0 and not an odd integer
///	Pow(-1, ±Inf) = 1
///	Pow(x, +Inf) = +Inf for |x| > 1
///	Pow(x, -Inf) = +0 for |x| > 1
///	Pow(x, +Inf) = +0 for |x| < 1
///	Pow(x, -Inf) = +Inf for |x| < 1
///	Pow(+Inf, y) = +Inf for y > 0
///	Pow(+Inf, y) = +0 for y < 0
///	Pow(-Inf, y) = Pow(-0, -y)
///	Pow(x, y) = NaN for finite x < 0 and finite non-integer y
pub fn pow(x: Arc<Mutex<Option<f64>>>, y: Arc<Mutex<Option<f64>>>) -> f64 {
    if HAVE_ARCH_POW {
        return arch_pow(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    pow_1(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn pow_1(x: Arc<Mutex<Option<f64>>>, y: Arc<Mutex<Option<f64>>>) -> f64 {
    if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1.0; __tmp_x == __tmp_y } {
            return 1.0;
        } else if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1.0; __tmp_x == __tmp_y } {
            return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
        } else if is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return na_n();
        } else if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } {
            if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
            if signbit(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && is_odd_int(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return inf(Arc::new(Mutex::new(Some(-1))));
    }
            return inf(Arc::new(Mutex::new(Some(1))));
        } else if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x > __tmp_y } {
            if signbit(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && is_odd_int(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
            return 0.0;
        }
        } else if is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) {
            if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1.0; __tmp_x == __tmp_y } {
            return 1.0;
        } else if { let __tmp_x = ({ let __tmp_x = abs(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 1.0; __tmp_x < __tmp_y }); let __tmp_y = is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1)))); __tmp_x == __tmp_y } {
            return 0.0;
        } else {
            return inf(Arc::new(Mutex::new(Some(1))));
        }
        } else if is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) {
            if is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(-1)))) {
        return pow(Arc::new(Mutex::new(Some({ let __tmp_x = 1.0; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some(-((*y.lock().unwrap().as_ref().unwrap()))))));
    }
                        // Pow(-0, -y)
            if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
            return 0.0;
        } else if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x > __tmp_y } {
            return inf(Arc::new(Mutex::new(Some(1))));
        }
        } else if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.5; __tmp_x == __tmp_y } {
            return sqrt(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        } else if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -0.5; __tmp_x == __tmp_y } {
            return { let __tmp_x = 1.0; let __tmp_y = sqrt(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x / __tmp_y };
        }

        // Pow(-0, -y)
    let (mut yi, mut yf) = modf(Arc::new(Mutex::new(Some(abs(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))));
    if { let __tmp_x = yf; let __tmp_y = 0.0; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
        return na_n();
    }
    if { let __tmp_x = yi; let __tmp_y = 9.223372036854776e+18; __tmp_x >= __tmp_y } {
                // yi is a large even int that will lead to overflow (or underflow to 0)
                // for all x except -1 (x == 1 was handled earlier)
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1.0; __tmp_x == __tmp_y } {
            return 1.0;
        } else if { let __tmp_x = ({ let __tmp_x = abs(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 1.0; __tmp_x < __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x > __tmp_y }); __tmp_x == __tmp_y } {
            return 0.0;
        } else {
            return inf(Arc::new(Mutex::new(Some(1))));
        }
    }

        // yi is a large even int that will lead to overflow (or underflow to 0)
        // for all x except -1 (x == 1 was handled earlier)
        // ans = a1 * 2**ae (= 1 for now).
    let mut a1 = Arc::new(Mutex::new(Some(1.0)));
    let mut ae = Arc::new(Mutex::new(Some(0)));

        // ans *= x**yf
    if { let __tmp_x = yf; let __tmp_y = 0.0; __tmp_x != __tmp_y } {
        if { let __tmp_x = yf; let __tmp_y = 0.5; __tmp_x > __tmp_y } {
        { yf -= 1.0; }
        { yi += 1.0; }
    }
        { let new_val = exp(Arc::new(Mutex::new(Some({ let __tmp_x = yf; let __tmp_y = log(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x * __tmp_y })))); *a1.lock().unwrap() = Some(new_val); };
    }

        // ans *= x**yi
        // by multiplying in successive squarings
        // of x according to bits of yi.
        // accumulate powers of two into exp.
    let (mut x1, mut xe) = frexp(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut i = Arc::new(Mutex::new(Some(yi as i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        if { let __tmp_x = xe; let __tmp_y = -4096; __tmp_x < __tmp_y } || { let __tmp_x = 4096; let __tmp_y = xe; __tmp_x < __tmp_y } {
                // catch xe before it overflows the left shift below
                // Since i !=0 it has at least one bit still set, so ae will accumulate xe
                // on at least one more iteration, ae += xe is a lower bound on ae
                // the lower bound on ae exceeds the size of a float64 exp
                // so the final call to Ldexp will produce under/overflow (0/Inf)
        { let __rhs = xe; let mut guard = ae.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        break
    }
                // catch xe before it overflows the left shift below
                // Since i !=0 it has at least one bit still set, so ae will accumulate xe
                // on at least one more iteration, ae += xe is a lower bound on ae
                // the lower bound on ae exceeds the size of a float64 exp
                // so the final call to Ldexp will produce under/overflow (0/Inf)
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i64; __tmp_x & __tmp_y }; let __tmp_y = 1 as i64; __tmp_x == __tmp_y } {
        { let __rhs = x1; let mut guard = a1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let __rhs = xe; let mut guard = ae.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let __rhs = x1; x1 = x1 * __rhs; };
        { let __rhs = 1; xe = xe << __rhs; };
        if { let __tmp_x = x1; let __tmp_y = 0.5; __tmp_x < __tmp_y } {
        { let __rhs = x1; x1 = x1 + __rhs; };
        { xe -= 1; }
    }
        { let __rhs = 1 as i64; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }

        // catch xe before it overflows the left shift below
        // Since i !=0 it has at least one bit still set, so ae will accumulate xe
        // on at least one more iteration, ae += xe is a lower bound on ae
        // the lower bound on ae exceeds the size of a float64 exp
        // so the final call to Ldexp will produce under/overflow (0/Inf)
        // ans = a1*2**ae
        // if y < 0 { ans = 1 / ans }
        // but in the opposite order
    if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
        { let new_val = 0.021243741118258296; *a1.lock().unwrap() = Some(new_val); };
        { let new_val = -((*ae.lock().unwrap().as_ref().unwrap())); *ae.lock().unwrap() = Some(new_val); };
    }
    return ldexp(Arc::new(Mutex::new(Some({ let __arg_holder = a1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ae.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}