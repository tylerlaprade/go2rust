use go2rust_stdlib_stubs::*;

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
    {
        let mut __go_array = Vec::<u8>::with_capacity(32);
        __go_array.push(0 as u8);
        __go_array.push(1 as u8);
        __go_array.push(28 as u8);
        __go_array.push(2 as u8);
        __go_array.push(29 as u8);
        __go_array.push(14 as u8);
        __go_array.push(24 as u8);
        __go_array.push(3 as u8);
        __go_array.push(30 as u8);
        __go_array.push(22 as u8);
        __go_array.push(20 as u8);
        __go_array.push(15 as u8);
        __go_array.push(25 as u8);
        __go_array.push(17 as u8);
        __go_array.push(4 as u8);
        __go_array.push(8 as u8);
        __go_array.push(31 as u8);
        __go_array.push(27 as u8);
        __go_array.push(13 as u8);
        __go_array.push(23 as u8);
        __go_array.push(21 as u8);
        __go_array.push(19 as u8);
        __go_array.push(16 as u8);
        __go_array.push(7 as u8);
        __go_array.push(26 as u8);
        __go_array.push(12 as u8);
        __go_array.push(18 as u8);
        __go_array.push(6 as u8);
        __go_array.push(11 as u8);
        __go_array.push(5 as u8);
        __go_array.push(10 as u8);
        __go_array.push(9 as u8);
        let __go_array: [u8; 32] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *deBruijn32tab.lock().unwrap() = Some(__go_array);
    }
    {
        let mut __go_array = Vec::<u8>::with_capacity(64);
        __go_array.push(0 as u8);
        __go_array.push(1 as u8);
        __go_array.push(56 as u8);
        __go_array.push(2 as u8);
        __go_array.push(57 as u8);
        __go_array.push(49 as u8);
        __go_array.push(28 as u8);
        __go_array.push(3 as u8);
        __go_array.push(61 as u8);
        __go_array.push(58 as u8);
        __go_array.push(42 as u8);
        __go_array.push(50 as u8);
        __go_array.push(38 as u8);
        __go_array.push(29 as u8);
        __go_array.push(17 as u8);
        __go_array.push(4 as u8);
        __go_array.push(62 as u8);
        __go_array.push(47 as u8);
        __go_array.push(59 as u8);
        __go_array.push(36 as u8);
        __go_array.push(45 as u8);
        __go_array.push(43 as u8);
        __go_array.push(51 as u8);
        __go_array.push(22 as u8);
        __go_array.push(53 as u8);
        __go_array.push(39 as u8);
        __go_array.push(33 as u8);
        __go_array.push(30 as u8);
        __go_array.push(24 as u8);
        __go_array.push(18 as u8);
        __go_array.push(12 as u8);
        __go_array.push(5 as u8);
        __go_array.push(63 as u8);
        __go_array.push(55 as u8);
        __go_array.push(48 as u8);
        __go_array.push(27 as u8);
        __go_array.push(60 as u8);
        __go_array.push(41 as u8);
        __go_array.push(37 as u8);
        __go_array.push(16 as u8);
        __go_array.push(46 as u8);
        __go_array.push(35 as u8);
        __go_array.push(44 as u8);
        __go_array.push(21 as u8);
        __go_array.push(52 as u8);
        __go_array.push(32 as u8);
        __go_array.push(23 as u8);
        __go_array.push(11 as u8);
        __go_array.push(54 as u8);
        __go_array.push(26 as u8);
        __go_array.push(40 as u8);
        __go_array.push(15 as u8);
        __go_array.push(34 as u8);
        __go_array.push(20 as u8);
        __go_array.push(31 as u8);
        __go_array.push(10 as u8);
        __go_array.push(25 as u8);
        __go_array.push(14 as u8);
        __go_array.push(19 as u8);
        __go_array.push(9 as u8);
        __go_array.push(13 as u8);
        __go_array.push(8 as u8);
        __go_array.push(7 as u8);
        __go_array.push(6 as u8);
        let __go_array: [u8; 64] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *deBruijn64tab.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *deBruijn32tab.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *deBruijn64tab.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_0() {
    {
        let mut __go_array = Vec::<u8>::with_capacity(32);
        __go_array.push(0 as u8);
        __go_array.push(1 as u8);
        __go_array.push(28 as u8);
        __go_array.push(2 as u8);
        __go_array.push(29 as u8);
        __go_array.push(14 as u8);
        __go_array.push(24 as u8);
        __go_array.push(3 as u8);
        __go_array.push(30 as u8);
        __go_array.push(22 as u8);
        __go_array.push(20 as u8);
        __go_array.push(15 as u8);
        __go_array.push(25 as u8);
        __go_array.push(17 as u8);
        __go_array.push(4 as u8);
        __go_array.push(8 as u8);
        __go_array.push(31 as u8);
        __go_array.push(27 as u8);
        __go_array.push(13 as u8);
        __go_array.push(23 as u8);
        __go_array.push(21 as u8);
        __go_array.push(19 as u8);
        __go_array.push(16 as u8);
        __go_array.push(7 as u8);
        __go_array.push(26 as u8);
        __go_array.push(12 as u8);
        __go_array.push(18 as u8);
        __go_array.push(6 as u8);
        __go_array.push(11 as u8);
        __go_array.push(5 as u8);
        __go_array.push(10 as u8);
        __go_array.push(9 as u8);
        let __go_array: [u8; 32] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *deBruijn32tab.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_order_1() {
    {
        let mut __go_array = Vec::<u8>::with_capacity(64);
        __go_array.push(0 as u8);
        __go_array.push(1 as u8);
        __go_array.push(56 as u8);
        __go_array.push(2 as u8);
        __go_array.push(57 as u8);
        __go_array.push(49 as u8);
        __go_array.push(28 as u8);
        __go_array.push(3 as u8);
        __go_array.push(61 as u8);
        __go_array.push(58 as u8);
        __go_array.push(42 as u8);
        __go_array.push(50 as u8);
        __go_array.push(38 as u8);
        __go_array.push(29 as u8);
        __go_array.push(17 as u8);
        __go_array.push(4 as u8);
        __go_array.push(62 as u8);
        __go_array.push(47 as u8);
        __go_array.push(59 as u8);
        __go_array.push(36 as u8);
        __go_array.push(45 as u8);
        __go_array.push(43 as u8);
        __go_array.push(51 as u8);
        __go_array.push(22 as u8);
        __go_array.push(53 as u8);
        __go_array.push(39 as u8);
        __go_array.push(33 as u8);
        __go_array.push(30 as u8);
        __go_array.push(24 as u8);
        __go_array.push(18 as u8);
        __go_array.push(12 as u8);
        __go_array.push(5 as u8);
        __go_array.push(63 as u8);
        __go_array.push(55 as u8);
        __go_array.push(48 as u8);
        __go_array.push(27 as u8);
        __go_array.push(60 as u8);
        __go_array.push(41 as u8);
        __go_array.push(37 as u8);
        __go_array.push(16 as u8);
        __go_array.push(46 as u8);
        __go_array.push(35 as u8);
        __go_array.push(44 as u8);
        __go_array.push(21 as u8);
        __go_array.push(52 as u8);
        __go_array.push(32 as u8);
        __go_array.push(23 as u8);
        __go_array.push(11 as u8);
        __go_array.push(54 as u8);
        __go_array.push(26 as u8);
        __go_array.push(40 as u8);
        __go_array.push(15 as u8);
        __go_array.push(34 as u8);
        __go_array.push(20 as u8);
        __go_array.push(31 as u8);
        __go_array.push(10 as u8);
        __go_array.push(25 as u8);
        __go_array.push(14 as u8);
        __go_array.push(19 as u8);
        __go_array.push(9 as u8);
        __go_array.push(13 as u8);
        __go_array.push(8 as u8);
        __go_array.push(7 as u8);
        __go_array.push(6 as u8);
        let __go_array: [u8; 64] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *deBruijn64tab.lock().unwrap() = Some(__go_array);
    }
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
