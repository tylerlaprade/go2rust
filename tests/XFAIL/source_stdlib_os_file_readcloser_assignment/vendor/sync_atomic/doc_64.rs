use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use std::sync::{Arc, Mutex};

/// SwapUint64 atomically stores new into *addr and returns the previous *addr value.
/// Consider using the more ergonomic and less error-prone [Uint64.Swap] instead
/// (particularly if you target 32-bit platforms; see the bugs section).
///
///go:noescape
pub fn swap_uint64(addr: Arc<Mutex<Option<u64>>>, new: Arc<Mutex<Option<u64>>>) -> u64 {
    let __new = (*new.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() = __new;
    __old
}


/// CompareAndSwapUint64 executes the compare-and-swap operation for a uint64 value.
/// Consider using the more ergonomic and less error-prone [Uint64.CompareAndSwap] instead
/// (particularly if you target 32-bit platforms; see the bugs section).
///
///go:noescape
pub fn compare_and_swap_uint64(addr: Arc<Mutex<Option<u64>>>, old: Arc<Mutex<Option<u64>>>, new: Arc<Mutex<Option<u64>>>) -> bool {
    let __old = (*old.lock().unwrap().as_ref().unwrap()).clone();
    let __new = (*new.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    if *__guard.as_ref().unwrap() == __old {
        *__guard.as_mut().unwrap() = __new;
        true
    } else {
        false
    }
}


/// AddUint64 atomically adds delta to *addr and returns the new value.
/// To subtract a signed positive constant value c from x, do AddUint64(&x, ^uint64(c-1)).
/// In particular, to decrement x, do AddUint64(&x, ^uint64(0)).
/// Consider using the more ergonomic and less error-prone [Uint64.Add] instead
/// (particularly if you target 32-bit platforms; see the bugs section).
///
///go:noescape
pub fn add_uint64(addr: Arc<Mutex<Option<u64>>>, delta: Arc<Mutex<Option<u64>>>) -> u64 {
    let __delta = (*delta.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __current = (*__guard.as_ref().unwrap()).clone();
    let __new = __current.wrapping_add(__delta);
    *__guard.as_mut().unwrap() = __new;
    __new
}


/// AndUint64 atomically performs a bitwise AND operation on *addr using the bitmask provided as mask
/// and returns the old.
/// Consider using the more ergonomic and less error-prone [Uint64.And] instead.
///
///go:noescape
pub fn and_uint64(addr: Arc<Mutex<Option<u64>>>, mask: Arc<Mutex<Option<u64>>>) -> u64 {
    let __mask = (*mask.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() &= __mask;
    __old
}


/// OrUint64 atomically performs a bitwise OR operation on *addr using the bitmask provided as mask
/// and returns the old value.
/// Consider using the more ergonomic and less error-prone [Uint64.Or] instead.
///
///go:noescape
pub fn or_uint64(addr: Arc<Mutex<Option<u64>>>, mask: Arc<Mutex<Option<u64>>>) -> u64 {
    let __mask = (*mask.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() |= __mask;
    __old
}


/// LoadUint64 atomically loads *addr.
/// Consider using the more ergonomic and less error-prone [Uint64.Load] instead
/// (particularly if you target 32-bit platforms; see the bugs section).
///
///go:noescape
pub fn load_uint64(addr: Arc<Mutex<Option<u64>>>) -> u64 {
    (*addr.lock().unwrap().as_ref().unwrap()).clone()
}


/// StoreUint64 atomically stores val into *addr.
/// Consider using the more ergonomic and less error-prone [Uint64.Store] instead
/// (particularly if you target 32-bit platforms; see the bugs section).
///
///go:noescape
pub fn store_uint64(addr: Arc<Mutex<Option<u64>>>, val: Arc<Mutex<Option<u64>>>) {
    let __val = (*val.lock().unwrap().as_ref().unwrap()).clone();
    *addr.lock().unwrap().as_mut().unwrap() = __val;
}
