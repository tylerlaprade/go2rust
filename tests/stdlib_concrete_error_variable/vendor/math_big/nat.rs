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

/// An unsigned integer x of the form
///
///	x = x[n-1]*_B^(n-1) + x[n-2]*_B^(n-2) + ... + x[1]*_B + x[0]
///
/// with 0 <= x[i] < _B and 0 <= i < n is stored in a slice of length n,
/// with the digits x[i] as the slice elements.
///
/// A number is normalized if the slice contains no leading 0 digits.
/// During arithmetic operations, denormalized values may occur but are
/// always normalized before returning the final result. The normalized
/// representation of 0 is the empty or nil slice (length = 0).
#[derive(Debug, Clone, Default)]
pub struct nat(pub Arc<Mutex<Option<Vec<Word>>>>);

impl Display for nat {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


pub(crate) static natOne: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<nat>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static natTwo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<nat>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static natFive: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<nat>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static natTen: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<nat>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static karatsubaThreshold: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static basicSqrThreshold: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static karatsubaSqrThreshold: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static natPool: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync_Pool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *natOne.lock().unwrap() = Some(Default::default());
    *natTwo.lock().unwrap() = Some(Default::default());
    *natFive.lock().unwrap() = Some(Default::default());
    *natTen.lock().unwrap() = Some(Default::default());
    *karatsubaThreshold.lock().unwrap() = Some(0);
    *basicSqrThreshold.lock().unwrap() = Some(0);
    *karatsubaSqrThreshold.lock().unwrap() = Some(0);
    *natPool.lock().unwrap() = Some(Default::default());
    *natOne.lock().unwrap() = Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))])))));
    *natTwo.lock().unwrap() = Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(2 as u64))))])))));
    *natFive.lock().unwrap() = Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(5 as u64))))])))));
    *natTen.lock().unwrap() = Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(10 as u64))))])))));
    *karatsubaThreshold.lock().unwrap() = Some(40);
    *basicSqrThreshold.lock().unwrap() = Some(20);
    *karatsubaSqrThreshold.lock().unwrap() = Some(260);
}


pub(crate) fn __go_zero_globals() {
    *natOne.lock().unwrap() = Some(Default::default());
    *natTwo.lock().unwrap() = Some(Default::default());
    *natFive.lock().unwrap() = Some(Default::default());
    *natTen.lock().unwrap() = Some(Default::default());
    *karatsubaThreshold.lock().unwrap() = Some(0);
    *basicSqrThreshold.lock().unwrap() = Some(0);
    *karatsubaSqrThreshold.lock().unwrap() = Some(0);
    *natPool.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_4() {
    *natOne.lock().unwrap() = Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))])))));
}


pub(crate) fn __go_init_order_8() {
    *natTwo.lock().unwrap() = Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(2 as u64))))])))));
}


pub(crate) fn __go_init_order_9() {
    *natFive.lock().unwrap() = Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(5 as u64))))])))));
}


pub(crate) fn __go_init_order_10() {
    *natTen.lock().unwrap() = Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(10 as u64))))])))));
}


pub(crate) fn __go_init_order_11() {
    *karatsubaThreshold.lock().unwrap() = Some(40);
}


pub(crate) fn __go_init_order_12() {
    *basicSqrThreshold.lock().unwrap() = Some(20);
}


pub(crate) fn __go_init_order_13() {
    *karatsubaSqrThreshold.lock().unwrap() = Some(260);
}


