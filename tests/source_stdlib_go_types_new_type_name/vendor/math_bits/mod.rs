use go2rust_stdlib_stubs::*;

use crate::bits_errors::*;
use crate::bits_tables::*;

use std::any::Any;
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


/// LeadingZeros returns the number of leading zero bits in x; the result is [UintSize] for x == 0.
pub fn leading_zeros(x: Arc<Mutex<Option<u64>>>) -> i32 {
    return { let __tmp_x = 64; let __tmp_y = len(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x - __tmp_y };
}

/// LeadingZeros64 returns the number of leading zero bits in x; the result is 64 for x == 0.
pub fn leading_zeros64(x: Arc<Mutex<Option<u64>>>) -> i32 {
    return { let __tmp_x = 64; let __tmp_y = len64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x - __tmp_y };
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

/// Len returns the minimum number of bits required to represent x; the result is 0 for x == 0.
pub fn len(x: Arc<Mutex<Option<u64>>>) -> i32 {
    if { let __tmp_x = UINT_SIZE; let __tmp_y = 32; __tmp_x == __tmp_y } {
        return len32(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u32))));
    }
    len64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))
}

/// Len32 returns the minimum number of bits required to represent x; the result is 0 for x == 0.
pub fn len32(mut x: Arc<Mutex<Option<u32>>>) -> i32 {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as u32) << (16 as u32)) as u32; __tmp_x >= __tmp_y } {
        { let __rhs = 16 as u32; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let new_val = 16; *n.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as u32) << (8 as u32)) as u32; __tmp_x >= __tmp_y } {
        { let __rhs = 8 as u32; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = 8; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __s = &(LEN8TAB); __s.as_bytes()[({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
}

/// Len64 returns the minimum number of bits required to represent x; the result is 0 for x == 0.
pub fn len64(mut x: Arc<Mutex<Option<u64>>>) -> i32 {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as u64) << (32 as u64)) as u64; __tmp_x >= __tmp_y } {
        { let __rhs = 32 as u64; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let new_val = 32; *n.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as u64) << (16 as u64)) as u64; __tmp_x >= __tmp_y } {
        { let __rhs = 16 as u64; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = 16; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as u64) << (8 as u64)) as u64; __tmp_x >= __tmp_y } {
        { let __rhs = 8 as u64; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = 8; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __s = &(LEN8TAB); __s.as_bytes()[({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
}

/// Add returns the sum with carry of x, y and carry: sum = x + y + carry.
/// The carry input must be 0 or 1; otherwise the behavior is undefined.
/// The carryOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn add(x: Arc<Mutex<Option<u64>>>, y: Arc<Mutex<Option<u64>>>, carry: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut sum: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut carryOut: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = UINT_SIZE; let __tmp_y = 32; __tmp_x == __tmp_y } {
        let (mut s32, mut c32) = add32(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*carry.lock().unwrap().as_ref().unwrap()) as u32))));
        return ((*Arc::new(Mutex::new(Some(s32 as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(c32 as u64))).lock().unwrap().as_ref().unwrap()));
    }
    let (mut s64, mut c64) = add64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*carry.lock().unwrap().as_ref().unwrap()) as u64))));
    ((*Arc::new(Mutex::new(Some(s64 as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(c64 as u64))).lock().unwrap().as_ref().unwrap()))
}

/// Add32 returns the sum with carry of x, y and carry: sum = x + y + carry.
/// The carry input must be 0 or 1; otherwise the behavior is undefined.
/// The carryOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn add32(x: Arc<Mutex<Option<u32>>>, y: Arc<Mutex<Option<u32>>>, carry: Arc<Mutex<Option<u32>>>) -> (u32, u32) {
    let mut sum: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut carryOut: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

    let mut sum64 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*carry.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
    { let new_val = Arc::new(Mutex::new(Some((*sum64.lock().unwrap().as_ref().unwrap()) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *sum.lock().unwrap() = __moved_val; };
    { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*sum64.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *carryOut.lock().unwrap() = __moved_val; };
    return ((*sum.lock().unwrap().as_ref().unwrap()), (*carryOut.lock().unwrap().as_ref().unwrap()));
}

/// Add64 returns the sum with carry of x, y and carry: sum = x + y + carry.
/// The carry input must be 0 or 1; otherwise the behavior is undefined.
/// The carryOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn add64(x: Arc<Mutex<Option<u64>>>, y: Arc<Mutex<Option<u64>>>, carry: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut sum: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut carryOut: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*carry.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *sum.lock().unwrap() = Some(new_val); };

        // The sum will overflow if both top bits are set (x & y) or if one of them
        // is (x | y), and a carry from the lower place happened. If such a carry
        // happens, the top bit will be 1 + 0 + 1 = 0 (&^ sum).
    { let new_val = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }); let __tmp_y = { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & ! __tmp_y }); __tmp_x | __tmp_y }); let __tmp_y = 63; __tmp_x >> __tmp_y }; *carryOut.lock().unwrap() = Some(new_val); };
    return ((*sum.lock().unwrap().as_ref().unwrap()), (*carryOut.lock().unwrap().as_ref().unwrap()));
}

