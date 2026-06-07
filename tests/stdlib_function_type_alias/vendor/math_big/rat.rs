use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone};

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
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Rat represents a quotient a/b of arbitrary precision.
/// The zero value for a Rat represents the value 0.
///
/// Operations always take pointer arguments (*Rat) rather
/// than Rat values, and each unique Rat value requires
/// its own unique *Rat pointer. To "copy" a Rat value,
/// an existing (or newly allocated) Rat must be set to
/// a new value using the [Rat.Set] method; shallow copies
/// of Rats are not supported and may lead to errors.
#[derive(Debug, Clone)]
pub struct Rat {
    pub a: Arc<Mutex<Option<Int>>>,
    pub b: Arc<Mutex<Option<Int>>>,
}

impl Rat {
    pub fn __go_value_clone(&self) -> Self {
        Self { a: { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, b: { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Rat {
    fn default() -> Self {
        Self { a: Arc::new(Mutex::new(Some(Int::default()))), b: Arc::new(Mutex::new(Some(Int::default()))) }
    }
}

impl std::fmt::Display for Rat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Rat {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub mutex: GoMutex,
    pub table: Arc<Mutex<Option<[divisor; 64]>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), table: { let __guard = self.table.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { mutex: GoMutex::new(), table: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
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


impl Rat {
    /// SetFloat64 sets z to exactly f and returns z.
    /// If f is not finite, SetFloat returns nil.
    pub fn set_float64(&mut self, f: Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<Rat>>> {
        const expMask: i32 = (1 << 11) - 1;

        let mut bits = math::float64bits(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut mantissa = Arc::new(Mutex::new(Some({ let __tmp_x = bits; let __tmp_y = (((1 as u64) << (52 as u64)) - (1 as u64)) as u64; __tmp_x & __tmp_y })));
        let mut exp = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = bits; let __tmp_y = 52; __tmp_x >> __tmp_y }); let __tmp_y = expMask as u64; __tmp_x & __tmp_y }) as i32)));
        { let _switch_val = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (2047) {
            return Arc::new(Mutex::new(None));
        } else if _switch_val == (0) {
            { let __rhs = 1022; let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        } else {
            { let __rhs = ((1 as u64) << (52 as u64)) as u64; let mut guard = mantissa.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
            { let __rhs = 1023; let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        }
    }
                // non-finite
                // denormal
                // normal
        let mut shift = Arc::new(Mutex::new(Some({ let __tmp_x = 52; let __tmp_y = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
                // Optimization (?): partially pre-normalise.
        while { let __tmp_x = { let __tmp_x = { let __v = (*mantissa.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __rhs = 1 as u64; let mut guard = mantissa.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let mut guard = shift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        (*self.a.lock().unwrap().as_mut().unwrap()).set_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = mantissa.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x < __tmp_y }; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        (*self.b.lock().unwrap().as_mut().unwrap()).set({ let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*self.b.lock().unwrap().as_mut().unwrap()).lsh(self.b.clone(), Arc::new(Mutex::new(Some((*shift.lock().unwrap().as_ref().unwrap()) as u64))));
    } else {
        (*self.a.lock().unwrap().as_mut().unwrap()).lsh(self.a.clone(), Arc::new(Mutex::new(Some(-((*shift.lock().unwrap().as_ref().unwrap())) as u64))));
    }
        self.norm()
    }

    /// Float32 returns the nearest float32 value for x and a bool indicating
    /// whether f represents x exactly. If the magnitude of x is too large to
    /// be represented by a float32, f is an infinity and exact is false.
    /// The sign of f always matches the sign of x, even if f == 0.
    pub fn float32(&self) -> (f32, bool) {
    let mut f: Arc<Mutex<Option<f32>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut exact: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut b = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = natOne.lock().unwrap().as_ref().unwrap().clone(); *b.lock().unwrap() = Some(new_val); };
    }
        { let (__tmp_0, __tmp_1) = quot_to_float32({ let __field = (*self.a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *f.lock().unwrap() = Some(__tmp_0); *exact.lock().unwrap() = Some(__tmp_1); };
        if (*(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap().as_ref().unwrap()) {
        { let new_val = -((*f.lock().unwrap().as_ref().unwrap())); *f.lock().unwrap() = Some(new_val); };
    }
        return ((*f.lock().unwrap().as_ref().unwrap()), (*exact.lock().unwrap().as_ref().unwrap()));
    }

    /// Float64 returns the nearest float64 value for x and a bool indicating
    /// whether f represents x exactly. If the magnitude of x is too large to
    /// be represented by a float64, f is an infinity and exact is false.
    /// The sign of f always matches the sign of x, even if f == 0.
    pub fn float64(&self) -> (f64, bool) {
    let mut f: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut exact: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut b = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = natOne.lock().unwrap().as_ref().unwrap().clone(); *b.lock().unwrap() = Some(new_val); };
    }
        { let (__tmp_0, __tmp_1) = quot_to_float64({ let __field = (*self.a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *f.lock().unwrap() = Some(__tmp_0); *exact.lock().unwrap() = Some(__tmp_1); };
        if (*(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap().as_ref().unwrap()) {
        { let new_val = -((*f.lock().unwrap().as_ref().unwrap())); *f.lock().unwrap() = Some(new_val); };
    }
        return ((*f.lock().unwrap().as_ref().unwrap()), (*exact.lock().unwrap().as_ref().unwrap()));
    }

    /// SetFrac sets z to a/b and returns z.
    /// If b == 0, SetFrac panics.
    pub fn set_frac(&mut self, a: Arc<Mutex<Option<Int>>>, b: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Rat>>> {
        { let new_val = { let __tmp_x = (*{ let __field = (*a.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y }; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        let mut babs = Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*babs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("division by zero".to_string()) as Box<dyn Any + Send + Sync>);
    }
        if { let __left = self.a.clone(); let __right = b.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } || alias({ let __field = (*self.a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = babs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set(babs.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *babs.lock().unwrap() = __moved_val; };
    }
                // make a copy
        { let new_val = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set({ let __field = (*a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set(babs.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        self.norm()
    }

    /// SetFrac64 sets z to a/b and returns z.
    /// If b == 0, SetFrac64 panics.
    pub fn set_frac64(&mut self, a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Rat>>> {
        if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("division by zero".to_string()) as Box<dyn Any + Send + Sync>);
    }
        (*self.a.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = -((*b.lock().unwrap().as_ref().unwrap())); *b.lock().unwrap() = Some(new_val); };
        { let new_val = !(*(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap().as_ref().unwrap()); *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
    }
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_uint64(Arc::new(Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        self.norm()
    }

    /// SetInt sets z to x (by making a copy of x) and returns z.
    pub fn set_int(&mut self, x: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Rat>>> {
        (*self.a.lock().unwrap().as_mut().unwrap()).set(x.clone());
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// SetInt64 sets z to x and returns z.
    pub fn set_int64(&mut self, x: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Rat>>> {
        (*self.a.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// SetUint64 sets z to x and returns z.
    pub fn set_uint64(&mut self, x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Rat>>> {
        (*self.a.lock().unwrap().as_mut().unwrap()).set_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Set sets z to x (by making a copy of x) and returns z.
    pub fn set(&mut self, x: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Rat>>> {
        if { let __peer = x.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        (*self.a.lock().unwrap().as_mut().unwrap()).set((*x.lock().unwrap().as_ref().unwrap()).a.clone());
        (*self.b.lock().unwrap().as_mut().unwrap()).set((*x.lock().unwrap().as_ref().unwrap()).b.clone());
    }
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Abs sets z to |x| (the absolute value of x) and returns z.
    pub fn abs(&mut self, x: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Rat>>> {
        self.set(x.clone());
        { let new_val = false; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Neg sets z to -x and returns z.
    pub fn neg(&mut self, x: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Rat>>> {
        self.set(x.clone());
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && !(*(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap().as_ref().unwrap()); *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Inv sets z to 1/x and returns z.
    /// If x == 0, Inv panics.
    pub fn inv(&mut self, x: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Rat>>> {
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*(*x.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("division by zero".to_string()) as Box<dyn Any + Send + Sync>);
    }
        self.set(x.clone());
        { let __tmp_0 = { let __selector_holder = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_1 = { let __selector_holder = (*self.a.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = Some(__tmp_0); *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = Some(__tmp_1); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Sign returns:
    ///   - -1 if x < 0;
    ///   - 0 if x == 0;
    ///   - +1 if x > 0.
    pub fn sign(&self) -> i32 {
        (*self.a.lock().unwrap().as_ref().unwrap()).sign()
    }

    /// IsInt reports whether the denominator of x is 1.
    pub fn is_int(&self) -> bool {
        return { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).cmp(natOne.clone()); let __tmp_y = 0; __tmp_x == __tmp_y };
    }

    /// Num returns the numerator of x; it may be <= 0.
    /// The result is a reference to x's numerator; it
    /// may change if a new value is assigned to x, and vice versa.
    /// The sign of the numerator corresponds to the sign of x.
    pub fn num(&self) -> Arc<Mutex<Option<crate::int::Int>>> {
        self.a.clone()
    }

    /// Denom returns the denominator of x; it is always > 0.
    /// The result is a reference to x's denominator, unless
    /// x is an uninitialized (zero value) [Rat], in which case
    /// the result is a new [Int] of value 1. (To initialize x,
    /// any operation that sets x will do, including x.Set(x).)
    /// If the result is a reference to x's denominator it
    /// may change if a new value is assigned to x, and vice versa.
    pub fn denom(&self) -> Arc<Mutex<Option<crate::int::Int>>> {
                // Note that x.b.neg is guaranteed false.
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Note: If this proves problematic, we could
                //       panic instead and require the Rat to
                //       be explicitly initialized.
        return Arc::new(Mutex::new(Some(Int { abs: Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))]))))))), ..Default::default() })));
    }
                // Note: If this proves problematic, we could
                //       panic instead and require the Rat to
                //       be explicitly initialized.
        self.b.clone()
    }

    pub fn norm(&mut self) -> Arc<Mutex<Option<Rat>>> {
        {
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y }) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // z == 0; normalize sign and denominator
            { let new_val = false; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
            _fallthrough = true;
        }
        if !_matched && ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y }) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // z is integer; normalize denominator
            { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // z is fraction; normalize numerator and denominator
            let mut neg = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.a.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            { let new_val = false; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
            { let new_val = false; *(*self.b.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
            {
        let mut f = { let __recv = new_int(Arc::new(Mutex::new(Some(0 as i64)))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).lehmer_g_c_d(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), self.a.clone(), self.b.clone()); __result };;
        if { let __tmp_x = { let __recv = f.clone(); let __recv_ptr: *const crate::int::Int = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::int::Int }; let __result = unsafe { &*__recv_ptr }.cmp({ let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); __result }; let __tmp_y = 0; __tmp_x != __tmp_y } {
            { let (__tmp_0, __tmp_1) = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).div(Arc::new(Mutex::new(None)), { let __field = (*self.a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*f.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_tmp_0; };;
            { let (__tmp_0, __tmp_1) = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).div(Arc::new(Mutex::new(None)), { let __field = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*f.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_tmp_0; };;
        }
    }
            { let new_val = neg.lock().unwrap().as_ref().unwrap().clone(); *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        }
    }
                // z == 0; normalize sign and denominator
                // z is integer; normalize denominator
                // z is fraction; normalize numerator and denominator
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Cmp compares x and y and returns:
    ///   - -1 if x < y;
    ///   - 0 if x == y;
    ///   - +1 if x > y.
    pub fn cmp(&self, y: Arc<Mutex<Option<Rat>>>) -> i32 {
        let mut a: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut b: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*a.lock().unwrap().as_mut().unwrap()).scale_denom(self.a.clone(), { let __field = (*(*y.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        (*b.lock().unwrap().as_mut().unwrap()).scale_denom((*y.lock().unwrap().as_ref().unwrap()).a.clone(), { let __field = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        return (*a.lock().unwrap().as_ref().unwrap()).cmp(b.clone());
    }

    /// Add sets z to the sum x+y and returns z.
    pub fn add(&mut self, x: Arc<Mutex<Option<Rat>>>, y: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Rat>>> {
        let mut a1: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut a2: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*a1.lock().unwrap().as_mut().unwrap()).scale_denom((*x.lock().unwrap().as_ref().unwrap()).a.clone(), { let __field = (*(*y.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        (*a2.lock().unwrap().as_mut().unwrap()).scale_denom((*y.lock().unwrap().as_ref().unwrap()).a.clone(), { let __field = (*(*x.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        (*self.a.lock().unwrap().as_mut().unwrap()).add(a1.clone(), a2.clone());
        { let new_val = mul_denom({ let __field = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*(*x.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*(*y.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        self.norm()
    }

    /// Sub sets z to the difference x-y and returns z.
    pub fn sub(&mut self, x: Arc<Mutex<Option<Rat>>>, y: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Rat>>> {
        let mut a1: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut a2: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*a1.lock().unwrap().as_mut().unwrap()).scale_denom((*x.lock().unwrap().as_ref().unwrap()).a.clone(), { let __field = (*(*y.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        (*a2.lock().unwrap().as_mut().unwrap()).scale_denom((*y.lock().unwrap().as_ref().unwrap()).a.clone(), { let __field = (*(*x.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        (*self.a.lock().unwrap().as_mut().unwrap()).sub(a1.clone(), a2.clone());
        { let new_val = mul_denom({ let __field = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*(*x.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*(*y.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        self.norm()
    }

    /// Mul sets z to the product x*y and returns z.
    pub fn mul(&mut self, x: Arc<Mutex<Option<Rat>>>, y: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Rat>>> {
        if { let __left = x.clone(); let __right = y.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // a squared Rat is positive and can't be reduced (no need to call norm())
        { let new_val = false; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).sqr({ let __field = (*(*x.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*(*x.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).sqr({ let __field = (*(*x.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    }
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // a squared Rat is positive and can't be reduced (no need to call norm())
        (*self.a.lock().unwrap().as_mut().unwrap()).mul((*x.lock().unwrap().as_ref().unwrap()).a.clone(), (*y.lock().unwrap().as_ref().unwrap()).a.clone());
        { let new_val = mul_denom({ let __field = (*self.b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*(*x.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*(*y.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        self.norm()
    }

    /// Quo sets z to the quotient x/y and returns z.
    /// If y == 0, Quo panics.
    pub fn quo(&mut self, x: Arc<Mutex<Option<Rat>>>, y: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Rat>>> {
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*(*y.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("division by zero".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut a: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut b: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*a.lock().unwrap().as_mut().unwrap()).scale_denom((*x.lock().unwrap().as_ref().unwrap()).a.clone(), { let __field = (*(*y.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        (*b.lock().unwrap().as_mut().unwrap()).scale_denom((*y.lock().unwrap().as_ref().unwrap()).a.clone(), { let __field = (*(*x.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.clone(); __field });
        { let new_val = { let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = (*{ let __field = (*a.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y }; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        self.norm()
    }
}

impl crate::int::Int {
    /// scaleDenom sets z to the product x*f.
    /// If f == 0 (zero value of denominator), z is set to (a copy of) x.
    pub fn scale_denom(&mut self, x: Arc<Mutex<Option<Int>>>, f: Arc<Mutex<Option<nat>>>) {
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*f.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.set(x.clone());
        return;
    }
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).mul({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, f.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
    }
}

/// NewRat creates a new [Rat] with numerator a and denominator b.
pub fn new_rat(a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Rat>>> {
    { let __recv = Arc::new(Mutex::new(Some(Rat::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_frac64(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }
}

/// quotToFloat32 returns the non-negative float32 value
/// nearest to the quotient a/b, using round-to-even in
/// halfway cases. It does not mutate its arguments.
/// Preconditions: b is non-zero; a and b have no common factors.
pub fn quot_to_float32(a: Arc<Mutex<Option<nat>>>, b: Arc<Mutex<Option<nat>>>) -> (f32, bool) {
    let mut f: Arc<Mutex<Option<f32>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut exact: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    const Fsize: i32 = 32;
const Msize: i32 = 23;
const Msize1: i32 = Msize + 1;
const Msize2: i32 = Msize1 + 1;
const Esize: i32 = Fsize - Msize1;
const Ebias: i32 = (1 << (Esize - 1)) - 1;
const Emin: i32 = 1 - Ebias;
const Emax: i32 = Ebias;


        // float size in bits
        // mantissa
        // incl. implicit 1
        // exponent
        // TODO(adonovan): specialize common degenerate cases: 1.0, integers.
    let mut alen = (*a.lock().unwrap().as_ref().unwrap()).bit_len();
    if { let __tmp_x = alen; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (0.0_f32, true);
    }
    let mut blen = (*b.lock().unwrap().as_ref().unwrap()).bit_len();
    if { let __tmp_x = blen; let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("division by zero".to_string()) as Box<dyn Any + Send + Sync>);
    }

        // 1. Left-shift A or B such that quotient A/B is in [1<<Msize1, 1<<(Msize2+1)
        // (Msize2 bits if A < B when they are left-aligned, Msize2+1 bits if A >= B).
        // This is 2 or 3 more than the float32 mantissa field width of Msize:
        // - the optional extra bit is shifted away in step 3 below.
        // - the high-order 1 is omitted in "normal" representation;
        // - the low-order 1 will be used during rounding then discarded.
    let mut exp = Arc::new(Mutex::new(Some({ let __tmp_x = alen; let __tmp_y = blen; __tmp_x - __tmp_y })));
    let mut a2: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));let mut b2: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = (*a2.lock().unwrap().as_ref().unwrap()).set(a.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *a2.lock().unwrap() = __moved_val; };
    { let new_val = (*b2.lock().unwrap().as_ref().unwrap()).set(b.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *b2.lock().unwrap() = __moved_val; };
    {
        let mut shift = Arc::new(Mutex::new(Some({ let __tmp_x = 25; let __tmp_y = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));;
        if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            { let new_val = (*a2.lock().unwrap().as_ref().unwrap()).shl(a2.clone(), Arc::new(Mutex::new(Some((*shift.lock().unwrap().as_ref().unwrap()) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *a2.lock().unwrap() = __moved_val; };;
        } else if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = (*b2.lock().unwrap().as_ref().unwrap()).shl(b2.clone(), Arc::new(Mutex::new(Some(-((*shift.lock().unwrap().as_ref().unwrap())) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *b2.lock().unwrap() = __moved_val; };
    }
    }

        // 2. Compute quotient and remainder (q, r).  NB: due to the
        // extra shift, the low-order bit of q is logically the
        // high-order bit of r.
    let mut q: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
    let (__tmp_0, mut r) = (*q.lock().unwrap().as_ref().unwrap()).div(a2.clone(), a2.clone(), b2.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_tmp_0;;
    let mut mantissa = low32(Arc::new(Mutex::new(Some({ let __arg_holder = q.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut haveRem = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y })));

        // 3. If quotient didn't fit in Msize2 bits, redo division by b2<<1
        // (in effect---we accomplish this incrementally).
    if { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = Msize2; __tmp_x >> __tmp_y }; let __tmp_y = 1 as u32; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = 1 as u32; __tmp_x & __tmp_y }; let __tmp_y = 1 as u32; __tmp_x == __tmp_y } {
        { let new_val = true; *haveRem.lock().unwrap() = Some(new_val); };
    }
        { let __rhs = 1 as u32; mantissa = mantissa >> __rhs; };
        { let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = Msize1; __tmp_x >> __tmp_y }; let __tmp_y = 1 as u32; __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("expected exactly {} bits of result", Msize2)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }

        // 4. Rounding.
    if { let __tmp_x = -149; let __tmp_y = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -126; __tmp_x <= __tmp_y } {
                // Denormal case; lose 'shift' bits of precision.
        let mut shift = Arc::new(Mutex::new(Some(({ let __tmp_x = -126; let __tmp_y = ({ let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }) as u64)));
        let mut lostbits = Arc::new(Mutex::new(Some({ let __tmp_x = mantissa; let __tmp_y = ({ let __tmp_x = { let __tmp_x = (1 as u32); let __tmp_y = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })));
        { let new_val = { let __v = (*haveRem.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __v = (*lostbits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y }; *haveRem.lock().unwrap() = Some(new_val); };
        { let __rhs = (*shift.lock().unwrap().as_ref().unwrap()); mantissa = mantissa >> __rhs; };
        { let new_val = -125; *exp.lock().unwrap() = Some(new_val); };
    }

        // Denormal case; lose 'shift' bits of precision.
        // [1..Esize1)
        // == exp + shift
        // Round q using round-half-to-even.
    { let new_val = !{ let __v = (*haveRem.lock().unwrap().as_ref().unwrap()).clone(); __v }; *exact.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = 1 as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        { let new_val = false; *exact.lock().unwrap() = Some(new_val); };
        if { let __v = (*haveRem.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = 2 as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        {
        { mantissa += 1; };
        if { let __tmp_x = mantissa; let __tmp_y = ((1 as u32) << (Msize2 as u32)) as u32; __tmp_x >= __tmp_y } {
            { let __rhs = 1 as u32; mantissa = mantissa >> __rhs; };;
            { let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        }
    }
    }
    }
        // Complete rollover 11...1 => 100...0, so shift is safe
    { let __rhs = 1 as u32; mantissa = mantissa >> __rhs; };

    { let new_val = Arc::new(Mutex::new(Some(math::ldexp(Arc::new(Mutex::new(Some(mantissa as f64))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 24; __tmp_x - __tmp_y })))) as f32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *f.lock().unwrap() = __moved_val; };
    if math::is_inf(Arc::new(Mutex::new(Some((*f.lock().unwrap().as_ref().unwrap()) as f64))), Arc::new(Mutex::new(Some(0)))) {
        { let new_val = false; *exact.lock().unwrap() = Some(new_val); };
    }
    return ((*f.lock().unwrap().as_ref().unwrap()), (*exact.lock().unwrap().as_ref().unwrap()));
}

/// quotToFloat64 returns the non-negative float64 value
/// nearest to the quotient a/b, using round-to-even in
/// halfway cases. It does not mutate its arguments.
/// Preconditions: b is non-zero; a and b have no common factors.
pub fn quot_to_float64(a: Arc<Mutex<Option<nat>>>, b: Arc<Mutex<Option<nat>>>) -> (f64, bool) {
    let mut f: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut exact: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    const Fsize: i32 = 64;
const Msize: i32 = 52;
const Msize1: i32 = Msize + 1;
const Msize2: i32 = Msize1 + 1;
const Esize: i32 = Fsize - Msize1;
const Ebias: i32 = (1 << (Esize - 1)) - 1;
const Emin: i32 = 1 - Ebias;
const Emax: i32 = Ebias;


        // float size in bits
        // mantissa
        // incl. implicit 1
        // exponent
        // TODO(adonovan): specialize common degenerate cases: 1.0, integers.
    let mut alen = (*a.lock().unwrap().as_ref().unwrap()).bit_len();
    if { let __tmp_x = alen; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (0.0, true);
    }
    let mut blen = (*b.lock().unwrap().as_ref().unwrap()).bit_len();
    if { let __tmp_x = blen; let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("division by zero".to_string()) as Box<dyn Any + Send + Sync>);
    }

        // 1. Left-shift A or B such that quotient A/B is in [1<<Msize1, 1<<(Msize2+1)
        // (Msize2 bits if A < B when they are left-aligned, Msize2+1 bits if A >= B).
        // This is 2 or 3 more than the float64 mantissa field width of Msize:
        // - the optional extra bit is shifted away in step 3 below.
        // - the high-order 1 is omitted in "normal" representation;
        // - the low-order 1 will be used during rounding then discarded.
    let mut exp = Arc::new(Mutex::new(Some({ let __tmp_x = alen; let __tmp_y = blen; __tmp_x - __tmp_y })));
    let mut a2: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));let mut b2: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = (*a2.lock().unwrap().as_ref().unwrap()).set(a.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *a2.lock().unwrap() = __moved_val; };
    { let new_val = (*b2.lock().unwrap().as_ref().unwrap()).set(b.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *b2.lock().unwrap() = __moved_val; };
    {
        let mut shift = Arc::new(Mutex::new(Some({ let __tmp_x = 54; let __tmp_y = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));;
        if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            { let new_val = (*a2.lock().unwrap().as_ref().unwrap()).shl(a2.clone(), Arc::new(Mutex::new(Some((*shift.lock().unwrap().as_ref().unwrap()) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *a2.lock().unwrap() = __moved_val; };;
        } else if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = (*b2.lock().unwrap().as_ref().unwrap()).shl(b2.clone(), Arc::new(Mutex::new(Some(-((*shift.lock().unwrap().as_ref().unwrap())) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *b2.lock().unwrap() = __moved_val; };
    }
    }

        // 2. Compute quotient and remainder (q, r).  NB: due to the
        // extra shift, the low-order bit of q is logically the
        // high-order bit of r.
    let mut q: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
    let (__tmp_0, mut r) = (*q.lock().unwrap().as_ref().unwrap()).div(a2.clone(), a2.clone(), b2.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_tmp_0;;
    let mut mantissa = low64(Arc::new(Mutex::new(Some({ let __arg_holder = q.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut haveRem = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y })));

        // 3. If quotient didn't fit in Msize2 bits, redo division by b2<<1
        // (in effect---we accomplish this incrementally).
    if { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = Msize2; __tmp_x >> __tmp_y }; let __tmp_y = 1 as u64; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = 1 as u64; __tmp_x & __tmp_y }; let __tmp_y = 1 as u64; __tmp_x == __tmp_y } {
        { let new_val = true; *haveRem.lock().unwrap() = Some(new_val); };
    }
        { let __rhs = 1 as u64; mantissa = mantissa >> __rhs; };
        { let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = Msize1; __tmp_x >> __tmp_y }; let __tmp_y = 1 as u64; __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("expected exactly {} bits of result", Msize2)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }

        // 4. Rounding.
    if { let __tmp_x = -1074; let __tmp_y = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1022; __tmp_x <= __tmp_y } {
                // Denormal case; lose 'shift' bits of precision.
        let mut shift = Arc::new(Mutex::new(Some(({ let __tmp_x = -1022; let __tmp_y = ({ let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }) as u64)));
        let mut lostbits = Arc::new(Mutex::new(Some({ let __tmp_x = mantissa; let __tmp_y = ({ let __tmp_x = { let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })));
        { let new_val = { let __v = (*haveRem.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __v = (*lostbits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y }; *haveRem.lock().unwrap() = Some(new_val); };
        { let __rhs = (*shift.lock().unwrap().as_ref().unwrap()); mantissa = mantissa >> __rhs; };
        { let new_val = -1021; *exp.lock().unwrap() = Some(new_val); };
    }

        // Denormal case; lose 'shift' bits of precision.
        // [1..Esize1)
        // == exp + shift
        // Round q using round-half-to-even.
    { let new_val = !{ let __v = (*haveRem.lock().unwrap().as_ref().unwrap()).clone(); __v }; *exact.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = 1 as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let new_val = false; *exact.lock().unwrap() = Some(new_val); };
        if { let __v = (*haveRem.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __tmp_x = mantissa; let __tmp_y = 2 as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        {
        { mantissa += 1; };
        if { let __tmp_x = mantissa; let __tmp_y = ((1 as u64) << (Msize2 as u64)) as u64; __tmp_x >= __tmp_y } {
            { let __rhs = 1 as u64; mantissa = mantissa >> __rhs; };;
            { let mut guard = exp.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        }
    }
    }
    }
        // Complete rollover 11...1 => 100...0, so shift is safe
    { let __rhs = 1 as u64; mantissa = mantissa >> __rhs; };

    { let new_val = math::ldexp(Arc::new(Mutex::new(Some(mantissa as f64))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 53; __tmp_x - __tmp_y })))); *f.lock().unwrap() = Some(new_val); };
    if math::is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) {
        { let new_val = false; *exact.lock().unwrap() = Some(new_val); };
    }
    return ((*f.lock().unwrap().as_ref().unwrap()), (*exact.lock().unwrap().as_ref().unwrap()));
}

/// mulDenom sets z to the denominator product x*y (by taking into
/// account that 0 values for x or y must be interpreted as 1) and
/// returns z.
pub fn mul_denom(z: Arc<Mutex<Option<nat>>>, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<crate::nat::nat>>> {
    if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
            return (*z.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))))));
        } else if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
            return (*z.lock().unwrap().as_ref().unwrap()).set(y.clone());
        } else if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
            return (*z.lock().unwrap().as_ref().unwrap()).set(x.clone());
        }
    (*z.lock().unwrap().as_ref().unwrap()).mul(x.clone(), y.clone())
}

impl GoValueClone for Rat {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
