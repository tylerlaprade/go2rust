use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use std::sync::{Arc, Mutex};

pub(crate) const C0: usize = ((((((8 as usize) - (internal_goarch::PTR_SIZE as usize)) / (4 as usize)) * (2860486313 as usize)) + ((((internal_goarch::PTR_SIZE as usize) - (4 as usize)) / (4 as usize)) * (33054211828000289 as usize))) as usize);
pub(crate) const C1: usize = ((((((8 as usize) - (internal_goarch::PTR_SIZE as usize)) / (4 as usize)) * (3267000013 as usize)) + ((((internal_goarch::PTR_SIZE as usize) - (4 as usize)) / (4 as usize)) * (23344194077549503 as usize))) as usize);


pub(crate) const HASH_RANDOM_BYTES: i32 = internal_goarch::PTR_SIZE / 4 * 64;


pub(crate) static useAeshash: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static aeskeysched: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 128]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static hashkey: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[usize; 4]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *useAeshash.lock().unwrap() = Some(false);
    *aeskeysched.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *hashkey.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_zero_globals() {
    *useAeshash.lock().unwrap() = Some(false);
    *aeskeysched.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *hashkey.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


/// memhash should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/aacfactory/fns
///   - github.com/dgraph-io/ristretto
///   - github.com/minio/simdjson-go
///   - github.com/nbd-wtf/go-nostr
///   - github.com/outcaste-io/ristretto
///   - github.com/puzpuzpuz/xsync/v2
///   - github.com/puzpuzpuz/xsync/v3
///   - github.com/authzed/spicedb
///   - github.com/pingcap/badger
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname memhash
pub fn memhash(p: Arc<Mutex<Option<usize>>>, h: Arc<Mutex<Option<usize>>>, s: Arc<Mutex<Option<usize>>>) -> usize {
    unimplemented!("Go function declaration has no body");
}


pub fn read_unaligned64(p: Arc<Mutex<Option<usize>>>) -> u64 {
    let mut q: GoPtr<[u8; 8]> = GoPtr::raw({ let __ptr = p.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    if internal_goarch::BIG_ENDIAN {
        return {
            let __go_binary_0 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(7) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(6) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_2 = 8;
            let __go_binary_3 = __go_binary_1 << __go_binary_2;
            let __go_binary_4 = __go_binary_0 | __go_binary_3;
            let __go_binary_5 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(5) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_6 = 16;
            let __go_binary_7 = __go_binary_5 << __go_binary_6;
            let __go_binary_8 = __go_binary_4 | __go_binary_7;
            let __go_binary_9 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(4) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_10 = 24;
            let __go_binary_11 = __go_binary_9 << __go_binary_10;
            let __go_binary_12 = __go_binary_8 | __go_binary_11;
            let __go_binary_13 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(3) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_14 = 32;
            let __go_binary_15 = __go_binary_13 << __go_binary_14;
            let __go_binary_16 = __go_binary_12 | __go_binary_15;
            let __go_binary_17 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(2) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_18 = 40;
            let __go_binary_19 = __go_binary_17 << __go_binary_18;
            let __go_binary_20 = __go_binary_16 | __go_binary_19;
            let __go_binary_21 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(1) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_22 = 48;
            let __go_binary_23 = __go_binary_21 << __go_binary_22;
            let __go_binary_24 = __go_binary_20 | __go_binary_23;
            let __go_binary_25 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_26 = 56;
            let __go_binary_27 = __go_binary_25 << __go_binary_26;
            let __go_binary_28 = __go_binary_24 | __go_binary_27;
            __go_binary_28
        };
    }
    return {
            let __go_binary_0 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(1) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_2 = 8;
            let __go_binary_3 = __go_binary_1 << __go_binary_2;
            let __go_binary_4 = __go_binary_0 | __go_binary_3;
            let __go_binary_5 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(2) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_6 = 16;
            let __go_binary_7 = __go_binary_5 << __go_binary_6;
            let __go_binary_8 = __go_binary_4 | __go_binary_7;
            let __go_binary_9 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(3) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_10 = 24;
            let __go_binary_11 = __go_binary_9 << __go_binary_10;
            let __go_binary_12 = __go_binary_8 | __go_binary_11;
            let __go_binary_13 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(4) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_14 = 32;
            let __go_binary_15 = __go_binary_13 << __go_binary_14;
            let __go_binary_16 = __go_binary_12 | __go_binary_15;
            let __go_binary_17 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(5) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_18 = 40;
            let __go_binary_19 = __go_binary_17 << __go_binary_18;
            let __go_binary_20 = __go_binary_16 | __go_binary_19;
            let __go_binary_21 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(6) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_22 = 48;
            let __go_binary_23 = __go_binary_21 << __go_binary_22;
            let __go_binary_24 = __go_binary_20 | __go_binary_23;
            let __go_binary_25 = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(7) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_26 = 56;
            let __go_binary_27 = __go_binary_25 << __go_binary_26;
            let __go_binary_28 = __go_binary_24 | __go_binary_27;
            __go_binary_28
        };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
