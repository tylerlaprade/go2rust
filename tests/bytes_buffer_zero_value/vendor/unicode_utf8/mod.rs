use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const RUNE_ERROR: i32 = ('\u{fffd}' as i32);
pub const RUNE_SELF: i32 = 0x80;
pub const MAX_RUNE: i32 = ('\u{10ffff}' as i32);
pub const U_T_F_MAX: i32 = 4;


pub(crate) const SURROGATE_MIN: i32 = 0xD800;
pub(crate) const SURROGATE_MAX: i32 = 0xDFFF;


pub(crate) const T1: i32 = 0b00000000;
pub(crate) const TX: i32 = 0b10000000;
pub(crate) const T2: i32 = 0b11000000;
pub(crate) const T3: i32 = 0b11100000;
pub(crate) const T4: i32 = 0b11110000;
pub(crate) const T5: i32 = 0b11111000;
pub(crate) const MASKX: i32 = 0b00111111;
pub(crate) const MASK2: i32 = 0b00011111;
pub(crate) const MASK3: i32 = 0b00001111;
pub(crate) const MASK4: i32 = 0b00000111;
pub(crate) const RUNE1_MAX: i32 = (((1 as i32) << (7 as i32)) - (1 as i32));
pub(crate) const RUNE2_MAX: i32 = (((1 as i32) << (11 as i32)) - (1 as i32));
pub(crate) const RUNE3_MAX: i32 = (((1 as i32) << (16 as i32)) - (1 as i32));
pub(crate) const LOCB: i32 = 0b10000000;
pub(crate) const HICB: i32 = 0b10111111;
pub(crate) const XX: i32 = 0xF1;
pub(crate) const AS: i32 = 0xF0;
pub(crate) const S1: i32 = 0x02;
pub(crate) const S2: i32 = 0x13;
pub(crate) const S3: i32 = 0x03;
pub(crate) const S4: i32 = 0x23;
pub(crate) const S5: i32 = 0x34;
pub(crate) const S6: i32 = 0x04;
pub(crate) const S7: i32 = 0x44;


pub(crate) const RUNE_ERROR_BYTE0: i32 = ((T3 as i32) | ((RUNE_ERROR as i32) >> (12 as i32)));
pub(crate) const RUNE_ERROR_BYTE1: i32 = ((TX as i32) | (((RUNE_ERROR as i32) >> (6 as i32)) & (MASKX as i32)));
pub(crate) const RUNE_ERROR_BYTE2: i32 = TX | RUNE_ERROR & MASKX;


/// acceptRange gives the range of valid values for the second byte in a UTF-8
/// sequence.
#[derive(Debug, Clone)]
pub struct acceptRange {
    pub lo: Arc<Mutex<Option<u8>>>,
    pub hi: Arc<Mutex<Option<u8>>>,
}

impl acceptRange {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.hi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lo: __go_clone_0_0,
            hi: __go_clone_1_0,
        }
    }
}


impl Default for acceptRange {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            lo: __go_default_0_0,
            hi: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for acceptRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lo.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.hi.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static first: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 256]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static acceptRanges: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[acceptRange; 16]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *first.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *acceptRanges.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    {
        let mut __go_array = Vec::<u8>::with_capacity(256);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S1 as u8);
        __go_array.push(S2 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S4 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S3 as u8);
        __go_array.push(S5 as u8);
        __go_array.push(S6 as u8);
        __go_array.push(S6 as u8);
        __go_array.push(S6 as u8);
        __go_array.push(S7 as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        __go_array.push(XX as u8);
        let __go_array: [u8; 256] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *first.lock().unwrap() = Some(__go_array);
    }
    {
        let mut __go_array = Vec::<acceptRange>::with_capacity(16);
        __go_array.push(acceptRange { lo: Arc::new(Mutex::new(Some(LOCB as u8))), hi: Arc::new(Mutex::new(Some(HICB as u8))), ..Default::default() });
        __go_array.push(acceptRange { lo: Arc::new(Mutex::new(Some(0xA0 as u8))), hi: Arc::new(Mutex::new(Some(HICB as u8))), ..Default::default() });
        __go_array.push(acceptRange { lo: Arc::new(Mutex::new(Some(LOCB as u8))), hi: Arc::new(Mutex::new(Some(0x9F as u8))), ..Default::default() });
        __go_array.push(acceptRange { lo: Arc::new(Mutex::new(Some(0x90 as u8))), hi: Arc::new(Mutex::new(Some(HICB as u8))), ..Default::default() });
        __go_array.push(acceptRange { lo: Arc::new(Mutex::new(Some(LOCB as u8))), hi: Arc::new(Mutex::new(Some(0x8F as u8))), ..Default::default() });
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        __go_array.push(Default::default());
        let __go_array: [acceptRange; 16] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *acceptRanges.lock().unwrap() = Some(__go_array);
    }
}


