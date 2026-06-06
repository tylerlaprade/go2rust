use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone, go_strconv_format_float, go_strconv_format_int};

use crate::accuracy_string::*;
use crate::arith::*;
use crate::arith_decl::*;
use crate::decimal::*;
use crate::float::*;
use crate::floatconv::*;
use crate::floatmarsh::*;
use crate::ftoa::*;
use crate::int::*;
use crate::intconv::*;
use crate::intmarsh::*;
use crate::nat::*;
use crate::natdiv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DIGITS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";


pub const MAX_BASE: i32 = 10 + (('z' as i32) - ('a' as i32) + 1) + (('Z' as i32) - ('A' as i32) + 1);


pub(crate) const MAX_BASE_SMALL: i32 = 10 + (('z' as i32) - ('a' as i32) + 1);


#[derive(Debug, Clone)]
pub struct divisor {
    pub bbb: Arc<Mutex<Option<nat>>>,
    pub nbits: Arc<Mutex<Option<i32>>>,
    pub ndigits: Arc<Mutex<Option<i32>>>,
}

impl divisor {
    pub fn __go_value_clone(&self) -> Self {
        Self { bbb: self.bbb.clone(), nbits: { let __guard = self.nbits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ndigits: { let __guard = self.ndigits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for divisor {
    fn default() -> Self {
        Self { bbb: Arc::new(Mutex::new(Some(Default::default()))), nbits: Arc::new(Mutex::new(Some(0))), ndigits: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for divisor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.bbb.lock().unwrap().as_ref().unwrap()), (*self.nbits.lock().unwrap().as_ref().unwrap()), (*self.ndigits.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for divisor {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static errNoDigits: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errInvalSep: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static leafSize: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cacheBase10: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct1>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errNoDigits.lock().unwrap() = None;
    *errInvalSep.lock().unwrap() = None;
    *leafSize.lock().unwrap() = Some(0);
    *cacheBase10.lock().unwrap() = Some(Default::default());
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("number has no digits".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errNoDigits.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("'_' must separate successive digits".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalSep.lock().unwrap() = new_val; }
    *leafSize.lock().unwrap() = Some(8);
}


pub(crate) fn __go_zero_globals() {
    *errNoDigits.lock().unwrap() = None;
    *errInvalSep.lock().unwrap() = None;
    *leafSize.lock().unwrap() = Some(0);
    *cacheBase10.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_14() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("number has no digits".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errNoDigits.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_15() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("'_' must separate successive digits".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalSep.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_16() {
    *leafSize.lock().unwrap() = Some(8);
}


impl crate::nat::nat {
    /// scan scans the number corresponding to the longest possible prefix
    /// from r representing an unsigned number in a given conversion base.
    /// scan returns the corresponding natural number res, the actual base b,
    /// a digit count, and a read or syntax error err, if any.
    ///
    /// For base 0, an underscore character “_” may appear between a base
    /// prefix and an adjacent digit, and between successive digits; such
    /// underscores do not change the value of the number, or the returned
    /// digit count. Incorrect placement of underscores is reported as an
    /// error if there are no other errors. If base != 0, underscores are
    /// not recognized and thus terminate scanning like any other character
    /// that is not a valid radix point or digit.
    ///
    ///	number    = mantissa | prefix pmantissa .
    ///	prefix    = "0" [ "b" | "B" | "o" | "O" | "x" | "X" ] .
    ///	mantissa  = digits "." [ digits ] | digits | "." digits .
    ///	pmantissa = [ "_" ] digits "." [ digits ] | [ "_" ] digits | "." digits .
    ///	digits    = digit { [ "_" ] digit } .
    ///	digit     = "0" ... "9" | "a" ... "z" | "A" ... "Z" .
    ///
    /// Unless fracOk is set, the base argument must be 0 or a value between
    /// 2 and MaxBase. If fracOk is set, the base argument must be one of
    /// 0, 2, 8, 10, or 16. Providing an invalid base argument leads to a run-
    /// time panic.
    ///
    /// For base 0, the number prefix determines the actual base: A prefix of
    /// “0b” or “0B” selects base 2, “0o” or “0O” selects base 8, and
    /// “0x” or “0X” selects base 16. If fracOk is false, a “0” prefix
    /// (immediately followed by digits) selects base 8 as well. Otherwise,
    /// the selected base is 10 and no prefix is accepted.
    ///
    /// If fracOk is set, a period followed by a fractional part is permitted.
    /// The result value is computed as if there were no period present; and
    /// the count value is used to determine the fractional part.
    ///
    /// For bases <= 36, lower and upper case letters are considered the same:
    /// The letters 'a' to 'z' and 'A' to 'Z' represent digit values 10 to 35.
    /// For bases > 36, the upper case letters 'A' to 'Z' represent the digit
    /// values 36 to 61.
    ///
    /// A result digit count > 0 corresponds to the number of (non-prefix) digits
    /// parsed. A digit count <= 0 indicates the presence of a period (if fracOk
    /// is set, only), and -count is the number of fractional digits found.
    /// In this case, the actual value of the scanned number is res * b**count.
    pub fn scan(&self, r: Arc<Mutex<Option<io_ByteScanner>>>, base: Arc<Mutex<Option<i32>>>, mut fracOk: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<crate::nat::nat>>>, i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut res: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut b: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut count: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut __self = self.clone();
                // reject invalid bases
        let mut baseOk = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || !{ let __v = (*fracOk.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = 2; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 62; __tmp_x <= __tmp_y } || { let __v = (*fracOk.lock().unwrap().as_ref().unwrap()).clone(); __v } && ({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x == __tmp_y }))));
        if !{ let __v = (*baseOk.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid number base {}", { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
                // prev encodes the previously seen char: it is one
                // of '_', '0' (a digit), or '.' (anything else). A
                // valid separator '_' may only occur after a digit
                // and if base == 0.
        let mut prev = Arc::new(Mutex::new(Some(('.' as i32))));
        let mut invalSep = Arc::new(Mutex::new(Some(false)));
                // one char look-ahead
        let (mut ch, __tmp_1) = (*r.lock().unwrap().as_ref().unwrap()).read_byte(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
                // determine actual base
        let (__tmp_0, mut prefix) = ({ let __owned = base.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some(0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *b.lock().unwrap() = __moved_tmp_0;;
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // actual base is 10 unless there's a base prefix
        { let new_val = 10; *b.lock().unwrap() = Some(new_val); };
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = ch; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = ('0' as i32); *prev.lock().unwrap() = Some(new_val); };
        { let new_val = 1; *count.lock().unwrap() = Some(new_val); };
        { let (__tmp_0, __tmp_1) = (*r.lock().unwrap().as_ref().unwrap()).read_byte(); ch = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
                // possibly one of 0b, 0B, 0o, 0O, 0x, 0X
        { let _switch_val = ch;
    if _switch_val == (('b' as i32) as u8) || _switch_val == (('B' as i32) as u8) {
            { let __tmp_0 = 2; let __tmp_1 = ('b' as i32); *b.lock().unwrap() = Some(__tmp_0); *prefix.lock().unwrap() = Some(__tmp_1); };
        } else if _switch_val == (('o' as i32) as u8) || _switch_val == (('O' as i32) as u8) {
            { let __tmp_0 = 8; let __tmp_1 = ('o' as i32); *b.lock().unwrap() = Some(__tmp_0); *prefix.lock().unwrap() = Some(__tmp_1); };
        } else if _switch_val == (('x' as i32) as u8) || _switch_val == (('X' as i32) as u8) {
            { let __tmp_0 = 16; let __tmp_1 = ('x' as i32); *b.lock().unwrap() = Some(__tmp_0); *prefix.lock().unwrap() = Some(__tmp_1); };
        } else {
            if !{ let __v = (*fracOk.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __tmp_0 = 8; let __tmp_1 = ('0' as i32); *b.lock().unwrap() = Some(__tmp_0); *prefix.lock().unwrap() = Some(__tmp_1); };
    }
        }
    }
        if { let __tmp_x = { let __v = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let new_val = 0; *count.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = (*r.lock().unwrap().as_ref().unwrap()).read_byte(); ch = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }
    }
    }
    }
    }
                // actual base is 10 unless there's a base prefix
                // possibly one of 0b, 0B, 0o, 0O, 0x, 0X
                // prefix is not counted
                // convert string
                // Algorithm: Collect digits in groups of at most n digits in di
                // and then use mulAddWW for every such group to add them to the
                // result.
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..(0) as usize].to_vec() })))); __self = new_val; };
        let mut b1 = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()) as u64)))))));
        let (mut bn, mut n) = max_pow(Arc::new(Mutex::new(Some({ let __arg_holder = b1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut di = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))))));
        let mut i = Arc::new(Mutex::new(Some(0)));
        let mut dp = Arc::new(Mutex::new(Some(-(1))));
        while { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        if { let __tmp_x = ch; let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } && { let __v = (*fracOk.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = false; *fracOk.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*prev.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        { let new_val = true; *invalSep.lock().unwrap() = Some(new_val); };
    }
        { let new_val = ('.' as i32); *prev.lock().unwrap() = Some(new_val); };
        { let new_val = count.lock().unwrap().as_ref().unwrap().clone(); *dp.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = ch; let __tmp_y = ('_' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*prev.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x != __tmp_y } {
        { let new_val = true; *invalSep.lock().unwrap() = Some(new_val); };
    }
        { let new_val = ('_' as i32); *prev.lock().unwrap() = Some(new_val); };
    } else {
        let mut d1: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
        if { let __tmp_x = ('0' as i32) as u8; let __tmp_y = ch; __tmp_x <= __tmp_y } && { let __tmp_x = ch; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } {
            { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some({ let __tmp_x = ch; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y } as u64)))); *d1.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = ('a' as i32) as u8; let __tmp_y = ch; __tmp_x <= __tmp_y } && { let __tmp_x = ch; let __tmp_y = ('z' as i32) as u8; __tmp_x <= __tmp_y } {
            { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ch; let __tmp_y = ('a' as i32) as u8; __tmp_x - __tmp_y }; let __tmp_y = 10 as u8; __tmp_x + __tmp_y } as u64)))); *d1.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = ('A' as i32) as u8; let __tmp_y = ch; __tmp_x <= __tmp_y } && { let __tmp_x = ch; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y } {
            if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 36; __tmp_x <= __tmp_y } {
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ch; let __tmp_y = ('A' as i32) as u8; __tmp_x - __tmp_y }; let __tmp_y = 10 as u8; __tmp_x + __tmp_y } as u64)))); *d1.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ch; let __tmp_y = ('A' as i32) as u8; __tmp_x - __tmp_y }; let __tmp_y = MAX_BASE_SMALL as u8; __tmp_x + __tmp_y } as u64)))); *d1.lock().unwrap() = Some(new_val); };
    }
        } else {
            { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((MAX_BASE + 1) as u64)))); *d1.lock().unwrap() = Some(new_val); };
        }
        if { let __tmp_x = (*d1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x >= __tmp_y } {
        (*r.lock().unwrap().as_ref().unwrap()).unread_byte();
        break
    }
        { let new_val = ('0' as i32); *prev.lock().unwrap() = Some(new_val); };
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((((*{ let __v = (*di.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) * (*{ let __v = (*b1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())) + (*{ let __v = (*d1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))))); *di.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = n; __tmp_x == __tmp_y } {
        { let new_val = __self.mul_add_w_w(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = bn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = di.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); *di.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    }
    }

                // convert rune into digit value d1
                // ch does not belong to number anymore
                // collect d1 in di
                // if di is "full", add it to the result
        { let (__tmp_0, __tmp_1) = (*r.lock().unwrap().as_ref().unwrap()).read_byte(); ch = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }
                // convert rune into digit value d1
                // ch does not belong to number anymore
                // collect d1 in di
                // if di is "full", add it to the result
        if { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        *err.lock().unwrap() = None;
    }
                // other errors take precedence over invalid separators
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && ({ let __v = (*invalSep.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __v = (*prev.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y }) {
        { let __rhs_holder = errInvalSep.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        if { let __tmp_x = { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // no digits found
        if { let __tmp_x = { let __v = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x == __tmp_y } {
                // there was only the octal prefix 0 (possibly followed by separators and digits > 7);
                // interpret as decimal 0
        return (Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..(0) as usize].to_vec() }))))))), 10, 1, err.clone());
    }
                // there was only the octal prefix 0 (possibly followed by separators and digits > 7);
                // interpret as decimal 0
        { let __rhs_holder = errNoDigits.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
                // no digits found
                // there was only the octal prefix 0 (possibly followed by separators and digits > 7);
                // interpret as decimal 0
                // fall through; result will be 0
                // add remaining digits to result
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = __self.mul_add_w_w(Arc::new(Mutex::new(Some(__self.clone()))), pow(Arc::new(Mutex::new(Some({ let __arg_holder = b1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(Some({ let __arg_holder = di.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        { let new_val = __self.norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
                // adjust count for fraction, if any
        if { let __tmp_x = { let __v = (*dp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
                // 0 <= dp <= count
        { let new_val = { let __tmp_x = { let __v = (*dp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *count.lock().unwrap() = Some(new_val); };
    }
                // 0 <= dp <= count
        return (res.clone(), (*b.lock().unwrap().as_ref().unwrap()), (*count.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// utoa converts x to an ASCII representation in the given base;
    /// base must be between 2 and MaxBase, inclusive.
    pub fn utoa(&self, base: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        self.itoa(Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// itoa is like utoa but it prepends a '-' if neg && x != 0.
    pub fn itoa(&self, neg: Arc<Mutex<Option<bool>>>, base: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 62; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("invalid base".to_string()) as Box<dyn Any + Send + Sync>);
    }
                // x == 0
        if { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(("0".to_string()).as_bytes().to_vec())));
    }
                // len(x) > 0
                // allocate buffer for conversion
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(self.bit_len() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = math::log2(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as f64)))); __tmp_x / __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        let mut s = Arc::new(Mutex::new(Some(vec![0; ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
                // convert power of two and non power of two bases separately
        {
        let mut b = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)))))));;
        if { let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ((*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())).wrapping_neg()))))); __tmp_x == __tmp_y } {
            let mut shift = Arc::new(Mutex::new(Some(math_bits::trailing_zeros(Arc::new(Mutex::new(Some((*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))) as u64)));;
            let mut mask = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((1 << { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }) - 1) as u64)))))));;
            let mut w = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));;
            let mut nbits = Arc::new(Mutex::new(Some(__W as u64)));;
            let mut k = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x < __tmp_y } {
        while { let __tmp_x = { let __v = (*nbits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(DIGITS); __s.as_bytes()[((*{ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & (*{ let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())) as usize] };
        { let __rhs = (*shift.lock().unwrap().as_ref().unwrap()); let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() >> __rhs); };
        { let __rhs = (*shift.lock().unwrap().as_ref().unwrap()); let mut guard = nbits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }

        if { let __tmp_x = { let __v = (*nbits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))); *w.lock().unwrap() = Some(new_val); };
        { let new_val = __W as u64; *nbits.lock().unwrap() = Some(new_val); };
    } else {
        { let __rhs = (*({ let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = { let __v = (*nbits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(DIGITS); __s.as_bytes()[((*{ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & (*{ let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())) as usize] };
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nbits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })))))); *w.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = __W as u64; let __tmp_y = ({ let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nbits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; *nbits.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
            while { let __tmp_x = (*w.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(DIGITS); __s.as_bytes()[((*{ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & (*{ let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())) as usize] };
        { let __rhs = (*shift.lock().unwrap().as_ref().unwrap()); let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() >> __rhs); };
    };
        } else {
            let (mut bb, mut ndigits) = max_pow(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            let mut table = divisors(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(ndigits))), Arc::new(Mutex::new(Some({ let __arg_holder = bb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            let mut q = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set(Arc::new(Mutex::new(Some(self.clone()))));;
            (*q.lock().unwrap().as_ref().unwrap()).convert_words(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(ndigits))), Arc::new(Mutex::new(Some({ let __arg_holder = bb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), table.clone());;
            { let new_val = 0; *i.lock().unwrap() = Some(new_val); };;
            while { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
        }
    }
                // shift is base b digit size in bits
                // shift > 0 because b >= 2
                // current word
                // number of unprocessed bits in w
                // convert less-significant words (include leading zeros)
                // convert full digits
                // convert any partial leading digit and advance to next word
                // no partial digit remaining, just advance
                // partial digit in current word w (== x[k-1]) and next word x[k]
                // advance
                // convert digits of most-significant word w (omit leading zeros)
                // construct table of successive squares of bb*leafSize to use in subdivisions
                // result (table != nil) <=> (len(x) > leafSize > 0)
                // preserve x, create local copy for use by convertWords
                // convert q to string s in base b
                // strip leading zeros
                // (x != 0; thus s must contain at least one non-zero digit
                // and the loop will terminate)
        if { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('-' as i32) as u8;
    }
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })));
    }

    /// Convert words of q to base b digits in s. If q is large, it is recursively "split in half"
    /// by nat/nat division using tabulated divisors. Otherwise, it is converted iteratively using
    /// repeated nat/Word division.
    ///
    /// The iterative method processes n Words by n divW() calls, each of which visits every Word in the
    /// incrementally shortened q for a total of n + (n-1) + (n-2) ... + 2 + 1, or n(n+1)/2 divW()'s.
    /// Recursive conversion divides q by its approximate square root, yielding two parts, each half
    /// the size of q. Using the iterative method on both halves means 2 * (n/2)(n/2 + 1)/2 divW()'s
    /// plus the expensive long div(). Asymptotically, the ratio is favorable at 1/2 the divW()'s, and
    /// is made better by splitting the subblocks recursively. Best is to split blocks until one more
    /// split would take longer (because of the nat/nat div()) than the twice as many divW()'s of the
    /// iterative approach. This threshold is represented by leafSize. Benchmarking of leafSize in the
    /// range 2..64 shows that values of 8 and 16 work well, with a 4x speedup at medium lengths and
    /// ~30x for 20000 digits. Use nat_test.go's BenchmarkLeafSize tests to optimize leafSize for
    /// specific hardware.
    pub fn convert_words(&self, mut s: Arc<Mutex<Option<Vec<u8>>>>, b: Arc<Mutex<Option<Word>>>, ndigits: Arc<Mutex<Option<i32>>>, bb: Arc<Mutex<Option<Word>>>, table: Arc<Mutex<Option<Vec<divisor>>>>) {
        let mut __self = self.clone();
                // split larger blocks recursively
        if { let __nil_result = (*table.lock().unwrap()).is_some(); __nil_result } {
                // len(q) > leafSize > 0
        let mut r: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut index = Arc::new(Mutex::new(Some({ let __tmp_x = ((*table.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        while { let __tmp_x = ({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ((*leafSize.lock().unwrap().as_ref().unwrap()) as i32); __tmp_x > __tmp_y } {
                // find divisor close to sqrt(q) if possible, but in any case < q
        let mut maxLength = __self.bit_len();
        let mut minLength = Arc::new(Mutex::new(Some({ let __tmp_x = maxLength; let __tmp_y = 1; __tmp_x >> __tmp_y })));
        while { let __tmp_x = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = (*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.nbits.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*minLength.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let mut guard = index.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // desired
        if { let __tmp_x = (*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.nbits.lock().unwrap().as_ref().unwrap()); let __tmp_y = maxLength; __tmp_x >= __tmp_y } && { let __tmp_x = (*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.bbb.lock().unwrap().as_ref().unwrap()).cmp(Arc::new(Mutex::new(Some(__self.clone())))); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let mut guard = index.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("internal inconsistency".to_string()) as Box<dyn Any + Send + Sync>);
    }
    }

                // split q into the two digit number (q'*bbb + r) to form independent subblocks
        { let (__tmp_0, __tmp_1) = __self.div(r.clone(), Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.bbb.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); { let __moved_val = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };

                // convert subblocks and collect results in s[:h] and s[h:]
        let mut h = Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.ndigits.lock().unwrap().as_ref().unwrap()) as i32); __tmp_x - __tmp_y })));
        (*r.lock().unwrap().as_ref().unwrap()).convert_words(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ndigits.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize..({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); s = new_val; };
    }
    }
                // len(q) > leafSize > 0
                // find divisor close to sqrt(q) if possible, but in any case < q
                // ~= log2 q, or at of least largest possible q of this bit length
                // ~= log2 sqrt(q)
                // desired
                // split q into the two digit number (q'*bbb + r) to form independent subblocks
                // convert subblocks and collect results in s[:h] and s[h:]
                // == q.convertWords(s, b, ndigits, bb, table[0:index+1])
                // having split any large blocks now process the remaining (small) block iteratively
        let mut i = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        let mut r: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
        if { let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(10 as u64)))); __tmp_x == __tmp_y } {
                // hard-coding for 10 here speeds this up by 1.25x (allows for / and % by constants)
        while { let __tmp_x = ({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // extract least significant, base bb "digit"
        { let (__tmp_0, __tmp_1) = __self.div_w(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = bb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); { let __moved_val = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ndigits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }

                // avoid % computation since r%10 == r - int(r/10)*10;
                // this appears to be faster for BenchmarkString10000Base10
                // and smaller strings (but a bit slower for larger ones)
        let mut t = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) / 10))))))));
        (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __tmp_x = ('0' as i32) as u8; let __tmp_y = (*Arc::new(Mutex::new(Some((((*{ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) - ((*{ let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) * 10))) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
        { let new_val = t.lock().unwrap().as_ref().unwrap().clone(); *r.lock().unwrap() = Some(new_val); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    } else {
        while { let __tmp_x = ({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // extract least significant, base bb "digit"
        { let (__tmp_0, __tmp_1) = __self.div_w(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = bb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); { let __moved_val = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ndigits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __s = &(DIGITS); __s.as_bytes()[((*{ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) % (*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())) as usize] };
        { let __rhs = (*({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() / __rhs); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    }
                // hard-coding for 10 here speeds this up by 1.25x (allows for / and % by constants)
                // extract least significant, base bb "digit"
                // avoid % computation since r%10 == r - int(r/10)*10;
                // this appears to be faster for BenchmarkString10000Base10
                // and smaller strings (but a bit slower for larger ones)
                // extract least significant, base bb "digit"
                // prepend high-order zeros
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('0' as i32) as u8;
    }
    }

    /// expWW computes x**y
    pub fn exp_w_w(&self, x: Arc<Mutex<Option<Word>>>, y: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<crate::nat::nat>>> {
        self.exp_n_n(crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))))
    }
}

/// maxPow returns (b**n, n) such that b**n is the largest power b**n <= _M.
/// For instance maxPow(10) == (1e19, 19) for 19 decimal digits in a 64bit Word.
/// In other words, at most n digits in base b fit into a Word.
/// TODO(gri) replace this with a table, generated at build time.
pub fn max_pow(b: Arc<Mutex<Option<Word>>>) -> (Arc<Mutex<Option<crate::arith::Word>>>, i32) {
    let mut p: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    { let __tmp_0 = (*b.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = 1; *p.lock().unwrap() = Some(__tmp_0); *n.lock().unwrap() = Some(__tmp_1); };
    let mut max = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((__M as u64 / (*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))))))));
    while { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*max.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } {
                // p == b**n && p <= max
        { let __rhs = (*({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = p.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() * __rhs); };
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // p == b**n && p <= max
        // p == b**n && p <= _M
    return (p.clone(), (*n.lock().unwrap().as_ref().unwrap()));
}

/// pow returns x**n for n > 0, and 1 otherwise.
pub fn pow(mut x: Arc<Mutex<Option<Word>>>, mut n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    let mut p: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));

        // n == sum of bi * 2**i, for 0 <= i < imax, and bi is 0 or 1
        // thus x**n == product of x**(2**i) for all i where bi == 1
        // (Russian Peasant Method for exponentiation)
    { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); *p.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let __rhs = (*({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = p.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() * __rhs); };
    }
        { let __rhs = (*({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() * __rhs); };
        { let __rhs = 1; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
    p.clone()
}

/// construct table of powers of bb*leafSize to use in subdivisions.
pub fn divisors(m: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<Word>>>, ndigits: Arc<Mutex<Option<i32>>>, bb: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<Vec<divisor>>>> {
        // only compute table when recursive conversion is enabled and x is large
    if { let __tmp_x = (*leafSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*leafSize.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(None));
    }

        // determine k where (bb**leafSize)**(2**k) >= sqrt(x)
    let mut k = Arc::new(Mutex::new(Some(1)));
    let mut words = { let __owned = leafSize.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*words.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; __tmp_x < __tmp_y } && { let __tmp_x = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 64; __tmp_x < __tmp_y } {
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = 1; let mut guard = words.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
    }

        // reuse and extend existing table of divisors or create new table as appropriate
    let mut table: Arc<Mutex<Option<Vec<divisor>>>> = Arc::new(Mutex::new(None));
    if { let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(10 as u64)))); __tmp_x == __tmp_y } {
        (*cacheBase10.lock().unwrap().as_ref().unwrap()).mutex.lock();
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*cacheBase10.lock().unwrap().as_ref().unwrap()).table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize..({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); table = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(vec![Default::default(); ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))); table = new_val; };
    }

        // reuse old table for this conversion
        // create new table for this conversion
        // extend table
    if { let __tmp_x = (*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.ndigits.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // add new entries as needed
        let mut larger: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = (*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.ndigits.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = (*crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).exp_w_w(Arc::new(Mutex::new(Some({ let __arg_holder = bb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*leafSize.lock().unwrap().as_ref().unwrap()) as u64)))))))).lock().unwrap().as_ref().unwrap()).clone(); *(*table.lock().unwrap().as_mut().unwrap())[(0) as usize].bbb.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __v = (*ndigits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*leafSize.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; *{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.ndigits.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = (*crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sqr(Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.bbb.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone(); *(*table.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].bbb.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = 2; let __tmp_y = (*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.ndigits.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; *{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.ndigits.lock().unwrap() = Some(new_val); };
    }
                // optimization: exploit aggregated extra bits in macro blocks
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set(Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.bbb.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *larger.lock().unwrap() = __moved_val; };
        while { let __tmp_x = (*mul_add_v_w_w({ let __named_slice = (*larger.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*larger.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))))))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let new_val = (*(*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.bbb.lock().unwrap().as_ref().unwrap()).set(larger.clone()).lock().unwrap().as_ref().unwrap()).clone(); *(*table.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].bbb.lock().unwrap() = Some(new_val); };
        { let __target = { let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.ndigits.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = (*{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.bbb.lock().unwrap().as_ref().unwrap()).bit_len(); *{ let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.nbits.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

        // add new entries as needed
        // optimization: exploit aggregated extra bits in macro blocks
    if { let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(10 as u64)))); __tmp_x == __tmp_y } {
        (*cacheBase10.lock().unwrap().as_ref().unwrap()).mutex.unlock();
    }

    return table.clone();
}

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub mutex: sync::mutex::Mutex,
    pub table: Arc<Mutex<Option<[divisor; 64]>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), table: { let __guard = self.table.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct1 {
    pub fn lock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.lock()
    }

    pub fn try_lock(&mut self) -> bool {
        let embedded_ref = &mut self.mutex;
        embedded_ref.try_lock()
    }

    pub fn unlock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.unlock()
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { mutex: Default::default(), table: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.table))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type cacheBase10 = AnonymousStruct1;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for divisor {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
