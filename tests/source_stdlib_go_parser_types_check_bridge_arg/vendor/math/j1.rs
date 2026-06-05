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

pub(crate) static p1R8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p1S8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p1R5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p1S5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p1R3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p1S3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p1R2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static p1S2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q1R8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q1S8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q1R5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q1S5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q1R3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q1S3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q1R2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static q1S2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 6]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *p1R8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1S8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1R5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1S5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1R3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1S3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1R2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1S2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1R8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1S8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1R5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1S5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1R3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1S3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1R2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1S2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1R8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.00000000000000000000e+00, 1.17187499999988647970e-01, 1.32394806593073575129e+01, 4.12051854307378562225e+02, 3.87474538913960532227e+03, 7.91447954031891731574e+03]))).lock().unwrap().as_ref().unwrap()).clone());
    *p1S8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.14207370375678408436e+02, 3.65093083420853463394e+03, 3.69562060269033463555e+04, 9.76027935934950801311e+04, 3.08042720627888811578e+04]))).lock().unwrap().as_ref().unwrap()).clone());
    *p1R5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.31990519556243522749e-11, 1.17187493190614097638e-01, 6.80275127868432871736e+00, 1.08308182990189109773e+02, 5.17636139533199752805e+02, 5.28715201363337541807e+02]))).lock().unwrap().as_ref().unwrap()).clone());
    *p1S5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([5.92805987221131331921e+01, 9.91401418733614377743e+02, 5.35326695291487976647e+03, 7.84469031749551231769e+03, 1.50404688810361062679e+03]))).lock().unwrap().as_ref().unwrap()).clone());
    *p1R3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([3.02503916137373618024e-09, 1.17186865567253592491e-01, 3.93297750033315640650e+00, 3.51194035591636932736e+01, 9.10550110750781271918e+01, 4.85590685197364919645e+01]))).lock().unwrap().as_ref().unwrap()).clone());
    *p1S3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([3.47913095001251519989e+01, 3.36762458747825746741e+02, 1.04687139975775130551e+03, 8.90811346398256432622e+02, 1.03787932439639277504e+02]))).lock().unwrap().as_ref().unwrap()).clone());
    *p1R2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.07710830106873743082e-07, 1.17176219462683348094e-01, 2.36851496667608785174e+00, 1.22426109148261232917e+01, 1.76939711271687727390e+01, 5.07352312588818499250e+00]))).lock().unwrap().as_ref().unwrap()).clone());
    *p1S2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([2.14364859363821409488e+01, 1.25290227168402751090e+02, 2.32276469057162813669e+02, 1.17679373287147100768e+02, 8.36463893371618283368e+00]))).lock().unwrap().as_ref().unwrap()).clone());
    *q1R8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.00000000000000000000e+00, -(1.02539062499992714161e-01), -(1.62717534544589987888e+01), -(7.59601722513950107896e+02), -(1.18498066702429587167e+04), -(4.84385124285750353010e+04)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q1S8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.61395369700722909556e+02, 7.82538599923348465381e+03, 1.33875336287249578163e+05, 7.19657723683240939863e+05, 6.66601232617776375264e+05, -(2.94490264303834643215e+05)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q1R5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(2.08979931141764104297e-11), -(1.02539050241375426231e-01), -(8.05644828123936029840e+00), -(1.83669607474888380239e+02), -(1.37319376065508163265e+03), -(2.61244440453215656817e+03)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q1S5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([8.12765501384335777857e+01, 1.99179873460485964642e+03, 1.74684851924908907677e+04, 4.98514270910352279316e+04, 2.79480751638918118260e+04, -(4.71918354795128470869e+03)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q1R3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(5.07831226461766561369e-09), -(1.02537829820837089745e-01), -(4.61011581139473403113e+00), -(5.78472216562783643212e+01), -(2.28244540737631695038e+02), -(2.19210128478909325622e+02)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q1S3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.76651550323729509273e+01, 6.73865112676699709482e+02, 3.38015286679526343505e+03, 5.54772909720722782367e+03, 1.90311919338810798763e+03, -(1.35201191444307340817e+02)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q1R2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(1.78381727510958865572e-07), -(1.02517042607985553460e-01), -(2.75220568278187460720e+00), -(1.96636162643703720221e+01), -(4.23253133372830490089e+01), -(2.13719211703704061733e+01)]))).lock().unwrap().as_ref().unwrap()).clone());
    *q1S2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([2.95333629060523854548e+01, 2.52981549982190529136e+02, 7.57502834868645436472e+02, 7.39393205320467245656e+02, 1.55949003336666123687e+02, -(4.95949898822628210127e+00)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *p1R8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1S8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1R5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1S5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1R3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1S3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1R2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *p1S2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1R8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1S8.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1R5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1S5.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1R3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1S3.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1R2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *q1S2.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_19() {
    *p1R8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.00000000000000000000e+00, 1.17187499999988647970e-01, 1.32394806593073575129e+01, 4.12051854307378562225e+02, 3.87474538913960532227e+03, 7.91447954031891731574e+03]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_20() {
    *p1S8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.14207370375678408436e+02, 3.65093083420853463394e+03, 3.69562060269033463555e+04, 9.76027935934950801311e+04, 3.08042720627888811578e+04]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_21() {
    *p1R5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.31990519556243522749e-11, 1.17187493190614097638e-01, 6.80275127868432871736e+00, 1.08308182990189109773e+02, 5.17636139533199752805e+02, 5.28715201363337541807e+02]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_22() {
    *p1S5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([5.92805987221131331921e+01, 9.91401418733614377743e+02, 5.35326695291487976647e+03, 7.84469031749551231769e+03, 1.50404688810361062679e+03]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_23() {
    *p1R3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([3.02503916137373618024e-09, 1.17186865567253592491e-01, 3.93297750033315640650e+00, 3.51194035591636932736e+01, 9.10550110750781271918e+01, 4.85590685197364919645e+01]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_24() {
    *p1S3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([3.47913095001251519989e+01, 3.36762458747825746741e+02, 1.04687139975775130551e+03, 8.90811346398256432622e+02, 1.03787932439639277504e+02]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_25() {
    *p1R2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.07710830106873743082e-07, 1.17176219462683348094e-01, 2.36851496667608785174e+00, 1.22426109148261232917e+01, 1.76939711271687727390e+01, 5.07352312588818499250e+00]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_26() {
    *p1S2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([2.14364859363821409488e+01, 1.25290227168402751090e+02, 2.32276469057162813669e+02, 1.17679373287147100768e+02, 8.36463893371618283368e+00]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_27() {
    *q1R8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.00000000000000000000e+00, -(1.02539062499992714161e-01), -(1.62717534544589987888e+01), -(7.59601722513950107896e+02), -(1.18498066702429587167e+04), -(4.84385124285750353010e+04)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_28() {
    *q1S8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1.61395369700722909556e+02, 7.82538599923348465381e+03, 1.33875336287249578163e+05, 7.19657723683240939863e+05, 6.66601232617776375264e+05, -(2.94490264303834643215e+05)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_29() {
    *q1R5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(2.08979931141764104297e-11), -(1.02539050241375426231e-01), -(8.05644828123936029840e+00), -(1.83669607474888380239e+02), -(1.37319376065508163265e+03), -(2.61244440453215656817e+03)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_30() {
    *q1S5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([8.12765501384335777857e+01, 1.99179873460485964642e+03, 1.74684851924908907677e+04, 4.98514270910352279316e+04, 2.79480751638918118260e+04, -(4.71918354795128470869e+03)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_31() {
    *q1R3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(5.07831226461766561369e-09), -(1.02537829820837089745e-01), -(4.61011581139473403113e+00), -(5.78472216562783643212e+01), -(2.28244540737631695038e+02), -(2.19210128478909325622e+02)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_32() {
    *q1S3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([4.76651550323729509273e+01, 6.73865112676699709482e+02, 3.38015286679526343505e+03, 5.54772909720722782367e+03, 1.90311919338810798763e+03, -(1.35201191444307340817e+02)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_33() {
    *q1R2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([-(1.78381727510958865572e-07), -(1.02517042607985553460e-01), -(2.75220568278187460720e+00), -(1.96636162643703720221e+01), -(4.23253133372830490089e+01), -(2.13719211703704061733e+01)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_34() {
    *q1S2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([2.95333629060523854548e+01, 2.52981549982190529136e+02, 7.57502834868645436472e+02, 7.39393205320467245656e+02, 1.55949003336666123687e+02, -(4.95949898822628210127e+00)]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
