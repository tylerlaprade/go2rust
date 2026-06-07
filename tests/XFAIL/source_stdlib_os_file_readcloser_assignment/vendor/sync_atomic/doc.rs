use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::doc_64::*;
use crate::r#type::*;
use crate::value::*;

use std::sync::{Arc, Mutex};

/// SwapInt32 atomically stores new into *addr and returns the previous *addr value.
/// Consider using the more ergonomic and less error-prone [Int32.Swap] instead.
///
///go:noescape
pub fn swap_int32(addr: Arc<Mutex<Option<i32>>>, new: Arc<Mutex<Option<i32>>>) -> i32 {
    let __new = (*new.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() = __new;
    __old
}


/// SwapUint32 atomically stores new into *addr and returns the previous *addr value.
/// Consider using the more ergonomic and less error-prone [Uint32.Swap] instead.
///
///go:noescape
pub fn swap_uint32(addr: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> u32 {
    let __new = (*new.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() = __new;
    __old
}


/// SwapPointer atomically stores new into *addr and returns the previous *addr value.
/// Consider using the more ergonomic and less error-prone [Pointer.Swap] instead.
pub fn swap_pointer(addr: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    let __new = new.lock().unwrap().as_ref().copied().unwrap_or(0);
    let mut __guard = addr.lock().unwrap();
    let __old = __guard.as_ref().copied().unwrap_or(0);
    *__guard.as_mut().unwrap() = __new;
    Arc::new(Mutex::new(Some(__old)))
}


/// CompareAndSwapInt32 executes the compare-and-swap operation for an int32 value.
/// Consider using the more ergonomic and less error-prone [Int32.CompareAndSwap] instead.
///
///go:noescape
pub fn compare_and_swap_int32(addr: Arc<Mutex<Option<i32>>>, old: Arc<Mutex<Option<i32>>>, new: Arc<Mutex<Option<i32>>>) -> bool {
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


/// CompareAndSwapUint32 executes the compare-and-swap operation for a uint32 value.
/// Consider using the more ergonomic and less error-prone [Uint32.CompareAndSwap] instead.
///
///go:noescape
pub fn compare_and_swap_uint32(addr: Arc<Mutex<Option<u32>>>, old: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> bool {
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


/// CompareAndSwapPointer executes the compare-and-swap operation for a unsafe.Pointer value.
/// Consider using the more ergonomic and less error-prone [Pointer.CompareAndSwap] instead.
pub fn compare_and_swap_pointer(addr: Arc<Mutex<Option<usize>>>, old: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> bool {
    let __old = old.lock().unwrap().as_ref().copied().unwrap_or(0);
    let __new = new.lock().unwrap().as_ref().copied().unwrap_or(0);
    let mut __guard = addr.lock().unwrap();
    if __guard.as_ref().copied().unwrap_or(0) == __old {
        *__guard.as_mut().unwrap() = __new;
        true
    } else {
        false
    }
}


/// AddInt32 atomically adds delta to *addr and returns the new value.
/// Consider using the more ergonomic and less error-prone [Int32.Add] instead.
///
///go:noescape
pub fn add_int32(addr: Arc<Mutex<Option<i32>>>, delta: Arc<Mutex<Option<i32>>>) -> i32 {
    let __delta = (*delta.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __current = (*__guard.as_ref().unwrap()).clone();
    let __new = __current.wrapping_add(__delta);
    *__guard.as_mut().unwrap() = __new;
    __new
}


/// AddUint32 atomically adds delta to *addr and returns the new value.
/// To subtract a signed positive constant value c from x, do AddUint32(&x, ^uint32(c-1)).
/// In particular, to decrement x, do AddUint32(&x, ^uint32(0)).
/// Consider using the more ergonomic and less error-prone [Uint32.Add] instead.
///
///go:noescape
pub fn add_uint32(addr: Arc<Mutex<Option<u32>>>, delta: Arc<Mutex<Option<u32>>>) -> u32 {
    let __delta = (*delta.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __current = (*__guard.as_ref().unwrap()).clone();
    let __new = __current.wrapping_add(__delta);
    *__guard.as_mut().unwrap() = __new;
    __new
}


/// AndInt32 atomically performs a bitwise AND operation on *addr using the bitmask provided as mask
/// and returns the old value.
/// Consider using the more ergonomic and less error-prone [Int32.And] instead.
///
///go:noescape
pub fn and_int32(addr: Arc<Mutex<Option<i32>>>, mask: Arc<Mutex<Option<i32>>>) -> i32 {
    let __mask = (*mask.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() &= __mask;
    __old
}


/// AndUint32 atomically performs a bitwise AND operation on *addr using the bitmask provided as mask
/// and returns the old value.
/// Consider using the more ergonomic and less error-prone [Uint32.And] instead.
///
///go:noescape
pub fn and_uint32(addr: Arc<Mutex<Option<u32>>>, mask: Arc<Mutex<Option<u32>>>) -> u32 {
    let __mask = (*mask.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() &= __mask;
    __old
}


/// OrInt32 atomically performs a bitwise OR operation on *addr using the bitmask provided as mask
/// and returns the old value.
/// Consider using the more ergonomic and less error-prone [Int32.Or] instead.
///
///go:noescape
pub fn or_int32(addr: Arc<Mutex<Option<i32>>>, mask: Arc<Mutex<Option<i32>>>) -> i32 {
    let __mask = (*mask.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() |= __mask;
    __old
}


/// OrUint32 atomically performs a bitwise OR operation on *addr using the bitmask provided as mask
/// and returns the old value.
/// Consider using the more ergonomic and less error-prone [Uint32.Or] instead.
///
///go:noescape
pub fn or_uint32(addr: Arc<Mutex<Option<u32>>>, mask: Arc<Mutex<Option<u32>>>) -> u32 {
    let __mask = (*mask.lock().unwrap().as_ref().unwrap()).clone();
    let mut __guard = addr.lock().unwrap();
    let __old = (*__guard.as_ref().unwrap()).clone();
    *__guard.as_mut().unwrap() |= __mask;
    __old
}


/// LoadInt32 atomically loads *addr.
/// Consider using the more ergonomic and less error-prone [Int32.Load] instead.
///
///go:noescape
pub fn load_int32(addr: Arc<Mutex<Option<i32>>>) -> i32 {
    (*addr.lock().unwrap().as_ref().unwrap()).clone()
}


/// LoadUint32 atomically loads *addr.
/// Consider using the more ergonomic and less error-prone [Uint32.Load] instead.
///
///go:noescape
pub fn load_uint32(addr: Arc<Mutex<Option<u32>>>) -> u32 {
    (*addr.lock().unwrap().as_ref().unwrap()).clone()
}


/// LoadUintptr atomically loads *addr.
/// Consider using the more ergonomic and less error-prone [Uintptr.Load] instead.
///
///go:noescape
pub fn load_uintptr(addr: Arc<Mutex<Option<usize>>>) -> usize {
    (*addr.lock().unwrap().as_ref().unwrap()).clone()
}


/// LoadPointer atomically loads *addr.
/// Consider using the more ergonomic and less error-prone [Pointer.Load] instead.
pub fn load_pointer(addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    let __value = addr.lock().unwrap().as_ref().copied().unwrap_or(0);
    Arc::new(Mutex::new(Some(__value)))
}


/// StoreInt32 atomically stores val into *addr.
/// Consider using the more ergonomic and less error-prone [Int32.Store] instead.
///
///go:noescape
pub fn store_int32(addr: Arc<Mutex<Option<i32>>>, val: Arc<Mutex<Option<i32>>>) {
    let __val = (*val.lock().unwrap().as_ref().unwrap()).clone();
    *addr.lock().unwrap().as_mut().unwrap() = __val;
}


/// StoreUint32 atomically stores val into *addr.
/// Consider using the more ergonomic and less error-prone [Uint32.Store] instead.
///
///go:noescape
pub fn store_uint32(addr: Arc<Mutex<Option<u32>>>, val: Arc<Mutex<Option<u32>>>) {
    let __val = (*val.lock().unwrap().as_ref().unwrap()).clone();
    *addr.lock().unwrap().as_mut().unwrap() = __val;
}


/// StoreUintptr atomically stores val into *addr.
/// Consider using the more ergonomic and less error-prone [Uintptr.Store] instead.
///
///go:noescape
pub fn store_uintptr(addr: Arc<Mutex<Option<usize>>>, val: Arc<Mutex<Option<usize>>>) {
    let __val = (*val.lock().unwrap().as_ref().unwrap()).clone();
    *addr.lock().unwrap().as_mut().unwrap() = __val;
}


/// StorePointer atomically stores val into *addr.
/// Consider using the more ergonomic and less error-prone [Pointer.Store] instead.
pub fn store_pointer(addr: Arc<Mutex<Option<usize>>>, val: Arc<Mutex<Option<usize>>>) {
    let __val = val.lock().unwrap().as_ref().copied().unwrap_or(0);
    let mut __guard = addr.lock().unwrap();
    *__guard.as_mut().unwrap() = __val;
}
