use go2rust_stdlib_stubs::*;

use crate::{GoInteger, go_integer_add_one, go_integer_cast, go_integer_from_i128, go_integer_sub_one};

use crate::atob::*;
use crate::atoc::*;
use crate::atof::*;
use crate::atoi::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::isprint::*;
use crate::itoa::*;
use crate::quote::*;

use std::sync::{Arc, Mutex};

pub(crate) static uint64pow10: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 20]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *uint64pow10.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *uint64pow10.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1 as u64, 1e1 as u64, 1e2 as u64, 1e3 as u64, 1e4 as u64, 1e5 as u64, 1e6 as u64, 1e7 as u64, 1e8 as u64, 1e9 as u64, 1e10 as u64, 1e11 as u64, 1e12 as u64, 1e13 as u64, 1e14 as u64, 1e15 as u64, 1e16 as u64, 1e17 as u64, 1e18 as u64, 1e19 as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *uint64pow10.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_10() {
    *uint64pow10.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1 as u64, 1e1 as u64, 1e2 as u64, 1e3 as u64, 1e4 as u64, 1e5 as u64, 1e6 as u64, 1e7 as u64, 1e8 as u64, 1e9 as u64, 1e10 as u64, 1e11 as u64, 1e12 as u64, 1e13 as u64, 1e14 as u64, 1e15 as u64, 1e16 as u64, 1e17 as u64, 1e18 as u64, 1e19 as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
