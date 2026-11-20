use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

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
    pub lo: Rc<RefCell<Option<u8>>>,
    pub hi: Rc<RefCell<Option<u8>>>,
}

impl acceptRange {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lo.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.hi.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            lo: __go_clone_0_0,
            hi: __go_clone_1_0,
        }
    }
}


impl Default for acceptRange {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            lo: __go_default_0_0,
            hi: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for acceptRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lo.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.hi.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static first: GoGlobal<[u8; 256]> = GoGlobal::new();

pub(crate) static acceptRanges: GoGlobal<[acceptRange; 16]> = GoGlobal::new();


fn __go_init_globals() {
    *first.borrow_mut() = Some(std::array::from_fn::<_, 256, _>(|_| 0));
    *acceptRanges.borrow_mut() = Some(std::array::from_fn::<_, 16, _>(|_| Default::default()));
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
        *first.borrow_mut() = Some(__go_array);
    }
    {
        let mut __go_array = Vec::<acceptRange>::with_capacity(16);
        __go_array.push(acceptRange { lo: Rc::new(RefCell::new(Some(LOCB as u8))), hi: Rc::new(RefCell::new(Some(HICB as u8))), ..Default::default() });
        __go_array.push(acceptRange { lo: Rc::new(RefCell::new(Some(0xA0 as u8))), hi: Rc::new(RefCell::new(Some(HICB as u8))), ..Default::default() });
        __go_array.push(acceptRange { lo: Rc::new(RefCell::new(Some(LOCB as u8))), hi: Rc::new(RefCell::new(Some(0x9F as u8))), ..Default::default() });
        __go_array.push(acceptRange { lo: Rc::new(RefCell::new(Some(0x90 as u8))), hi: Rc::new(RefCell::new(Some(HICB as u8))), ..Default::default() });
        __go_array.push(acceptRange { lo: Rc::new(RefCell::new(Some(LOCB as u8))), hi: Rc::new(RefCell::new(Some(0x8F as u8))), ..Default::default() });
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
        *acceptRanges.borrow_mut() = Some(__go_array);
    }
}


/// RuneCountInString is like [RuneCount] but its input is a string.
pub fn rune_count_in_string(s: Rc<RefCell<Option<String>>>) -> i32 {
    let mut n: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));

    for _ in (*s.borrow().as_ref().unwrap()).chars() {
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (*n.borrow().as_ref().unwrap());
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for acceptRange {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
