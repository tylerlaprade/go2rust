use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone, go_strconv_format_float, go_strconv_format_int};

use crate::accuracy_string::*;
use crate::arith::*;
use crate::arith_decl::*;
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
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_SHIFT: i32 = __W - 4;


/// A decimal represents an unsigned floating-point number in decimal representation.
/// The value of a non-zero decimal d is d.mant * 10**d.exp with 0.1 <= d.mant < 1,
/// with the most-significant mantissa digit at index 0. For the zero decimal, the
/// mantissa length and exponent are 0.
/// The zero value for decimal represents a ready-to-use 0.0.
#[derive(Debug, Clone)]
pub struct decimal {
    pub mant: Arc<Mutex<Option<Vec<u8>>>>,
    pub exp: Arc<Mutex<Option<i32>>>,
}

impl decimal {
    pub fn __go_value_clone(&self) -> Self {
        Self { mant: self.mant.clone(), exp: { let __guard = self.exp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for decimal {
    fn default() -> Self {
        Self { mant: Arc::new(Mutex::new(None)), exp: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for decimal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl decimal {
    /// at returns the i'th mantissa digit, starting with the most significant digit at 0.
    pub fn at(&self, i: Arc<Mutex<Option<i32>>>) -> u8 {
        if { let __tmp_x = 0; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        return { let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
    }
        ('0' as u8)
    }

    /// Init initializes x to the decimal representation of m << shift (for
    /// shift >= 0), or m >> -shift (for shift < 0).
    pub fn init(&mut self, mut m: Arc<Mutex<Option<nat>>>, mut shift: Arc<Mutex<Option<i32>>>) {
                // special case 0
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(0) as usize].to_vec() }))); self.mant = new_val; };
        { let new_val = 0; *self.exp.lock().unwrap() = Some(new_val); };
        return;
    }
                // Optimization: If we need to shift right, first remove any trailing
                // zero bits from m to reduce shift amount that needs to be done in
                // decimal format (since that is likely slower).
        if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        let mut ntz = (*m.lock().unwrap().as_ref().unwrap()).trailing_zero_bits();
        let mut s = Arc::new(Mutex::new(Some(-((*shift.lock().unwrap().as_ref().unwrap())) as u64)));
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ntz; __tmp_x >= __tmp_y } {
        { let new_val = ntz; *s.lock().unwrap() = Some(new_val); };
    }
                // shift at most ntz bits
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shr(m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *m.lock().unwrap() = __moved_val; };
        { let __rhs = (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let mut guard = shift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // shift at most ntz bits
                // Do any shift left in binary representation.
        if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shl(m.clone(), Arc::new(Mutex::new(Some((*shift.lock().unwrap().as_ref().unwrap()) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *m.lock().unwrap() = __moved_val; };
        { let new_val = 0; *shift.lock().unwrap() = Some(new_val); };
    }
                // Convert mantissa into decimal representation.
        let mut s = (*m.lock().unwrap().as_ref().unwrap()).utoa(Arc::new(Mutex::new(Some(10))));
        let mut n = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *self.exp.lock().unwrap() = Some(new_val); };
                // Trim trailing zeros; instead the exponent is tracking
                // the decimal point independent of the number of digits.
        while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let new_val = { let __append_target = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(0) as usize].to_vec() }))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; self.mant = new_val; };
                // Do any (remaining) shift right in decimal representation.
        if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        while { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -60; __tmp_x < __tmp_y } {
        shr(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some(MAX_SHIFT as u64))));
        { let __rhs = 60; let mut guard = shift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        shr(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some(-((*shift.lock().unwrap().as_ref().unwrap())) as u64))));
    }
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("0".to_string())));
    }
        let mut buf: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x <= __tmp_y } {
                        // 0.00ddd
            { let new_val = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __tmp_x = ({ let __tmp_x = 2; let __tmp_y = (-({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })); __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y }) as usize)))); buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("0.".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
            { let new_val = append_zeros(buf.clone(), Arc::new(Mutex::new(Some(-({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))); buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = self.mant.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else if { let __tmp_x = ((*self.exp.clone().lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
                        // dd.ddd
            { let new_val = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __tmp_x = 1; let __tmp_y = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y }) as usize)))); buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (*self.exp.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[..__high].to_vec() }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('.' as i32) as u8); __append_target.clone() }; buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = (*self.exp.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[__low..].to_vec() }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else {
                        // ddd00
            { let new_val = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity((*self.exp.clone().lock().unwrap().as_ref().unwrap()) as usize)))); buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = self.mant.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; buf = new_val; };
            { let new_val = append_zeros(buf.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = ((*self.exp.clone().lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x - __tmp_y })))); buf = new_val; };
        }
                // 0.00ddd
                /* 0 < */
                // dd.ddd
                // len(x.mant) <= x.exp
                // ddd00
        return Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    /// round sets x to (at most) n mantissa digits by rounding it
    /// to the nearest even value with n (or fever) mantissa digits.
    /// If n < 0, x remains unchanged.
    pub fn round(&mut self, n: Arc<Mutex<Option<i32>>>) {
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
        return;
    }
                // nothing to do
        if should_round_up(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        self.round_up(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        self.round_down(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }

    pub fn round_up(&mut self, mut n: Arc<Mutex<Option<i32>>>) {
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
        return;
    }
                // nothing to do
                // 0 <= n < len(x.mant)
                // find first digit < '9'
        while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('9' as i32) as u8; __tmp_x >= __tmp_y } {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // all digits are '9's => round up to '1' and update exponent
        (*self.mant.lock().unwrap().as_mut().unwrap())[(0) as usize] = ('1' as i32) as u8;
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(1) as usize].to_vec() }))); self.mant = new_val; };
        { let __target = self.exp.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return;
    }
                // all digits are '9's => round up to '1' and update exponent
                // ok since len(x.mant) > n
                // n > 0 && x.mant[n-1] < '9'
        { let __idx = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y } as usize; let mut __seq_guard = self.mant.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); self.mant = new_val; };
    }

    pub fn round_down(&mut self, n: Arc<Mutex<Option<i32>>>) {
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
        return;
    }
                // nothing to do
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); self.mant = new_val; };
        trim(Arc::new(Mutex::new(Some(self.clone()))));
    }
}

