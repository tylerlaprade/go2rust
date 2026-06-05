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
pub(crate) const RUNE1_MAX: i32 = (1 << 7) - 1;
pub(crate) const RUNE2_MAX: i32 = (1 << 11) - 1;
pub(crate) const RUNE3_MAX: i32 = (1 << 16) - 1;
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


pub(crate) const RUNE_ERROR_BYTE0: i32 = T3 | (RUNE_ERROR >> 12);
pub(crate) const RUNE_ERROR_BYTE1: i32 = TX | (RUNE_ERROR >> 6) & MASKX;
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
        Self { lo: { let __guard = self.lo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hi: { let __guard = self.hi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for acceptRange {
    fn default() -> Self {
        Self { lo: Arc::new(Mutex::new(Some(0))), hi: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for acceptRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lo.lock().unwrap().as_ref().unwrap()), (*self.hi.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for acceptRange {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static first: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 256]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static acceptRanges: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[acceptRange; 16]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *first.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *acceptRanges.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *first.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, AS as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S1 as u8, S2 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S3 as u8, S4 as u8, S3 as u8, S3 as u8, S5 as u8, S6 as u8, S6 as u8, S6 as u8, S7 as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8, XX as u8]))).lock().unwrap().as_ref().unwrap()).clone());
    *acceptRanges.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([acceptRange { lo: Arc::new(Mutex::new(Some(LOCB as u8))), hi: Arc::new(Mutex::new(Some(HICB as u8))), ..Default::default() }, acceptRange { lo: Arc::new(Mutex::new(Some(0xA0 as u8))), hi: Arc::new(Mutex::new(Some(HICB as u8))), ..Default::default() }, acceptRange { lo: Arc::new(Mutex::new(Some(LOCB as u8))), hi: Arc::new(Mutex::new(Some(0x9F as u8))), ..Default::default() }, acceptRange { lo: Arc::new(Mutex::new(Some(0x90 as u8))), hi: Arc::new(Mutex::new(Some(HICB as u8))), ..Default::default() }, acceptRange { lo: Arc::new(Mutex::new(Some(LOCB as u8))), hi: Arc::new(Mutex::new(Some(0x8F as u8))), ..Default::default() }, Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default(), Default::default()]))).lock().unwrap().as_ref().unwrap()).clone());
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
        return ({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & ! __tmp_y }; let __tmp_y = { let __tmp_x = RUNE_ERROR as i32; let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }, 1);
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
        return ({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*p0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK2 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 6; __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, 2);
    }
    let mut b2 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() })));
    if { let __tmp_x = { let __v = (*b2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = LOCB as u8; __tmp_x < __tmp_y } || { let __tmp_x = HICB as u8; let __tmp_y = { let __v = (*b2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    if { let __tmp_x = { let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        return ({ let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*p0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK3 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 12; __tmp_x << __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 6; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, 3);
    }
    let mut b3 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() })));
    if { let __tmp_x = { let __v = (*b3.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = LOCB as u8; __tmp_x < __tmp_y } || { let __tmp_x = HICB as u8; let __tmp_y = { let __v = (*b3.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    return ({ let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*p0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK4 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 18; __tmp_x << __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 12; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 6; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b3.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, 4);
}

/// DecodeRuneInString is like [DecodeRune] but its input is a string. If s is
/// empty it returns ([RuneError], 0). Otherwise, if the encoding is invalid, it
/// returns (RuneError, 1). Both are impossible results for correct, non-empty
/// UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
pub fn decode_rune_in_string(s: Arc<Mutex<Option<String>>>) -> (i32, i32) {
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut size: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    let mut n = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 0);
    }
    let mut s0 = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })));
    let mut x = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = first.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*s0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = AS as u8; __tmp_x >= __tmp_y } {
                // The following code simulates an additional check for x == xx and
                // handling the ASCII and invalid cases accordingly. This mask-and-or
                // approach prevents an additional branch.
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 31; __tmp_x << __tmp_y }; let __tmp_y = 31; __tmp_x >> __tmp_y })));
        return ({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & ! __tmp_y }; let __tmp_y = { let __tmp_x = RUNE_ERROR as i32; let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }, 1);
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
    let mut s1 = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })));
    if { let __tmp_x = { let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; let __tmp_y = (*{ let __field = (*accept.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*accept.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    if { let __tmp_x = { let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x <= __tmp_y } {
        return ({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK2 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 6; __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, 2);
    }
    let mut s2 = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(2) as usize] })));
    if { let __tmp_x = { let __v = (*s2.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; let __tmp_y = LOCB as u8; __tmp_x < __tmp_y } || { let __tmp_x = HICB as u8; let __tmp_y = { let __v = (*s2.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    if { let __tmp_x = { let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        return ({ let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK3 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 12; __tmp_x << __tmp_y }; let __tmp_y = { let __tmp_x = (({ let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8 & MASKX as u8) as i32); let __tmp_y = 6; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s2.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, 3);
    }
    let mut s3 = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(3) as usize] })));
    if { let __tmp_x = { let __v = (*s3.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; let __tmp_y = LOCB as u8; __tmp_x < __tmp_y } || { let __tmp_x = HICB as u8; let __tmp_y = { let __v = (*s3.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; __tmp_x < __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    return ({ let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MASK4 as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 18; __tmp_x << __tmp_y }; let __tmp_y = { let __tmp_x = (({ let __v = (*s1.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8 & MASKX as u8) as i32); let __tmp_y = 12; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (({ let __v = (*s2.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8 & MASKX as u8) as i32); let __tmp_y = 6; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s3.lock().unwrap().as_ref().unwrap()).clone(); __v } as u8; let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, 4);
}

/// DecodeLastRuneInString is like [DecodeLastRune] but its input is a string. If
/// s is empty it returns ([RuneError], 0). Otherwise, if the encoding is invalid,
/// it returns (RuneError, 1). Both are impossible results for correct,
/// non-empty UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
pub fn decode_last_rune_in_string(s: Arc<Mutex<Option<String>>>) -> (i32, i32) {
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut size: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    let mut end = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)));
    if { let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (RUNE_ERROR as i32, 0);
    }
    let mut start = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE_SELF as i32; __tmp_x < __tmp_y } {
        return ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }, 1);
    }

        // guard against O(n^2) behavior when traversing
        // backwards through strings with long sequences of
        // invalid UTF-8.
    let mut lim = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*lim.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = 0; *lim.lock().unwrap() = Some(new_val); };
    }
    { let mut guard = start.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    while { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*lim.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        if rune_start(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))) {
        break
    }
        { let mut guard = start.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = 0; *start.lock().unwrap() = Some(new_val); };
    }
    { let (__tmp_0, __tmp_1) = decode_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() })))); *r.lock().unwrap() = Some(__tmp_0); *size.lock().unwrap() = Some(__tmp_1); };
    if { let __tmp_x = { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return (RUNE_ERROR as i32, 1);
    }
    return ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

/// RuneLen returns the number of bytes in the UTF-8 encoding of the rune.
/// It returns -1 if the rune is not a valid value to encode in UTF-8.
pub fn rune_len(r: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
            return -(1);
        } else if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE1_MAX as i32; __tmp_x <= __tmp_y } {
            return 1;
        } else if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE2_MAX as i32; __tmp_x <= __tmp_y } {
            return 2;
        } else if { let __tmp_x = SURROGATE_MIN as i32; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SURROGATE_MAX as i32; __tmp_x <= __tmp_y } {
            return -(1);
        } else if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE3_MAX as i32; __tmp_x <= __tmp_y } {
            return 3;
        } else if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_RUNE as i32; __tmp_x <= __tmp_y } {
            return 4;
        }
    -(1)
}

