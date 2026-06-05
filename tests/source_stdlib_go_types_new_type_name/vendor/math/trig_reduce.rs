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
use crate::sqrt::*;
use crate::stubs::*;
use crate::tan::*;
use crate::tanh::*;
use crate::r#unsafe::*;

use std::sync::{Arc, Mutex};

pub(crate) const REDUCE_THRESHOLD: i32 = 1 << 29;


pub(crate) static mPi4: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 20]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *mPi4.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *mPi4.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0x0000000000000001 as u64, 0x45f306dc9c882a53 as u64, 0xf84eafa3ea69bb81 as u64, 0xb6c52b3278872083 as u64, 0xfca2c757bd778ac3 as u64, 0x6e48dc74849ba5c0 as u64, 0x0c925dd413a32439 as u64, 0xfc3bd63962534e7d as u64, 0xd1046bea5d768909 as u64, 0xd338e04d68befc82 as u64, 0x7323ac7306a673e9 as u64, 0x3908bf177bf25076 as u64, 0x3ff12fffbc0b301f as u64, 0xde5e2316b414da3e as u64, 0xda6cfd9e4f96136e as u64, 0x9e8c7ecd3cbfd45a as u64, 0xea4f758fd7cbe2f6 as u64, 0x7a0e73ef14a525d4 as u64, 0xd7f6bf623f1aba10 as u64, 0xac06608df8f6d757 as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *mPi4.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_51() {
    *mPi4.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0x0000000000000001 as u64, 0x45f306dc9c882a53 as u64, 0xf84eafa3ea69bb81 as u64, 0xb6c52b3278872083 as u64, 0xfca2c757bd778ac3 as u64, 0x6e48dc74849ba5c0 as u64, 0x0c925dd413a32439 as u64, 0xfc3bd63962534e7d as u64, 0xd1046bea5d768909 as u64, 0xd338e04d68befc82 as u64, 0x7323ac7306a673e9 as u64, 0x3908bf177bf25076 as u64, 0x3ff12fffbc0b301f as u64, 0xde5e2316b414da3e as u64, 0xda6cfd9e4f96136e as u64, 0x9e8c7ecd3cbfd45a as u64, 0xea4f758fd7cbe2f6 as u64, 0x7a0e73ef14a525d4 as u64, 0xd7f6bf623f1aba10 as u64, 0xac06608df8f6d757 as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
