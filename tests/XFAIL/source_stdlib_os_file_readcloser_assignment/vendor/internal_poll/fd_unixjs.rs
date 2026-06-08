use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{fd_poll_runtime::{pollDesc}, fd_unix::{FD, ignoring_e_i_n_t_r_i_o}, hook_unix::{CloseFunc}};

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct SysFile {
    pub iovecs: Arc<Mutex<Option<Vec<syscall::ztypes_darwin_arm64::Iovec>>>>,
}

impl SysFile {
    pub fn __go_value_clone(&self) -> Self {
        Self { iovecs: self.iovecs.clone() }
    }
}

impl std::fmt::Display for SysFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.iovecs))
    }
}

impl GoJsonDecode for SysFile {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl SysFile {
    pub fn init(&self) {
    }

    pub fn destroy(&self, fd: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
                // We don't use ignoringEINTR here because POSIX does not define
                // whether the descriptor is closed if close returns EINTR.
                // If the descriptor is indeed closed, using a loop would race
                // with some other goroutine opening a new descriptor.
                // (The Linux kernel guarantees that it is closed on an EINTR error.)
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = CloseFunc.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(fd.clone()) }
    }
}

impl crate::fd_unix::FD {
    /// Fchdir wraps syscall.Fchdir.
    pub fn fchdir(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.incref();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.decref();
    }));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return syscall::fchdir(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                Arc::new(Mutex::new(None))
            }
        }
    }

    /// ReadDirent wraps syscall.ReadDirent.
    /// We treat this like an ordinary system call rather than a call
    /// that tries to fill the buffer.
    pub fn read_dirent(&mut self, buf: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.incref();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.decref();
    }));
            loop {
        let (mut n, mut err) = ignoring_e_i_n_t_r_i_o(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>, __arg1: Arc<Mutex<Option<Vec<u8>>>>| -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) { syscall::read_dirent(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>))), Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), buf.clone());
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = 0; n = new_val; };
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
    }

                // Do not call eofError; caller does not expect to see io.EOF.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, err.clone());
    }
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                (0 as i32, Arc::new(Mutex::new(None)))
            }
        }
    }

    /// Seek wraps syscall.Seek.
    pub fn seek(&mut self, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.incref();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.decref();
    }));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return syscall::seek(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = whence.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                (0 as i64, Arc::new(Mutex::new(None)))
            }
        }
    }
}

/// dupCloseOnExecOld is the traditional way to dup an fd and
/// set its O_CLOEXEC bit, using two system calls.
pub fn dup_close_on_exec_old(fd: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        (*syscall::ForkLock.lock().unwrap().as_mut().unwrap()).r_lock();
        __defer_stack.push(Box::new(move || {
        (*syscall::ForkLock.lock().unwrap().as_mut().unwrap()).r_unlock();
    }));
        let (mut newfd, mut err) = syscall::dup(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (-(1), Arc::new(Mutex::new(Some("dup".to_string()))), err.clone());
    }
    }
        syscall::close_on_exec(Arc::new(Mutex::new(Some(newfd))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (newfd, Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(None)));
    }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            (0 as i32, Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(None)))
        }
    }
}

impl GoValueClone for SysFile {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
