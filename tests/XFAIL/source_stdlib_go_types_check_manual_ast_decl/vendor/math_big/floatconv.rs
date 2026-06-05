use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_strconv_format_float, go_strconv_format_int};

use crate::accuracy_string::*;
use crate::arith::*;
use crate::arith_decl::*;
use crate::decimal::*;
use crate::float::*;
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
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) static floatZero: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::float::Float>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pow5tab: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 28]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *floatZero.lock().unwrap() = Some(Default::default());
    *pow5tab.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *pow5tab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1 as u64, 5 as u64, 25 as u64, 125 as u64, 625 as u64, 3125 as u64, 15625 as u64, 78125 as u64, 390625 as u64, 1953125 as u64, 9765625 as u64, 48828125 as u64, 244140625 as u64, 1220703125 as u64, 6103515625 as u64, 30517578125 as u64, 152587890625 as u64, 762939453125 as u64, 3814697265625 as u64, 19073486328125 as u64, 95367431640625 as u64, 476837158203125 as u64, 2384185791015625 as u64, 11920928955078125 as u64, 59604644775390625 as u64, 298023223876953125 as u64, 1490116119384765625 as u64, 7450580596923828125 as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *floatZero.lock().unwrap() = Some(Default::default());
    *pow5tab.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_1() {
    *pow5tab.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([1 as u64, 5 as u64, 25 as u64, 125 as u64, 625 as u64, 3125 as u64, 15625 as u64, 78125 as u64, 390625 as u64, 1953125 as u64, 9765625 as u64, 48828125 as u64, 244140625 as u64, 1220703125 as u64, 6103515625 as u64, 30517578125 as u64, 152587890625 as u64, 762939453125 as u64, 3814697265625 as u64, 19073486328125 as u64, 95367431640625 as u64, 476837158203125 as u64, 2384185791015625 as u64, 11920928955078125 as u64, 59604644775390625 as u64, 298023223876953125 as u64, 1490116119384765625 as u64, 7450580596923828125 as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::float::Float {
    /// SetString sets z to the value of s and returns z and a boolean indicating
    /// success. s must be a floating-point number of the same format as accepted
    /// by [Float.Parse], with base argument 0. The entire string (not just a prefix) must
    /// be valid for success. If the operation failed, the value of z is undefined
    /// but the returned value is nil.
    pub fn set_string(&mut self, s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::float::Float>>>, bool) {
        {
        let (mut f, _, mut err) = self.parse(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0))));;
        if (*err.lock().unwrap()).is_none() {
            return (f.clone(), true);;
        }
    }
        return (Arc::new(Mutex::new(None)), false);
    }

    /// scan is like Parse but reads the longest possible prefix representing a valid
    /// floating point number from an io.ByteScanner rather than a string. It serves
    /// as the implementation of Parse. It does not recognize ±Inf and does not expect
    /// EOF at the end.
    pub fn scan_1(&mut self, r: Arc<Mutex<Option<io_ByteScanner>>>, base: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<crate::float::Float>>>, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut f: Arc<Mutex<Option<Float>>> = Arc::new(Mutex::new(None));
    let mut b: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut prec = Arc::new(Mutex::new(Some({ let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = 64 as u32; *prec.lock().unwrap() = Some(new_val); };
    }
                // A reasonable value in case of an error.
        { let new_val = crate::float::form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
                // sign
        { let (__tmp_0, __tmp_1) = scan_sign(r.clone()); *self.neg.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if (*err.lock().unwrap()).is_some() {
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }
                // mantissa
        let mut fcount: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2, __tmp_3) = (*self.mant.lock().unwrap().as_ref().unwrap()).scan(r.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_tmp_0; *b.lock().unwrap() = Some(__tmp_1); *fcount.lock().unwrap() = Some(__tmp_2); let __moved_tmp_3 = { let mut __guard = __tmp_3.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_3; };
        if (*err.lock().unwrap()).is_some() {
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }
                // exponent
        let mut exp: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        let mut ebase: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1, __tmp_2) = scan_exponent(r.clone(), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y })))); *exp.lock().unwrap() = Some(__tmp_0); *ebase.lock().unwrap() = Some(__tmp_1); let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        if (*err.lock().unwrap()).is_some() {
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }
                // special-case 0
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = prec.lock().unwrap().as_ref().unwrap().clone(); *self.prec.lock().unwrap() = Some(new_val); };
        { let new_val = crate::float::Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = crate::float::form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some(self.clone()))); f = new_val; };
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }
                // len(z.mant) > 0
                // The mantissa may have a radix point (fcount <= 0) and there
                // may be a nonzero exponent exp. The radix point amounts to a
                // division by b**(-fcount). An exponent means multiplication by
                // ebase**exp. Finally, mantissa normalization (shift left) requires
                // a correcting multiplication by 2**(-shiftcount). Multiplications
                // are commutative, so we can apply them in any order as long as there
                // is no loss of precision. We only have powers of 2 and 10, and
                // we split powers of 10 into the product of the same powers of
                // 2 and 5. This reduces the size of the multiplication factor
                // needed for base-10 exponents.
                // normalize mantissa and determine initial exponent contributions
        let mut exp2 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i64; __tmp_x * __tmp_y }; let __tmp_y = fnorm({ let __field = self.mant.clone(); __field }); __tmp_x - __tmp_y })));
        let mut exp5 = Arc::new(Mutex::new(Some(0 as i64)));
                // determine binary or decimal exponent contribution of radix point
        if { let __tmp_x = { let __v = (*fcount.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
                // The mantissa has a radix point ddd.dddd; and
                // -fcount is the number of digits to the right
                // of '.'. Adjust relevant exponent accordingly.
        let mut d = Arc::new(Mutex::new(Some((*fcount.lock().unwrap().as_ref().unwrap()) as i64)));
        {
        let _switch_val = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
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
            { let __rhs = (*d.lock().unwrap().as_ref().unwrap()); let mut guard = exp2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        }
        if !_matched && (_switch_val == 8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let __rhs = { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as i64; __tmp_x * __tmp_y }; let mut guard = exp2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        }
        if !_matched && (_switch_val == 16) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let __rhs = { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4 as i64; __tmp_x * __tmp_y }; let mut guard = exp2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
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
                // apply 2**exp2
        if { let __tmp_x = MIN_EXP as i64; let __tmp_y = { let __v = (*exp2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*exp2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_EXP as i64; __tmp_x <= __tmp_y } {
        { let new_val = prec.lock().unwrap().as_ref().unwrap().clone(); *self.prec.lock().unwrap() = Some(new_val); };
        { let new_val = crate::float::form(Arc::new(Mutex::new(Some(FINITE as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some((*exp2.lock().unwrap().as_ref().unwrap()) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.exp.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some(self.clone()))); f = new_val; };
    } else {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("exponent overflow"))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }
        if { let __tmp_x = { let __v = (*exp5.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // no decimal exponent contribution
        self.round(Arc::new(Mutex::new(Some(0 as u64))));
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }
                // no decimal exponent contribution
                // exp5 != 0
                // apply 5**exp5
        let mut p = { let __recv = Arc::new(Mutex::new(Some(Float::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_prec(Arc::new(Mutex::new(Some({ let __tmp_x = self.prec(); let __tmp_y = 64 as u64; __tmp_x + __tmp_y })))); __result };
        if { let __tmp_x = { let __v = (*exp5.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = { let __recv = p.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.pow5(Arc::new(Mutex::new(Some(-((*exp5.lock().unwrap().as_ref().unwrap())) as u64)))); __result }; self.quo(__method_arg0, __method_arg1) };
    } else {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = { let __recv = p.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.pow5(Arc::new(Mutex::new(Some((*exp5.lock().unwrap().as_ref().unwrap()) as u64)))); __result }; self.mul(__method_arg0, __method_arg1) };
    }
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// pow5 sets z to 5**n and returns z.
    /// n must not be negative.
    pub fn pow5(&mut self, mut n: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<crate::float::Float>>> {
        const m: u64 = ((28 - 1) as u64);

        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = m as u64; __tmp_x <= __tmp_y } {
        return self.set_uint64(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = pow5tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }))));
    }
                // n > m
        self.set_uint64(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = pow5tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(m) as usize].clone() }))));
        { let __rhs = m as u64; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
                // use more bits for f than for z
                // TODO(gri) what is the right number?
        let mut f = { let __recv = { let __recv = Arc::new(Mutex::new(Some(Float::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_prec(Arc::new(Mutex::new(Some({ let __tmp_x = self.prec(); let __tmp_y = 64 as u64; __tmp_x + __tmp_y })))); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_uint64(Arc::new(Mutex::new(Some(5 as u64)))); __result };
        while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = f.clone(); self.mul(__method_arg0, __method_arg1) };
    }
        { let __recv = f.clone(); let __recv_ptr: *mut crate::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::float::Float }; let __result = unsafe { &mut *__recv_ptr }.mul(f.clone(), f.clone()); __result };
        { let __rhs = 1 as u64; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Parse parses s which must contain a text representation of a floating-
    /// point number with a mantissa in the given conversion base (the exponent
    /// is always a decimal number), or a string representing an infinite value.
    ///
    /// For base 0, an underscore character “_” may appear between a base
    /// prefix and an adjacent digit, and between successive digits; such
    /// underscores do not change the value of the number, or the returned
    /// digit count. Incorrect placement of underscores is reported as an
    /// error if there are no other errors. If base != 0, underscores are
    /// not recognized and thus terminate scanning like any other character
    /// that is not a valid radix point or digit.
    ///
    /// It sets z to the (possibly rounded) value of the corresponding floating-
    /// point value, and returns z, the actual base b, and an error err, if any.
    /// The entire string (not just a prefix) must be consumed for success.
    /// If z's precision is 0, it is changed to 64 before rounding takes effect.
    /// The number must be of the form:
    ///
    ///	number    = [ sign ] ( float | "inf" | "Inf" ) .
    ///	sign      = "+" | "-" .
    ///	float     = ( mantissa | prefix pmantissa ) [ exponent ] .
    ///	prefix    = "0" [ "b" | "B" | "o" | "O" | "x" | "X" ] .
    ///	mantissa  = digits "." [ digits ] | digits | "." digits .
    ///	pmantissa = [ "_" ] digits "." [ digits ] | [ "_" ] digits | "." digits .
    ///	exponent  = ( "e" | "E" | "p" | "P" ) [ sign ] digits .
    ///	digits    = digit { [ "_" ] digit } .
    ///	digit     = "0" ... "9" | "a" ... "z" | "A" ... "Z" .
    ///
    /// The base argument must be 0, 2, 8, 10, or 16. Providing an invalid base
    /// argument will lead to a run-time panic.
    ///
    /// For base 0, the number prefix determines the actual base: A prefix of
    /// “0b” or “0B” selects base 2, “0o” or “0O” selects base 8, and
    /// “0x” or “0X” selects base 16. Otherwise, the actual base is 10 and
    /// no prefix is accepted. The octal prefix "0" is not supported (a leading
    /// "0" is simply considered a "0").
    ///
    /// A "p" or "P" exponent indicates a base 2 (rather than base 10) exponent;
    /// for instance, "0x1.fffffffffffffp1023" (using base 0) represents the
    /// maximum float64 value. For hexadecimal mantissae, the exponent character
    /// must be one of 'p' or 'P', if present (an "e" or "E" exponent indicator
    /// cannot be distinguished from a mantissa digit).
    ///
    /// The returned *Float f is nil and the value of z is valid but not
    /// defined if an error is reported.
    pub fn parse(&mut self, s: Arc<Mutex<Option<String>>>, base: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<crate::float::Float>>>, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut f: Arc<Mutex<Option<Float>>> = Arc::new(Mutex::new(None));
    let mut b: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

                // scan doesn't handle ±Inf
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x == __tmp_y } && ({ let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Inf".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "inf".to_string(); __tmp_x == __tmp_y }) {
        { let new_val = self.set_inf(Arc::new(Mutex::new(Some(false)))).clone(); f = new_val; };
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 4; __tmp_x == __tmp_y } && ({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y }) && ({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "Inf".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "inf".to_string(); __tmp_x == __tmp_y }) {
        { let new_val = self.set_inf(Arc::new(Mutex::new(Some({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y })))).clone(); f = new_val; };
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }
        let mut r = strings::new_reader(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
        { let (__tmp_0, __tmp_1, __tmp_2) = self.scan_1(Arc::new(Mutex::new(Some(io_ByteScanner::__go_from(r.clone())))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); f = __tmp_0.clone(); *b.lock().unwrap() = Some(__tmp_1); let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };;
        if (*err.lock().unwrap()).is_some() {
            return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());;
        }
    }
                // entire string must have been consumed
        {
        let (mut ch, mut err2) = { let __recv = r.clone(); let __recv_ptr: *mut strings::reader::Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut strings::reader::Reader }; let __result = unsafe { &mut *__recv_ptr }.read_byte(); __result };;
        if (*err2.lock().unwrap()).is_none() {
            { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("expected end of string, found {:?}", ch))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        } else if { let __left = err2.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; !__eq } {
        { let __rhs_holder = err2.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    }
        return (f.clone(), (*b.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// Scan is a support routine for [fmt.Scanner]; it sets z to the value of
    /// the scanned number. It accepts formats whose verbs are supported by
    /// [fmt.Scan] for floating point values, which are:
    /// 'b' (binary), 'e', 'E', 'f', 'F', 'g' and 'G'.
    /// Scan doesn't handle ±Inf.
    pub fn scan(&mut self, s: Arc<Mutex<Option<fmt_ScanState>>>, ch: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        (*s.lock().unwrap().as_ref().unwrap()).skip_space();
        let (_, _, mut err) = self.scan_1(Arc::new(Mutex::new(Some(io_ByteScanner::__go_from(byteReader { scan_state: s.clone(), ..Default::default() })))), Arc::new(Mutex::new(Some(0))));
        return err.clone();
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
