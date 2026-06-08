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


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for acceptRange {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
