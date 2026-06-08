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

use crate::{fd::{err_closing}, fd_unix::{FD}};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MUTEX_CLOSED: i32 = 1 << 0;
pub(crate) const MUTEX_R_LOCK: i32 = 1 << 1;
pub(crate) const MUTEX_W_LOCK: i32 = 1 << 2;
pub(crate) const MUTEX_REF: i32 = 1 << 3;
pub(crate) const MUTEX_REF_MASK: i32 = ((1 << 20) - 1) << 3;
pub(crate) const MUTEX_R_WAIT: i32 = 1 << 23;
pub(crate) const MUTEX_R_MASK: i64 = ((1 << 20) - 1) << 23;
pub(crate) const MUTEX_W_WAIT: i64 = 1 << 43;
pub(crate) const MUTEX_W_MASK: i64 = ((1 << 20) - 1) << 43;


pub(crate) const OVERFLOW_MSG: &'static str = "too many concurrent operations on a single file or socket (max 1048575)";


/// fdMutex is a specialized synchronization primitive that manages
/// lifetime of an fd and serializes access to Read, Write and Close
/// methods on FD.
#[derive(Debug, Clone)]
pub struct fdMutex {
    pub state: Arc<Mutex<Option<u64>>>,
    pub rsema: Arc<Mutex<Option<u32>>>,
    pub wsema: Arc<Mutex<Option<u32>>>,
}

impl fdMutex {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.rsema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.wsema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            state: __go_clone_0_0,
            rsema: __go_clone_1_0,
            wsema: __go_clone_2_0,
        }
    }
}


