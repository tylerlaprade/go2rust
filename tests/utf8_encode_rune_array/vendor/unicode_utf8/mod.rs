use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub const RUNE_ERROR: i32 = ('\u{fffd}' as i32);
pub const RUNE_SELF: i64 = 0x80;
pub const MAX_RUNE: i32 = ('\u{10ffff}' as i32);
pub const U_T_F_MAX: i64 = 4;


pub(crate) const SURROGATE_MIN: i64 = 0xD800;
pub(crate) const SURROGATE_MAX: i64 = 0xDFFF;


pub(crate) const T1: i64 = 0b00000000;
pub(crate) const TX: i64 = 0b10000000;
pub(crate) const T2: i64 = 0b11000000;
pub(crate) const T3: i64 = 0b11100000;
pub(crate) const T4: i64 = 0b11110000;
pub(crate) const T5: i64 = 0b11111000;
pub(crate) const MASKX: i64 = 0b00111111;
pub(crate) const MASK2: i64 = 0b00011111;
pub(crate) const MASK3: i64 = 0b00001111;
pub(crate) const MASK4: i64 = 0b00000111;
pub(crate) const RUNE1_MAX: i64 = (((1 as i64) << (7 as i64)) - (1 as i64));
pub(crate) const RUNE2_MAX: i64 = (((1 as i64) << (11 as i64)) - (1 as i64));
pub(crate) const RUNE3_MAX: i64 = (((1 as i64) << (16 as i64)) - (1 as i64));
pub(crate) const LOCB: i64 = 0b10000000;
pub(crate) const HICB: i64 = 0b10111111;
pub(crate) const XX: i64 = 0xF1;
pub(crate) const AS: i64 = 0xF0;
pub(crate) const S1: i64 = 0x02;
pub(crate) const S2: i64 = 0x13;
pub(crate) const S3: i64 = 0x03;
pub(crate) const S4: i64 = 0x23;
pub(crate) const S5: i64 = 0x34;
pub(crate) const S6: i64 = 0x04;
pub(crate) const S7: i64 = 0x44;


pub(crate) const RUNE_ERROR_BYTE0: i32 = ((T3 as i32) | ((RUNE_ERROR as i32) >> (12 as i32)));
pub(crate) const RUNE_ERROR_BYTE1: i32 = ((TX as i32) | (((RUNE_ERROR as i32) >> (6 as i32)) & (MASKX as i32)));
pub(crate) const RUNE_ERROR_BYTE2: i32 = ((TX as i32) | ((RUNE_ERROR as i32) & (MASKX as i32)));


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
        let mut __go_array = std::vec::Vec::<u8>::with_capacity(256);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
        __go_array.push(AS as u8);
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
        let mut __go_array = std::vec::Vec::<acceptRange>::with_capacity(16);
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


/// EncodeRune writes into p (which must be large enough) the UTF-8 encoding of the rune.
/// If the rune is out of range, it writes the encoding of [RuneError].
/// It returns the number of bytes written.
pub fn encode_rune(p: Rc<RefCell<Option<Vec<u8>>>>, r: Rc<RefCell<Option<i32>>>) -> i64 {
        // This function is inlineable for fast handling of ASCII.
    if ((*Rc::new(RefCell::new(Some((*r.borrow().as_ref().unwrap()) as u32))).borrow().as_ref().unwrap())) <= RUNE1_MAX as u32 {
        { let __idx = (0) as usize; let __rhs = (*Rc::new(RefCell::new(Some((*r.borrow().as_ref().unwrap()) as u8))).borrow().as_ref().unwrap()).clone(); p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
        return 1;
    }
    encode_rune_non_a_s_c_i_i(p.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn encode_rune_non_a_s_c_i_i(p: Rc<RefCell<Option<Vec<u8>>>>, r: Rc<RefCell<Option<i32>>>) -> i64 {
        // Negative values are erroneous. Making it unsigned addresses the problem.
    let mut i = Rc::new(RefCell::new(Some((*r.borrow().as_ref().unwrap()) as u32)));
    if (*i.borrow().as_ref().unwrap()) <= RUNE2_MAX as u32 {
            let _ = p.borrow().as_ref().unwrap().get_cloned((1) as usize);
            { let __idx = (0) as usize; let __rhs = T2 as u8 | (*Rc::new(RefCell::new(Some((((*r.borrow().as_ref().unwrap()) >> 6)) as u8))).borrow().as_ref().unwrap()); p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            { let __idx = (1) as usize; let __rhs = TX as u8 | (*Rc::new(RefCell::new(Some((*r.borrow().as_ref().unwrap()) as u8))).borrow().as_ref().unwrap()) & MASKX as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            return 2;
        } else if (*i.borrow().as_ref().unwrap()) < SURROGATE_MIN as u32 || (SURROGATE_MAX as u32) < (*i.borrow().as_ref().unwrap()) && (*i.borrow().as_ref().unwrap()) <= RUNE3_MAX as u32 {
            let _ = p.borrow().as_ref().unwrap().get_cloned((2) as usize);
            { let __idx = (0) as usize; let __rhs = T3 as u8 | (*Rc::new(RefCell::new(Some((((*r.borrow().as_ref().unwrap()) >> 12)) as u8))).borrow().as_ref().unwrap()); p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            { let __idx = (1) as usize; let __rhs = TX as u8 | (*Rc::new(RefCell::new(Some((((*r.borrow().as_ref().unwrap()) >> 6)) as u8))).borrow().as_ref().unwrap()) & MASKX as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            { let __idx = (2) as usize; let __rhs = TX as u8 | (*Rc::new(RefCell::new(Some((*r.borrow().as_ref().unwrap()) as u8))).borrow().as_ref().unwrap()) & MASKX as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            return 3;
        } else if (*i.borrow().as_ref().unwrap()) > RUNE3_MAX as u32 && (*i.borrow().as_ref().unwrap()) <= MAX_RUNE as u32 {
            let _ = p.borrow().as_ref().unwrap().get_cloned((3) as usize);
            { let __idx = (0) as usize; let __rhs = T4 as u8 | (*Rc::new(RefCell::new(Some((((*r.borrow().as_ref().unwrap()) >> 18)) as u8))).borrow().as_ref().unwrap()); p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            { let __idx = (1) as usize; let __rhs = TX as u8 | (*Rc::new(RefCell::new(Some((((*r.borrow().as_ref().unwrap()) >> 12)) as u8))).borrow().as_ref().unwrap()) & MASKX as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            { let __idx = (2) as usize; let __rhs = TX as u8 | (*Rc::new(RefCell::new(Some((((*r.borrow().as_ref().unwrap()) >> 6)) as u8))).borrow().as_ref().unwrap()) & MASKX as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            { let __idx = (3) as usize; let __rhs = TX as u8 | (*Rc::new(RefCell::new(Some((*r.borrow().as_ref().unwrap()) as u8))).borrow().as_ref().unwrap()) & MASKX as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            return 4;
        } else {
            let _ = p.borrow().as_ref().unwrap().get_cloned((2) as usize);
            { let __idx = (0) as usize; let __rhs = RUNE_ERROR_BYTE0 as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            { let __idx = (1) as usize; let __rhs = RUNE_ERROR_BYTE1 as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            { let __idx = (2) as usize; let __rhs = RUNE_ERROR_BYTE2 as u8; p.borrow_mut().as_mut().unwrap().set(__idx, __rhs); };
            return 3;
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
