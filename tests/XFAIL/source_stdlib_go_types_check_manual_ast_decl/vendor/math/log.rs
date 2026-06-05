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

/// Log returns the natural logarithm of x.
///
/// Special cases are:
///
///	Log(+Inf) = +Inf
///	Log(0) = -Inf
///	Log(x < 0) = NaN
///	Log(NaN) = NaN
pub fn log(x: Arc<Mutex<Option<f64>>>) -> f64 {
    if HAVE_ARCH_LOG {
        return arch_log(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    log_1(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn log_1(x: Arc<Mutex<Option<f64>>>) -> f64 {
    const Ln2Hi: f64 = 6.93147180369123816490e-01;
const Ln2Lo: f64 = 1.90821492927058770002e-10;
const L1: f64 = 6.666666666666735130e-01;
const L2: f64 = 3.999999999940941908e-01;
const L3: f64 = 2.857142874366239149e-01;
const L4: f64 = 2.222219843214978396e-01;
const L5: f64 = 1.818357216161805012e-01;
const L6: f64 = 1.531383769920937332e-01;
const L7: f64 = 1.479819860511658591e-01;


        /* 3fe62e42 fee00000 */
        /* 3dea39ef 35793c76 */
        /* 3FE55555 55555593 */
        /* 3FD99999 9997FA04 */
        /* 3FD24924 94229359 */
        /* 3FCC71C5 1D8E78AF */
        /* 3FC74664 96CB03DE */
        /* 3FC39A09 D078C69F */
        /* 3FC2F112 DF3E5244 */
        // special cases
    if is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(1)))) {
            return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
        } else if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y } {
            return na_n();
        } else if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } {
            return inf(Arc::new(Mutex::new(Some(-1))));
        }

        // reduce
    let (mut f1, mut ki) = frexp(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = f1 as f64; let __tmp_y = 0.7071067811865476; __tmp_x < __tmp_y } {
        { let __rhs = 2.0; f1 = f1 * __rhs; };
        { ki -= 1; }
    }
    let mut f = Arc::new(Mutex::new(Some({ let __tmp_x = f1 as f64; let __tmp_y = 1.0; __tmp_x - __tmp_y })));
    let mut k = Arc::new(Mutex::new(Some(ki as f64)));

        // compute
    let mut s = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = 2.0; let __tmp_y = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); __tmp_x / __tmp_y })));
    let mut s2 = Arc::new(Mutex::new(Some({ let __bin_s = (*s.lock().unwrap().as_ref().unwrap()).clone(); __bin_s * __bin_s })));
    let mut s4 = Arc::new(Mutex::new(Some({ let __bin_s2 = (*s2.lock().unwrap().as_ref().unwrap()).clone(); __bin_s2 * __bin_s2 })));
    let mut t1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*s2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = L1 as f64; let __tmp_y = { let __tmp_x = { let __v = (*s4.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = L3 as f64; let __tmp_y = { let __tmp_x = { let __v = (*s4.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = L5 as f64; let __tmp_y = { let __tmp_x = { let __v = (*s4.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = L7 as f64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y })));
    let mut t2 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*s4.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = L2 as f64; let __tmp_y = { let __tmp_x = { let __v = (*s4.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = L4 as f64; let __tmp_y = { let __tmp_x = { let __v = (*s4.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = L6 as f64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y })));
    let mut R = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*t1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*t2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
    let mut hfsq = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 0.5; let __tmp_y = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })));
    return { let __tmp_x = { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = Ln2Hi as f64; __tmp_x * __tmp_y }; let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*hfsq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*hfsq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*R.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = Ln2Lo as f64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y };
}