impl Default for fdMutex {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            state: __go_default_0_0,
            rsema: __go_default_1_0,
            wsema: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for fdMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.state.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.rsema.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.wsema.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl fdMutex {
    /// incref adds a reference to mu.
    /// It reports whether mu is available for reading or writing.
    pub fn incref(&self) -> bool {
        loop {
        let mut old = sync_atomic::load_uint64(self.state.clone());
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = MUTEX_CLOSED as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return false;
    }
        let mut new = Arc::new(Mutex::new(Some({ let __tmp_x = old; let __tmp_y = MUTEX_REF as u64; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_REF_MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new(OVERFLOW_MSG) as Box<dyn Any + Send + Sync>);
    }
        if sync_atomic::compare_and_swap_uint64(self.state.clone(), Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    }
    }
    }

    /// increfAndClose sets the state of mu to closed.
    /// It returns false if the file was already closed.
    pub fn incref_and_close(&self) -> bool {
        loop {
        let mut old = sync_atomic::load_uint64(self.state.clone());
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = MUTEX_CLOSED as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return false;
    }

                // Mark as closed and acquire a reference.
        let mut new = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = old; let __tmp_y = MUTEX_CLOSED as u64; __tmp_x | __tmp_y }); let __tmp_y = MUTEX_REF as u64; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_REF_MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new(OVERFLOW_MSG) as Box<dyn Any + Send + Sync>);
    }

                // Remove all read and write waiters.
        { let __rhs = ((MUTEX_R_MASK as u64) | (MUTEX_W_MASK as u64)) as u64; let mut guard = new.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        if sync_atomic::compare_and_swap_uint64(self.state.clone(), Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // Wake all read and write waiters,
                // they will observe closed flag after wakeup.
        while { let __tmp_x = { let __tmp_x = old; let __tmp_y = MUTEX_R_MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = MUTEX_R_WAIT as u64; old = old - __rhs; };
        runtime__semrelease(self.rsema.clone());
    }
        while { let __tmp_x = { let __tmp_x = old; let __tmp_y = MUTEX_W_MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = MUTEX_W_WAIT as u64; old = old - __rhs; };
        runtime__semrelease(self.wsema.clone());
    }
        return true;
    }
    }
    }

    /// decref removes a reference from mu.
    /// It reports whether there is no remaining reference.
    pub fn decref(&self) -> bool {
        loop {
        let mut old = sync_atomic::load_uint64(self.state.clone());
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = MUTEX_REF_MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("inconsistent poll.fdMutex".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut new = Arc::new(Mutex::new(Some({ let __tmp_x = old; let __tmp_y = MUTEX_REF as u64; __tmp_x - __tmp_y })));
        if sync_atomic::compare_and_swap_uint64(self.state.clone(), Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return { let __tmp_x = { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((MUTEX_CLOSED as u64) | (MUTEX_REF_MASK as u64)) as u64; __tmp_x & __tmp_y }; let __tmp_y = MUTEX_CLOSED as u64; __tmp_x == __tmp_y };
    }
    }
    }

    /// lock adds a reference to mu and locks mu.
    /// It reports whether mu is available for reading or writing.
    pub fn rwlock(&self, read: Arc<Mutex<Option<bool>>>) -> bool {
        let mut mutexBit: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut mutexWait: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut mutexMask: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        let mut mutexSema: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(None));
        if { let __v = (*read.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = MUTEX_R_LOCK as u64; *mutexBit.lock().unwrap() = Some(new_val); };
        { let new_val = MUTEX_R_WAIT as u64; *mutexWait.lock().unwrap() = Some(new_val); };
        { let new_val = MUTEX_R_MASK as u64; *mutexMask.lock().unwrap() = Some(new_val); };
        { let new_val = self.rsema.clone().clone(); mutexSema = new_val; };
    } else {
        { let new_val = MUTEX_W_LOCK as u64; *mutexBit.lock().unwrap() = Some(new_val); };
        { let new_val = MUTEX_W_WAIT as u64; *mutexWait.lock().unwrap() = Some(new_val); };
        { let new_val = MUTEX_W_MASK as u64; *mutexMask.lock().unwrap() = Some(new_val); };
        { let new_val = self.wsema.clone().clone(); mutexSema = new_val; };
    }
        loop {
        let mut old = sync_atomic::load_uint64(self.state.clone());
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = MUTEX_CLOSED as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return false;
    }
        let mut new: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = { let __v = (*mutexBit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
                // Lock is free, acquire it.
        { let new_val = { let __tmp_x = ({ let __tmp_x = old; let __tmp_y = { let __v = (*mutexBit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }); let __tmp_y = MUTEX_REF as u64; __tmp_x + __tmp_y }; *new.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_REF_MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new(OVERFLOW_MSG) as Box<dyn Any + Send + Sync>);
    }
    } else {
                // Wait for lock.
        { let new_val = { let __tmp_x = old; let __tmp_y = { let __v = (*mutexWait.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *new.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mutexMask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new(OVERFLOW_MSG) as Box<dyn Any + Send + Sync>);
    }
    }
                // Lock is free, acquire it.
                // Wait for lock.
        if sync_atomic::compare_and_swap_uint64(self.state.clone(), Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = { let __v = (*mutexBit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return true;
    }
        runtime__semacquire(mutexSema.clone());
    }
    }
    }

    /// unlock removes a reference from mu and unlocks mu.
    /// It reports whether there is no remaining reference.
    pub fn rwunlock(&self, read: Arc<Mutex<Option<bool>>>) -> bool {
        let mut mutexBit: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut mutexWait: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut mutexMask: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        let mut mutexSema: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(None));
        if { let __v = (*read.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = MUTEX_R_LOCK as u64; *mutexBit.lock().unwrap() = Some(new_val); };
        { let new_val = MUTEX_R_WAIT as u64; *mutexWait.lock().unwrap() = Some(new_val); };
        { let new_val = MUTEX_R_MASK as u64; *mutexMask.lock().unwrap() = Some(new_val); };
        { let new_val = self.rsema.clone().clone(); mutexSema = new_val; };
    } else {
        { let new_val = MUTEX_W_LOCK as u64; *mutexBit.lock().unwrap() = Some(new_val); };
        { let new_val = MUTEX_W_WAIT as u64; *mutexWait.lock().unwrap() = Some(new_val); };
        { let new_val = MUTEX_W_MASK as u64; *mutexMask.lock().unwrap() = Some(new_val); };
        { let new_val = self.wsema.clone().clone(); mutexSema = new_val; };
    }
        loop {
        let mut old = sync_atomic::load_uint64(self.state.clone());
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = { let __v = (*mutexBit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = old; let __tmp_y = MUTEX_REF_MASK as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("inconsistent poll.fdMutex".to_string()) as Box<dyn Any + Send + Sync>);
    }

                // Drop lock, drop reference and wake read waiter if present.
        let mut new = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = old; let __tmp_y = { let __v = (*mutexBit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & ! __tmp_y }); let __tmp_y = MUTEX_REF as u64; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = { let __v = (*mutexMask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = (*mutexWait.lock().unwrap().as_ref().unwrap()); let mut guard = new.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        if sync_atomic::compare_and_swap_uint64(self.state.clone(), Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if { let __tmp_x = { let __tmp_x = old; let __tmp_y = { let __v = (*mutexMask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        runtime__semrelease(mutexSema.clone());
    }
        return { let __tmp_x = { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((MUTEX_CLOSED as u64) | (MUTEX_REF_MASK as u64)) as u64; __tmp_x & __tmp_y }; let __tmp_y = MUTEX_CLOSED as u64; __tmp_x == __tmp_y };
    }
    }
    }
}

impl crate::fd_unix::FD {
    /// incref adds a reference to fd.
    /// It returns an error when fd cannot be used.
    pub fn incref(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if !(*self.fdmu.lock().unwrap().as_mut().unwrap()).incref() {
        return err_closing(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        return Arc::new(Mutex::new(None));
    }

    /// decref removes a reference from fd.
    /// It also closes fd when the state of fd is set to closed and there
    /// is no remaining reference.
    pub fn decref(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if (*self.fdmu.lock().unwrap().as_mut().unwrap()).decref() {
        return self.destroy();
    }
        return Arc::new(Mutex::new(None));
    }

    /// readLock adds a reference to fd and locks fd for reading.
    /// It returns an error when fd cannot be used for reading.
    pub fn read_lock(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if !(*self.fdmu.lock().unwrap().as_mut().unwrap()).rwlock(Arc::new(Mutex::new(Some(true)))) {
        return err_closing(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        return Arc::new(Mutex::new(None));
    }

    /// readUnlock removes a reference from fd and unlocks fd for reading.
    /// It also closes fd when the state of fd is set to closed and there
    /// is no remaining reference.
    pub fn read_unlock(&mut self) {
        if (*self.fdmu.lock().unwrap().as_mut().unwrap()).rwunlock(Arc::new(Mutex::new(Some(true)))) {
        self.destroy();
    }
    }

    /// writeLock adds a reference to fd and locks fd for writing.
    /// It returns an error when fd cannot be used for writing.
    pub fn write_lock(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if !(*self.fdmu.lock().unwrap().as_mut().unwrap()).rwlock(Arc::new(Mutex::new(Some(false)))) {
        return err_closing(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        return Arc::new(Mutex::new(None));
    }

    /// writeUnlock removes a reference from fd and unlocks fd for writing.
    /// It also closes fd when the state of fd is set to closed and there
    /// is no remaining reference.
    pub fn write_unlock(&mut self) {
        if (*self.fdmu.lock().unwrap().as_mut().unwrap()).rwunlock(Arc::new(Mutex::new(Some(false)))) {
        self.destroy();
    }
    }
}

/// Implemented in runtime package.
pub fn runtime__semacquire(sema: Arc<Mutex<Option<u32>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn runtime__semrelease(sema: Arc<Mutex<Option<u32>>>) {
    unimplemented!("Go function declaration has no body");
}


impl GoValueClone for fdMutex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
