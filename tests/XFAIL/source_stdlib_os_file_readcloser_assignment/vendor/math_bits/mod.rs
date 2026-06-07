use go2rust_stdlib_stubs::*;

use crate::bits_errors::*;
use crate::bits_tables::*;

use std::sync::{Arc, Mutex};

pub(crate) const UINT_SIZE_1: i32 = 32 << (!(0 as u64) >> 63);


pub const UINT_SIZE: i32 = UINT_SIZE_1;


pub(crate) const DE_BRUIJN32: i32 = 0x077CB531;


pub(crate) const DE_BRUIJN64: i64 = 0x03f79d71b4ca8b09;


pub(crate) const M0: i64 = 0x5555555555555555;


pub(crate) const M1: i64 = 0x3333333333333333;


pub(crate) const M2: i64 = 0x0f0f0f0f0f0f0f0f;


pub(crate) const M3: i64 = 0x00ff00ff00ff00ff;


pub(crate) const M4: i64 = 0x0000ffff0000ffff;


pub(crate) static deBruijn32tab: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 32]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static deBruijn64tab: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 64]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *deBruijn32tab.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *deBruijn64tab.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *deBruijn32tab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 1 as u8, 28 as u8, 2 as u8, 29 as u8, 14 as u8, 24 as u8, 3 as u8, 30 as u8, 22 as u8, 20 as u8, 15 as u8, 25 as u8, 17 as u8, 4 as u8, 8 as u8, 31 as u8, 27 as u8, 13 as u8, 23 as u8, 21 as u8, 19 as u8, 16 as u8, 7 as u8, 26 as u8, 12 as u8, 18 as u8, 6 as u8, 11 as u8, 5 as u8, 10 as u8, 9 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
    *deBruijn64tab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 1 as u8, 56 as u8, 2 as u8, 57 as u8, 49 as u8, 28 as u8, 3 as u8, 61 as u8, 58 as u8, 42 as u8, 50 as u8, 38 as u8, 29 as u8, 17 as u8, 4 as u8, 62 as u8, 47 as u8, 59 as u8, 36 as u8, 45 as u8, 43 as u8, 51 as u8, 22 as u8, 53 as u8, 39 as u8, 33 as u8, 30 as u8, 24 as u8, 18 as u8, 12 as u8, 5 as u8, 63 as u8, 55 as u8, 48 as u8, 27 as u8, 60 as u8, 41 as u8, 37 as u8, 16 as u8, 46 as u8, 35 as u8, 44 as u8, 21 as u8, 52 as u8, 32 as u8, 23 as u8, 11 as u8, 54 as u8, 26 as u8, 40 as u8, 15 as u8, 34 as u8, 20 as u8, 31 as u8, 10 as u8, 25 as u8, 14 as u8, 19 as u8, 9 as u8, 13 as u8, 8 as u8, 7 as u8, 6 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *deBruijn32tab.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *deBruijn64tab.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_0() {
    *deBruijn32tab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 1 as u8, 28 as u8, 2 as u8, 29 as u8, 14 as u8, 24 as u8, 3 as u8, 30 as u8, 22 as u8, 20 as u8, 15 as u8, 25 as u8, 17 as u8, 4 as u8, 8 as u8, 31 as u8, 27 as u8, 13 as u8, 23 as u8, 21 as u8, 19 as u8, 16 as u8, 7 as u8, 26 as u8, 12 as u8, 18 as u8, 6 as u8, 11 as u8, 5 as u8, 10 as u8, 9 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_1() {
    *deBruijn64tab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 1 as u8, 56 as u8, 2 as u8, 57 as u8, 49 as u8, 28 as u8, 3 as u8, 61 as u8, 58 as u8, 42 as u8, 50 as u8, 38 as u8, 29 as u8, 17 as u8, 4 as u8, 62 as u8, 47 as u8, 59 as u8, 36 as u8, 45 as u8, 43 as u8, 51 as u8, 22 as u8, 53 as u8, 39 as u8, 33 as u8, 30 as u8, 24 as u8, 18 as u8, 12 as u8, 5 as u8, 63 as u8, 55 as u8, 48 as u8, 27 as u8, 60 as u8, 41 as u8, 37 as u8, 16 as u8, 46 as u8, 35 as u8, 44 as u8, 21 as u8, 52 as u8, 32 as u8, 23 as u8, 11 as u8, 54 as u8, 26 as u8, 40 as u8, 15 as u8, 34 as u8, 20 as u8, 31 as u8, 10 as u8, 25 as u8, 14 as u8, 19 as u8, 9 as u8, 13 as u8, 8 as u8, 7 as u8, 6 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


/// Mul32 returns the 64-bit product of x and y: (hi, lo) = x * y
/// with the product bits' upper half returned in hi and the lower
/// half returned in lo.
///
/// This function's execution time does not depend on the inputs.
pub fn mul32(x: Arc<Mutex<Option<u32>>>, y: Arc<Mutex<Option<u32>>>) -> (u32, u32) {
    let mut hi: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut lo: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

    let mut tmp = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
    { let __tmp_0 = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*tmp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }) as u32))); let __tmp_1 = Arc::new(Mutex::new(Some((*tmp.lock().unwrap().as_ref().unwrap()) as u32))); *hi.lock().unwrap() = __tmp_0.lock().unwrap().take(); *lo.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    return ((*hi.lock().unwrap().as_ref().unwrap()), (*lo.lock().unwrap().as_ref().unwrap()));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