/// Sub returns the difference of x, y and borrow: diff = x - y - borrow.
/// The borrow input must be 0 or 1; otherwise the behavior is undefined.
/// The borrowOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn sub(x: Arc<Mutex<Option<u64>>>, y: Arc<Mutex<Option<u64>>>, borrow: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut diff: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut borrowOut: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = UINT_SIZE; let __tmp_y = 32; __tmp_x == __tmp_y } {
        let (mut d32, mut b32) = sub32(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*borrow.lock().unwrap().as_ref().unwrap()) as u32))));
        return ((*Arc::new(Mutex::new(Some(d32 as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(b32 as u64))).lock().unwrap().as_ref().unwrap()));
    }
    let (mut d64, mut b64) = sub64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*borrow.lock().unwrap().as_ref().unwrap()) as u64))));
    ((*Arc::new(Mutex::new(Some(d64 as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(b64 as u64))).lock().unwrap().as_ref().unwrap()))
}

/// Sub32 returns the difference of x, y and borrow, diff = x - y - borrow.
/// The borrow input must be 0 or 1; otherwise the behavior is undefined.
/// The borrowOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn sub32(x: Arc<Mutex<Option<u32>>>, y: Arc<Mutex<Option<u32>>>, borrow: Arc<Mutex<Option<u32>>>) -> (u32, u32) {
    let mut diff: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut borrowOut: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

    { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*borrow.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *diff.lock().unwrap() = Some(new_val); };

        // The difference will underflow if the top bit of x is not set and the top
        // bit of y is set (^x & y) or if they are the same (^(x ^ y)) and a borrow
        // from the lower place happens. If that borrow happens, the result will be
        // 1 - 1 - 1 = 0 - 0 - 1 = 1 (& diff).
    { let new_val = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = !(*x.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = ({ let __tmp_x = !({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x ^ __tmp_y }); let __tmp_y = { let __v = (*diff.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); __tmp_x | __tmp_y }); let __tmp_y = 31; __tmp_x >> __tmp_y }; *borrowOut.lock().unwrap() = Some(new_val); };
    return ((*diff.lock().unwrap().as_ref().unwrap()), (*borrowOut.lock().unwrap().as_ref().unwrap()));
}

/// Sub64 returns the difference of x, y and borrow: diff = x - y - borrow.
/// The borrow input must be 0 or 1; otherwise the behavior is undefined.
/// The borrowOut output is guaranteed to be 0 or 1.
///
/// This function's execution time does not depend on the inputs.
pub fn sub64(x: Arc<Mutex<Option<u64>>>, y: Arc<Mutex<Option<u64>>>, borrow: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut diff: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut borrowOut: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*borrow.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *diff.lock().unwrap() = Some(new_val); };

        // See Sub32 for the bit logic.
    { let new_val = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = !(*x.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = ({ let __tmp_x = !({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x ^ __tmp_y }); let __tmp_y = { let __v = (*diff.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); __tmp_x | __tmp_y }); let __tmp_y = 63; __tmp_x >> __tmp_y }; *borrowOut.lock().unwrap() = Some(new_val); };
    return ((*diff.lock().unwrap().as_ref().unwrap()), (*borrowOut.lock().unwrap().as_ref().unwrap()));
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

/// Div returns the quotient and remainder of (hi, lo) divided by y:
/// quo = (hi, lo)/y, rem = (hi, lo)%y with the dividend bits' upper
/// half in parameter hi and the lower half in parameter lo.
/// Div panics for y == 0 (division by zero) or y <= hi (quotient overflow).
pub fn div(hi: Arc<Mutex<Option<u64>>>, lo: Arc<Mutex<Option<u64>>>, y: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut quo: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut rem: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = UINT_SIZE; let __tmp_y = 32; __tmp_x == __tmp_y } {
        let (mut q, mut r) = div32(Arc::new(Mutex::new(Some((*hi.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*lo.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u32))));
        return ((*Arc::new(Mutex::new(Some(q as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(r as u64))).lock().unwrap().as_ref().unwrap()));
    }
    let (mut q, mut r) = div64(Arc::new(Mutex::new(Some((*hi.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*lo.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u64))));
    ((*Arc::new(Mutex::new(Some(q as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(r as u64))).lock().unwrap().as_ref().unwrap()))
}

/// Div32 returns the quotient and remainder of (hi, lo) divided by y:
/// quo = (hi, lo)/y, rem = (hi, lo)%y with the dividend bits' upper
/// half in parameter hi and the lower half in parameter lo.
/// Div32 panics for y == 0 (division by zero) or y <= hi (quotient overflow).
pub fn div32(hi: Arc<Mutex<Option<u32>>>, lo: Arc<Mutex<Option<u32>>>, y: Arc<Mutex<Option<u32>>>) -> (u32, u32) {
    let mut quo: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut rem: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        std::panic::panic_any({ let __err_holder = overflowError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => panic!("type info required: error-to-any has no visible dynamic error implementors") } });
    }
    let mut z = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*hi.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*lo.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y })));
    { let __tmp_0 = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }) as u32))); let __tmp_1 = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }) as u32))); *quo.lock().unwrap() = __tmp_0.lock().unwrap().take(); *rem.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    return ((*quo.lock().unwrap().as_ref().unwrap()), (*rem.lock().unwrap().as_ref().unwrap()));
}

