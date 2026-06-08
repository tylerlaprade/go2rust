use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoLocalPtrKey,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{zerrors_darwin_arm64::{E_I_N_V_A_L}, ztypes_darwin_arm64::{Timespec}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) static _zero: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_zero.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *_zero.lock().unwrap() = Some(0);
}


impl crate::ztypes_darwin_arm64::Timespec {
    /// Unix returns the time stored in ts as seconds plus nanoseconds.
    pub fn unix(&self) -> (i64, i64) {
    let mut sec: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut nsec: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        (
            (*Arc::new(Mutex::new(Some({ let __selector_holder = self.sec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()),
            (*Arc::new(Mutex::new(Some({ let __selector_holder = self.nsec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap())
        )
    }

    /// Nano returns the time stored in ts as nanoseconds.
    pub fn nano(&self) -> i64 {
        return {
            let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.sec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1e9 as i64; __tmp_x * __tmp_y };
            let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.nsec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap());
            __tmp_x + __tmp_y
        };
    }
}

/// ByteSliceFromString returns a NUL-terminated slice of bytes
/// containing the text of s. If s contains a NUL byte at any
/// location, it returns (nil, [EINVAL]).
pub fn byte_slice_from_string(s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __tmp_x = internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as u8)))); let __tmp_y = -1; __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
    let mut a = Arc::new(Mutex::new(Some(vec![0; ({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x + __tmp_y }) as usize])));
    {
        let _src = (*s.lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec();
        let _n = std::cmp::min((*a.lock().unwrap().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*a.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i32)))
    };
    return (a.clone(), Arc::new(Mutex::new(None)));
}

/// BytePtrFromString returns a pointer to a NUL-terminated array of
/// bytes containing the text of s. If s contains a NUL byte at any
/// location, it returns (nil, [EINVAL]).
pub fn byte_ptr_from_string(s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut a, mut err) = byte_slice_from_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
    return (unimplemented!("slice element pointer return requires pointer representation support"), Arc::new(Mutex::new(None)));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
