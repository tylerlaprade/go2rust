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
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// An Int represents a signed multi-precision integer.
/// The zero value for an Int represents the value 0.
///
/// Operations always take pointer arguments (*Int) rather
/// than Int values, and each unique Int value requires
/// its own unique *Int pointer. To "copy" an Int value,
/// an existing (or newly allocated) Int must be set to
/// a new value using the [Int.Set] method; shallow copies
/// of Ints are not supported and may lead to errors.
///
/// Note that methods may leak the Int's value through timing side-channels.
/// Because of this and because of the scope and complexity of the
/// implementation, Int is not well-suited to implement cryptographic operations.
/// The standard library avoids exposing non-trivial Int methods to
/// attacker-controlled inputs and the determination of whether a bug in math/big
/// is considered a security vulnerability might depend on the impact on the
/// standard library.
#[derive(Debug, Clone)]
pub struct Int {
    pub neg: Arc<Mutex<Option<bool>>>,
    pub abs: Arc<Mutex<Option<nat>>>,
}

impl Int {
    pub fn __go_value_clone(&self) -> Self {
        Self { neg: { let __guard = self.neg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, abs: self.abs.clone() }
    }
}


impl Default for Int {
    fn default() -> Self {
        Self { neg: Arc::new(Mutex::new(Some(false))), abs: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Int {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static intOne: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Int>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *intOne.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *intOne.lock().unwrap() = Some(Arc::new(Mutex::new(Some(Int { neg: Arc::new(Mutex::new(Some(false))), abs: natOne.clone(), ..Default::default() }))));
}


pub(crate) fn __go_zero_globals() {
    *intOne.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_init_order_5() {
    *intOne.lock().unwrap() = Some(Arc::new(Mutex::new(Some(Int { neg: Arc::new(Mutex::new(Some(false))), abs: natOne.clone(), ..Default::default() }))));
}


impl Int {
    /// Sign returns:
    ///   - -1 if x < 0;
    ///   - 0 if x == 0;
    ///   - +1 if x > 0.
    pub fn sign(&self) -> i32 {
                // This function is used in cryptographic operations. It must not leak
                // anything but the Int's sign and bit size through side-channels. Any
                // changes must be reviewed by a security expert.
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return -(1);
    }
        1
    }

    /// SetInt64 sets z to x and returns z.
    pub fn set_int64(&mut self, mut x: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Int>>> {
        let mut neg = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = true; *neg.lock().unwrap() = Some(new_val); };
        { let new_val = -((*x.lock().unwrap().as_ref().unwrap())); *x.lock().unwrap() = Some(new_val); };
    }
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).set_uint64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = neg.lock().unwrap().as_ref().unwrap().clone(); *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// SetUint64 sets z to x and returns z.
    pub fn set_uint64(&mut self, x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Int>>> {
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).set_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Set sets z to x and returns z.
    pub fn set(&mut self, x: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __peer = x.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).set({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Bits provides raw (unchecked but fast) access to x by returning its
    /// absolute value as a little-endian [Word] slice. The result and x share
    /// the same underlying array.
    /// Bits is intended to support implementation of missing low-level [Int]
    /// functionality outside this package; it should be avoided otherwise.
    pub fn bits(&self) -> Arc<Mutex<Option<Vec<crate::arith::Word>>>> {
                // This function is used in cryptographic operations. It must not leak
                // anything but the Int's sign and bit size through side-channels. Any
                // changes must be reviewed by a security expert.
        return { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice };
    }

    /// SetBits provides raw (unchecked but fast) access to z by setting its
    /// value to abs, interpreted as a little-endian [Word] slice, and returning
    /// z. The result and abs share the same underlying array.
    /// SetBits is intended to support implementation of missing low-level [Int]
    /// functionality outside this package; it should be avoided otherwise.
    pub fn set_bits(&mut self, abs: Arc<Mutex<Option<Vec<Word>>>>) -> Arc<Mutex<Option<Int>>> {
        { let new_val = crate::nat::nat(abs.clone()).norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Abs sets z to |x| (the absolute value of x) and returns z.
    pub fn abs(&mut self, x: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        self.set(x.clone());
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Neg sets z to -x and returns z.
    pub fn neg(&mut self, x: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        self.set(x.clone());
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && !((*self.neg.clone().lock().unwrap().as_ref().unwrap())); *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Add sets z to the sum x+y and returns z.
    pub fn add(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        let mut neg = Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // x + y == x + y
                // (-x) + (-y) == -(x + y)
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).add({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
    } else {
                // x + (-y) == x - y == -(y - x)
                // (-x) + y == y - x == -(x - y)
        if { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).cmp({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = !{ let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v }; *neg.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
    }
    }
                // x + y == x + y
                // (-x) + (-y) == -(x + y)
                // x + (-y) == x - y == -(y - x)
                // (-x) + y == y - x == -(x - y)
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v }; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Sub sets z to the difference x-y and returns z.
    pub fn sub(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        let mut neg = Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
                // x - (-y) == x + y
                // (-x) - y == -(x + y)
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).add({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
    } else {
                // x - y == x - y == -(y - x)
                // (-x) - (-y) == y - x == -(x - y)
        if { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).cmp({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = !{ let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v }; *neg.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
    }
    }
                // x - (-y) == x + y
                // (-x) - y == -(x + y)
                // x - y == x - y == -(y - x)
                // (-x) - (-y) == y - x == -(x - y)
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __v = (*neg.lock().unwrap().as_ref().unwrap()).clone(); __v }; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Mul sets z to the product x*y and returns z.
    pub fn mul(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
                // x * y == x * y
                // x * (-y) == -(x * y)
                // (-x) * y == -(x * y)
                // (-x) * (-y) == x * y
        if { let __left = x.clone(); let __right = y.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).sqr({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).mul({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y }; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// MulRange sets z to the product of all integers
    /// in the range [a, b] inclusively and returns z.
    /// If a > b (empty range), the result is 1.
    pub fn mul_range(&mut self, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            return self.set_int64(Arc::new(Mutex::new(Some(1 as i64))));
        } else if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
            return self.set_int64(Arc::new(Mutex::new(Some(0 as i64))));
        }
                // empty range
                // range includes 0
                // a <= b && (b < 0 || a > 0)
        let mut neg = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = 1 as i64; __tmp_x & __tmp_y }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y }; *neg.lock().unwrap() = Some(new_val); };
        { let __tmp_0 = -((*b.lock().unwrap().as_ref().unwrap())); let __tmp_1 = -((*a.lock().unwrap().as_ref().unwrap())); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    }
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).mul_range(Arc::new(Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = neg.lock().unwrap().as_ref().unwrap().clone(); *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Binomial sets z to the binomial coefficient C(n, k) and returns z.
    pub fn binomial(&mut self, n: Arc<Mutex<Option<i64>>>, mut k: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return self.set_int64(Arc::new(Mutex::new(Some(0 as i64))));
    }
                // reduce the number of multiplications by reducing k
        if { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *k.lock().unwrap() = Some(new_val); };
    }
                // C(n, k) == C(n, n-k)
                // C(n, k) == n * (n-1) * ... * (n-k+1) / k * (k-1) * ... * 1
                //         == n * (n-1) * ... * (n-k+1) / 1 * (1+1) * ... * k
                //
                // Using the multiplicative formula produces smaller values
                // at each step, requiring fewer allocations and computations:
                //
                // z = 1
                // for i := 0; i < k; i = i+1 {
                //     z *= n-i
                //     z /= i+1
                // }
                //
                // finally to avoid computing i+1 twice per loop:
                //
                // z = 1
                // i := 0
                // for i < k {
                //     z *= n-i
                //     i++
                //     z /= i
                // }
        let mut N: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut K: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut i: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut t: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*N.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*K.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.set({ let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        while { let __tmp_x = (*i.lock().unwrap().as_ref().unwrap()).cmp(K.clone()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = (*t.lock().unwrap().as_mut().unwrap()).sub(N.clone(), i.clone()); self.mul(__method_arg0, __method_arg1) };
        (*i.lock().unwrap().as_mut().unwrap()).add(i.clone(), { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = i.clone(); self.quo(__method_arg0, __method_arg1) };
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Quo sets z to the quotient x/y for y != 0 and returns z.
    /// If y == 0, a division-by-zero run-time panic occurs.
    /// Quo implements truncated division (like Go); see [Int.QuoRem] for more details.
    pub fn quo(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        { let (__tmp_0, __tmp_1) = (*self.abs.lock().unwrap().as_ref().unwrap()).div(Arc::new(Mutex::new(None)), { let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_tmp_0; };
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y }; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Rem sets z to the remainder x%y for y != 0 and returns z.
    /// If y == 0, a division-by-zero run-time panic occurs.
    /// Rem implements truncated modulus (like Go); see [Int.QuoRem] for more details.
    pub fn rem(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        { let (__tmp_0, __tmp_1) = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).div({ let __field = self.abs.clone(); __field }, { let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_tmp_1; };
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// QuoRem sets z to the quotient x/y and r to the remainder x%y
    /// and returns the pair (z, r) for y != 0.
    /// If y == 0, a division-by-zero run-time panic occurs.
    ///
    /// QuoRem implements T-division and modulus (like Go):
    ///
    ///	q = x/y      with the result truncated to zero
    ///	r = x - y*q
    ///
    /// (See Daan Leijen, “Division and Modulus for Computer Scientists”.)
    /// See [Int.DivMod] for Euclidean division and modulus (unlike Go).
    pub fn quo_rem(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>, r: Arc<Mutex<Option<Int>>>) -> (Arc<Mutex<Option<Int>>>, Arc<Mutex<Option<Int>>>) {
        { let (__tmp_0, __tmp_1) = (*self.abs.lock().unwrap().as_ref().unwrap()).div({ let __field = (*r.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *(*r.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_tmp_1; };
        { let __tmp_0 = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y }; let __tmp_1 = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*r.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); *self.neg.lock().unwrap() = Some(__tmp_0); *(*r.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(__tmp_1); };
        (Arc::new(Mutex::new(Some(self.clone()))), r.clone())
    }

    /// Div sets z to the quotient x/y for y != 0 and returns z.
    /// If y == 0, a division-by-zero run-time panic occurs.
    /// Div implements Euclidean division (unlike Go); see [Int.DivMod] for more details.
    pub fn div(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        let mut y_neg = Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut r: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.quo_rem(x.clone(), y.clone(), r.clone());
        if (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if { let __v = (*y_neg.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }; self.add(__method_arg0, __method_arg1) };
    } else {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }; self.sub(__method_arg0, __method_arg1) };
    }
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Mod sets z to the modulus x%y for y != 0 and returns z.
    /// If y == 0, a division-by-zero run-time panic occurs.
    /// Mod implements Euclidean modulus (unlike Go); see [Int.DivMod] for more details.
    pub fn r#mod(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        let mut y0 = y.clone();
        if { let __peer = y.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq } || alias({ let __field = self.abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }) {
        { let new_val = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set(y.clone()); __result }.clone(); y0 = new_val; };
    }
        let mut q: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*q.lock().unwrap().as_mut().unwrap()).quo_rem(x.clone(), y.clone(), Arc::new(Mutex::new(Some(self.clone()))));
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        if (*{ let __field = (*y0.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = y0.clone(); self.sub(__method_arg0, __method_arg1) };
    } else {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = y0.clone(); self.add(__method_arg0, __method_arg1) };
    }
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// DivMod sets z to the quotient x div y and m to the modulus x mod y
    /// and returns the pair (z, m) for y != 0.
    /// If y == 0, a division-by-zero run-time panic occurs.
    ///
    /// DivMod implements Euclidean division and modulus (unlike Go):
    ///
    ///	q = x div y  such that
    ///	m = x - y*q  with 0 <= m < |y|
    ///
    /// (See Raymond T. Boute, “The Euclidean definition of the functions
    /// div and mod”. ACM Transactions on Programming Languages and
    /// Systems (TOPLAS), 14(2):127-144, New York, NY, USA, 4/1992.
    /// ACM press.)
    /// See [Int.QuoRem] for T-division and modulus (like Go).
    pub fn div_mod(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>, m: Arc<Mutex<Option<Int>>>) -> (Arc<Mutex<Option<Int>>>, Arc<Mutex<Option<Int>>>) {
        let mut y0 = y.clone();
        if { let __peer = y.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq } || alias({ let __field = self.abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }) {
        { let new_val = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set(y.clone()); __result }.clone(); y0 = new_val; };
    }
        self.quo_rem(x.clone(), y.clone(), m.clone());
        if (*{ let __field = (*m.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if (*{ let __field = (*y0.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }; self.add(__method_arg0, __method_arg1) };
        { let __recv = m.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.sub(m.clone(), y0.clone()); __result };
    } else {
        { let __method_arg0 = Arc::new(Mutex::new(Some(self.clone()))); let __method_arg1 = { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }; self.sub(__method_arg0, __method_arg1) };
        { let __recv = m.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.add(m.clone(), y0.clone()); __result };
    }
    }
        (Arc::new(Mutex::new(Some(self.clone()))), m.clone())
    }

    /// Cmp compares x and y and returns:
    ///   - -1 if x < y;
    ///   - 0 if x == y;
    ///   - +1 if x > y.
    pub fn cmp(&self, y: Arc<Mutex<Option<Int>>>) -> i32 {
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

                // x cmp y == x cmp y
                // x cmp (-y) == x
                // (-x) cmp y == y
                // (-x) cmp (-y) == -(x cmp y)
        if { let __peer = y.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq } {
        } else if { let __tmp_x = (*self.neg.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
            { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).cmp({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); *r.lock().unwrap() = Some(new_val); };
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = -((*r.lock().unwrap().as_ref().unwrap())); *r.lock().unwrap() = Some(new_val); };
    }
        } else if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
            { let new_val = -1; *r.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = 1; *r.lock().unwrap() = Some(new_val); };
        }
                // nothing to do
        return (*r.lock().unwrap().as_ref().unwrap());
    }

    /// CmpAbs compares the absolute values of x and y and returns:
    ///   - -1 if |x| < |y|;
    ///   - 0 if |x| == |y|;
    ///   - +1 if |x| > |y|.
    pub fn cmp_abs(&self, y: Arc<Mutex<Option<Int>>>) -> i32 {
        (*self.abs.lock().unwrap().as_ref().unwrap()).cmp({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field })
    }

    /// Int64 returns the int64 representation of x.
    /// If x cannot be represented in an int64, the result is undefined.
    pub fn int64(&self) -> i64 {
        let mut v = Arc::new(Mutex::new(Some(low64({ let __field = self.abs.clone(); __field }) as i64)));
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = -((*v.lock().unwrap().as_ref().unwrap())); *v.lock().unwrap() = Some(new_val); };
    }
        return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// Uint64 returns the uint64 representation of x.
    /// If x cannot be represented in a uint64, the result is undefined.
    pub fn uint64(&self) -> u64 {
        low64({ let __field = self.abs.clone(); __field })
    }

    /// IsInt64 reports whether x can be represented as an int64.
    pub fn is_int64(&self) -> bool {
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x <= __tmp_y } {
        let mut w = Arc::new(Mutex::new(Some(low64({ let __field = self.abs.clone(); __field }) as i64)));
        return { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } || (*self.neg.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -((*w.lock().unwrap().as_ref().unwrap())); __tmp_x == __tmp_y };
    }
        false
    }

    /// IsUint64 reports whether x can be represented as a uint64.
    pub fn is_uint64(&self) -> bool {
        !((*self.neg.clone().lock().unwrap().as_ref().unwrap())) && { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x <= __tmp_y }
    }

    /// Float64 returns the float64 value nearest x,
    /// and an indication of any rounding that occurred.
    pub fn float64(&self) -> (f64, Arc<Mutex<Option<crate::float::Accuracy>>>) {
        let mut n = (*self.abs.lock().unwrap().as_ref().unwrap()).bit_len();
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (0.0, Arc::new(Mutex::new(Some(crate::float::Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
    }
                // Fast path: no more than 53 significant bits.
        if { let __tmp_x = n; let __tmp_y = 53; __tmp_x <= __tmp_y } || { let __tmp_x = n; let __tmp_y = 64; __tmp_x < __tmp_y } && { let __tmp_x = { let __tmp_x = n; let __tmp_y = (*Arc::new(Mutex::new(Some((*self.abs.lock().unwrap().as_ref().unwrap()).trailing_zero_bits() as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = 53; __tmp_x <= __tmp_y } {
        let mut f = Arc::new(Mutex::new(Some(low64({ let __field = self.abs.clone(); __field }) as f64)));
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = -((*f.lock().unwrap().as_ref().unwrap())); *f.lock().unwrap() = Some(new_val); };
    }
        return ({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(crate::float::Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
    }
        { let __recv = { let __recv = Arc::new(Mutex::new(Some(Float::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int(Arc::new(Mutex::new(Some(self.clone())))); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).float64(); __result }
    }

    /// SetString sets z to the value of s, interpreted in the given base,
    /// and returns z and a boolean indicating success. The entire string
    /// (not just a prefix) must be valid for success. If SetString fails,
    /// the value of z is undefined but the returned value is nil.
    ///
    /// The base argument must be 0 or a value between 2 and [MaxBase].
    /// For base 0, the number prefix determines the actual base: A prefix of
    /// “0b” or “0B” selects base 2, “0”, “0o” or “0O” selects base 8,
    /// and “0x” or “0X” selects base 16. Otherwise, the selected base is 10
    /// and no prefix is accepted.
    ///
    /// For bases <= 36, lower and upper case letters are considered the same:
    /// The letters 'a' to 'z' and 'A' to 'Z' represent digit values 10 to 35.
    /// For bases > 36, the upper case letters 'A' to 'Z' represent the digit
    /// values 36 to 61.
    ///
    /// For base 0, an underscore character “_” may appear between a base
    /// prefix and an adjacent digit, and between successive digits; such
    /// underscores do not change the value of the number.
    /// Incorrect placement of underscores is reported as an error if there
    /// are no other errors. If base != 0, underscores are not recognized
    /// and act like any other character that is not a valid digit.
    pub fn set_string(&mut self, s: Arc<Mutex<Option<String>>>, base: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Int>>>, bool) {
        self.set_from_scanner({ let __arg = strings::new_reader({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<io_ByteScanner> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// setFromScanner implements SetString given an io.ByteScanner.
    /// For documentation see comments of SetString.
    pub fn set_from_scanner(&mut self, r: Arc<Mutex<Option<io_ByteScanner>>>, base: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Int>>>, bool) {
        {
        let (_, _, mut err) = self.scan_1(r.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if (*err.lock().unwrap()).is_some() {
            return (Arc::new(Mutex::new(None)), false);;
        }
    }
                // entire content must have been consumed
        {
        let (_, mut err) = (*r.lock().unwrap().as_ref().unwrap()).read_byte();;
        if { let __left = err.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; !__eq } {
            return (Arc::new(Mutex::new(None)), false);;
        }
    }
        (Arc::new(Mutex::new(Some(self.clone()))), true)
    }

    /// SetBytes interprets buf as the bytes of a big-endian unsigned
    /// integer, sets z to that value, and returns z.
    pub fn set_bytes(&mut self, buf: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Int>>> {
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).set_bytes(buf.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Bytes returns the absolute value of x as a big-endian byte slice.
    ///
    /// To use a fixed length slice, or a preallocated one, use [Int.FillBytes].
    pub fn bytes(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
                // This function is used in cryptographic operations. It must not leak
                // anything but the Int's sign and bit size through side-channels. Any
                // changes must be reviewed by a security expert.
        let mut buf = Arc::new(Mutex::new(Some(vec![0; ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 8; __tmp_x * __tmp_y }) as usize])));
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = ((*self.abs.lock().unwrap().as_ref().unwrap()).bytes(buf.clone())) as usize; __seq[__low..].to_vec() })));
    }

    /// FillBytes sets buf to the absolute value of x, storing it as a zero-extended
    /// big-endian byte slice, and returns buf.
    ///
    /// If the absolute value of x doesn't fit in buf, FillBytes will panic.
    pub fn fill_bytes(&self, buf: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
                // Clear whole buffer.
        { let __clear_holder = buf.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = 0; } } };
        (*self.abs.lock().unwrap().as_ref().unwrap()).bytes(buf.clone());
        return buf.clone();
    }

    /// BitLen returns the length of the absolute value of x in bits.
    /// The bit length of 0 is 0.
    pub fn bit_len(&self) -> i32 {
                // This function is used in cryptographic operations. It must not leak
                // anything but the Int's sign and bit size through side-channels. Any
                // changes must be reviewed by a security expert.
        (*self.abs.lock().unwrap().as_ref().unwrap()).bit_len()
    }

    /// TrailingZeroBits returns the number of consecutive least significant zero
    /// bits of |x|.
    pub fn trailing_zero_bits(&self) -> u64 {
        (*self.abs.lock().unwrap().as_ref().unwrap()).trailing_zero_bits()
    }

    /// Exp sets z = x**y mod |m| (i.e. the sign of m is ignored), and returns z.
    /// If m == nil or m == 0, z = x**y unless y <= 0 then z = 1. If m != 0, y < 0,
    /// and x and m are not relatively prime, z is unchanged and nil is returned.
    ///
    /// Modular exponentiation of inputs of a particular size is not a
    /// cryptographically constant-time operation.
    pub fn exp(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>, m: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        self.exp_1(x.clone(), y.clone(), m.clone(), Arc::new(Mutex::new(Some(false))))
    }

    pub fn exp_slow(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>, m: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        self.exp_1(x.clone(), y.clone(), m.clone(), Arc::new(Mutex::new(Some(true))))
    }

    pub fn exp_1(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>, mut m: Arc<Mutex<Option<Int>>>, slow: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Int>>> {
                // See Knuth, volume 2, section 4.6.3.
        let mut xWords = Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if (*m.lock().unwrap()).is_none() || { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*m.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return self.set_int64(Arc::new(Mutex::new(Some(1 as i64))));
    }
                // for y < 0: x**y mod m == (x**(-1))**|y| mod m
        let mut inverse = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).mod_inverse(x.clone(), m.clone()); __result };
        if (*inverse.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(None));
    }
        { let new_val = { let __selector_holder = (*inverse.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *xWords.lock().unwrap() = Some(new_val); };
    }
                // for y < 0: x**y mod m == (x**(-1))**|y| mod m
        let mut yWords = Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut mWords: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        if (*m.lock().unwrap()).is_some() {
        if { let __peer = m.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq } || alias({ let __field = self.abs.clone(); __field }, { let __field = (*m.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }) {
        { let new_val = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set(m.clone()); __result }.clone(); m = new_val; };
    }
        { let new_val = { let __selector_holder = (*m.lock().unwrap().as_ref().unwrap()).abs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *mWords.lock().unwrap() = Some(new_val); };
    }
                // m.abs may be nil for m == 0
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).exp_n_n(xWords.clone(), yWords.clone(), mWords.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = slow.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*yWords.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*yWords.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 1))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x == __tmp_y }; *self.neg.lock().unwrap() = Some(new_val); };
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*mWords.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // make modulus result positive
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).sub(mWords.clone(), { let __field = self.abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
    }
                // make modulus result positive
                // z == x**y mod |m| && 0 <= z < |m|
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// GCD sets z to the greatest common divisor of a and b and returns z.
    /// If x or y are not nil, GCD sets their value such that z = a*x + b*y.
    ///
    /// a and b may be positive, zero or negative. (Before Go 1.14 both had
    /// to be > 0.) Regardless of the signs of a and b, z is always >= 0.
    ///
    /// If a == b == 0, GCD sets z = x = y = 0.
    ///
    /// If a == 0 and b != 0, GCD sets z = |b|, x = 0, y = sign(b) * 1.
    ///
    /// If a != 0 and b == 0, GCD sets z = |a|, x = sign(a) * 1, y = 0.
    pub fn g_c_d(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>, a: Arc<Mutex<Option<Int>>>, b: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        let (mut lenA, mut lenB, mut negA, mut negB) = (Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32))), Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32))), Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __tmp_x = { let __v = (*lenA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.set(b.clone());
    } else {
        self.set(a.clone());
    }
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        if (*x.lock().unwrap()).is_some() {
        if { let __tmp_x = { let __v = (*lenA.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __recv = x.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.set_uint64(Arc::new(Mutex::new(Some(0 as u64)))); __result };
    } else {
        { let __recv = x.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.set_uint64(Arc::new(Mutex::new(Some(1 as u64)))); __result };
        { let new_val = negA.lock().unwrap().as_ref().unwrap().clone(); *(*x.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
    }
    }
        if (*y.lock().unwrap()).is_some() {
        if { let __tmp_x = { let __v = (*lenB.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __recv = y.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.set_uint64(Arc::new(Mutex::new(Some(0 as u64)))); __result };
    } else {
        { let __recv = y.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.set_uint64(Arc::new(Mutex::new(Some(1 as u64)))); __result };
        { let new_val = negB.lock().unwrap().as_ref().unwrap().clone(); *(*y.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
    }
    }
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        self.lehmer_g_c_d(x.clone(), y.clone(), a.clone(), b.clone())
    }

    /// lehmerGCD sets z to the greatest common divisor of a and b,
    /// which both must be != 0, and returns z.
    /// If x or y are not nil, their values are set such that z = a*x + b*y.
    /// See Knuth, The Art of Computer Programming, Vol. 2, Section 4.5.2, Algorithm L.
    /// This implementation uses the improved condition by Collins requiring only one
    /// quotient and avoiding the possibility of single Word overflow.
    /// See Jebelean, "Improving the multiprecision Euclidean algorithm",
    /// Design and Implementation of Symbolic Computation Systems, pp 45-58.
    /// The cosequences are updated according to Algorithm 10.45 from
    /// Cohen et al. "Handbook of Elliptic and Hyperelliptic Curve Cryptography" pp 192.
    pub fn lehmer_g_c_d(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>, a: Arc<Mutex<Option<Int>>>, b: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        let mut A: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(None));let mut B: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(None));let mut Ua: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(None));let mut Ub: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(None));
        { let new_val = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).abs(a.clone()); __result }.clone(); A = new_val; };
        { let new_val = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).abs(b.clone()); __result }.clone(); B = new_val; };
        let mut extended = Arc::new(Mutex::new(Some((*x.lock().unwrap()).is_some() || (*y.lock().unwrap()).is_some())));
        if { let __v = (*extended.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Ua (Ub) tracks how many times input a has been accumulated into A (B).
        { let new_val = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some(1 as i64)))); __result }.clone(); Ua = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(Int::default()))).clone(); Ub = new_val; };
    }
                // Ua (Ub) tracks how many times input a has been accumulated into A (B).
                // temp variables for multiprecision update
        let mut q = Arc::new(Mutex::new(Some(Int::default())));
        let mut r = Arc::new(Mutex::new(Some(Int::default())));
        let mut s = Arc::new(Mutex::new(Some(Int::default())));
        let mut t = Arc::new(Mutex::new(Some(Int::default())));
                // ensure A >= B
        if { let __tmp_x = (*(*A.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).cmp({ let __field = (*B.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let __tmp_0 = B.clone(); let __tmp_1 = A.clone(); A = __tmp_0.clone(); B = __tmp_1.clone(); };
        { let __tmp_0 = Ua.clone(); let __tmp_1 = Ub.clone(); Ub = __tmp_0.clone(); Ua = __tmp_1.clone(); };
    }
                // loop invariant A >= B
        while { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*B.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
                // Attempt to calculate in single-precision using leading words of A and B.
        let (mut u0, mut u1, mut v0, mut v1, mut even) = lehmer_simulate(A.clone(), B.clone());

                // multiprecision Step
        if { let __tmp_x = (*v0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
                // Simulate the effect of the single-precision steps using the cosequences.
                // A = u0*A + v0*B
                // B = u1*A + v1*B
        lehmer_update(A.clone(), B.clone(), q.clone(), r.clone(), s.clone(), t.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = u0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = u1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(even))));
        if { let __v = (*extended.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Ua = u0*Ua + v0*Ub
                // Ub = u1*Ua + v1*Ub
        lehmer_update(Ua.clone(), Ub.clone(), q.clone(), r.clone(), s.clone(), t.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = u0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = u1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(even))));
    }
    } else {
                // Single-digit calculations failed to simulate any quotients.
                // Do a standard Euclidean step.
        euclid_update(A.clone(), B.clone(), Ua.clone(), Ub.clone(), q.clone(), r.clone(), s.clone(), t.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = extended.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
                // Attempt to calculate in single-precision using leading words of A and B.
                // multiprecision Step
                // Simulate the effect of the single-precision steps using the cosequences.
                // A = u0*A + v0*B
                // B = u1*A + v1*B
                // Ua = u0*Ua + v0*Ub
                // Ub = u1*Ua + v1*Ub
                // Single-digit calculations failed to simulate any quotients.
                // Do a standard Euclidean step.
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*B.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // extended Euclidean algorithm base case if B is a single Word
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*A.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
                // A is longer than a single Word, so one update is needed.
        euclid_update(A.clone(), B.clone(), Ua.clone(), Ub.clone(), q.clone(), r.clone(), s.clone(), t.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = extended.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // A is longer than a single Word, so one update is needed.
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*B.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // A and B are both a single Word.
        let (mut aWord, mut bWord) = (Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*(*A.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*(*B.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))));
        if { let __v = (*extended.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut ua: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));let mut ub: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));let mut va: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));let mut vb: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
        { let __tmp_0 = 1; let __tmp_1 = 0; *ua.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_0 as u64))))); *ub.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_1 as u64))))); };
        { let __tmp_0 = 0; let __tmp_1 = 1; *va.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_0 as u64))))); *vb.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_1 as u64))))); };
        let mut even = Arc::new(Mutex::new(Some(true)));
        while { let __tmp_x = (*bWord.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        let (mut q, mut r) = (Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*aWord.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) / (*{ let __v = (*bWord.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))))))))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*aWord.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) % (*{ let __v = (*bWord.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))))))))));
        { let __tmp_0 = (*bWord.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = (*r.lock().unwrap().as_ref().unwrap()).clone(); *aWord.lock().unwrap() = Some(__tmp_0); *bWord.lock().unwrap() = Some(__tmp_1); };
        { let __tmp_0 = (*ub.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = { let __tmp_x = (*ua.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __tmp_x = (*q.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ub.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *ua.lock().unwrap() = Some(__tmp_0); *ub.lock().unwrap() = Some(__tmp_1); };
        { let __tmp_0 = (*vb.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = { let __tmp_x = (*va.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __tmp_x = (*q.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*vb.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *va.lock().unwrap() = Some(__tmp_0); *vb.lock().unwrap() = Some(__tmp_1); };
        { let new_val = !{ let __v = (*even.lock().unwrap().as_ref().unwrap()).clone(); __v }; *even.lock().unwrap() = Some(new_val); };
    }
        { let new_val = (*(*t.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = ua.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*t.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        { let new_val = (*(*s.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = va.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*s.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        { let new_val = !{ let __v = (*even.lock().unwrap().as_ref().unwrap()).clone(); __v }; *(*t.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        { let new_val = even.lock().unwrap().as_ref().unwrap().clone(); *(*s.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        { let __recv = t.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(Ua.clone(), t.clone()); __result };
        { let __recv = s.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(Ub.clone(), s.clone()); __result };
        { let __recv = Ua.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.add(t.clone(), s.clone()); __result };
    } else {
        while { let __tmp_x = (*bWord.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let __tmp_0 = (*bWord.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = { let __tmp_x = (*aWord.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*bWord.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x % __tmp_y }; *aWord.lock().unwrap() = Some(__tmp_0); *bWord.lock().unwrap() = Some(__tmp_1); };
    }
    }
        (*{ let __named_slice = (*(*A.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[(0) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __v = (*aWord.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
    }
    }
                // extended Euclidean algorithm base case if B is a single Word
                // A is longer than a single Word, so one update is needed.
                // A and B are both a single Word.
        let mut negA = Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*y.lock().unwrap()).is_some() {
                // avoid aliasing b needed in the division below
        if { let __left = y.clone(); let __right = b.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let __recv = B.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.set(b.clone()); __result };
    } else {
        { let new_val = b.clone(); B = new_val; };
    }
                // y = (z - a*x)/b
        { let __recv = y.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(a.clone(), Ua.clone()); __result };
        if { let __v = (*negA.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = !(*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); *(*y.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
    }
        { let __recv = y.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.sub(A.clone(), y.clone()); __result };
        { let __recv = y.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.div(y.clone(), B.clone()); __result };
    }
                // avoid aliasing b needed in the division below
                // y = (z - a*x)/b
                // y can safely alias a
        if (*x.lock().unwrap()).is_some() {
        { let new_val = { let __v = (*Ua.lock().unwrap().as_ref().unwrap()).clone(); __v }; *x.lock().unwrap() = Some(new_val); };
        if { let __v = (*negA.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = !(*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); *(*x.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
    }
    }
        { let new_val = { let __v = (*A.lock().unwrap().as_ref().unwrap()).clone(); __v }; *self = new_val; };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Rand sets z to a pseudo-random number in [0, n) and returns z.
    ///
    /// As this uses the [math/rand] package, it must not be used for
    /// security-sensitive work. Use [crypto/rand.Int] instead.
    pub fn rand(&mut self, rnd: Arc<Mutex<Option<rand_Rand>>>, n: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
                // z.neg is not modified before the if check, because z and n might alias.
        if (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*n.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        *self.abs.lock().unwrap() = None;
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).random(rnd.clone(), { let __field = (*n.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some((*(*n.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).bit_len())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// ModInverse sets z to the multiplicative inverse of g in the ring ℤ/nℤ
    /// and returns z. If g and n are not relatively prime, g has no multiplicative
    /// inverse in the ring ℤ/nℤ.  In this case, z is unchanged and the return value
    /// is nil. If n == 0, a division-by-zero run-time panic occurs.
    pub fn mod_inverse(&mut self, mut g: Arc<Mutex<Option<Int>>>, mut n: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
                // GCD expects parameters a and b to be > 0.
        if (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        let mut n2: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = (*n2.lock().unwrap().as_mut().unwrap()).neg(n.clone()).clone(); n = new_val; };
    }
        if (*{ let __field = (*g.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        let mut g2: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = (*g2.lock().unwrap().as_mut().unwrap()).r#mod(g.clone(), n.clone()).clone(); g = new_val; };
    }
        let mut d: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut x: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*d.lock().unwrap().as_mut().unwrap()).g_c_d(x.clone(), Arc::new(Mutex::new(None)), g.clone(), n.clone());
                // if and only if d==1, g and n are relatively prime
        if { let __tmp_x = (*d.lock().unwrap().as_ref().unwrap()).cmp({ let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __tmp_y = 0; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
                // x and y are such that g*x + n*y = 1, therefore x is the inverse element,
                // but it may be negative, so convert to the range 0 <= z < |n|
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        self.add(x.clone(), n.clone());
    } else {
        self.set(x.clone());
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// modSqrt3Mod4 uses the identity
    ///
    ///	   (a^((p+1)/4))^2  mod p
    ///	== u^(p+1)          mod p
    ///	== u^2              mod p
    ///
    /// to calculate the square root of any quadratic residue mod p quickly for 3
    /// mod 4 primes.
    pub fn mod_sqrt3_mod4_prime(&mut self, x: Arc<Mutex<Option<Int>>>, p: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        let mut e = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).add(p.clone(), { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); __result };
        { let __recv = e.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.rsh(e.clone(), Arc::new(Mutex::new(Some(2 as u64)))); __result };
        self.exp(x.clone(), e.clone(), p.clone());
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// modSqrt5Mod8Prime uses Atkin's observation that 2 is not a square mod p
    ///
    ///	alpha ==  (2*a)^((p-5)/8)    mod p
    ///	beta  ==  2*a*alpha^2        mod p  is a square root of -1
    ///	b     ==  a*alpha*(beta-1)   mod p  is a square root of a
    ///
    /// to calculate the square root of any quadratic residue mod p quickly for 5
    /// mod 8 primes.
    pub fn mod_sqrt5_mod8_prime(&mut self, x: Arc<Mutex<Option<Int>>>, p: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
                // p == 5 mod 8 implies p = e*8 + 5
                // e is the quotient and 5 the remainder on division by 8
        let mut e = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).rsh(p.clone(), Arc::new(Mutex::new(Some(3 as u64)))); __result };
        let mut tx = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).lsh(x.clone(), Arc::new(Mutex::new(Some(1 as u64)))); __result };
        let mut alpha = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).exp(tx.clone(), e.clone(), p.clone()); __result };
        let mut beta = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).mul(alpha.clone(), alpha.clone()); __result };
        { let __recv = beta.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.r#mod(beta.clone(), p.clone()); __result };
        { let __recv = beta.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(beta.clone(), tx.clone()); __result };
        { let __recv = beta.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.r#mod(beta.clone(), p.clone()); __result };
        { let __recv = beta.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.sub(beta.clone(), { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); __result };
        { let __recv = beta.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(beta.clone(), x.clone()); __result };
        { let __recv = beta.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.r#mod(beta.clone(), p.clone()); __result };
        { let __recv = beta.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(beta.clone(), alpha.clone()); __result };
        self.r#mod(beta.clone(), p.clone());
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// modSqrtTonelliShanks uses the Tonelli-Shanks algorithm to find the square
    /// root of a quadratic residue modulo any prime.
    pub fn mod_sqrt_tonelli_shanks(&mut self, x: Arc<Mutex<Option<Int>>>, p: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
                // Break p-1 into s*2^e such that s is odd.
        let mut s: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*s.lock().unwrap().as_mut().unwrap()).sub(p.clone(), { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        let mut e = (*(*s.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).trailing_zero_bits();
        (*s.lock().unwrap().as_mut().unwrap()).rsh(s.clone(), Arc::new(Mutex::new(Some(e))));
                // find some non-square n
        let mut n: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*n.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some(2 as i64))));
        while { let __tmp_x = jacobi(n.clone(), p.clone()); let __tmp_y = -1; __tmp_x != __tmp_y } {
        (*n.lock().unwrap().as_mut().unwrap()).add(n.clone(), { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    }
                // Core of the Tonelli-Shanks algorithm. Follows the description in
                // section 6 of "Square roots from 1; 24, 51, 10 to Dan Shanks" by Ezra
                // Brown:
                // https://www.maa.org/sites/default/files/pdf/upload_library/22/Polya/07468342.di020786.02p0470a.pdf
        let mut y: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut b: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut g: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut t: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*y.lock().unwrap().as_mut().unwrap()).add(s.clone(), { let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        (*y.lock().unwrap().as_mut().unwrap()).rsh(y.clone(), Arc::new(Mutex::new(Some(1 as u64))));
        (*y.lock().unwrap().as_mut().unwrap()).exp(x.clone(), y.clone(), p.clone());
        (*b.lock().unwrap().as_mut().unwrap()).exp(x.clone(), s.clone(), p.clone());
        (*g.lock().unwrap().as_mut().unwrap()).exp(n.clone(), s.clone(), p.clone());
        let mut r = Arc::new(Mutex::new(Some(e)));
        loop {
                // find the least m such that ord_p(b) = 2^m
        let mut m: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        (*t.lock().unwrap().as_mut().unwrap()).set(b.clone());
        while { let __tmp_x = (*t.lock().unwrap().as_ref().unwrap()).cmp({ let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let __recv = (*t.lock().unwrap().as_mut().unwrap()).mul(t.clone(), t.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r#mod(t.clone(), p.clone()); __result };
        { let mut guard = m.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return self.set(y.clone());
    }

        { let __recv = { let __recv = (*t.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some(0 as i64)))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_bit(t.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }) as i32))), Arc::new(Mutex::new(Some(1 as u64)))); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).exp(g.clone(), t.clone(), p.clone()); __result };

                // t = g^(2^(r-m-1)) mod p
        { let __recv = (*g.lock().unwrap().as_mut().unwrap()).mul(t.clone(), t.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r#mod(g.clone(), p.clone()); __result };
        { let __recv = (*y.lock().unwrap().as_mut().unwrap()).mul(y.clone(), t.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r#mod(y.clone(), p.clone()); __result };
        { let __recv = (*b.lock().unwrap().as_mut().unwrap()).mul(b.clone(), g.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r#mod(b.clone(), p.clone()); __result };
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *r.lock().unwrap() = Some(new_val); };
    }
    }

    /// ModSqrt sets z to a square root of x mod p if such a square root exists, and
    /// returns z. The modulus p must be an odd prime. If x is not a square mod p,
    /// ModSqrt leaves z unchanged and returns nil. This function panics if p is
    /// not an odd integer, its behavior is undefined if p is odd but not prime.
    pub fn mod_sqrt(&mut self, mut x: Arc<Mutex<Option<Int>>>, p: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        { let _switch_val = jacobi(x.clone(), p.clone());
    if _switch_val == (-1) {
            return Arc::new(Mutex::new(None));
        } else if _switch_val == (0) {
            return self.set_int64(Arc::new(Mutex::new(Some(0 as i64))));
        } else if _switch_val == (1) {
        }
    }
                // x is not a square mod p
                // sqrt(0) mod p = 0
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *const Int = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Int }; let __result = unsafe { &*__recv_ptr }.cmp(p.clone()); __result }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = { let __recv = Arc::new(Mutex::new(Some(Int::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r#mod(x.clone(), p.clone()); __result }.clone(); x = new_val; };
    }
        if { let __tmp_x = { let __tmp_x = { let __seq_holder = { let __named_slice = (*(*p.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(4 as u64)))); __tmp_x % __tmp_y }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(3 as u64)))); __tmp_x == __tmp_y } {
                        // Check whether p is 3 mod 4, and if so, use the faster algorithm.
            return self.mod_sqrt3_mod4_prime(x.clone(), p.clone());
        } else if { let __tmp_x = { let __tmp_x = { let __seq_holder = { let __named_slice = (*(*p.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(8 as u64)))); __tmp_x % __tmp_y }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(5 as u64)))); __tmp_x == __tmp_y } {
                        // Check whether p is 5 mod 8, use Atkin's algorithm.
            return self.mod_sqrt5_mod8_prime(x.clone(), p.clone());
        } else {
                        // Otherwise, use Tonelli-Shanks.
            return self.mod_sqrt_tonelli_shanks(x.clone(), p.clone());
        }
    }

    /// Lsh sets z = x << n and returns z.
    pub fn lsh(&mut self, x: Arc<Mutex<Option<Int>>>, n: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Int>>> {
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).shl({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Rsh sets z = x >> n and returns z.
    pub fn rsh(&mut self, x: Arc<Mutex<Option<Int>>>, n: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Int>>> {
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // (-x) >> s == ^(x-1) >> s == ^((x-1) >> s) == -(((x-1) >> s) + 1)
        let mut t = (*self.abs.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).shr(t.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).add(t.clone(), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) >> s == ^(x-1) >> s == ^((x-1) >> s) == -(((x-1) >> s) + 1)
                // no underflow because |x| > 0
                // z cannot be zero if x is negative
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).shr({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Bit returns the value of the i'th bit of x. That is, it
    /// returns (x>>i)&1. The bit index i must be >= 0.
    pub fn bit(&self, i: Arc<Mutex<Option<i32>>>) -> u64 {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // optimization for common case: odd/even test of x
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return (*Arc::new(Mutex::new(Some((((*{ let __seq_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 1)) as u64))).lock().unwrap().as_ref().unwrap());
    }
                // bit 0 is same for -x
        return 0;
    }
                // optimization for common case: odd/even test of x
                // bit 0 is same for -x
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("negative bit index".to_string()) as Box<dyn Any + Send + Sync>);
    }
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        let mut t = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = self.abs.clone(); __field }, natOne.clone());
        return { let __tmp_x = (*t.lock().unwrap().as_ref().unwrap()).bit(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64)))); let __tmp_y = 1 as u64; __tmp_x ^ __tmp_y };
    }
        (*self.abs.lock().unwrap().as_ref().unwrap()).bit(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64))))
    }

    /// SetBit sets z to x, with x's i'th bit set to b (0 or 1).
    /// That is,
    ///   - if b is 1, SetBit sets z = x | (1 << i);
    ///   - if b is 0, SetBit sets z = x &^ (1 << i);
    ///   - if b is not 0 or 1, SetBit will panic.
    pub fn set_bit(&mut self, x: Arc<Mutex<Option<Int>>>, i: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("negative bit index".to_string()) as Box<dyn Any + Send + Sync>);
    }
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        let mut t = (*self.abs.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).set_bit(t.clone(), Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x ^ __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).add(t.clone(), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y }; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).set_bit({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// And sets z = x & y and returns z.
    pub fn and(&mut self, mut x: Arc<Mutex<Option<Int>>>, mut y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // (-x) & (-y) == ^(x-1) & ^(y-1) == ^((x-1) | (y-1)) == -(((x-1) | (y-1)) + 1)
        let mut x1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        let mut y1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).add((*self.abs.lock().unwrap().as_ref().unwrap()).or(x1.clone(), y1.clone()), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) & (-y) == ^(x-1) & ^(y-1) == ^((x-1) | (y-1)) == -(((x-1) | (y-1)) + 1)
                // z cannot be zero if x and y are negative
                // x & y == x & y
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).and({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) & (-y) == ^(x-1) & ^(y-1) == ^((x-1) | (y-1)) == -(((x-1) | (y-1)) + 1)
                // z cannot be zero if x and y are negative
                // x & y == x & y
                // x.neg != y.neg
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __tmp_0 = y.clone(); let __tmp_1 = x.clone(); x = __tmp_0.clone(); y = __tmp_1.clone(); };
    }
                // & is symmetric
                // x & (-y) == x & ^(y-1) == x &^ (y-1)
        let mut y1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).and_not({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, y1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// AndNot sets z = x &^ y and returns z.
    pub fn and_not(&mut self, x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // (-x) &^ (-y) == ^(x-1) &^ ^(y-1) == ^(x-1) & (y-1) == (y-1) &^ (x-1)
        let mut x1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        let mut y1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).and_not(y1.clone(), x1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) &^ (-y) == ^(x-1) &^ ^(y-1) == ^(x-1) & (y-1) == (y-1) &^ (x-1)
                // x &^ y == x &^ y
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).and_not({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) &^ (-y) == ^(x-1) &^ ^(y-1) == ^(x-1) & (y-1) == (y-1) &^ (x-1)
                // x &^ y == x &^ y
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // (-x) &^ y == ^(x-1) &^ y == ^(x-1) & ^y == ^((x-1) | y) == -(((x-1) | y) + 1)
        let mut x1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).add((*self.abs.lock().unwrap().as_ref().unwrap()).or(x1.clone(), { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) &^ y == ^(x-1) &^ y == ^(x-1) & ^y == ^((x-1) | y) == -(((x-1) | y) + 1)
                // z cannot be zero if x is negative and y is positive
                // x &^ (-y) == x &^ ^(y-1) == x & (y-1)
        let mut y1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).and({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, y1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Or sets z = x | y and returns z.
    pub fn or(&mut self, mut x: Arc<Mutex<Option<Int>>>, mut y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // (-x) | (-y) == ^(x-1) | ^(y-1) == ^((x-1) & (y-1)) == -(((x-1) & (y-1)) + 1)
        let mut x1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        let mut y1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).add((*self.abs.lock().unwrap().as_ref().unwrap()).and(x1.clone(), y1.clone()), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) | (-y) == ^(x-1) | ^(y-1) == ^((x-1) & (y-1)) == -(((x-1) & (y-1)) + 1)
                // z cannot be zero if x and y are negative
                // x | y == x | y
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).or({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) | (-y) == ^(x-1) | ^(y-1) == ^((x-1) & (y-1)) == -(((x-1) & (y-1)) + 1)
                // z cannot be zero if x and y are negative
                // x | y == x | y
                // x.neg != y.neg
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __tmp_0 = y.clone(); let __tmp_1 = x.clone(); x = __tmp_0.clone(); y = __tmp_1.clone(); };
    }
                // | is symmetric
                // x | (-y) == x | ^(y-1) == ^((y-1) &^ x) == -(^((y-1) &^ x) + 1)
        let mut y1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).add((*self.abs.lock().unwrap().as_ref().unwrap()).and_not(y1.clone(), { let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Xor sets z = x ^ y and returns z.
    pub fn xor(&mut self, mut x: Arc<Mutex<Option<Int>>>, mut y: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        if { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // (-x) ^ (-y) == ^(x-1) ^ ^(y-1) == (x-1) ^ (y-1)
        let mut x1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        let mut y1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).xor(x1.clone(), y1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) ^ (-y) == ^(x-1) ^ ^(y-1) == (x-1) ^ (y-1)
                // x ^ y == x ^ y
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).xor({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // (-x) ^ (-y) == ^(x-1) ^ ^(y-1) == (x-1) ^ (y-1)
                // x ^ y == x ^ y
                // x.neg != y.neg
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __tmp_0 = y.clone(); let __tmp_1 = x.clone(); x = __tmp_0.clone(); y = __tmp_1.clone(); };
    }
                // ^ is symmetric
                // x ^ (-y) == x ^ ^(y-1) == ^(x ^ (y-1)) == -((x ^ (y-1)) + 1)
        let mut y1 = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).sub({ let __field = (*y.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone());
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).add((*self.abs.lock().unwrap().as_ref().unwrap()).xor({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, y1.clone()), natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Not sets z = ^x and returns z.
    pub fn not(&mut self, x: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // ^(-x) == ^(^(x-1)) == x-1
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // ^(-x) == ^(^(x-1)) == x-1
                // ^x == -x-1 == -(x+1)
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).add({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }, natOne.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Sqrt sets z to ⌊√x⌋, the largest integer such that z² ≤ x, and returns z.
    /// It panics if x is negative.
    pub fn sqrt(&mut self, x: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Int>>> {
        if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        std::panic::panic_any(Box::new("square root of negative number".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).sqrt({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        Arc::new(Mutex::new(Some(self.clone())))
    }
}

impl crate::nat::nat {
    pub fn mod_inverse(&self, g: Arc<Mutex<Option<nat>>>, n: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<crate::nat::nat>>> {
                // TODO(rsc): ModInverse should be implemented in terms of this function.
        return (*{ let __recv = (Arc::new(Mutex::new(Some(Int { abs: Arc::new(Mutex::new(Some(self.clone()))), ..Default::default() })))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).mod_inverse(Arc::new(Mutex::new(Some(Int { abs: g.clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(Int { abs: n.clone(), ..Default::default() })))); __result }.lock().unwrap().as_ref().unwrap()).abs.clone();
    }
}

/// NewInt allocates and returns a new [Int] set to x.
pub fn new_int(x: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Int>>> {
        // This code is arranged to be inlineable and produce
        // zero allocations when inlined. See issue 29951.
    let mut u = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64)));
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = ((*u.lock().unwrap().as_ref().unwrap())).wrapping_neg(); *u.lock().unwrap() = Some(new_val); };
    }
    let mut abs: Arc<Mutex<Option<Vec<Word>>>> = Arc::new(Mutex::new(None));
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
    } else if { let __tmp_x = __W; let __tmp_y = 32; __tmp_x == __tmp_y } && { let __tmp_x = { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u64)))), crate::arith::Word(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y } as u64))))]))); abs = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u64))))]))); abs = new_val; };
    }
    return Arc::new(Mutex::new(Some(Int { neg: Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y }))), abs: Arc::new(Mutex::new(Some(crate::nat::nat(abs.clone())))), ..Default::default() })));
}

/// low32 returns the least significant 32 bits of x.
pub fn low32(x: Arc<Mutex<Option<nat>>>) -> u32 {
    if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
    (*Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap())
}

/// low64 returns the least significant 64 bits of x.
pub fn low64(x: Arc<Mutex<Option<nat>>>) -> u64 {
    if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
    let mut v = Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64)));
    if { let __tmp_x = __W; let __tmp_y = 32; __tmp_x == __tmp_y } && { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
        return { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }; let __tmp_y = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y };
    }
    return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// lehmerSimulate attempts to simulate several Euclidean update steps
/// using the leading digits of A and B.  It returns u0, u1, v0, v1
/// such that A and B can be updated as:
///
///	A = u0*A + v0*B
///	B = u1*A + v1*B
///
/// Requirements: A >= B and len(B.abs) >= 2
/// Since we are calculating with full words to avoid overflow,
/// we use 'even' to track the sign of the cosequences.
/// For even iterations: u0, v1 >= 0 && u1, v0 <= 0
/// For odd  iterations: u0, v1 <= 0 && u1, v0 >= 0
pub fn lehmer_simulate(A: Arc<Mutex<Option<Int>>>, B: Arc<Mutex<Option<Int>>>) -> (Arc<Mutex<Option<crate::arith::Word>>>, Arc<Mutex<Option<crate::arith::Word>>>, Arc<Mutex<Option<crate::arith::Word>>>, Arc<Mutex<Option<crate::arith::Word>>>, bool) {
    let mut u0: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut u1: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut v0: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut v1: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut even: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        // initialize the digits
    let mut a1: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));let mut a2: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));let mut u2: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));let mut v2: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));

    let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*B.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
    let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*A.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));

        // extract the top Word of bits from A and B
    let mut h = nlz(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*(*A.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))));
    { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((((*{ let __seq_holder = { let __named_slice = (*(*A.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) << h) | ((*{ let __seq_holder = { let __named_slice = (*(*A.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = __W as u64; let __tmp_y = h; __tmp_x - __tmp_y }))))))); *a1.lock().unwrap() = Some(new_val); };

        // B may have implicit zero words in the high bits if the lengths differ
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
            { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((((*{ let __seq_holder = { let __named_slice = (*(*B.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) << h) | ((*{ let __seq_holder = { let __named_slice = (*(*B.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = __W as u64; let __tmp_y = h; __tmp_x - __tmp_y }))))))); *a2.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x == __tmp_y } {
            { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*(*B.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = __W as u64; let __tmp_y = h; __tmp_x - __tmp_y })))))); *a2.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); *a2.lock().unwrap() = Some(new_val); };
        }

        // Since we are calculating with full words to avoid overflow,
        // we use 'even' to track the sign of the cosequences.
        // For even iterations: u0, v1 >= 0 && u1, v0 <= 0
        // For odd  iterations: u0, v1 <= 0 && u1, v0 >= 0
        // The first iteration starts with k=1 (odd).
    { let new_val = false; *even.lock().unwrap() = Some(new_val); };

        // variables to track the cosequences
    { let __tmp_0 = 0; let __tmp_1 = 1; let __tmp_2 = 0; *u0.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_0 as u64))))); *u1.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_1 as u64))))); *u2.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_2 as u64))))); };
    { let __tmp_0 = 0; let __tmp_1 = 0; let __tmp_2 = 1; *v0.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_0 as u64))))); *v1.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_1 as u64))))); *v2.lock().unwrap() = Some(crate::arith::Word(Arc::new(Mutex::new(Some(__tmp_2 as u64))))); };

        // Calculate the quotient and cosequences using Collins' stopping condition.
        // Note that overflow of a Word is not possible when computing the remainder
        // sequence and cosequences since the cosequence size is bounded by the input size.
        // See section 4.2 of Jebelean for details.
    while { let __tmp_x = (*a2.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*v2.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x >= __tmp_y } && { let __tmp_x = { let __tmp_x = (*a1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*a2.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x - __tmp_y }; let __tmp_y = { let __tmp_x = (*v1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*v2.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x + __tmp_y }; __tmp_x >= __tmp_y } {
        let (mut q, mut r) = (Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*a1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) / (*{ let __v = (*a2.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))))))))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*a1.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) % (*{ let __v = (*a2.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))))))))));
        { let __tmp_0 = (*a2.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = (*r.lock().unwrap().as_ref().unwrap()).clone(); *a1.lock().unwrap() = Some(__tmp_0); *a2.lock().unwrap() = Some(__tmp_1); };
        { let __tmp_0 = (*u1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = (*u2.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_2 = { let __tmp_x = (*u1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __tmp_x = (*q.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*u2.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *u0.lock().unwrap() = Some(__tmp_0); *u1.lock().unwrap() = Some(__tmp_1); *u2.lock().unwrap() = Some(__tmp_2); };
        { let __tmp_0 = (*v1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = (*v2.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_2 = { let __tmp_x = (*v1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __tmp_x = (*q.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*v2.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *v0.lock().unwrap() = Some(__tmp_0); *v1.lock().unwrap() = Some(__tmp_1); *v2.lock().unwrap() = Some(__tmp_2); };
        { let new_val = !{ let __v = (*even.lock().unwrap().as_ref().unwrap()).clone(); __v }; *even.lock().unwrap() = Some(new_val); };
    }
    return (u0.clone(), u1.clone(), v0.clone(), v1.clone(), (*even.lock().unwrap().as_ref().unwrap()));
}

/// lehmerUpdate updates the inputs A and B such that:
///
///	A = u0*A + v0*B
///	B = u1*A + v1*B
///
/// where the signs of u0, u1, v0, v1 are given by even
/// For even == true: u0, v1 >= 0 && u1, v0 <= 0
/// For even == false: u0, v1 <= 0 && u1, v0 >= 0
/// q, r, s, t are temporary variables to avoid allocations in the multiplication.
pub fn lehmer_update(A: Arc<Mutex<Option<Int>>>, B: Arc<Mutex<Option<Int>>>, q: Arc<Mutex<Option<Int>>>, r: Arc<Mutex<Option<Int>>>, s: Arc<Mutex<Option<Int>>>, t: Arc<Mutex<Option<Int>>>, u0: Arc<Mutex<Option<Word>>>, u1: Arc<Mutex<Option<Word>>>, v0: Arc<Mutex<Option<Word>>>, v1: Arc<Mutex<Option<Word>>>, even: Arc<Mutex<Option<bool>>>) {
    { let new_val = (*(*t.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = u0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*t.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    { let new_val = (*(*s.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = v0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*s.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    { let new_val = !{ let __v = (*even.lock().unwrap().as_ref().unwrap()).clone(); __v }; *(*t.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
    { let new_val = even.lock().unwrap().as_ref().unwrap().clone(); *(*s.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };

    { let __recv = t.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(A.clone(), t.clone()); __result };
    { let __recv = s.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(B.clone(), s.clone()); __result };

    { let new_val = (*(*r.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = u1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*r.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    { let new_val = (*(*q.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_word(Arc::new(Mutex::new(Some({ let __arg_holder = v1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*q.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
    { let new_val = even.lock().unwrap().as_ref().unwrap().clone(); *(*r.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
    { let new_val = !{ let __v = (*even.lock().unwrap().as_ref().unwrap()).clone(); __v }; *(*q.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };

    { let __recv = r.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(A.clone(), r.clone()); __result };
    { let __recv = q.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(B.clone(), q.clone()); __result };

    { let __recv = A.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.add(t.clone(), s.clone()); __result };
    { let __recv = B.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.add(r.clone(), q.clone()); __result };
}

/// euclidUpdate performs a single step of the Euclidean GCD algorithm
/// if extended is true, it also updates the cosequence Ua, Ub.
pub fn euclid_update(A: Arc<Mutex<Option<Int>>>, B: Arc<Mutex<Option<Int>>>, Ua: Arc<Mutex<Option<Int>>>, Ub: Arc<Mutex<Option<Int>>>, mut q: Arc<Mutex<Option<Int>>>, mut r: Arc<Mutex<Option<Int>>>, s: Arc<Mutex<Option<Int>>>, t: Arc<Mutex<Option<Int>>>, extended: Arc<Mutex<Option<bool>>>) {
    { let (__tmp_0, __tmp_1) = { let __recv = q.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.quo_rem(A.clone(), B.clone(), r.clone()); __result }; q = __tmp_0.clone(); r = __tmp_1.clone(); };

    { let __tmp_0 = { let __v = (*B.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_1 = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_2 = { let __v = (*A.lock().unwrap().as_ref().unwrap()).clone(); __v }; *A.lock().unwrap() = Some(__tmp_0); *B.lock().unwrap() = Some(__tmp_1); *r.lock().unwrap() = Some(__tmp_2); };

    if { let __v = (*extended.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Ua, Ub = Ub, Ua - q*Ub
        { let __recv = t.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.set(Ub.clone()); __result };
        { let __recv = s.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.mul(Ub.clone(), q.clone()); __result };
        { let __recv = Ub.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.sub(Ua.clone(), s.clone()); __result };
        { let __recv = Ua.clone(); let __recv_ptr: *mut Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Int }; let __result = unsafe { &mut *__recv_ptr }.set(t.clone()); __result };
    }
}

/// Jacobi returns the Jacobi symbol (x/y), either +1, -1, or 0.
/// The y argument must be an odd integer.
pub fn jacobi(x: Arc<Mutex<Option<Int>>>, y: Arc<Mutex<Option<Int>>>) -> i32 {
    if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 1))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("big: invalid 2nd argument to Int.Jacobi: need odd integer but got {}", (*{ let __recv = y.clone(); let __recv_ptr: *const Int = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Int }; let __result = unsafe { &*__recv_ptr }.string(); __result }.lock().unwrap().as_ref().unwrap()))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }

        // We use the formulation described in chapter 2, section 2.4,
        // "The Yacas Book of Algorithms":
        // http://yacas.sourceforge.net/Algo.book.pdf
    let mut a: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut b: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));let mut c: Arc<Mutex<Option<Int>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*a.lock().unwrap().as_mut().unwrap()).set(x.clone());
    (*b.lock().unwrap().as_mut().unwrap()).set(y.clone());
    let mut j = Arc::new(Mutex::new(Some(1)));

    if (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if (*{ let __field = (*a.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = -1; *j.lock().unwrap() = Some(new_val); };
    }
        { let new_val = false; *(*b.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
    }

    loop {
        if { let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).cmp({ let __arg_holder = intOne.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
        (*a.lock().unwrap().as_mut().unwrap()).r#mod(a.clone(), b.clone());
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }

                // a > 0
                // handle factors of 2 in 'a'
        let mut s = (*(*a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).trailing_zero_bits();
        if { let __tmp_x = { let __tmp_x = s; let __tmp_y = 1 as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        let mut bmod8 = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*(*b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 7))))))));
        if { let __tmp_x = (*bmod8.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(3 as u64)))); __tmp_x == __tmp_y } || { let __tmp_x = (*bmod8.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(5 as u64)))); __tmp_x == __tmp_y } {
        { let new_val = -((*j.lock().unwrap().as_ref().unwrap())); *j.lock().unwrap() = Some(new_val); };
    }
    }
        (*c.lock().unwrap().as_mut().unwrap()).rsh(a.clone(), Arc::new(Mutex::new(Some(s))));

                // swap numerator and denominator
        if { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*(*b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 3))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(3 as u64)))); __tmp_x == __tmp_y } && { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*(*c.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 3))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(3 as u64)))); __tmp_x == __tmp_y } {
        { let new_val = -((*j.lock().unwrap().as_ref().unwrap())); *j.lock().unwrap() = Some(new_val); };
    }
        (*a.lock().unwrap().as_mut().unwrap()).set(b.clone());
        (*b.lock().unwrap().as_mut().unwrap()).set(c.clone());
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Int {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