/// DecodeRune unpacks the first UTF-8 encoding in p and returns the rune and
/// its width in bytes. If p is empty it returns ([RuneError], 0). Otherwise, if
/// the encoding is invalid, it returns (RuneError, 1). Both are impossible
/// results for correct, non-empty UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
pub fn decode_rune(p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, i32) {
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut size: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    let mut n = Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 0);
    }
    let mut p0 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
    let mut x = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = first.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*p0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = AS as u8; __tmp_x >= __tmp_y } {
                // The following code simulates an additional check for x == xx and
                // handling the ASCII and invalid cases accordingly. This mask-and-or
                // approach prevents an additional branch.
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 31; __tmp_x << __tmp_y }; let __tmp_y = 31; __tmp_x >> __tmp_y })));
        return (
            {
                let __tmp_x = {
                    let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap());
                    let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v };
                    __tmp_x & ! __tmp_y
                };
                let __tmp_y = { let __tmp_x = RUNE_ERROR as i32; let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y };
                __tmp_x | __tmp_y
            },
            1
        );
    }
        // The following code simulates an additional check for x == xx and
        // handling the ASCII and invalid cases accordingly. This mask-and-or
        // approach prevents an additional branch.
        // Create 0x0000 or 0xFFFF.
    let mut sz = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7 as u8; __tmp_x & __tmp_y }) as i32)));
    let mut accept = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = acceptRanges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x >> __tmp_y }) as usize].clone() })));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    let mut b1 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })));
    if { let __tmp_x = { let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*accept.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*accept.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    if { let __tmp_x = { let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x <= __tmp_y } {
        return (
            { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*p0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK2 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 6; __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y },
            2
        );
    }
    let mut b2 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() })));
    if { let __tmp_x = { let __v = (*b2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = LOCB as u8; __tmp_x < __tmp_y } || { let __tmp_x = HICB as u8; let __tmp_y = { let __v = (*b2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    if { let __tmp_x = { let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        return (
            {
            let __go_binary_0 = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*p0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK3 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = 12;
            let __go_binary_2 = __go_binary_0 << __go_binary_1;
            let __go_binary_3 = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
            let __go_binary_4 = 6;
            let __go_binary_5 = __go_binary_3 << __go_binary_4;
            let __go_binary_6 = __go_binary_2 | __go_binary_5;
            let __go_binary_7 = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
            let __go_binary_8 = __go_binary_6 | __go_binary_7;
            __go_binary_8
        },
            3
        );
    }
    let mut b3 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() })));
    if { let __tmp_x = { let __v = (*b3.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = LOCB as u8; __tmp_x < __tmp_y } || { let __tmp_x = HICB as u8; let __tmp_y = { let __v = (*b3.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    return (
        {
            let __go_binary_0 = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*p0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK4 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = 18;
            let __go_binary_2 = __go_binary_0 << __go_binary_1;
            let __go_binary_3 = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
            let __go_binary_4 = 12;
            let __go_binary_5 = __go_binary_3 << __go_binary_4;
            let __go_binary_6 = __go_binary_2 | __go_binary_5;
            let __go_binary_7 = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
            let __go_binary_8 = 6;
            let __go_binary_9 = __go_binary_7 << __go_binary_8;
            let __go_binary_10 = __go_binary_6 | __go_binary_9;
            let __go_binary_11 = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b3.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
            let __go_binary_12 = __go_binary_10 | __go_binary_11;
            __go_binary_12
        },
        4
    );
}

/// AppendRune appends the UTF-8 encoding of r to the end of p and
/// returns the extended buffer. If the rune is out of range,
/// it appends the encoding of [RuneError].
pub fn append_rune(p: Arc<Mutex<Option<Vec<u8>>>>, r: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // This function is inlineable for fast handling of ASCII.
    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = RUNE1_MAX as u32; __tmp_x <= __tmp_y } {
        return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() };
    }
    append_rune_non_a_s_c_i_i(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn append_rune_non_a_s_c_i_i(p: Arc<Mutex<Option<Vec<u8>>>>, r: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Negative values are erroneous. Making it unsigned addresses the problem.
    let mut i = Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32)));
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE2_MAX as u32; __tmp_x <= __tmp_y } {
            return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![
                { let __tmp_x = T2 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y },
                { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y },
            ]); __append_target.clone() };
        } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SURROGATE_MIN as u32; __tmp_x < __tmp_y } || { let __tmp_x = SURROGATE_MAX as u32; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE3_MAX as u32; __tmp_x <= __tmp_y } {
            return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![
                { let __tmp_x = T3 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y },
                { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y },
                { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y },
            ]); __append_target.clone() };
        } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE3_MAX as u32; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_RUNE as u32; __tmp_x <= __tmp_y } {
            return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![
                { let __tmp_x = T4 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 18; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y },
                { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y },
                { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y },
                { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y },
            ]); __append_target.clone() };
        } else {
            return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![RUNE_ERROR_BYTE0 as u8, RUNE_ERROR_BYTE1 as u8, RUNE_ERROR_BYTE2 as u8]); __append_target.clone() };
        }
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for acceptRange {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
