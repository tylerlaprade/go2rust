use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// errNetClosing is the type of the variable ErrNetClosing.
/// This is used to implement the net.Error interface.
#[derive(Debug, Clone, Default)]
pub struct errNetClosing {
}

impl errNetClosing {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for errNetClosing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for errNetClosing {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// DeadlineExceededError is returned for an expired deadline.
#[derive(Debug, Clone, Default)]
pub struct DeadlineExceededError {
}

impl DeadlineExceededError {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for DeadlineExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for DeadlineExceededError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub static ErrNetClosing: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<errNetClosing>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrFileClosing: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrNoDeadline: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrDeadlineExceeded: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrNotPollable: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static TestHookDidWritev: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrNetClosing.lock().unwrap() = Some(Default::default());
    *ErrFileClosing.lock().unwrap() = None;
    *ErrNoDeadline.lock().unwrap() = None;
    *ErrDeadlineExceeded.lock().unwrap() = None;
    *ErrNotPollable.lock().unwrap() = None;
    *ErrNetClosing.lock().unwrap() = Some(errNetClosing {  });
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("use of closed file".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrFileClosing.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("file type does not support deadline".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNoDeadline.lock().unwrap() = new_val; }
    *ErrDeadlineExceeded.lock().unwrap() = Some(Box::new(DeadlineExceededError {  }) as Box<dyn StdError + Send + Sync>);
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("not pollable".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNotPollable.lock().unwrap() = new_val; }
    *TestHookDidWritev.lock().unwrap() = Some(Box::new(move |wrote: Arc<Mutex<Option<i32>>>| {
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>);
}


pub(crate) fn __go_zero_globals() {
    *ErrNetClosing.lock().unwrap() = Some(Default::default());
    *ErrFileClosing.lock().unwrap() = None;
    *ErrNoDeadline.lock().unwrap() = None;
    *ErrDeadlineExceeded.lock().unwrap() = None;
    *ErrNotPollable.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_3() {
    *ErrNetClosing.lock().unwrap() = Some(errNetClosing {  });
}


pub(crate) fn __go_init_order_4() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("use of closed file".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrFileClosing.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_5() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("file type does not support deadline".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNoDeadline.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_6() {
    *ErrDeadlineExceeded.lock().unwrap() = Some(Box::new(DeadlineExceededError {  }) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_7() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("not pollable".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNotPollable.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_8() {
    *TestHookDidWritev.lock().unwrap() = Some(Box::new(move |wrote: Arc<Mutex<Option<i32>>>| {
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>);
}


impl errNetClosing {
    /// Error returns the error message for ErrNetClosing.
    /// Keep this string consistent because of issue #4373:
    /// since historically programs have not been able to detect
    /// this error, they look for the string.
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("use of closed network connection".to_string())))
    }

    pub fn timeout(&self) -> bool {
        false
    }

    pub fn temporary(&self) -> bool {
        false
    }
}

impl StdError for errNetClosing {}


impl DeadlineExceededError {
    /// Implement the net.Error interface.
    /// The string is "i/o timeout" because that is what was returned
    /// by earlier Go versions. Changing it may break programs that
    /// match on error strings.
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("i/o timeout".to_string())))
    }

    pub fn timeout(&self) -> bool {
        true
    }

    pub fn temporary(&self) -> bool {
        true
    }
}

impl StdError for DeadlineExceededError {}


/// Return the appropriate closing error based on isFile.
pub fn err_closing(isFile: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    if { let __v = (*isFile.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return ErrFileClosing.clone();
    }
    Arc::new(Mutex::new(Some(Box::new((*ErrNetClosing.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>)))
}

/// consume removes data from a slice of byte slices, for writev.
pub fn consume(v: Arc<Mutex<Option<Vec<Vec<u8>>>>>, mut n: Arc<Mutex<Option<i64>>>) {
    while { let __tmp_x = ({ let __slice_holder = v.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut ln0 = Arc::new(Mutex::new(Some({ let __seq = ({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }); __seq[(0) as usize].clone() }.len() as i64)));
        if { let __tmp_x = { let __v = (*ln0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let __slice_holder = v.clone(); let mut __slice_guard = __slice_holder.lock().unwrap(); let __slice = __slice_guard.as_mut().unwrap(); __slice[(0) as usize] = (*Arc::new(Mutex::new(Some({ let mut __seq = { let __seq = ({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }); __seq[(0) as usize].clone() }; let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __seq.capacity(); if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); };
        return;
    }
        { let __rhs = (*ln0.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let __slice_holder = v.clone(); let mut __slice_guard = __slice_holder.lock().unwrap(); let __slice = __slice_guard.as_mut().unwrap(); __slice[(0) as usize] = vec![]; };
        { let new_val = Arc::new(Mutex::new(Some({ let __slice_holder = v.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); let __cloned_val = { let __guard = new_val.lock().unwrap(); (*__guard).clone() }; *v.lock().unwrap() = __cloned_val; };
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for errNetClosing {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for DeadlineExceededError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
