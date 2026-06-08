use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub(crate) const DE_BRUIJN32: i32 = 0x077CB531;


pub(crate) const DE_BRUIJN64: i64 = 0x03f79d71b4ca8b09;


pub(crate) const NTZ8TAB: &'static str = "\u{8}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{4}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{5}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{4}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{6}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{4}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{5}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{4}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{7}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{4}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{5}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{4}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{6}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{4}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{5}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{4}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}\u{3}\u{0}\u{1}\u{0}\u{2}\u{0}\u{1}\u{0}";


pub(crate) const LEN8TAB: &'static str = "\u{0}\u{1}\u{2}\u{2}\u{3}\u{3}\u{3}\u{3}\u{4}\u{4}\u{4}\u{4}\u{4}\u{4}\u{4}\u{4}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{5}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{6}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{7}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}\u{8}";


pub(crate) const M0: i64 = 0x5555555555555555;


pub(crate) const M1: i64 = 0x3333333333333333;


pub(crate) const M2: i64 = 0x0f0f0f0f0f0f0f0f;


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


pub(crate) fn __go_init_order_1() {
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


pub(crate) fn __go_init_order_2() {
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

/// TrailingZeros8 returns the number of trailing zero bits in x; the result is 8 for x == 0.
pub fn trailing_zeros8(x: Arc<Mutex<Option<u8>>>) -> i32 {
    (*Arc::new(Mutex::new(Some({ let __s = &(NTZ8TAB); __s.as_bytes()[({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i32))).lock().unwrap().as_ref().unwrap())
}

/// Len64 returns the minimum number of bits required to represent x; the result is 0 for x == 0.
///
/// nosplit because this is used in src/runtime/histogram.go, which make run in sensitive contexts.
///
///go:nosplit
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

/// OnesCount64 returns the number of one bits ("population count") in x.
pub fn ones_count64(mut x: Arc<Mutex<Option<u64>>>) -> i32 {
        // Implementation: Parallel summing of adjacent bits.
        // See "Hacker's Delight", Chap. 5: Counting Bits.
        // The following pattern shows the general approach:
        //
        //   x = x>>1&(m0&m) + x&(m0&m)
        //   x = x>>2&(m1&m) + x&(m1&m)
        //   x = x>>4&(m2&m) + x&(m2&m)
        //   x = x>>8&(m3&m) + x&(m3&m)
        //   x = x>>16&(m4&m) + x&(m4&m)
        //   x = x>>32&(m5&m) + x&(m5&m)
        //   return int(x)
        //
        // Masking (& operations) can be left away when there's no
        // danger that a field's sum will carry over into the next
        // field: Since the result cannot be > 64, 8 bits is enough
        // and we can ignore the masks for the shifts by 8 and up.
        // Per "Hacker's Delight", the first line can be simplified
        // more, but it saves at best one instruction, so we leave
        // it alone for clarity.
    const m: u128 = (((1 as u128) << (64 as u128)) - (1 as u128));

    { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; let __tmp_y = ((M0 as u64) & (m as u64)) as u64; __tmp_x & __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((M0 as u64) & (m as u64)) as u64; __tmp_x & __tmp_y }; __tmp_x + __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x >> __tmp_y }; let __tmp_y = ((M1 as u64) & (m as u64)) as u64; __tmp_x & __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((M1 as u64) & (m as u64)) as u64; __tmp_x & __tmp_y }; __tmp_x + __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x >> __tmp_y }; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = ((M2 as u64) & (m as u64)) as u64; __tmp_x & __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    { let __rhs = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >> __tmp_y }; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __rhs = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y }; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __rhs = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    return { let __tmp_x = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 127; __tmp_x & __tmp_y };
}

/// LeadingZeros64 returns the number of leading zero bits in x; the result is 64 for x == 0.
pub fn leading_zeros64(x: Arc<Mutex<Option<u64>>>) -> i32 {
    return { let __tmp_x = 64; let __tmp_y = len64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x - __tmp_y };
}

/// LeadingZeros8 returns the number of leading zero bits in x; the result is 8 for x == 0.
pub fn leading_zeros8(x: Arc<Mutex<Option<u8>>>) -> i32 {
    return { let __tmp_x = 8; let __tmp_y = len8(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x - __tmp_y };
}

/// Len8 returns the minimum number of bits required to represent x; the result is 0 for x == 0.
pub fn len8(x: Arc<Mutex<Option<u8>>>) -> i32 {
    (*Arc::new(Mutex::new(Some({ let __s = &(LEN8TAB); __s.as_bytes()[({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i32))).lock().unwrap().as_ref().unwrap())
}

/// Bswap64 returns its input with byte order reversed
/// 0x0102030405060708 -> 0x0807060504030201
pub fn bswap64(mut x: Arc<Mutex<Option<u64>>>) -> u64 {
    let mut c8 = Arc::new(Mutex::new(Some(0x00ff00ff00ff00ff as u64)));
    let mut a = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >> __tmp_y }; let __tmp_y = { let __v = (*c8.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y })));
    let mut b = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c8.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = 8; __tmp_x << __tmp_y })));
    { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    let mut c16 = Arc::new(Mutex::new(Some(0x0000ffff0000ffff as u64)));
    { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y }; let __tmp_y = { let __v = (*c16.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c16.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = 16; __tmp_x << __tmp_y }; *b.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    let mut c32 = Arc::new(Mutex::new(Some(0x00000000ffffffff as u64)));
    { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }; let __tmp_y = { let __v = (*c32.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c32.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = 32; __tmp_x << __tmp_y }; *b.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// Bswap32 returns its input with byte order reversed
/// 0x01020304 -> 0x04030201
pub fn bswap32(mut x: Arc<Mutex<Option<u32>>>) -> u32 {
    let mut c8 = Arc::new(Mutex::new(Some(0x00ff00ff as u32)));
    let mut a = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >> __tmp_y }; let __tmp_y = { let __v = (*c8.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y })));
    let mut b = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c8.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = 8; __tmp_x << __tmp_y })));
    { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    let mut c16 = Arc::new(Mutex::new(Some(0x0000ffff as u32)));
    { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y }; let __tmp_y = { let __v = (*c16.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c16.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = 16; __tmp_x << __tmp_y }; *b.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; *x.lock().unwrap() = Some(new_val); };
    return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// Prefetch prefetches data from memory addr to cache
///
/// AMD64: Produce PREFETCHT0 instruction
///
/// ARM64: Produce PRFM instruction with PLDL1KEEP option
pub fn prefetch(addr: Arc<Mutex<Option<usize>>>) {
}

pub fn get_caller_p_c() -> usize {
    unimplemented!("Go function declaration has no body");
}


pub fn get_caller_s_p() -> usize {
    unimplemented!("Go function declaration has no body");
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
