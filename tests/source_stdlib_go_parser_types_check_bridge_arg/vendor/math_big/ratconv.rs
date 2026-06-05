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
use crate::natconv::*;
use crate::natdiv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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


pub(crate) static ratZero: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::rat::Rat>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ratZero.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *ratZero.lock().unwrap() = Some(Default::default());
}


impl crate::rat::Rat {
    /// Scan is a support routine for fmt.Scanner. It accepts the formats
    /// 'e', 'E', 'f', 'F', 'g', 'G', and 'v'. All formats are equivalent.
    pub fn scan(&mut self, s: Arc<Mutex<Option<fmt_ScanState>>>, ch: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut tok, mut err) = (*s.lock().unwrap().as_ref().unwrap()).token(true, rat_tok);
        if (*err.lock().unwrap()).is_some() {
        return err.clone();
    }
        if !strings::contains_rune(Arc::new(Mutex::new(Some("efgEFGv".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Rat.Scan: invalid verb".to_string()))));
    }
        {
        let (_, mut ok) = self.set_string(Arc::new(Mutex::new(Some(String::from_utf8((*tok.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))));;
        if !ok {
            return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Rat.Scan: invalid syntax".to_string()))));;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// SetString sets z to the value of s and returns z and a boolean indicating
    /// success. s can be given as a (possibly signed) fraction "a/b", or as a
    /// floating-point number optionally followed by an exponent.
    /// If a fraction is provided, both the dividend and the divisor may be a
    /// decimal integer or independently use a prefix of “0b”, “0” or “0o”,
    /// or “0x” (or their upper-case variants) to denote a binary, octal, or
    /// hexadecimal integer, respectively. The divisor may not be signed.
    /// If a floating-point number is provided, it may be in decimal form or
    /// use any of the same prefixes as above but for “0” to denote a non-decimal
    /// mantissa. A leading “0” is considered a decimal leading 0; it does not
    /// indicate octal representation in this case.
    /// An optional base-10 “e” or base-2 “p” (or their upper-case variants)
    /// exponent may be provided as well, except for hexadecimal floats which
    /// only accept an (optional) “p” exponent (because an “e” or “E” cannot
    /// be distinguished from a mantissa digit). If the exponent's absolute value
    /// is too large, the operation may fail.
    /// The entire string, not just a prefix, must be valid for success. If the
    /// operation failed, the value of z is undefined but the returned value is nil.
    pub fn set_string(&mut self, s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::rat::Rat>>>, bool) {
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), false);
    }
                // len(s) > 0
                // parse fraction a/b, if any
        {
        let mut sep = strings::index(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("/".to_string()))));;
        if { let __tmp_x = sep; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            {
        let (_, mut ok) = (*self.a.lock().unwrap().as_mut().unwrap()).set_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = (sep) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some(0))));;
        if !ok {
            return (Arc::new(Mutex::new(None)), false);;
        }
    };
            let mut r = strings::new_reader(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = sep; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));;
            let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));;
            {
        { let (__tmp_0, __tmp_1, __tmp_2, __tmp_3) = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).scan(Arc::new(Mutex::new(Some(io_ByteScanner::__go_from(r.clone())))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_tmp_0; let __moved_tmp_3 = { let mut __guard = __tmp_3.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_3; };;
        if (*err.lock().unwrap()).is_some() {
            return (Arc::new(Mutex::new(None)), false);;
        }
    };
            {
        { let (__tmp_0, __tmp_1) = { let __recv = r.clone(); let __recv_ptr: *mut strings::reader::Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut strings::reader::Reader }; let __result = unsafe { &mut *__recv_ptr }.read_byte(); __result }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; !__eq } {
            return (Arc::new(Mutex::new(None)), false);;
        }
    };
            if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), false);
    };
            return (self.norm(), true);;
        }
    }
                // entire string must have been consumed
                // parse floating-point number
        let mut r = strings::new_reader(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // sign
        let (mut neg, mut err) = scan_sign(Arc::new(Mutex::new(Some(io_ByteScanner::__go_from(r.clone())))));
        if (*err.lock().unwrap()).is_some() {
        return (Arc::new(Mutex::new(None)), false);
    }
                // mantissa
        let mut base: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        let mut fcount: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2, __tmp_3) = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).scan(Arc::new(Mutex::new(Some(io_ByteScanner::__go_from(r.clone())))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(true)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_tmp_0; *base.lock().unwrap() = Some(__tmp_1); *fcount.lock().unwrap() = Some(__tmp_2); let __moved_tmp_3 = { let mut __guard = __tmp_3.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_3; };
        if (*err.lock().unwrap()).is_some() {
        return (Arc::new(Mutex::new(None)), false);
    }
                // exponent
        let mut exp: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        let mut ebase: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2) = scan_exponent(Arc::new(Mutex::new(Some(io_ByteScanner::__go_from(r.clone())))), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(true)))); *exp.lock().unwrap() = Some(__tmp_0); *ebase.lock().unwrap() = Some(__tmp_1); let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        if (*err.lock().unwrap()).is_some() {
        return (Arc::new(Mutex::new(None)), false);
    }
                // there should be no unread characters left
        {
        { let (__tmp_0, __tmp_1) = { let __recv = r.clone(); let __recv_ptr: *mut strings::reader::Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut strings::reader::Reader }; let __result = unsafe { &mut *__recv_ptr }.read_byte(); __result }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; !__eq } {
            return (Arc::new(Mutex::new(None)), false);;
        }
    }
                // special-case 0 (see also issue #16176)
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (self.norm(), true);
    }
                // len(z.a.abs) > 0
                // The mantissa may have a radix point (fcount <= 0) and there
                // may be a nonzero exponent exp. The radix point amounts to a
                // division by base**(-fcount), which equals a multiplication by
                // base**fcount. An exponent means multiplication by ebase**exp.
                // Multiplications are commutative, so we can apply them in any
                // order. We only have powers of 2 and 10, and we split powers
                // of 10 into the product of the same powers of 2 and 5. This
                // may reduce the size of shift/multiplication factors or
                // divisors required to create the final fraction, depending
                // on the actual floating-point value.
                // determine binary or decimal exponent contribution of radix point
        let mut exp2: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut exp5: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __v = (*fcount.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
                // The mantissa has a radix point ddd.dddd; and
                // -fcount is the number of digits to the right
                // of '.'. Adjust relevant exponent accordingly.
        let mut d = Arc::new(Mutex::new(Some((*fcount.lock().unwrap().as_ref().unwrap()) as i64)));
        {
        let _switch_val = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == 10) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = d.lock().unwrap().as_ref().unwrap().clone(); *exp5.lock().unwrap() = Some(new_val); };
            _fallthrough = true;
        }
        if !_matched && (_switch_val == 2) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = d.lock().unwrap().as_ref().unwrap().clone(); *exp2.lock().unwrap() = Some(new_val); };
        }
        if !_matched && (_switch_val == 8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as i64; __tmp_x * __tmp_y }; *exp2.lock().unwrap() = Some(new_val); };
        }
        if !_matched && (_switch_val == 16) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4 as i64; __tmp_x * __tmp_y }; *exp2.lock().unwrap() = Some(new_val); };
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            std::panic::panic_any(Box::new("unexpected mantissa base".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
    }
                // The mantissa has a radix point ddd.dddd; and
                // -fcount is the number of digits to the right
                // of '.'. Adjust relevant exponent accordingly.
                // 10**e == 5**e * 2**e
                // octal digits are 3 bits each
                // hexadecimal digits are 4 bits each
                // fcount consumed - not needed anymore
                // take actual exponent into account
        {
        let _switch_val = { let __v = (*ebase.lock().unwrap().as_ref().unwrap()).clone(); __v };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == 10) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let __rhs = (*exp.lock().unwrap().as_ref().unwrap()); let mut guard = exp5.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
            _fallthrough = true;
        }
        if !_matched && (_switch_val == 2) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let __rhs = (*exp.lock().unwrap().as_ref().unwrap()); let mut guard = exp2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            std::panic::panic_any(Box::new("unexpected exponent base".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
                // see fallthrough above
                // exp consumed - not needed anymore
                // apply exp5 contributions
                // (start with exp5 so the numbers to multiply are smaller)
        if { let __tmp_x = { let __v = (*exp5.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        let mut n = { let __owned = exp5.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = -((*n.lock().unwrap().as_ref().unwrap())); *n.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // This can occur if -n overflows. -(-1 << 63) would become
                // -1 << 63, which is still negative.
        return (Arc::new(Mutex::new(None)), false);
    }
    }
                // This can occur if -n overflows. -(-1 << 63) would become
                // -1 << 63, which is still negative.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e6 as i64; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(None)), false);
    }
                // avoid excessively large exponents
        let mut pow5 = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).exp_n_n(natFive.clone(), crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u64)))))))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));
        if { let __tmp_x = { let __v = (*exp5.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { let new_val = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).mul({ let __field = (*self.a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, pow5.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = pow5.lock().unwrap().as_ref().unwrap().clone(); *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = Some(new_val); };
    }
    } else {
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    }
                // This can occur if -n overflows. -(-1 << 63) would become
                // -1 << 63, which is still negative.
                // avoid excessively large exponents
                // use underlying array of z.b.abs
                // apply exp2 contributions
        if { let __tmp_x = { let __v = (*exp2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1e7 as i64; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*exp2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e7 as i64; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(None)), false);
    }
                // avoid excessively large exponents
        if { let __tmp_x = { let __v = (*exp2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { let new_val = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).shl({ let __field = (*self.a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some((*exp2.lock().unwrap().as_ref().unwrap()) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    } else if { let __tmp_x = { let __v = (*exp2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).shl({ let __field = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some(-((*exp2.lock().unwrap().as_ref().unwrap())) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    }
        { let new_val = neg && { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y }; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        (self.norm(), true)
    }

    /// String returns a string representation of x in the form "a/b" (even if b == 1).
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(String::from_utf8((*self.marshal(Arc::new(Mutex::new(None))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())))
    }

    /// marshal implements [Rat.String] returning a slice of bytes.
    /// It appends the string representation of x in the form "a/b" (even if b == 1) to buf,
    /// and returns the extended buffer.
    pub fn marshal(&self, mut buf: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        { let new_val = (*self.a.lock().unwrap().as_ref().unwrap()).append(buf.clone(), Arc::new(Mutex::new(Some(10)))); buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('/' as i32) as u8); __append_target.clone() }; buf = new_val; };
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let new_val = (*self.b.lock().unwrap().as_ref().unwrap()).append(buf.clone(), Arc::new(Mutex::new(Some(10)))); buf = new_val; };
    } else {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('1' as i32) as u8); __append_target.clone() }; buf = new_val; };
    }
        return buf.clone();
    }

    /// RatString returns a string representation of x in the form "a/b" if b != 1,
    /// and in the form "a" if b == 1.
    pub fn rat_string(&self) -> Arc<Mutex<Option<String>>> {
        if self.is_int() {
        return (*self.a.lock().unwrap().as_ref().unwrap()).string();
    }
        self.string()
    }

    /// FloatString returns a string representation of x in decimal form with prec
    /// digits of precision after the radix point. The last digit is rounded to
    /// nearest, with halves rounded away from zero.
    pub fn float_string(&self, prec: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
        let mut buf: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
        if self.is_int() {
        { let new_val = (*self.a.lock().unwrap().as_ref().unwrap()).append(buf.clone(), Arc::new(Mutex::new(Some(10)))); buf = new_val; };
        if { let __tmp_x = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('.' as i32) as u8); __append_target.clone() }; buf = new_val; };
        let mut i = { let __owned = prec.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('0' as i32) as u8); __append_target.clone() }; buf = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        return Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }
                // x.b.abs != 0
        let (mut q, mut r) = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).div(Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>)))))), { let __field = (*self.a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        let mut p = Arc::new(Mutex::new(Some((*natOne.lock().unwrap().as_ref().unwrap()).clone())));
        if { let __tmp_x = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).exp_n_n(natTen.clone(), crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_uint64(Arc::new(Mutex::new(Some((*prec.lock().unwrap().as_ref().unwrap()) as u64)))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_val; };
    }
        { let new_val = (*r.lock().unwrap().as_ref().unwrap()).mul(r.clone(), p.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
        let (__tmp_0, mut r2) = (*r.lock().unwrap().as_ref().unwrap()).div(Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>)))))), r.clone(), { let __field = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_0;;
                // see if we need to round up
        { let new_val = (*r2.lock().unwrap().as_ref().unwrap()).add(r2.clone(), r2.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r2.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).cmp(r2.clone()); let __tmp_y = 0; __tmp_x <= __tmp_y } {
        { let new_val = (*r.lock().unwrap().as_ref().unwrap()).add(r.clone(), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*r.lock().unwrap().as_ref().unwrap()).cmp(p.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).add(q.clone(), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub(r.clone(), p.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
    }
    }
        if (*(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap().as_ref().unwrap()) {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; buf = new_val; };
    }
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = (*q.lock().unwrap().as_ref().unwrap()).utoa(Arc::new(Mutex::new(Some(10)))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
        if { let __tmp_x = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('.' as i32) as u8); __append_target.clone() }; buf = new_val; };
        let mut rs = (*r.lock().unwrap().as_ref().unwrap()).utoa(Arc::new(Mutex::new(Some(10))));
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*rs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('0' as i32) as u8); __append_target.clone() }; buf = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = rs.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
    }
        return Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    /// FloatPrec returns the number n of non-repeating digits immediately
    /// following the decimal point of the decimal representation of x.
    /// The boolean result indicates whether a decimal representation of x
    /// with that many fractional digits is exact or rounded.
    ///
    /// Examples:
    ///
    ///	x      n    exact    decimal representation n fractional digits
    ///	0      0    true     0
    ///	1      0    true     1
    ///	1/2    1    true     0.5
    ///	1/3    0    false    0       (0.333... rounded)
    ///	1/4    2    true     0.25
    ///	1/6    1    false    0.2     (0.166... rounded)
    pub fn float_prec(&self) -> (i32, bool) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut exact: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

                // Determine q and largest p2, p5 such that d = q·2^p2·5^p5.
                // The results n, exact are:
                //
                //     n = max(p2, p5)
                //     exact = q == 1
                //
                // For details see:
                // https://en.wikipedia.org/wiki/Repeating_decimal#Reciprocals_of_integers_not_coprime_to_10
        let mut d = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.denom().lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
                // Determine p2 by counting factors of 2.
                // p2 corresponds to the trailing zero bits in d.
                // Do this first to reduce q as much as possible.
        let mut q: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut p2 = (*d.lock().unwrap().as_ref().unwrap()).trailing_zero_bits();
        { let new_val = (*q.lock().unwrap().as_ref().unwrap()).shr(d.clone(), Arc::new(Mutex::new(Some(p2)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
                // Determine p5 by counting factors of 5.
                // Build a table starting with an initial power of 5,
                // and use repeated squaring until the factor doesn't
                // divide q anymore. Then use the table to determine
                // the power of 5 in q.
        const fp: i32 = 13;

        let mut tab: Arc<Mutex<Option<Vec<nat>>>> = Arc::new(Mutex::new(None));
        let mut f = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(1220703125 as u64))))])))))));
        let mut t: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));let mut r: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        loop {
        {
        { let (__tmp_0, __tmp_1) = (*t.lock().unwrap().as_ref().unwrap()).div(r.clone(), q.clone(), f.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };;
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
            break;
        }
    }
                // f doesn't divide q evenly
        { let new_val = { let __append_target = tab.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*f.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; tab = new_val; };
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sqr(f.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *f.lock().unwrap() = __moved_val; };
    }
                // f doesn't divide q evenly
                // nat(nil) to ensure a new f for each table entry
                // Factor q using the table entries, if any.
                // We start with the largest factor f = tab[len(tab)-1]
                // that evenly divides q. It does so at most once because
                // otherwise f·f would also divide q. That can't be true
                // because f·f is the next higher table entry, contradicting
                // how f was chosen in the first place.
                // The same reasoning applies to the subsequent factors.
        let mut p5: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*tab.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
        { let (__tmp_0, __tmp_1) = (*t.lock().unwrap().as_ref().unwrap()).div(r.clone(), q.clone(), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };;
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
            { let __rhs = { let __tmp_x = fp as u64; let __tmp_y = ({ let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x * __tmp_y }; let mut guard = p5.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };;
            { let new_val = (*q.lock().unwrap().as_ref().unwrap()).set(t.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // tab[i] == 5^(fp·2^i)
                // If fp != 1, we may still have multiples of 5 left.
        loop {
        {
        { let (__tmp_0, __tmp_1) = (*t.lock().unwrap().as_ref().unwrap()).div(r.clone(), q.clone(), natFive.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };;
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
            break;
        }
    }
        { let mut guard = p5.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*q.lock().unwrap().as_ref().unwrap()).set(t.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
    }
        return ((*Arc::new(Mutex::new(Some(std::cmp::max((p2 as u64), ({ let __v = (*p5.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64)) as i32))).lock().unwrap().as_ref().unwrap()), { let __tmp_x = (*q.lock().unwrap().as_ref().unwrap()).cmp(natOne.clone()); let __tmp_y = 0; __tmp_x == __tmp_y });
    }
}

pub fn rat_tok(ch: Arc<Mutex<Option<i32>>>) -> bool {
    strings::contains_rune(Arc::new(Mutex::new(Some("+-/0123456789.eE".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// scanExponent scans the longest possible prefix of r representing a base 10
/// (“e”, “E”) or a base 2 (“p”, “P”) exponent, if any. It returns the
/// exponent, the exponent base (10 or 2), or a read or syntax error, if any.
///
/// If sepOk is set, an underscore character “_” may appear between successive
/// exponent digits; such underscores do not change the value of the exponent.
/// Incorrect placement of underscores is reported as an error if there are no
/// other errors. If sepOk is not set, underscores are not recognized and thus
/// terminate scanning like any other character that is not a valid digit.
///
///	exponent = ( "e" | "E" | "p" | "P" ) [ sign ] digits .
///	sign     = "+" | "-" .
///	digits   = digit { [ '_' ] digit } .
///	digit    = "0" ... "9" .
///
/// A base 2 exponent is only permitted if base2ok is set.
pub fn scan_exponent(r: Arc<Mutex<Option<io_ByteScanner>>>, base2ok: Arc<Mutex<Option<bool>>>, sepOk: Arc<Mutex<Option<bool>>>) -> (i64, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut exp: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut base: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // one char look-ahead
    let (mut ch, __tmp_1) = (*r.lock().unwrap().as_ref().unwrap()).read_byte(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if (*err.lock().unwrap()).is_some() {
        if { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        *err.lock().unwrap() = None;
    }
        return (0, 10, err.clone());
    }

        // exponent char
    '__go_switch_1: loop {
        {
        let _switch_val = ch;
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == ('e' as i32) as u8 || _switch_val == ('E' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = 10; *base.lock().unwrap() = Some(new_val); };
        }
        if !_matched && (_switch_val == ('p' as i32) as u8 || _switch_val == ('P' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __v = (*base2ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = 2; *base.lock().unwrap() = Some(new_val); };
        break '__go_switch_1
    }
            _fallthrough = true;
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            (*r.lock().unwrap().as_ref().unwrap()).unread_byte();
            return (0, 10, Arc::new(Mutex::new(None)));
        }
    };
        break;
    }

        // ok
        // binary exponent not permitted
        // ch does not belong to exponent anymore
        // sign
    let mut digits: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = (*r.lock().unwrap().as_ref().unwrap()).read_byte(); ch = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if (*err.lock().unwrap()).is_none() && ({ let __tmp_x = ch; let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = ch; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y }) {
        if { let __tmp_x = ch; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = digits.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('-' as i32) as u8); __append_target.clone() }; digits = new_val; };
    }
        { let (__tmp_0, __tmp_1) = (*r.lock().unwrap().as_ref().unwrap()).read_byte(); ch = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }

        // prev encodes the previously seen char: it is one
        // of '_', '0' (a digit), or '.' (anything else). A
        // valid separator '_' may only occur after a digit.
    let mut prev = Arc::new(Mutex::new(Some(('.' as i32))));
    let mut invalSep = Arc::new(Mutex::new(Some(false)));

        // exponent value
    let mut hasDigits = Arc::new(Mutex::new(Some(false)));
    while (*err.lock().unwrap()).is_none() {
        if { let __tmp_x = ('0' as i32) as u8; let __tmp_y = ch; __tmp_x <= __tmp_y } && { let __tmp_x = ch; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } {
        { let new_val = { let __append_target = digits.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(ch); __append_target.clone() }; digits = new_val; };
        { let new_val = ('0' as i32); *prev.lock().unwrap() = Some(new_val); };
        { let new_val = true; *hasDigits.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = ch; let __tmp_y = ('_' as i32) as u8; __tmp_x == __tmp_y } && { let __v = (*sepOk.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = { let __v = (*prev.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x != __tmp_y } {
        { let new_val = true; *invalSep.lock().unwrap() = Some(new_val); };
    }
        { let new_val = ('_' as i32); *prev.lock().unwrap() = Some(new_val); };
    } else {
        (*r.lock().unwrap().as_ref().unwrap()).unread_byte();
        break
    }
                // ch does not belong to number anymore
        { let (__tmp_0, __tmp_1) = (*r.lock().unwrap().as_ref().unwrap()).read_byte(); ch = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }

        // ch does not belong to number anymore
    if { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        *err.lock().unwrap() = None;
    }
    if (*err.lock().unwrap()).is_none() && !{ let __v = (*hasDigits.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __rhs_holder = errNoDigits.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    if (*err.lock().unwrap()).is_none() {
        { let (__tmp_0, __tmp_1) = strconv::parse_int(Arc::new(Mutex::new(Some(String::from_utf8((*digits.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), 10, 64); *exp.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }

        // other errors take precedence over invalid separators
    if (*err.lock().unwrap()).is_none() && ({ let __v = (*invalSep.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __v = (*prev.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y }) {
        { let __rhs_holder = errInvalSep.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }

    return ((*exp.lock().unwrap().as_ref().unwrap()), (*base.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