/// shr implements x >> s, for s <= maxShift.
pub fn shr(x: Arc<Mutex<Option<decimal>>>, s: Arc<Mutex<Option<u64>>>) {
        // Division by 1<<s using shift-and-subtract algorithm.
        // pick up enough leading digits to cover first shift
    let mut r = Arc::new(Mutex::new(Some(0)));
    let mut n: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
    while { let __tmp_x = { let __tmp_x = (*n.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } && { let __tmp_x = ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        let mut ch = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as u64)))))));
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(((((*{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) * 10) + (*{ let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())) - ('0' as u64)))))); *n.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = (*n.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
                // x == 0; shouldn't get here, but handle anyway
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(0) as usize].to_vec() }))); (*x.lock().unwrap().as_mut().unwrap()).mant = new_val; };
        return;
    }
        // x == 0; shouldn't get here, but handle anyway
    while { let __tmp_x = { let __tmp_x = (*n.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = crate::arith::Word(Arc::new(Mutex::new(Some(10 as u64)))); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() * __rhs); };
    }
    { let __target = (*x.lock().unwrap().as_ref().unwrap()).exp.clone(); let __rhs = { let __tmp_x = 1; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

        // read a digit, write a digit
    let mut w = Arc::new(Mutex::new(Some(0)));
    let mut mask = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((((1 as u64) << { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }) - 1))))))));
    while { let __tmp_x = ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        let mut ch = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as u64)))))));
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut d = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) >> { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }))))))));
        { let __rhs = (*({ let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & __rhs); };
        (*(*x.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some((((*{ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + ('0' as u64))) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(((((*{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) * 10) + (*{ let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())) - ('0' as u64)))))); *n.lock().unwrap() = Some(new_val); };
    }

        // n -= d << s
        // write extra digits that still fit
    while { let __tmp_x = (*n.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x > __tmp_y } && { let __tmp_x = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        let mut d = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) >> { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }))))))));
        { let __rhs = (*({ let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & __rhs); };
        (*(*x.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some((((*{ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + ('0' as u64))) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) * 10))))); *n.lock().unwrap() = Some(new_val); };
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); (*x.lock().unwrap().as_mut().unwrap()).mant = new_val; };

        // append additional digits that didn't fit
    while { let __tmp_x = (*n.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x > __tmp_y } {
        let mut d = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) >> { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }))))))));
        { let __rhs = (*({ let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & __rhs); };
        { let new_val = { let __append_target = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some((((*{ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + ('0' as u64))) as u8))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; (*x.lock().unwrap().as_mut().unwrap()).mant = new_val; };
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) * 10))))); *n.lock().unwrap() = Some(new_val); };
    }

    trim(x.clone());
}

/// appendZeros appends n 0 digits to buf and returns buf.
pub fn append_zeros(mut buf: Arc<Mutex<Option<Vec<u8>>>>, mut n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('0' as i32) as u8); __append_target.clone() }; buf = new_val; };
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return buf.clone();
}

/// shouldRoundUp reports if x should be rounded up
/// if shortened to n digits. n must be a valid index
/// for x.mant.
pub fn should_round_up(x: Arc<Mutex<Option<decimal>>>, n: Arc<Mutex<Option<i32>>>) -> bool {
    if { let __tmp_x = { let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = ('5' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x == __tmp_y } {
                // exactly halfway - round to even
        return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }); let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

        // exactly halfway - round to even
        // not halfway - digit tells all (x.mant has no trailing zeros)
    return { let __tmp_x = { let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = ('5' as i32) as u8; __tmp_x >= __tmp_y };
}

/// trim cuts off any trailing zeros from x's mantissa;
/// they are meaningless for the value of x.
pub fn trim(x: Arc<Mutex<Option<decimal>>>) {
    let mut i = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); (*x.lock().unwrap().as_mut().unwrap()).mant = new_val; };
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 0; *(*x.lock().unwrap().as_ref().unwrap()).exp.lock().unwrap() = Some(new_val); };
    }
}

impl GoValueClone for decimal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