/// EncodeRune writes into p (which must be large enough) the UTF-8 encoding of the rune.
/// If the rune is out of range, it writes the encoding of [RuneError].
/// It returns the number of bytes written.
pub fn encode_rune(p: Arc<Mutex<Option<Vec<u8>>>>, r: Arc<Mutex<Option<i32>>>) -> i32 {
        // This function is inlineable for fast handling of ASCII.
    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = RUNE1_MAX as u32; __tmp_x <= __tmp_y } {
        (*p.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        return 1;
    }
    encode_rune_non_a_s_c_i_i(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn encode_rune_non_a_s_c_i_i(p: Arc<Mutex<Option<Vec<u8>>>>, r: Arc<Mutex<Option<i32>>>) -> i32 {
        // Negative values are erroneous. Making it unsigned addresses the problem.
    let mut i = Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32)));
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE2_MAX as u32; __tmp_x <= __tmp_y } {
            let _ = { let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() };
            (*p.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __tmp_x = T2 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y };
            (*p.lock().unwrap().as_mut().unwrap())[(1) as usize] = { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y };
            return 2;
        } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SURROGATE_MIN as u32; __tmp_x < __tmp_y } || { let __tmp_x = SURROGATE_MAX as u32; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE3_MAX as u32; __tmp_x <= __tmp_y } {
            let _ = { let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() };
            (*p.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __tmp_x = T3 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y };
            (*p.lock().unwrap().as_mut().unwrap())[(1) as usize] = { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y };
            (*p.lock().unwrap().as_mut().unwrap())[(2) as usize] = { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y };
            return 3;
        } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE3_MAX as u32; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_RUNE as u32; __tmp_x <= __tmp_y } {
            let _ = { let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() };
            (*p.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __tmp_x = T4 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 18; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y };
            (*p.lock().unwrap().as_mut().unwrap())[(1) as usize] = { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y };
            (*p.lock().unwrap().as_mut().unwrap())[(2) as usize] = { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y };
            (*p.lock().unwrap().as_mut().unwrap())[(3) as usize] = { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y };
            return 4;
        } else {
            let _ = { let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() };
            (*p.lock().unwrap().as_mut().unwrap())[(0) as usize] = RUNE_ERROR_BYTE0 as u8;
            (*p.lock().unwrap().as_mut().unwrap())[(1) as usize] = RUNE_ERROR_BYTE1 as u8;
            (*p.lock().unwrap().as_mut().unwrap())[(2) as usize] = RUNE_ERROR_BYTE2 as u8;
            return 3;
        }
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
            return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![{ let __tmp_x = T2 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }]); __append_target.clone() };
        } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SURROGATE_MIN as u32; __tmp_x < __tmp_y } || { let __tmp_x = SURROGATE_MAX as u32; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE3_MAX as u32; __tmp_x <= __tmp_y } {
            return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![{ let __tmp_x = T3 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }, { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }]); __append_target.clone() };
        } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = RUNE3_MAX as u32; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_RUNE as u32; __tmp_x <= __tmp_y } {
            return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![{ let __tmp_x = T4 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 18; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }, { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }, { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }, { let __tmp_x = TX as u8; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MASKX as u8; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }]); __append_target.clone() };
        } else {
            return { let __append_target = p.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![RUNE_ERROR_BYTE0 as u8, RUNE_ERROR_BYTE1 as u8, RUNE_ERROR_BYTE2 as u8]); __append_target.clone() };
        }
}

/// RuneCountInString is like [RuneCount] but its input is a string.
pub fn rune_count_in_string(s: Arc<Mutex<Option<String>>>) -> i32 {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    for _ in (*s.lock().unwrap().as_ref().unwrap()).chars() {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// RuneStart reports whether the byte could be the first byte of an encoded,
/// possibly invalid rune. Second and subsequent bytes always have the top two
/// bits set to 10.
pub fn rune_start(b: Arc<Mutex<Option<u8>>>) -> bool {
    return { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xC0 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0x80 as u8; __tmp_x != __tmp_y };
}

/// ValidRune reports whether r can be legally encoded as UTF-8.
/// Code points that are out of range or a surrogate half are illegal.
pub fn valid_rune(r: Arc<Mutex<Option<i32>>>) -> bool {
    if { let __tmp_x = 0 as i32; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SURROGATE_MIN as i32; __tmp_x < __tmp_y } {
            return true;
        } else if { let __tmp_x = SURROGATE_MAX as i32; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_RUNE as i32; __tmp_x <= __tmp_y } {
            return true;
        }
    false
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for acceptRange {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
