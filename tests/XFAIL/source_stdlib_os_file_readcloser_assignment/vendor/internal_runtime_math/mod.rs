use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub const MAX_UINTPTR: usize = !(0 as usize);


/// MulUintptr returns a * b and whether the multiplication overflowed.
/// On supported platforms this is an intrinsic lowered by the compiler.
pub fn mul_uintptr(a: Arc<Mutex<Option<usize>>>, b: Arc<Mutex<Option<usize>>>) -> (usize, bool) {
    if { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; let __tmp_y = ((1 as usize) << ((4 as usize) * (internal_goarch::PTR_SIZE as usize))) as usize; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return (
            { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y },
            false
        );
    }
    let mut overflow = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = MAX_UINTPTR as usize; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; __tmp_x > __tmp_y })));
    return (
        { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y },
        { let __v = (*overflow.lock().unwrap().as_ref().unwrap()).clone(); __v }
    );
}

/// Mul64 returns the 128-bit product of x and y: (hi, lo) = x * y
/// with the product bits' upper half returned in hi and the lower
/// half returned in lo.
/// This is a copy from math/bits.Mul64
/// On supported platforms this is an intrinsic lowered by the compiler.
pub fn mul64(x: Arc<Mutex<Option<u64>>>, y: Arc<Mutex<Option<u64>>>) -> (u64, u64) {
    let mut hi: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut lo: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    const mask32: i64 = (((1 as i64) << (32 as i64)) - (1 as i64));

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