/// Div64 returns the quotient and remainder of (hi, lo) divided by y:
/// quo = (hi, lo)/y, rem = (hi, lo)%y with the dividend bits' upper
/// half in parameter hi and the lower half in parameter lo.
/// Div64 panics for y == 0 (division by zero) or y <= hi (quotient overflow).
pub fn div64(hi: Arc<Mutex<Option<u64>>>, lo: Arc<Mutex<Option<u64>>>, mut y: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut quo: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut rem: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any({ let __err_holder = divideError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => panic!("type info required: error-to-any has no visible dynamic error implementors") } });
    }
    if { let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        std::panic::panic_any({ let __err_holder = overflowError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => panic!("type info required: error-to-any has no visible dynamic error implementors") } });
    }

        // If high part is zero, we can directly return the results.
    if { let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return ({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }, { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y });
    }

    let mut s = Arc::new(Mutex::new(Some(leading_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)));
    { let __rhs = (*s.lock().unwrap().as_ref().unwrap()); let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };

    const two32: i64 = 1 << 32;
const mask32: i64 = two32 - 1;

    let mut yn1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y })));
    let mut yn0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mask32 as u64; __tmp_x & __tmp_y })));
    let mut un32 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = 64 as u64; let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }; __tmp_x | __tmp_y })));
    let mut un10 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y })));
    let mut un1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*un10.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y })));
    let mut un0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*un10.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mask32 as u64; __tmp_x & __tmp_y })));
    let mut q1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*un32.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*yn1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y })));
    let mut rhat = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*un32.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*q1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*yn1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));

    while { let __tmp_x = { let __v = (*q1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = two32 as u64; __tmp_x >= __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*q1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*yn0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = { let __tmp_x = two32 as u64; let __tmp_y = { let __v = (*rhat.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*un1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x > __tmp_y } {
        { let mut guard = q1.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let __rhs = (*yn1.lock().unwrap().as_ref().unwrap()); let mut guard = rhat.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*rhat.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = two32 as u64; __tmp_x >= __tmp_y } {
        break
    }
    }

    let mut un21 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*un32.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = two32 as u64; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*un1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*q1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
    let mut q0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*un21.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*yn1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y })));
    { let new_val = { let __tmp_x = { let __v = (*un21.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*q0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*yn1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x - __tmp_y }; *rhat.lock().unwrap() = Some(new_val); };

    while { let __tmp_x = { let __v = (*q0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = two32 as u64; __tmp_x >= __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*q0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*yn0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __tmp_x = { let __tmp_x = two32 as u64; let __tmp_y = { let __v = (*rhat.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*un0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x > __tmp_y } {
        { let mut guard = q0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let __rhs = (*yn1.lock().unwrap().as_ref().unwrap()); let mut guard = rhat.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*rhat.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = two32 as u64; __tmp_x >= __tmp_y } {
        break
    }
    }

    return ({ let __tmp_x = { let __tmp_x = { let __v = (*q1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = two32 as u64; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*q0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }, { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*un21.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = two32 as u64; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*un0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*q0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y });
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
