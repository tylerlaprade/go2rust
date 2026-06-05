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

pub(crate) static p0R8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p0S8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p0R5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p0S5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p0R3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p0S3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p0R2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p0S2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q0R8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q0S8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q0R5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q0S5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q0R3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q0S3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q0R2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q0S2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *p0R8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0S8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0R5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0S5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0R3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0S3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0R2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0S2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0R8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0S8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0R5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0S5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0R3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0S3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0R2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0S2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0R8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.00000000000000000000e+00, -(7.03124999999900357484e-02), -(8.08167041275349795626e+00), -(2.57063105679704847262e+02), -(2.48521641009428822144e+03), -(5.25304380490729545272e+03)]))).lock().unwrap().as_ref().unwrap()).clone());
    *p0S8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.16534364619668181717e+02, 3.83374475364121826715e+03, 4.05978572648472545552e+04, 1.16752972564375915681e+05, 4.76277284146730962675e+04]))).lock().unwrap().as_ref().unwrap()).clone());
    *p0R5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(1.14125464691894502584e-11), -(7.03124940873599280078e-02), -(4.15961064470587782438e+00), -(6.76747652265167261021e+01), -(3.31231299649172967747e+02), -(3.46433388365604912451e+02)]))).lock().unwrap().as_ref().unwrap()).clone());
    *p0S5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([6.07539382692300335975e+01, 1.05125230595704579173e+03, 5.97897094333855784498e+03, 9.62544514357774460223e+03, 2.40605815922939109441e+03]))).lock().unwrap().as_ref().unwrap()).clone());
    *p0R3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(2.54704601771951915620e-09), -(7.03119616381481654654e-02), -(2.40903221549529611423e+00), -(2.19659774734883086467e+01), -(5.80791704701737572236e+01), -(3.14479470594888503854e+01)]))).lock().unwrap().as_ref().unwrap()).clone());
    *p0S3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([3.58560338055209726349e+01, 3.61513983050303863820e+02, 1.19360783792111533330e+03, 1.12799679856907414432e+03, 1.73580930813335754692e+02]))).lock().unwrap().as_ref().unwrap()).clone());
    *p0R2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(8.87534333032526411254e-08), -(7.03030995483624743247e-02), -(1.45073846780952986357e+00), -(7.63569613823527770791e+00), -(1.11931668860356747786e+01), -(3.23364579351335335033e+00)]))).lock().unwrap().as_ref().unwrap()).clone());
    *p0S2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([2.22202997532088808441e+01, 1.36206794218215208048e+02, 2.70470278658083486789e+02, 1.53875394208320329881e+02, 1.46576176948256193810e+01]))).lock().unwrap().as_ref().unwrap()).clone());
    *q0R8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.00000000000000000000e+00, 7.32421874999935051953e-02, 1.17682064682252693899e+01, 5.57673380256401856059e+02, 8.85919720756468632317e+03, 3.70146267776887834771e+04]))).lock().unwrap().as_ref().unwrap()).clone());
    *q0S8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.63776026895689824414e+02, 8.09834494656449805916e+03, 1.42538291419120476348e+05, 8.03309257119514397345e+05, 8.40501579819060512818e+05, -(3.43899293537866615225e+05)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q0R5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.84085963594515531381e-11, 7.32421766612684765896e-02, 5.83563508962056953777e+00, 1.35111577286449829671e+02, 1.02724376596164097464e+03, 1.98997785864605384631e+03]))).lock().unwrap().as_ref().unwrap()).clone());
    *q0S5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([8.27766102236537761883e+01, 2.07781416421392987104e+03, 1.88472887785718085070e+04, 5.67511122894947329769e+04, 3.59767538425114471465e+04, -(5.35434275601944773371e+03)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q0R3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.37741014089738620906e-09, 7.32411180042911447163e-02, 3.34423137516170720929e+00, 4.26218440745412650017e+01, 1.70808091340565596283e+02, 1.66733948696651168575e+02]))).lock().unwrap().as_ref().unwrap()).clone());
    *q0S3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.87588729724587182091e+01, 7.09689221056606015736e+02, 3.70414822620111362994e+03, 6.46042516752568917582e+03, 2.51633368920368957333e+03, -(1.49247451836156386662e+02)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q0R2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.50444444886983272379e-07, 7.32234265963079278272e-02, 1.99819174093815998816e+00, 1.44956029347885735348e+01, 3.16662317504781540833e+01, 1.62527075710929267416e+01]))).lock().unwrap().as_ref().unwrap()).clone());
    *q0S2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([3.03655848355219184498e+01, 2.69348118608049844624e+02, 8.44783757595320139444e+02, 8.82935845112488550512e+02, 2.12666388511798828631e+02, -(5.31095493882666946917e+00)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *p0R8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0S8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0R5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0S5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0R3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0S3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0R2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p0S2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0R8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0S8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0R5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0S5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0R3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0S3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0R2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q0S2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_3() {
    *p0R8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.00000000000000000000e+00, -(7.03124999999900357484e-02), -(8.08167041275349795626e+00), -(2.57063105679704847262e+02), -(2.48521641009428822144e+03), -(5.25304380490729545272e+03)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_4() {
    *p0S8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.16534364619668181717e+02, 3.83374475364121826715e+03, 4.05978572648472545552e+04, 1.16752972564375915681e+05, 4.76277284146730962675e+04]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_5() {
    *p0R5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(1.14125464691894502584e-11), -(7.03124940873599280078e-02), -(4.15961064470587782438e+00), -(6.76747652265167261021e+01), -(3.31231299649172967747e+02), -(3.46433388365604912451e+02)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_6() {
    *p0S5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([6.07539382692300335975e+01, 1.05125230595704579173e+03, 5.97897094333855784498e+03, 9.62544514357774460223e+03, 2.40605815922939109441e+03]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_7() {
    *p0R3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(2.54704601771951915620e-09), -(7.03119616381481654654e-02), -(2.40903221549529611423e+00), -(2.19659774734883086467e+01), -(5.80791704701737572236e+01), -(3.14479470594888503854e+01)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_8() {
    *p0S3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([3.58560338055209726349e+01, 3.61513983050303863820e+02, 1.19360783792111533330e+03, 1.12799679856907414432e+03, 1.73580930813335754692e+02]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_9() {
    *p0R2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(8.87534333032526411254e-08), -(7.03030995483624743247e-02), -(1.45073846780952986357e+00), -(7.63569613823527770791e+00), -(1.11931668860356747786e+01), -(3.23364579351335335033e+00)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_10() {
    *p0S2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([2.22202997532088808441e+01, 1.36206794218215208048e+02, 2.70470278658083486789e+02, 1.53875394208320329881e+02, 1.46576176948256193810e+01]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_11() {
    *q0R8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.00000000000000000000e+00, 7.32421874999935051953e-02, 1.17682064682252693899e+01, 5.57673380256401856059e+02, 8.85919720756468632317e+03, 3.70146267776887834771e+04]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_12() {
    *q0S8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.63776026895689824414e+02, 8.09834494656449805916e+03, 1.42538291419120476348e+05, 8.03309257119514397345e+05, 8.40501579819060512818e+05, -(3.43899293537866615225e+05)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_13() {
    *q0R5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.84085963594515531381e-11, 7.32421766612684765896e-02, 5.83563508962056953777e+00, 1.35111577286449829671e+02, 1.02724376596164097464e+03, 1.98997785864605384631e+03]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_14() {
    *q0S5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([8.27766102236537761883e+01, 2.07781416421392987104e+03, 1.88472887785718085070e+04, 5.67511122894947329769e+04, 3.59767538425114471465e+04, -(5.35434275601944773371e+03)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_15() {
    *q0R3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.37741014089738620906e-09, 7.32411180042911447163e-02, 3.34423137516170720929e+00, 4.26218440745412650017e+01, 1.70808091340565596283e+02, 1.66733948696651168575e+02]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_16() {
    *q0S3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.87588729724587182091e+01, 7.09689221056606015736e+02, 3.70414822620111362994e+03, 6.46042516752568917582e+03, 2.51633368920368957333e+03, -(1.49247451836156386662e+02)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_17() {
    *q0R2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.50444444886983272379e-07, 7.32234265963079278272e-02, 1.99819174093815998816e+00, 1.44956029347885735348e+01, 3.16662317504781540833e+01, 1.62527075710929267416e+01]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_18() {
    *q0S2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([3.03655848355219184498e+01, 2.69348118608049844624e+02, 8.44783757595320139444e+02, 8.82935845112488550512e+02, 2.12666388511798828631e+02, -(5.31095493882666946917e+00)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
