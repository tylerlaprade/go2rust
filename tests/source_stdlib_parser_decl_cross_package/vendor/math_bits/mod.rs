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


/// TrailingZeros returns the number of trailing zero bits in x; the result is [UintSize] for x == 0.
pub fn trailing_zeros(x: Arc<Mutex<Option<u64>>>) -> i32 {
    if { let __tmp_x = UINT_SIZE; let __tmp_y = 32; __tmp_x == __tmp_y } {
        return trailing_zeros32(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u32))));
    }
    trailing_zeros64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))
}

/// TrailingZeros32 returns the number of trailing zero bits in x; the result is 32 for x == 0.
pub fn trailing_zeros32(x: Arc<Mutex<Option<u32>>>) -> i32 {
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return 32;
    }

        // see comment in TrailingZeros64
    (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = deBruijn32tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((*x.lock().unwrap().as_ref().unwrap())).wrapping_neg(); __tmp_x & __tmp_y }); let __tmp_y = DE_BRUIJN32 as u32; __tmp_x * __tmp_y }; let __tmp_y = ({ let __tmp_x = 32; let __tmp_y = 5; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap())
}

/// TrailingZeros64 returns the number of trailing zero bits in x; the result is 64 for x == 0.
pub fn trailing_zeros64(x: Arc<Mutex<Option<u64>>>) -> i32 {
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return 64;
    }

        // If popcount is fast, replace code below with return popcount(^x & (x - 1)).
        //
        // x & -x leaves only the right-most bit set in the word. Let k be the
        // index of that bit. Since only a single bit is set, the value is two
        // to the power of k. Multiplying by a power of two is equivalent to
        // left shifting, in this case by k bits. The de Bruijn (64 bit) constant
        // is such that all six bit, consecutive substrings are distinct.
        // Therefore, if we have a left shifted version of this constant we can
        // find by how many bits it was shifted by looking at which six bit
        // substring ended up at the top of the word.
        // (Knuth, volume 4, section 7.3.1)
    (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = deBruijn64tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((*x.lock().unwrap().as_ref().unwrap())).wrapping_neg(); __tmp_x & __tmp_y }); let __tmp_y = DE_BRUIJN64 as u64; __tmp_x * __tmp_y }; let __tmp_y = ({ let __tmp_x = 64; let __tmp_y = 6; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap())
}

/// Mul returns the full-width product of x and y: (hi, lo) = x * y
/// with the product bits' upper half returned in hi and the lower
/// half returned in lo.
///
/// This function's execution time does not depend on the inputs.
pub fn mul(x: Arc<Mutex<Option<u64>>>, y: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut hi: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut lo: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = UINT_SIZE; let __tmp_y = 32; __tmp_x == __tmp_y } {
        let (mut h, mut l) = mul32(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u32))));
        return ((*Arc::new(Mutex::new(Some(h as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(l as u64))).lock().unwrap().as_ref().unwrap()));
    }
    let (mut h, mut l) = mul64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u64))));
    ((*Arc::new(Mutex::new(Some(h as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(l as u64))).lock().unwrap().as_ref().unwrap()))
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

/// Mul64 returns the 128-bit product of x and y: (hi, lo) = x * y
/// with the product bits' upper half returned in hi and the lower
/// half returned in lo.
///
/// This function's execution time does not depend on the inputs.
pub fn mul64(x: Arc<Mutex<Option<u64>>>, y: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut hi: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut lo: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    const mask32: i64 = (1 << 32) - 1;

    let mut x0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mask32 as u64; __tmp_x & __tmp_y })));
    let mut x1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y })));
    let mut y0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mask32 as u64; __tmp_x & __tmp_y })));
    let mut y1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y })));
    let mut w0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })));
    let mut t = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*w0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y })));
    let mut w1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mask32 as u64; __tmp_x & __tmp_y })));
    let mut w2 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y })));
    { let __rhs = { let __tmp_x = { let __v = (*x0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let mut guard = w1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*w2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*w1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y }; *hi.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; *lo.lock().unwrap() = Some(new_val); };
    return ((*hi.lock().unwrap().as_ref().unwrap()), (*lo.lock().unwrap().as_ref().unwrap()));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