impl nat {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "0x".to_string(), (*Arc::new(Mutex::new(Some(String::from_utf8((*self.itoa(Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(16)))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap())))));
    }

    pub fn norm(&self) -> Arc<Mutex<Option<nat>>> {
        let mut i = Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    }

    pub fn make(&self, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<nat>>> {
        if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) } as i32); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    }
                // reuse z
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
                // Most nats start small and stay that way; don't over-allocate.
        return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(0)))); (1) as usize])))))));
    }
                // Most nats start small and stay that way; don't over-allocate.
                // Choosing a good value for e has significant performance impact
                // because it increases the chance that a value can be reused.
        const e: i32 = 4;

        Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let mut v = Vec::with_capacity(({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x + __tmp_y }) as usize); v.resize(({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize, crate::arith::Word(Arc::new(Mutex::new(Some(0))))); v })))))))
    }

    pub fn set_word(&self, x: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    }
        { let new_val = __self.make(Arc::new(Mutex::new(Some(1)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[(0) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    pub fn set_uint64(&self, x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
                // single-word value
        {
        let mut w = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64)))))));;
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
            return __self.set_word(Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
                // 2-word value
        { let new_val = __self.make(Arc::new(Mutex::new(Some(2)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[(1) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y } as u64))));
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[(0) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))));
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    pub fn set(&self, x: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let _dst_holder = __self.0.clone(); let _src = { let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    pub fn add(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            return __self.add(y.clone(), x.clone());
        } else if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                        // n == 0 because m >= n; result is 0
            return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                        // result is x
            return __self.set(x.clone());
        }
                // n == 0 because m >= n; result is 0
                // result is x
                // m > 0
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut c = add_v_v({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = add_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *c.lock().unwrap() = __moved_val; };
    }
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
        __self.norm()
    }

    pub fn sub(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            std::panic::panic_any(Box::new("underflow".to_string()) as Box<dyn Any + Send + Sync>);
        } else if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                        // n == 0 because m >= n; result is 0
            return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                        // result is x
            return __self.set(x.clone());
        }
                // n == 0 because m >= n; result is 0
                // result is x
                // m > 0
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut c = sub_v_v({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = sub_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *c.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("underflow".to_string()) as Box<dyn Any + Send + Sync>);
    }
        __self.norm()
    }

    pub fn cmp(&self, y: Arc<Mutex<Option<nat>>>) -> i32 {
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            { let new_val = -1; *r.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            { let new_val = 1; *r.lock().unwrap() = Some(new_val); };
        }
        return (*r.lock().unwrap().as_ref().unwrap());
    }
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = { let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = { let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x < __tmp_y } {
            { let new_val = -1; *r.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = { let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x > __tmp_y } {
            { let new_val = 1; *r.lock().unwrap() = Some(new_val); };
        }
        return (*r.lock().unwrap().as_ref().unwrap());
    }

    pub fn mul_add_w_w(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<Word>>>, r: Arc<Mutex<Option<Word>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = (*y.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        return __self.set_word(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // result is r
                // m > 0
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*(*mul_add_v_w_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
        __self.norm()
    }

    /// montgomery computes z mod m = x*y*2**(-n*_W) mod m,
    /// assuming k = -1/m mod 2**_W.
    /// z is used for storing the result which is returned;
    /// z must not alias x, y or m.
    /// See Gueron, "Efficient Software Implementations of Modular Exponentiation".
    /// https://eprint.iacr.org/2011/239.pdf
    /// In the terminology of that paper, this is an "Almost Montgomery Multiplication":
    /// x and y are required to satisfy 0 <= z < 2**(n*_W) and then the result
    /// z is guaranteed to satisfy 0 <= z < 2**(n*_W), but it may not be < m.
    pub fn montgomery(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>, m: Arc<Mutex<Option<nat>>>, k: Arc<Mutex<Option<Word>>>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
                // This code assumes x, y, m are all the same length, n.
                // (required by addMulVVW and the for loop).
                // It also assumes that x, y are already reduced mod m,
                // or else the result will not be properly reduced.
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x != __tmp_y } || { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x != __tmp_y } || { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("math/big: mismatched montgomery number lengths".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let __clear_holder = __self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        let mut c: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut d = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        let mut c2 = add_mul_v_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut t = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = __self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) * (*{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))))))));
        let mut c3 = add_mul_v_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut cx = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + (*{ let __v = (*c2.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))))))));
        let mut cy = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*cx.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + (*{ let __v = (*c3.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))))))));
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __v = (*cy.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
        if { let __tmp_x = (*cx.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*c2.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } || { let __tmp_x = (*cy.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*c3.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); *c.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); *c.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        sub_v_v({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
    } else {
        { let _dst_holder = __self.0.clone(); let _dst_start = 0; let _dst_len = (({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) - _dst_start; let _src = { let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    }
        Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))))
    }

    pub fn mul(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            return __self.mul(y.clone(), x.clone());
        } else if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
            return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
            return __self.mul_add_w_w(x.clone(), Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64))))))));
        }
                // m >= n > 1
                // determine if z can be reused
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        *__self.0.lock().unwrap() = None;
    }
                // z is an alias for x or y - cannot reuse
                // use basic multiplication if the numbers are small
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*karatsubaThreshold.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        basic_mul(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return __self.norm();
    }
                // m >= n && n >= karatsubaThreshold && n >= 2
                // determine Karatsuba length k such that
                //
                //   x = xh*b + x0  (0 <= x0 < b)
                //   y = yh*b + y0  (0 <= y0 < b)
                //   b = 1<<(_W*k)  ("base" of digits xi, yi)
                //
        let mut k = karatsuba_len(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = karatsubaThreshold.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // k <= n
                // multiply x0 and y0 via Karatsuba
        let mut x0 = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = (k) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        let mut y0 = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = (k) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        { let new_val = __self.make(Arc::new(Mutex::new(Some(std::cmp::max(({ let __tmp_x = 6; let __tmp_y = k; __tmp_x * __tmp_y } as i32), ({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as i32)))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        karatsuba(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __self = new_val; };
        { let __clear_start = ({ let __tmp_x = 2; let __tmp_y = k; __tmp_x * __tmp_y }) as usize; let __clear_end = { let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = __self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
                // If xh != 0 or yh != 0, add the missing terms to z. For
                //
                //   xh = xi*b^i + ... + x2*b^2 + x1*b (0 <= xi < b)
                //   yh =                         y1*b (0 <= y1 < b)
                //
                // the missing terms are
                //
                //   x0*y1*b and xi*y0*b^i, xi*y1*b^(i+1) for i > 0
                //
                // since all the yi for i > 1 are 0 by choice of k: If any of them
                // were > 0, then yh >= b^2 and thus y >= b^2. Then k' = k*2 would
                // be a larger valid threshold contradicting the assumption about k.
                //
        if { let __tmp_x = k; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        let mut tp = get_nat(Arc::new(Mutex::new(Some({ let __tmp_x = 3; let __tmp_y = k; __tmp_x * __tmp_y }))));
        let mut t = Arc::new(Mutex::new(Some({ let __v = (*tp.lock().unwrap().as_ref().unwrap()).clone(); __v })));
                // add x0*y1*b
        let mut x0 = (*x0.lock().unwrap().as_ref().unwrap()).norm();
        let mut y1 = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (k) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).mul(x0.clone(), y1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_val; };
        add_at(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(k))));
                // add xi*y0<<i, xi*y1*b<<(i+k)
        let mut y0 = (*y0.lock().unwrap().as_ref().unwrap()).norm();
        let mut i = Arc::new(Mutex::new(Some(k)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x < __tmp_y } {
        let mut xi = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*xi.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = (k as i32); __tmp_x > __tmp_y } {
        { let new_val = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*xi.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (k) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *xi.lock().unwrap() = Some(new_val); };
    }
        { let new_val = (*xi.lock().unwrap().as_ref().unwrap()).norm(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *xi.lock().unwrap() = __moved_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).mul(xi.clone(), y0.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_val; };
        add_at(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).mul(xi.clone(), y1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_val; };
        add_at(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = k; __tmp_x + __tmp_y }))));
        { let __rhs = k; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        put_nat(tp.clone());
    }
                // add x0*y1*b
                // y1 is normalized because y is
                // update t so we don't lose t's underlying array
                // add xi*y0<<i, xi*y1*b<<(i+k)
        __self.norm()
    }

    /// z = x*x
    pub fn sqr(&self, x: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
            return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
            let mut d = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
            { let new_val = __self.make(Arc::new(Mutex::new(Some(2)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
            { let (__tmp_0, __tmp_1) = mul_w_w(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); (*__self.0.clone().lock().unwrap().as_mut().unwrap())[(1) as usize] = __tmp_0.lock().unwrap().take().unwrap_or_default(); (*__self.0.clone().lock().unwrap().as_mut().unwrap())[(0) as usize] = __tmp_1.lock().unwrap().take().unwrap_or_default(); };
            return __self.norm();
        }
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        *__self.0.lock().unwrap() = None;
    }
                // z is an alias for x - cannot reuse
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*basicSqrThreshold.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        basic_mul(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return __self.norm();
    }
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*karatsubaSqrThreshold.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        basic_sqr(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return __self.norm();
    }
                // Use Karatsuba multiplication optimized for x == y.
                // The algorithm and layout of z are the same as for mul.
                // z = (x1*b + x0)^2 = x1^2*b^2 + 2*x1*x0*b + x0^2
        let mut k = karatsuba_len(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = karatsubaSqrThreshold.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut x0 = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = (k) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        { let new_val = __self.make(Arc::new(Mutex::new(Some(std::cmp::max(({ let __tmp_x = 6; let __tmp_y = k; __tmp_x * __tmp_y } as i32), ({ let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y } as i32)))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        karatsuba_sqr(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __self = new_val; };
        { let __clear_start = ({ let __tmp_x = 2; let __tmp_y = k; __tmp_x * __tmp_y }) as usize; let __clear_end = { let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = __self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        if { let __tmp_x = k; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut tp = get_nat(Arc::new(Mutex::new(Some({ let __tmp_x = 2; let __tmp_y = k; __tmp_x * __tmp_y }))));
        let mut t = Arc::new(Mutex::new(Some({ let __v = (*tp.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        let mut x0 = (*x0.lock().unwrap().as_ref().unwrap()).norm();
        let mut x1 = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (k) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).mul(x0.clone(), x1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_val; };
        add_at(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(k))));
        add_at(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(k))));
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).sqr(x1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_val; };
        add_at(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = 2; let __tmp_y = k; __tmp_x * __tmp_y }))));
        put_nat(tp.clone());
    }
                // z = 2*x1*x0*b + x0^2
                // z = x1^2*b^2 + 2*x1*x0*b + x0^2
        __self.norm()
    }

    /// mulRange computes the product of all the unsigned integers in the
    /// range [a, b] inclusively. If a > b (empty range), the result is 1.
    pub fn mul_range(&self, a: Arc<Mutex<Option<u64>>>, b: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<nat>>> {
        if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
                        // cut long ranges short (optimization)
            return self.set_uint64(Arc::new(Mutex::new(Some(0 as u64))));
        } else if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            return self.set_uint64(Arc::new(Mutex::new(Some(1 as u64))));
        } else if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
            return self.set_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        } else if { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
            return self.mul(nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_uint64(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
        }
                // cut long ranges short (optimization)
        let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = 2 as u64; __tmp_x / __tmp_y }; __tmp_x + __tmp_y })));
        return self.mul(nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).mul_range(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).mul_range(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
    }

    /// bitLen returns the length of x in bits.
    /// Unlike most methods, it works even if x is not normalized.
    pub fn bit_len(&self) -> i32 {
                // This function is used in cryptographic operations. It must not leak
                // anything but the Int's sign and bit size through side-channels. Any
                // changes must be reviewed by a security expert.
        {
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));;
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            let mut top = Arc::new(Mutex::new(Some((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64)));;
            { let __rhs = { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; let mut guard = top.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };;
            { let __rhs = { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x >> __tmp_y }; let mut guard = top.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };;
            { let __rhs = { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x >> __tmp_y }; let mut guard = top.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };;
            { let __rhs = { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >> __tmp_y }; let mut guard = top.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };;
            { let __rhs = { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y }; let mut guard = top.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };;
            { let __rhs = { let __tmp_x = { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y }; let __tmp_y = 16; __tmp_x >> __tmp_y }; let mut guard = top.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };;
            return { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x * __tmp_y }; let __tmp_y = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_holder = top.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x + __tmp_y };;
        }
    }
                // bits.Len uses a lookup table for the low-order bits on some
                // architectures. Neutralize any input-dependent behavior by setting all
                // bits after the first one bit.
                // ">> 32" doesn't compile on 32-bit architectures
        0
    }

    /// trailingZeroBits returns the number of consecutive least significant zero
    /// bits of x.
    pub fn trailing_zero_bits(&self) -> u64 {
        if { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
        let mut i: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        while { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // x[i] != 0
        return { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(math_bits::trailing_zeros(Arc::new(Mutex::new(Some((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64)))) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
    }

    /// isPow2 returns i, true when x == 2**i and 0, false otherwise.
    pub fn is_pow2(&self) -> (u64, bool) {
        let mut i: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        while { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }; __tmp_x == __tmp_y } && { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & (((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) - 1))))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        return ({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(math_bits::trailing_zeros(Arc::new(Mutex::new(Some((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64)))) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }, true);
    }
        (0, false)
    }

    /// z = x << s
    pub fn shl(&self, x: Arc<Mutex<Option<nat>>>, s: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        if same(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
        if !alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return __self.set(x.clone());
    }
    }
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    }
                // m > 0
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x / __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*(*shl_v_u({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x % __tmp_y })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
        { let __clear_start = (0) as usize; let __clear_end = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize; let __clear_holder = __self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
        __self.norm()
    }

    /// z = x >> s
    pub fn shr(&self, x: Arc<Mutex<Option<nat>>>, s: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        if same(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
        if !alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return __self.set(x.clone());
    }
    }
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x / __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    }
                // n > 0
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        shr_v_u(__self.0.clone(), { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x % __tmp_y }))));
        __self.norm()
    }

    pub fn set_bit(&self, x: Arc<Mutex<Option<nat>>>, i: Arc<Mutex<Option<u64>>>, b: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut j = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x / __tmp_y }) as i32)));
        let mut m = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((1 as u64) << ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x % __tmp_y })))))))));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        { let _switch_val = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0 as u64) {
            { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
            { let _dst_holder = __self.0.clone(); let _src = { let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
            if { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // no need to grow
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
                        // no need to grow
            { let __idx = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as usize; let __rhs = (*({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let __seq_holder = __self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() & ! __rhs; };
            return __self.norm();
        } else if _switch_val == (1 as u64) {
            if { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let __clear_start = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __clear_end = { let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = __self.0.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
    } else {
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
            { let _dst_holder = __self.0.clone(); let _src = { let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
            { let __idx = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as usize; let __rhs = (*({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let __seq_holder = __self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() | __rhs; };
                        // no need to normalize
            return Arc::new(Mutex::new(Some(__self.clone())));
        }
    }
                // no need to grow
                // no need to normalize
        std::panic::panic_any(Box::new("set bit is not 0 or 1".to_string()) as Box<dyn Any + Send + Sync>);
    }

    /// bit returns the value of the i'th bit, with lsb == bit 0.
    pub fn bit(&self, i: Arc<Mutex<Option<u64>>>) -> u64 {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return 0;
    }
                // 0 <= j < len(x)
        return (*Arc::new(Mutex::new(Some(((((*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x % __tmp_y })) & 1)) as u64))).lock().unwrap().as_ref().unwrap());
    }

    /// sticky returns 1 if there's a 1 bit within the
    /// i least significant bits, otherwise it returns 0.
    pub fn sticky(&self, i: Arc<Mutex<Option<u64>>>) -> u64 {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        if { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
        return 1;
    }
                // 0 <= j < len(x)
        { let __range_holder = { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().cloned() {
        if { let __tmp_x = x.clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        return 1;
    }
    } }
        if { let __tmp_x = { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = ({ let __tmp_x = __W as u64; let __tmp_y = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x % __tmp_y }; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        return 1;
    }
        0
    }

    pub fn and(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *m.lock().unwrap() = Some(new_val); };
    }
                // m <= n
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & (*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        __self.norm()
    }

    /// trunc returns z = x mod 2ⁿ.
    pub fn trunc(&self, x: Arc<Mutex<Option<nat>>>, n: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut w = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x + __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); let __tmp_y = __W as u64; __tmp_x / __tmp_y })));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return __self.set(x.clone());
    }
        { let new_val = __self.make(Arc::new(Mutex::new(Some((*w.lock().unwrap().as_ref().unwrap()) as i32)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let _dst_holder = __self.0.clone(); let _src = { let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x % __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __idx = { let __tmp_x = ({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as usize; let __rhs = (*({ let __tmp_x = { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); let __tmp_y = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x - __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let __seq_holder = __self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() & __rhs; };
    }
        __self.norm()
    }

    pub fn and_not(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *n.lock().unwrap() = Some(new_val); };
    }
                // m >= n
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & ! (*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let _dst_holder = __self.0.clone(); let _dst_start = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = (({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) - _dst_start; let _src = { let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        __self.norm()
    }

    pub fn or(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut s = Arc::new(Mutex::new(Some({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __tmp_0 = (*m.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = (*n.lock().unwrap().as_ref().unwrap()).clone(); *n.lock().unwrap() = Some(__tmp_0); *m.lock().unwrap() = Some(__tmp_1); };
        { let new_val = y.lock().unwrap().as_ref().unwrap().clone(); *s.lock().unwrap() = Some(new_val); };
    }
                // m >= n
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) | (*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let _dst_holder = __self.0.clone(); let _dst_start = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = (({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) - _dst_start; let _src = { let __slice_holder = { let __named_slice = (*s.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        __self.norm()
    }

    pub fn xor(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut s = Arc::new(Mutex::new(Some({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __tmp_0 = (*m.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = (*n.lock().unwrap().as_ref().unwrap()).clone(); *n.lock().unwrap() = Some(__tmp_0); *m.lock().unwrap() = Some(__tmp_1); };
        { let new_val = y.lock().unwrap().as_ref().unwrap().clone(); *s.lock().unwrap() = Some(new_val); };
    }
                // m >= n
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) ^ (*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let _dst_holder = __self.0.clone(); let _dst_start = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = (({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize) - _dst_start; let _src = { let __slice_holder = { let __named_slice = (*s.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        __self.norm()
    }

    /// random creates a random integer in [0..limit), using the space in z if
    /// possible. n is the bit length of limit.
    pub fn random(&self, rand: Arc<Mutex<Option<rand_Rand>>>, limit: Arc<Mutex<Option<nat>>>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        *__self.0.lock().unwrap() = None;
    }
                // z is an alias for limit - cannot reuse
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*limit.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut bitLengthOfMSW = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x % __tmp_y }) as u64)));
        if { let __tmp_x = { let __v = (*bitLengthOfMSW.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = __W as u64; *bitLengthOfMSW.lock().unwrap() = Some(new_val); };
    }
        let mut mask = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((((1 << { let __v = (*bitLengthOfMSW.lock().unwrap().as_ref().unwrap()).clone(); __v })) - 1) as u64)))))));
        loop {
        { let _switch_val = __W;
    if _switch_val == (32) {
            for i in 0..(({ let __range_holder = __self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[(i) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some({ let __recv = rand.clone(); let __recv_ptr: *mut rand_Rand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut rand_Rand }; let __result = unsafe { &mut *__recv_ptr }.uint32(); __result } as u64))));
    }
        } else if _switch_val == (64) {
            for i in 0..(({ let __range_holder = __self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[(i) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(({ let __recv = rand.clone(); let __recv_ptr: *mut rand_Rand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut rand_Rand }; let __result = unsafe { &mut *__recv_ptr }.uint32(); __result } as u64 | (({ let __recv = rand.clone(); let __recv_ptr: *mut rand_Rand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut rand_Rand }; let __result = unsafe { &mut *__recv_ptr }.uint32(); __result } as u64) << 32i32))))));
    }
        } else {
            std::panic::panic_any(Box::new("unknown word size".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
        { let __idx = { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*limit.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as usize; let __rhs = (*({ let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let __seq_holder = __self.0.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() & __rhs; };
        if { let __tmp_x = __self.cmp(limit.clone()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        break
    }
    }
        __self.norm()
    }

    /// If m != 0 (i.e., len(m) != 0), expNN sets z to x**y mod m;
    /// otherwise it sets z to x**y. The result is the value of z.
    pub fn exp_n_n(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>, m: Arc<Mutex<Option<nat>>>, slow: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // We cannot allow in-place modification of x or y.
        *__self.0.lock().unwrap() = None;
    }
                // We cannot allow in-place modification of x or y.
                // x**y mod 1 == 0
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x == __tmp_y } {
        return __self.set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64))))))));
    }
                // m == 0 || m > 1
                // x**0 == 1
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return __self.set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))))));
    }
                // y > 0
                // 0**y = 0
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return __self.set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64))))))));
    }
                // x > 0
                // 1**y = 1
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x == __tmp_y } {
        return __self.set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))))));
    }
                // x > 1
                // x**1 == x
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x == __tmp_y } {
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        return __self.rem(x.clone(), m.clone());
    }
        return __self.set(x.clone());
    }
                // y > 1
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
                // We likely end up being as long as the modulus.
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
                // If the exponent is large, we use the Montgomery method for odd values,
                // and a 4-bit, windowed exponentiation for powers of two,
                // and a CRT-decomposed Montgomery method for the remaining values
                // (even values times non-trivial odd values, which decompose into one
                // instance of each of the first two cases).
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x > __tmp_y } && !{ let __v = (*slow.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 1))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x == __tmp_y } {
        return __self.exp_n_n_montgomery(x.clone(), y.clone(), m.clone());
    }
        {
        let (mut logM, mut ok) = (*m.lock().unwrap().as_ref().unwrap()).is_pow2();;
        if ok {
            return __self.exp_n_n_windowed(x.clone(), y.clone(), Arc::new(Mutex::new(Some(logM))));;
        }
    }
        return __self.exp_n_n_montgomery_even(x.clone(), y.clone(), m.clone());
    }
    }
                // We likely end up being as long as the modulus.
                // If the exponent is large, we use the Montgomery method for odd values,
                // and a 4-bit, windowed exponentiation for powers of two,
                // and a CRT-decomposed Montgomery method for the remaining values
                // (even values times non-trivial odd values, which decompose into one
                // instance of each of the first two cases).
        { let new_val = __self.set(x.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut v = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        let mut shift = Arc::new(Mutex::new(Some({ let __tmp_x = nlz(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 1 as u64; __tmp_x + __tmp_y })));
        { let __rhs = (*shift.lock().unwrap().as_ref().unwrap()); let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() << __rhs); };
        let mut q: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        const mask: u64 = 1 << (__W - 1);

                // We walk through the bits of the exponent one by one. Each time we
                // see a bit, we square, thus doubling the power. If the bit is a one,
                // we also multiply by x, thus adding one to the power.
        let mut w = Arc::new(Mutex::new(Some({ let __tmp_x = 64; let __tmp_y = (*Arc::new(Mutex::new(Some((*shift.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
                // zz and r are used to avoid allocating in mul and div as
                // otherwise the arguments would alias.
        let mut zz: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));let mut r: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).sqr(Arc::new(Mutex::new(Some(__self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };

        if { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & mask as u64))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).mul(Arc::new(Mutex::new(Some(__self.clone()))), x.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
    }

        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = (*zz.lock().unwrap().as_ref().unwrap()).div(r.clone(), Arc::new(Mutex::new(Some(__self.clone()))), m.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };
        { let __tmp_0 = q.clone(); let __tmp_1 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_2 = zz.clone(); let __tmp_3 = r.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); *r.lock().unwrap() = __tmp_1.lock().unwrap().take(); *q.lock().unwrap() = __tmp_2.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_3.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
    }

        { let __rhs = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() << __rhs); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 2; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))); *v.lock().unwrap() = Some(new_val); };

        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x < __tmp_y } {
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).sqr(Arc::new(Mutex::new(Some(__self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };

        if { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & mask as u64))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).mul(Arc::new(Mutex::new(Some(__self.clone()))), x.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
    }

        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = (*zz.lock().unwrap().as_ref().unwrap()).div(r.clone(), Arc::new(Mutex::new(Some(__self.clone()))), m.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };
        { let __tmp_0 = q.clone(); let __tmp_1 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_2 = zz.clone(); let __tmp_3 = r.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); *r.lock().unwrap() = __tmp_1.lock().unwrap().take(); *q.lock().unwrap() = __tmp_2.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_3.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
    }

        { let __rhs = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() << __rhs); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        __self.norm()
    }

    /// expNNMontgomeryEven calculates x**y mod m where m = m1 × m2 for m1 = 2ⁿ and m2 odd.
    /// It uses two recursive calls to expNN for x**y mod m1 and x**y mod m2
    /// and then uses the Chinese Remainder Theorem to combine the results.
    /// The recursive call using m1 will use expNNWindowed,
    /// while the recursive call using m2 will use expNNMontgomery.
    /// For more details, see Ç. K. Koç, “Montgomery Reduction with Even Modulus”,
    /// IEE Proceedings: Computers and Digital Techniques, 141(5) 314-316, September 1994.
    /// http://www.people.vcu.edu/~jwang3/CMSC691/j34monex.pdf
    pub fn exp_n_n_montgomery_even(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>, m: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
                // Split m = m₁ × m₂ where m₁ = 2ⁿ
        let mut n = (*m.lock().unwrap().as_ref().unwrap()).trailing_zero_bits();
        let mut m1 = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shl(natOne.clone(), Arc::new(Mutex::new(Some(n))));
        let mut m2 = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shr(m.clone(), Arc::new(Mutex::new(Some(n))));
                // We want z = x**y mod m.
                // z₁ = x**y mod m1 = (x**y mod m) mod m1 = z mod m1
                // z₂ = x**y mod m2 = (x**y mod m) mod m2 = z mod m2
                // (We are using the math/big convention for names here,
                // where the computation is z = x**y mod m, so its parts are z1 and z2.
                // The paper is computing x = a**e mod n; it refers to these as x2 and z1.)
        let mut z1 = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).exp_n_n(x.clone(), y.clone(), m1.clone(), Arc::new(Mutex::new(Some(false))));
        let mut z2 = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).exp_n_n(x.clone(), y.clone(), m2.clone(), Arc::new(Mutex::new(Some(false))));
                // Reconstruct z from z₁, z₂ using CRT, using algorithm from paper,
                // which uses only a single modInverse (and an easy one at that).
                //	p = (z₁ - z₂) × m₂⁻¹ (mod m₁)
                //	z = z₂ + p × m₂
                // The final addition is in range because:
                //	z = z₂ + p × m₂
                //	  ≤ z₂ + (m₁-1) × m₂
                //	  < m₂ + (m₁-1) × m₂
                //	  = m₁ × m₂
                //	  = m.
        { let new_val = __self.set(z2.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
                // Compute (z₁ - z₂) mod m1 [m1 == 2**n] into z1.
        { let new_val = (*z1.lock().unwrap().as_ref().unwrap()).sub_mod2_n(z1.clone(), z2.clone(), Arc::new(Mutex::new(Some(n)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z1.lock().unwrap() = __moved_val; };
                // Reuse z2 for p = (z₁ - z₂) [in z1] * m2⁻¹ (mod m₁ [= 2ⁿ]).
        let mut m2inv = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).mod_inverse(m2.clone(), m1.clone());
        { let new_val = (*z2.lock().unwrap().as_ref().unwrap()).mul(z1.clone(), m2inv.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z2.lock().unwrap() = __moved_val; };
        { let new_val = (*z2.lock().unwrap().as_ref().unwrap()).trunc(z2.clone(), Arc::new(Mutex::new(Some(n)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z2.lock().unwrap() = __moved_val; };
                // Reuse z1 for p * m2.
        { let new_val = __self.add(Arc::new(Mutex::new(Some(__self.clone()))), (*z1.lock().unwrap().as_ref().unwrap()).mul(z2.clone(), m2.clone())); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// expNNWindowed calculates x**y mod m using a fixed, 4-bit window,
    /// where m = 2**logM.
    pub fn exp_n_n_windowed(&self, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>, logM: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x <= __tmp_y } {
        std::panic::panic_any(Box::new("big: misuse of expNNWindowed".to_string()) as Box<dyn Any + Send + Sync>);
    }
        if { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & 1))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
                // len(y) > 1, so y  > logM.
                // x is even, so x**y is a multiple of 2**y which is a multiple of 2**logM.
        return __self.set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64))))))));
    }
                // len(y) > 1, so y  > logM.
                // x is even, so x**y is a multiple of 2**y which is a multiple of 2**logM.
        if { let __tmp_x = { let __v = (*logM.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x == __tmp_y } {
        return __self.set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))))));
    }
                // zz is used to avoid allocating in mul as otherwise
                // the arguments would alias.
        let mut w = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*logM.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u64; __tmp_x + __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); let __tmp_y = __W as u64; __tmp_x / __tmp_y }) as i32)));
        let mut zzp = get_nat(Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut zz = Arc::new(Mutex::new(Some({ let __v = (*zzp.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        const n: i32 = 4;

                // powers[i] contains x^i.
        let mut powers: Arc<Mutex<Option<[Arc<Mutex<Option<nat>>>; 16]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None))))));
        for i in 0..(({ let __range_holder = powers.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*powers.lock().unwrap().as_mut().unwrap())[(i) as usize] = get_nat(Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        { let new_val = (*{ let __recv = { let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).set(natOne.clone()); __result }.lock().unwrap().as_ref().unwrap()).clone(); *{ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap() = Some(new_val); };
        { let new_val = (*{ let __recv = { let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).trunc(x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = logM.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *{ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap() = Some(new_val); };
        let mut i = Arc::new(Mutex::new(Some(2)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x < __tmp_y } {
        let (mut p2, mut p, mut p1) = ({ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x / __tmp_y }) as usize].clone() }.clone(), { let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone(), { let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }.clone());
        { let new_val = (*{ let __recv = p.clone(); let __recv_ptr: *const nat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const nat }; let __result = unsafe { &*__recv_ptr }.sqr(Arc::new(Mutex::new(Some({ let __v = (*p2.lock().unwrap().as_ref().unwrap()).clone(); __v })))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *p.lock().unwrap() = Some(new_val); };
        { let new_val = (*{ let __recv = p.clone(); let __recv_ptr: *const nat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const nat }; let __result = unsafe { &*__recv_ptr }.trunc(Arc::new(Mutex::new(Some({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }))), Arc::new(Mutex::new(Some({ let __arg_holder = logM.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *p.lock().unwrap() = Some(new_val); };
        { let new_val = (*{ let __recv = p1.clone(); let __recv_ptr: *const nat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const nat }; let __result = unsafe { &*__recv_ptr }.mul(Arc::new(Mutex::new(Some({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }))), x.clone()); __result }.lock().unwrap().as_ref().unwrap()).clone(); *p1.lock().unwrap() = Some(new_val); };
        { let new_val = (*{ let __recv = p1.clone(); let __recv_ptr: *const nat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const nat }; let __result = unsafe { &*__recv_ptr }.trunc(Arc::new(Mutex::new(Some({ let __v = (*p1.lock().unwrap().as_ref().unwrap()).clone(); __v }))), Arc::new(Mutex::new(Some({ let __arg_holder = logM.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *p1.lock().unwrap() = Some(new_val); };
        { let __rhs = 2; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // Because phi(2**logM) = 2**(logM-1), x**(2**(logM-1)) = 1,
                // so we can compute x**(y mod 2**(logM-1)) instead of x**y.
                // That is, we can throw away all but the bottom logM-1 bits of y.
                // Instead of allocating a new y, we start reading y at the right word
                // and truncate it appropriately at the start of the loop.
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut mtop = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __v = (*logM.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u64; __tmp_x - __tmp_y }); let __tmp_y = __W as u64; __tmp_x / __tmp_y }) as i32)));
        let mut mmask = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(!0 as u64)))))));
        {
        let mut mbits = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*logM.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); let __tmp_y = ((__W as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y })));;
        if { let __tmp_x = { let __v = (*mbits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
            { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((((1 << { let __v = (*mbits.lock().unwrap().as_ref().unwrap()).clone(); __v })) - 1))))); *mmask.lock().unwrap() = Some(new_val); };;
        }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mtop.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = mtop.lock().unwrap().as_ref().unwrap().clone(); *i.lock().unwrap() = Some(new_val); };
    }
        let mut advance = Arc::new(Mutex::new(Some(false)));
        { let new_val = __self.set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        let mut yi = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mtop.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let __rhs = (*({ let __v = (*mmask.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = yi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & __rhs); };
    }
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x < __tmp_y } {
        if { let __v = (*advance.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Account for use of 4 bits in previous iteration.
                // Unrolled loop for significant performance
                // gain. Use go test -bench=".*" in crypto/rsa
                // to check performance before making changes.
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).sqr(Arc::new(Mutex::new(Some(__self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
        { let new_val = __self.trunc(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = logM.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).sqr(Arc::new(Mutex::new(Some(__self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
        { let new_val = __self.trunc(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = logM.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).sqr(Arc::new(Mutex::new(Some(__self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
        { let new_val = __self.trunc(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = logM.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).sqr(Arc::new(Mutex::new(Some(__self.clone())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
        { let new_val = __self.trunc(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = logM.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }

                // Account for use of 4 bits in previous iteration.
                // Unrolled loop for significant performance
                // gain. Use go test -bench=".*" in crypto/rsa
                // to check performance before making changes.
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).mul(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __v = (*{ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*{ let __v = (*yi.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = __W; let __tmp_y = n; __tmp_x - __tmp_y })) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone()))); let __tmp_1 = zz.clone(); *zz.lock().unwrap() = __tmp_0.lock().unwrap().take(); { let __moved_val = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
        { let new_val = __self.trunc(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = logM.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };

        { let __rhs = crate::arith::Word(Arc::new(Mutex::new(Some(n as u64)))); let mut guard = yi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() << __rhs); };
        { let new_val = true; *advance.lock().unwrap() = Some(new_val); };
        { let __rhs = 4; let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // Account for use of 4 bits in previous iteration.
                // Unrolled loop for significant performance
                // gain. Use go test -bench=".*" in crypto/rsa
                // to check performance before making changes.
        { let new_val = { let __v = (*zz.lock().unwrap().as_ref().unwrap()).clone(); __v }; *zzp.lock().unwrap() = Some(new_val); };
        put_nat(zzp.clone());
        for i in 0..(({ let __range_holder = powers.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        put_nat({ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() });
    }
        __self.norm()
    }

    /// expNNMontgomery calculates x**y mod m using a fixed, 4-bit window.
    /// Uses Montgomery representation.
    pub fn exp_n_n_montgomery(&self, mut x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>, m: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        let mut numWords = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
                // We want the lengths of x and m to be equal.
                // It is OK if x >= m as long as len(x) == len(m).
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*numWords.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x > __tmp_y } {
        { let (__tmp_0, __tmp_1) = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).div(Arc::new(Mutex::new(None)), x.clone(), m.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_1; };
    }
                // Note: now len(x) <= numWords, not guaranteed ==.
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*numWords.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        let mut rr = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(0)))); ({ let __v = (*numWords.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])))))));
        { let _dst_holder = { let __named_slice = (*rr.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let _src = { let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = rr.lock().unwrap().as_ref().unwrap().clone(); *x.lock().unwrap() = Some(new_val); };
    }
                // Ideally the precomputations would be performed outside, and reused
                // k0 = -m**-1 mod 2**_W. Algorithm from: Dumas, J.G. "On Newton–Raphson
                // Iteration for Multiplicative Inverses Modulo Prime Powers".
        let mut k0 = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((2 - (*{ let __seq_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))));
        let mut t = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) - 1))))))));
        let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x < __tmp_y } {
        { let __rhs = (*({ let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() * __rhs); };
        { let __rhs = (*(({ let __tmp_x = (*t.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x + __tmp_y })).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = k0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() * __rhs); };
        { let __rhs = 1; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
    }
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __v = (*k0.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())).wrapping_neg())))); *k0.lock().unwrap() = Some(new_val); };
                // RR = 2**(2*_W*len(m)) mod m
        let mut RR = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).set_word(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))))))));
        let mut zz = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shl(RR.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*numWords.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 64; __tmp_x * __tmp_y }) as u64))));
        { let (__tmp_0, __tmp_1) = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).div(RR.clone(), zz.clone(), m.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *RR.lock().unwrap() = __moved_tmp_1; };
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*RR.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*numWords.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).make(Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let _dst_holder = { let __named_slice = (*zz.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let _src = { let __slice_holder = { let __named_slice = (*RR.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = zz.lock().unwrap().as_ref().unwrap().clone(); *RR.lock().unwrap() = Some(new_val); };
    }
                // one = 1, with equal length to that of m
        let mut one = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(0)))); ({ let __v = (*numWords.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])))))));
        (*{ let __named_slice = (*one.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[(0) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64))));
        const n: i32 = 4;

                // powers[i] contains x^i
        let mut powers: Arc<Mutex<Option<[nat; 16]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        (*powers.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*{ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.montgomery(one.clone(), RR.clone(), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone();
        (*powers.lock().unwrap().as_mut().unwrap())[(1) as usize] = (*{ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.montgomery(x.clone(), RR.clone(), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone();
        let mut i = Arc::new(Mutex::new(Some(2)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x < __tmp_y } {
        (*powers.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*{ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.montgomery(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }))), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // initialize z = 1 (Montgomery 1)
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let _dst_holder = __self.0.clone(); let _src = { let __slice_holder = { let __named_slice = { let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; __named_slice.0.clone() }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).make(Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
                // same windowed exponent, but with Montgomery multiplications
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        let mut yi = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x < __tmp_y } {
        if { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).montgomery(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some(__self.clone()))), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let new_val = __self.montgomery(zz.clone(), zz.clone(), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).montgomery(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some(__self.clone()))), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let new_val = __self.montgomery(zz.clone(), zz.clone(), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).montgomery(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = powers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*{ let __v = (*yi.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = __W; let __tmp_y = n; __tmp_x - __tmp_y })) as usize].clone() }))), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        { let __tmp_0 = zz.clone(); let __tmp_1 = Arc::new(Mutex::new(Some(__self.clone()))); { let __moved_val = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } *zz.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
        { let __rhs = crate::arith::Word(Arc::new(Mutex::new(Some(n as u64)))); let mut guard = yi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() << __rhs); };
        { let __rhs = 4; let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // convert to regular number
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).montgomery(Arc::new(Mutex::new(Some(__self.clone()))), one.clone(), m.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = numWords.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
                // One last reduction, just in case.
                // See golang.org/issue/13907.
        if { let __tmp_x = (*zz.lock().unwrap().as_ref().unwrap()).cmp(m.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y } {
                // Common case is m has high bit set; in that case,
                // since zz is the same length as m, there can be just
                // one multiple of m to remove. Just subtract.
                // We think that the subtract should be sufficient in general,
                // so do that unconditionally, but double-check,
                // in case our beliefs are wrong.
                // The div is not expected to be reached.
        { let new_val = (*zz.lock().unwrap().as_ref().unwrap()).sub(zz.clone(), m.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*zz.lock().unwrap().as_ref().unwrap()).cmp(m.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let (__tmp_0, __tmp_1) = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).div(Arc::new(Mutex::new(None)), zz.clone(), m.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *zz.lock().unwrap() = __moved_tmp_1; };
    }
    }
                // Common case is m has high bit set; in that case,
                // since zz is the same length as m, there can be just
                // one multiple of m to remove. Just subtract.
                // We think that the subtract should be sufficient in general,
                // so do that unconditionally, but double-check,
                // in case our beliefs are wrong.
                // The div is not expected to be reached.
        return (*zz.lock().unwrap().as_ref().unwrap()).norm();
    }

    /// bytes writes the value of z into buf using big-endian encoding.
    /// The value of z is encoded in the slice buf[i:]. If the value of z
    /// cannot be represented in buf, bytes panics. The number i of unused
    /// bytes at the beginning of buf is returned as result.
    pub fn bytes(&self, buf: Arc<Mutex<Option<Vec<u8>>>>) -> i32 {
    let mut i: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

                // This function is used in cryptographic operations. It must not leak
                // anything but the Int's sign and bit size through side-channels. Any
                // changes must be reviewed by a security expert.
        { let new_val = (*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; *i.lock().unwrap() = Some(new_val); };
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut d in __range_values.iter().cloned() {
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x < __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some((*d.0.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone();
    } else if { let __tmp_x = (*Arc::new(Mutex::new(Some((*d.0.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("math/big: buffer too small to fit value".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __rhs = crate::arith::Word(Arc::new(Mutex::new(Some(8 as u64)))); d = d >> __rhs; };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    }
        while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return (*i.lock().unwrap().as_ref().unwrap());
    }

    /// setBytes interprets buf as the bytes of a big-endian unsigned
    /// integer, sets z to that value, and returns z.
    pub fn set_bytes(&self, buf: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        { let new_val = __self.make(Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __tmp_x = ({ let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 8; __tmp_x + __tmp_y } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as i32); let __tmp_y = 8; __tmp_x / __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        let mut i = Arc::new(Mutex::new(Some((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        let mut k = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*(*big_endian_word(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x - __tmp_y }) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
        { let __rhs = 8; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut d: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
        let mut s = Arc::new(Mutex::new(Some(0 as u64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __rhs = (*({ let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() } as u64)))); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let __rhs = 8 as u64; let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __tmp_x = ({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
    }
        __self.norm()
    }

    /// sqrt sets z = ⌊√x⌋
    pub fn sqrt(&self, x: Arc<Mutex<Option<nat>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).cmp(natOne.clone()); let __tmp_y = 0; __tmp_x <= __tmp_y } {
        return __self.set(x.clone());
    }
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        *__self.0.lock().unwrap() = None;
    }
                // Start with value known to be too large and repeat "z = ⌊(z + ⌊x/z⌋)/2⌋" until it stops getting smaller.
                // See Brent and Zimmermann, Modern Computer Arithmetic, Algorithm 1.13 (SqrtInt).
                // https://members.loria.fr/PZimmermann/mca/pub226.html
                // If x is one less than a perfect square, the sequence oscillates between the correct z and z+1;
                // otherwise it converges to the correct z and stays there.
        let mut z1: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));let mut z2: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = __self.clone(); *z1.lock().unwrap() = Some(new_val); };
        { let new_val = (*z1.lock().unwrap().as_ref().unwrap()).set_uint64(Arc::new(Mutex::new(Some(1 as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z1.lock().unwrap() = __moved_val; };
        { let new_val = (*z1.lock().unwrap().as_ref().unwrap()).shl(z1.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).bit_len(); let __tmp_y = 1; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as u64; __tmp_x / __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z1.lock().unwrap() = __moved_val; };
        let mut n = Arc::new(Mutex::new(Some(0)));
    loop {
        { let (__tmp_0, __tmp_1) = (*z2.lock().unwrap().as_ref().unwrap()).div(Arc::new(Mutex::new(None)), x.clone(), z1.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *z2.lock().unwrap() = __moved_tmp_0; };
        { let new_val = (*z2.lock().unwrap().as_ref().unwrap()).add(z2.clone(), z1.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z2.lock().unwrap() = __moved_val; };
        { let new_val = (*z2.lock().unwrap().as_ref().unwrap()).shr(z2.clone(), Arc::new(Mutex::new(Some(1 as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z2.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*z2.lock().unwrap().as_ref().unwrap()).cmp(z1.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y } {
                // z1 is answer.
                // Figure out whether z1 or z2 is currently aliased to z by looking at loop count.
        if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return z1.clone();
    }
        return __self.set(z1.clone());
    }
                // z1 is answer.
                // Figure out whether z1 or z2 is currently aliased to z by looking at loop count.
        { let __tmp_0 = z2.clone(); let __tmp_1 = z1.clone(); *z1.lock().unwrap() = __tmp_0.lock().unwrap().take(); *z2.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    /// subMod2N returns z = (x - y) mod 2ⁿ.
    pub fn sub_mod2_n(&self, mut x: Arc<Mutex<Option<nat>>>, mut y: Arc<Mutex<Option<nat>>>, n: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<nat>>> {
        let mut __self = self.clone();
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()).bit_len() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // ok to overwrite x in place
        { let new_val = (*x.lock().unwrap().as_ref().unwrap()).trunc(x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).trunc(x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
    }
    }
                // ok to overwrite x in place
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()).bit_len() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        if alias(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // ok to overwrite y in place
        { let new_val = (*y.lock().unwrap().as_ref().unwrap()).trunc(y.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).trunc(y.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_val; };
    }
    }
                // ok to overwrite y in place
        if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).cmp(y.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        return __self.sub(x.clone(), y.clone());
    }
                // x - y < 0; x - y mod 2ⁿ = x - y + 2ⁿ = 2ⁿ - (y - x) = 1 + 2ⁿ-1 - (y - x) = 1 + ^(y - x).
        { let new_val = __self.sub(y.clone(), x.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        while { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = __self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as u64; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __base = __self.0.clone(); let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64))))); Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        for i in 0..(({ let __range_holder = __self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*__self.0.clone().lock().unwrap().as_mut().unwrap())[(i) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(!(*{ let __seq_holder = __self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(i) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))));
    }
        { let new_val = __self.trunc(Arc::new(Mutex::new(Some(__self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        __self.add(Arc::new(Mutex::new(Some(__self.clone()))), natOne.clone())
    }
}

/// basicMul multiplies x and y and leaves the result in z.
/// The (non-normalized) result is placed in z[0 : len(x) + len(y)].
pub fn basic_mul(z: Arc<Mutex<Option<nat>>>, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) {
    { let __clear_start = (0) as usize; let __clear_end = ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x + __tmp_y }) as usize; let __clear_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
    { let __range_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, d) in __range_values.iter().cloned().enumerate() {
        if { let __tmp_x = d.clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        (*{ let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = (i as i32); __tmp_x + __tmp_y }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*(*add_mul_v_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (i) as usize; let __high = ({ let __tmp_x = (i as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(d.clone())))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
    }
    } }
}

/// Fast version of z[0:n+n>>1].add(z[0:n+n>>1], x[0:n]) w/o bounds checks.
/// Factored out for readability - do not use outside karatsuba.
pub fn karatsuba_add(z: Arc<Mutex<Option<nat>>>, x: Arc<Mutex<Option<nat>>>, n: Arc<Mutex<Option<i32>>>) {
    {
        let mut c = add_v_v({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });;
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
            add_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
}

/// Like karatsubaAdd, but does subtract.
pub fn karatsuba_sub(z: Arc<Mutex<Option<nat>>>, x: Arc<Mutex<Option<nat>>>, n: Arc<Mutex<Option<i32>>>) {
    {
        let mut c = sub_v_v({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });;
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
            sub_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
}

/// karatsuba multiplies x and y and leaves the result in z.
/// Both x and y must have the same length n and n must be a
/// power of 2. The result vector z must have len(z) >= 6*n.
/// The (non-normalized) result is placed in z[0 : 2*n].
pub fn karatsuba(z: Arc<Mutex<Option<nat>>>, x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) {
    let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));

        // Switch to basic multiplication if numbers are odd or small.
        // (n is always even if karatsubaThreshold is even, but be
        // conservative)
    if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*karatsubaThreshold.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        basic_mul(Arc::new(Mutex::new(Some({ let __arg_holder = z.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }

        // n&1 == 0 && n >= karatsubaThreshold && n >= 2
        // Karatsuba multiplication is based on the observation that
        // for two numbers x and y with:
        //
        //   x = x1*b + x0
        //   y = y1*b + y0
        //
        // the product x*y can be obtained with 3 products z2, z1, z0
        // instead of 4:
        //
        //   x*y = x1*y1*b*b + (x1*y0 + x0*y1)*b + x0*y0
        //       =    z2*b*b +              z1*b +    z0
        //
        // with:
        //
        //   xd = x1 - x0
        //   yd = y0 - y1
        //
        //   z1 =      xd*yd                    + z2 + z0
        //      = (x1-x0)*(y0 - y1)             + z2 + z0
        //      = x1*y0 - x1*y1 - x0*y0 + x0*y1 + z2 + z0
        //      = x1*y0 -    z2 -    z0 + x0*y1 + z2 + z0
        //      = x1*y0                 + x0*y1
        // split x, y into "digits"
    let mut n2 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y })));
    let (mut x1, mut x0) = (Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))));
    let (mut y1, mut y0) = (Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))));

        // z is used for the result and temporary storage:
        //
        //   6*n     5*n     4*n     3*n     2*n     1*n     0*n
        // z = [z2 copy|z0 copy| xd*yd | yd:xd | x1*y1 | x0*y0 ]
        //
        // For each recursive call of karatsuba, an unused slice of
        // z is passed in that has (at least) half the length of the
        // caller's z.
        // compute z0 and z2 with the result "in place" in z
    karatsuba(Arc::new(Mutex::new(Some({ let __arg_holder = z.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = x0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    karatsuba(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // compute xd (or the negative value if underflow occurs)
    let mut s = Arc::new(Mutex::new(Some(1)));
    let mut xd = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as usize; let __high = ({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    if { let __tmp_x = (*sub_v_v({ let __named_slice = (*xd.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x1.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x0.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let new_val = -((*s.lock().unwrap().as_ref().unwrap())); *s.lock().unwrap() = Some(new_val); };
        sub_v_v({ let __named_slice = (*xd.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x0.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x1.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
    }

        // x0-x1
        // compute yd (or the negative value if underflow occurs)
    let mut yd = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __high = ({ let __tmp_x = 3; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    if { let __tmp_x = (*sub_v_v({ let __named_slice = (*yd.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*y0.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*y1.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let new_val = -((*s.lock().unwrap().as_ref().unwrap())); *s.lock().unwrap() = Some(new_val); };
        sub_v_v({ let __named_slice = (*yd.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*y1.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*y0.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
    }

        // y1-y0
        // p = (x1-x0)*(y0-y1) == x1*y0 - x1*y1 - x0*y0 + x0*y1 for s > 0
        // p = (x0-x1)*(y0-y1) == x0*y0 - x0*y1 - x1*y0 + x1*y1 for s < 0
    let mut p = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x * __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    karatsuba(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = xd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = yd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // save original z2:z0
        // (ok to use upper half of z since we're done recurring)
    let mut r = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x * __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    { let _dst_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let _src = { let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }) as usize].to_vec() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };

        // add up all partial products
        //
        //   2*n     n     0
        // z = [ z2  | z0  ]
        //   +    [ z0  ]
        //   +    [ z2  ]
        //   +    [  p  ]
        //
    karatsuba_add(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    karatsuba_add(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        karatsuba_add(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    } else {
        karatsuba_sub(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    }
}

/// alias reports whether x and y share the same base array.
///
/// Note: alias assumes that the capacity of underlying arrays
/// is never changed for nat values; i.e. that there are
/// no 3-operand slice expressions in this code (or worse,
/// reflect-based operations to the same effect).
pub fn alias(x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> bool {
    return { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __left = GoSliceElemPtr::new({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize); let __right = GoSliceElemPtr::new({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize); let __eq = Arc::ptr_eq(&__left.slice, &__right.slice) && __left.index == __right.index; __eq };
}

/// addAt implements z += x<<(_W*i); z must be long enough.
/// (we don't use nat.add because we need z to stay the same
/// slice, and we don't need to normalize z after each addition)
pub fn add_at(z: Arc<Mutex<Option<nat>>>, x: Arc<Mutex<Option<nat>>>, i: Arc<Mutex<Option<i32>>>) {
    {
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            {
        let mut c = add_v_v({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });;
        if { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
            let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));;
            if { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x < __tmp_y } {
        add_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    };
        }
    };
        }
    }
}

/// karatsubaLen computes an approximation to the maximum k <= n such that
/// k = p<<i for a number p <= threshold and an i >= 0. Thus, the
/// result is the largest number that can be divided repeatedly by 2 before
/// becoming about the value of threshold.
pub fn karatsuba_len(mut n: Arc<Mutex<Option<i32>>>, threshold: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut i = Arc::new(Mutex::new(Some(0 as u64)));
    while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*threshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let __rhs = 1; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y };
}

/// basicSqr sets z = x*x and is asymptotically faster than basicMul
/// by about a factor of 2, but slower for small arguments due to overhead.
/// Requirements: len(x) > 0, len(z) == 2*len(x)
/// The (non-normalized) result is placed in z.
pub fn basic_sqr(z: Arc<Mutex<Option<nat>>>, x: Arc<Mutex<Option<nat>>>) {
    let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
    let mut tp = get_nat(Arc::new(Mutex::new(Some({ let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y } as i32))));
    let mut t = Arc::new(Mutex::new(Some({ let __v = (*tp.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    { let __clear_holder = { let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = crate::arith::Word(Arc::new(Mutex::new(Some(0)))); } } };
    { let (__tmp_0, __tmp_1) = mul_w_w(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))), Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))))); (*{ let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[(1) as usize] = __tmp_0.lock().unwrap().take().unwrap_or_default(); (*{ let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[(0) as usize] = __tmp_1.lock().unwrap().take().unwrap_or_default(); };
    let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut d = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));

                // z collects the squares x[i] * x[i]
        { let (__tmp_0, __tmp_1) = mul_w_w(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); (*{ let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] = __tmp_0.lock().unwrap().take().unwrap_or_default(); (*{ let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = 2; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as usize] = __tmp_1.lock().unwrap().take().unwrap_or_default(); };

                // t collects the products x[i] * x[j] where j < i
        (*{ let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = 2; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*(*add_mul_v_v_w({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = 2; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // z collects the squares x[i] * x[i]
        // t collects the products x[i] * x[j] where j < i
    (*{ let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some((*(*shl_v_u({ let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (1) as usize; let __high = ({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, { let __named_slice = nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (1) as usize; let __high = ({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __named_slice.0.clone() }, Arc::new(Mutex::new(Some(1 as u64)))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
    add_v_v({ let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
    put_nat(tp.clone());
}

/// karatsubaSqr squares x and leaves the result in z.
/// len(x) must be a power of 2 and len(z) >= 6*len(x).
/// The (non-normalized) result is placed in z[0 : 2*len(x)].
///
/// The algorithm and the layout of z are the same as for karatsuba.
pub fn karatsuba_sqr(z: Arc<Mutex<Option<nat>>>, x: Arc<Mutex<Option<nat>>>) {
    let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));

    if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*karatsubaSqrThreshold.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        basic_sqr(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }

    let mut n2 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y })));
    let (mut x1, mut x0) = (Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))));

    karatsuba_sqr(Arc::new(Mutex::new(Some({ let __arg_holder = z.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = x0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    karatsuba_sqr(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // s = sign(xd*yd) == -1 for xd != 0; s == 1 for xd == 0
    let mut xd = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as usize; let __high = ({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    if { let __tmp_x = (*sub_v_v({ let __named_slice = (*xd.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x1.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x0.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        sub_v_v({ let __named_slice = (*xd.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x0.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*x1.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice });
    }

    let mut p = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x * __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    karatsuba_sqr(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = xd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    let mut r = Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x * __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))))));
    { let _dst_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let _src = { let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }) as usize].to_vec() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };

    karatsuba_add(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    karatsuba_add(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    karatsuba_sub(Arc::new(Mutex::new(Some(nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = ({ let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
}

/// getNat returns a *nat of len n. The contents may not be zero.
/// The pool holds *nat to avoid allocation when converting to interface{}.
pub fn get_nat(n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<nat>>> {
    let mut z: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(None));
    {
        let mut v = (*natPool.lock().unwrap().as_mut().unwrap()).get();;
        if { let __nil_result = (*v.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = ({
        let val = v.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<nat>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone(); z = new_val; };;
        }
    }
    if { let __nil_result = (*z.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(nat::default()))).clone(); z = new_val; };
    }
    { let new_val = (*{ let __recv = z.clone(); let __recv_ptr: *const nat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const nat }; let __result = unsafe { &*__recv_ptr }.make(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *z.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*{ let __named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[(0) as usize] = crate::arith::Word(Arc::new(Mutex::new(Some(0xfedcb as u64))));
    }
        // break code expecting zero
    return z.clone();
}

pub fn put_nat(x: Arc<Mutex<Option<nat>>>) {
    (*natPool.lock().unwrap().as_mut().unwrap()).put(x.clone());
}

pub fn same(x: Arc<Mutex<Option<nat>>>, y: Arc<Mutex<Option<nat>>>) -> bool {
    return { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x == __tmp_y } && { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __left = GoSliceElemPtr::new({ let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, (0) as usize); let __right = GoSliceElemPtr::new({ let __named_slice = (*y.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, (0) as usize); let __eq = Arc::ptr_eq(&__left.slice, &__right.slice) && __left.index == __right.index; __eq };
}

/// bigEndianWord returns the contents of buf interpreted as a big-endian encoded Word value.
pub fn big_endian_word(buf: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<crate::arith::Word>>> {
    if { let __tmp_x = __W; let __tmp_y = 64; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(byteorder::b_e_uint64(buf.clone()) as u64)))))));
    }
    Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(byteorder::b_e_uint32(buf.clone()) as u64)))))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
