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

pub(crate) static _lgamA: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 12]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _lgamR: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 7]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _lgamS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 7]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _lgamT: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 15]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _lgamU: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _lgamV: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _lgamW: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 7]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_lgamA.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamR.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamS.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamT.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamU.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamV.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamW.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamA.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([7.72156649015328655494e-02, 3.22467033424113591611e-01, 6.73523010531292681824e-02, 2.05808084325167332806e-02, 7.38555086081402883957e-03, 2.89051383673415629091e-03, 1.19270763183362067845e-03, 5.10069792153511336608e-04, 2.20862790713908385557e-04, 1.08011567247583939954e-04, 2.52144565451257326939e-05, 4.48640949618915160150e-05]))).lock().unwrap().as_ref().unwrap()).clone());
    *_lgamR.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.0, 1.39200533467621045958e+00, 7.21935547567138069525e-01, 1.71933865632803078993e-01, 1.86459191715652901344e-02, 7.77942496381893596434e-04, 7.32668430744625636189e-06]))).lock().unwrap().as_ref().unwrap()).clone());
    *_lgamS.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(7.72156649015328655494e-02), 2.14982415960608852501e-01, 3.25778796408930981787e-01, 1.46350472652464452805e-01, 2.66422703033638609560e-02, 1.84028451407337715652e-03, 3.19475326584100867617e-05]))).lock().unwrap().as_ref().unwrap()).clone());
    *_lgamT.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.83836122723810047042e-01, -(1.47587722994593911752e-01), 6.46249402391333854778e-02, -(3.27885410759859649565e-02), 1.79706750811820387126e-02, -(1.03142241298341437450e-02), 6.10053870246291332635e-03, -(3.68452016781138256760e-03), 2.25964780900612472250e-03, -(1.40346469989232843813e-03), 8.81081882437654011382e-04, -(5.38595305356740546715e-04), 3.15632070903625950361e-04, -(3.12754168375120860518e-04), 3.35529192635519073543e-04]))).lock().unwrap().as_ref().unwrap()).clone());
    *_lgamU.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(7.72156649015328655494e-02), 6.32827064025093366517e-01, 1.45492250137234768737e+00, 9.77717527963372745603e-01, 2.28963728064692451092e-01, 1.33810918536787660377e-02]))).lock().unwrap().as_ref().unwrap()).clone());
    *_lgamV.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.0, 2.45597793713041134822e+00, 2.12848976379893395361e+00, 7.69285150456672783825e-01, 1.04222645593369134254e-01, 3.21709242282423911810e-03]))).lock().unwrap().as_ref().unwrap()).clone());
    *_lgamW.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.18938533204672725052e-01, 8.33333333333329678849e-02, -(2.77777777728775536470e-03), 7.93650558643019558500e-04, -(5.95187557450339963135e-04), 8.36339918996282139126e-04, -(1.63092934096575273989e-03)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *_lgamA.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamR.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamS.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamT.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamU.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamV.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *_lgamW.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_35() {
    *_lgamA.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([7.72156649015328655494e-02, 3.22467033424113591611e-01, 6.73523010531292681824e-02, 2.05808084325167332806e-02, 7.38555086081402883957e-03, 2.89051383673415629091e-03, 1.19270763183362067845e-03, 5.10069792153511336608e-04, 2.20862790713908385557e-04, 1.08011567247583939954e-04, 2.52144565451257326939e-05, 4.48640949618915160150e-05]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_36() {
    *_lgamR.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.0, 1.39200533467621045958e+00, 7.21935547567138069525e-01, 1.71933865632803078993e-01, 1.86459191715652901344e-02, 7.77942496381893596434e-04, 7.32668430744625636189e-06]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_37() {
    *_lgamS.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(7.72156649015328655494e-02), 2.14982415960608852501e-01, 3.25778796408930981787e-01, 1.46350472652464452805e-01, 2.66422703033638609560e-02, 1.84028451407337715652e-03, 3.19475326584100867617e-05]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_38() {
    *_lgamT.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.83836122723810047042e-01, -(1.47587722994593911752e-01), 6.46249402391333854778e-02, -(3.27885410759859649565e-02), 1.79706750811820387126e-02, -(1.03142241298341437450e-02), 6.10053870246291332635e-03, -(3.68452016781138256760e-03), 2.25964780900612472250e-03, -(1.40346469989232843813e-03), 8.81081882437654011382e-04, -(5.38595305356740546715e-04), 3.15632070903625950361e-04, -(3.12754168375120860518e-04), 3.35529192635519073543e-04]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_39() {
    *_lgamU.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(7.72156649015328655494e-02), 6.32827064025093366517e-01, 1.45492250137234768737e+00, 9.77717527963372745603e-01, 2.28963728064692451092e-01, 1.33810918536787660377e-02]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_40() {
    *_lgamV.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.0, 2.45597793713041134822e+00, 2.12848976379893395361e+00, 7.69285150456672783825e-01, 1.04222645593369134254e-01, 3.21709242282423911810e-03]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_41() {
    *_lgamW.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.18938533204672725052e-01, 8.33333333333329678849e-02, -(2.77777777728775536470e-03), 7.93650558643019558500e-04, -(5.95187557450339963135e-04), 8.36339918996282139126e-04, -(1.63092934096575273989e-03)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
