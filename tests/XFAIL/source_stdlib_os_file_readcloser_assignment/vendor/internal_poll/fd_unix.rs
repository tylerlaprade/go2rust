use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{fd::{err_closing}, fd_mutex::{fdMutex, runtime__semacquire, runtime__semrelease}, fd_poll_runtime::{pollDesc}, fd_posix::{ignoring_e_i_n_t_r}, fd_unixjs::{SysFile, dup_close_on_exec_old}, sys_cloexec::{accept}};

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_R_W: i32 = 1 << 30;


/// FD is a file descriptor. The net and os packages use this type as a
/// field of a larger type representing a network connection or OS file.
#[derive(Clone)]
pub struct FD {
    pub fdmu: Arc<Mutex<Option<fdMutex>>>,
    pub sysfd: Arc<Mutex<Option<i32>>>,
    pub sys_file: Arc<Mutex<Option<SysFile>>>,
    pub pd: Arc<Mutex<Option<pollDesc>>>,
    pub csema: Arc<Mutex<Option<u32>>>,
    pub is_blocking: Arc<Mutex<Option<u32>>>,
    pub is_stream: Arc<Mutex<Option<bool>>>,
    pub zero_read_is_e_o_f: Arc<Mutex<Option<bool>>>,
    pub is_file: Arc<Mutex<Option<bool>>>,
}

impl FD {
    pub fn __go_value_clone(&self) -> Self {
        Self { fdmu: { let __guard = self.fdmu.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sysfd: { let __guard = self.sysfd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sys_file: { let __guard = self.sys_file.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pd: { let __guard = self.pd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, csema: { let __guard = self.csema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_blocking: { let __guard = self.is_blocking.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_stream: { let __guard = self.is_stream.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, zero_read_is_e_o_f: { let __guard = self.zero_read_is_e_o_f.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_file: { let __guard = self.is_file.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for FD {
    fn default() -> Self {
        Self { fdmu: Arc::new(Mutex::new(Some(fdMutex::default()))), sysfd: Arc::new(Mutex::new(Some(0))), sys_file: Arc::new(Mutex::new(Some(SysFile::default()))), pd: Arc::new(Mutex::new(Some(pollDesc::default()))), csema: Arc::new(Mutex::new(Some(0))), is_blocking: Arc::new(Mutex::new(Some(0))), is_stream: Arc::new(Mutex::new(Some(false))), zero_read_is_e_o_f: Arc::new(Mutex::new(Some(false))), is_file: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for FD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", (*self.fdmu.lock().unwrap().as_ref().unwrap()), (*self.sysfd.lock().unwrap().as_ref().unwrap()), (*self.sys_file.lock().unwrap().as_ref().unwrap()), (*self.pd.lock().unwrap().as_ref().unwrap()), (*self.csema.lock().unwrap().as_ref().unwrap()), (*self.is_blocking.lock().unwrap().as_ref().unwrap()), (*self.is_stream.lock().unwrap().as_ref().unwrap()), (*self.zero_read_is_e_o_f.lock().unwrap().as_ref().unwrap()), (*self.is_file.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for FD {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Sysfd") {
            out.sysfd = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IsStream") {
            out.is_stream = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ZeroReadIsEOF") {
            out.zero_read_is_e_o_f = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub(crate) static dupCloexecUnsupported: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync_atomic::r#type::Bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *dupCloexecUnsupported.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *dupCloexecUnsupported.lock().unwrap() = Some(Default::default());
}


impl FD {
    /// Init initializes the FD. The Sysfd field should already be set.
    /// This can be called multiple times on a single FD.
    /// The net argument is a network name from the net package (e.g., "tcp"),
    /// or "file".
    /// Set pollable to true if fd should be managed by runtime netpoll.
    pub fn init(&mut self, net: Arc<Mutex<Option<String>>>, pollable: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        (*self.sys_file.lock().unwrap().as_ref().unwrap()).init();
                // We don't actually care about the various network types.
        if { let __tmp_x = (*net.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "file".to_string(); __tmp_x == __tmp_y } {
        { let new_val = true; *self.is_file.lock().unwrap() = Some(new_val); };
    }
        if !{ let __v = (*pollable.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = 1 as u32; *self.is_blocking.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(None));
    }
        let mut err = (*self.pd.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(self.clone()))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
                // If we could not initialize the runtime poller,
                // assume we are using blocking mode.
        { let new_val = 1 as u32; *self.is_blocking.lock().unwrap() = Some(new_val); };
    }
                // If we could not initialize the runtime poller,
                // assume we are using blocking mode.
        return err.clone();
    }

    /// Destroy closes the file descriptor. This is called when there are
    /// no remaining references.
    pub fn destroy(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
                // Poller may want to unregister fd in readiness notification mechanism,
                // so this must be executed before CloseFunc.
        (*self.pd.lock().unwrap().as_mut().unwrap()).close();
        let mut err = (*self.sys_file.lock().unwrap().as_ref().unwrap()).destroy(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = -1; *self.sysfd.lock().unwrap() = Some(new_val); };
        runtime__semrelease(self.csema.clone());
        return err.clone();
    }

    /// Close closes the FD. The underlying file descriptor is closed by the
    /// destroy method when there are no remaining references.
    pub fn close(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if !(*self.fdmu.lock().unwrap().as_mut().unwrap()).incref_and_close() {
        return err_closing(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
                // Unblock any I/O.  Once it all unblocks and returns,
                // so that it cannot be referring to fd.sysfd anymore,
                // the final decref will close fd.sysfd. This should happen
                // fairly quickly, since all the I/O is non-blocking, and any
                // attempts to block in the pollDesc will return errClosing(fd.isFile).
        (*self.pd.lock().unwrap().as_ref().unwrap()).evict();
                // The call to decref will call destroy if there are no other
                // references.
        let mut err = self.decref();
                // Wait until the descriptor is closed. If this was the only
                // reference, it is already closed. Only wait if the file has
                // not been set to blocking mode, as otherwise any current I/O
                // may be blocking, and that would block the Close.
                // No need for an atomic read of isBlocking, increfAndClose means
                // we have exclusive access to fd.
        if { let __tmp_x = (*self.is_blocking.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        runtime__semacquire(self.csema.clone());
    }
        return err.clone();
    }

    /// SetBlocking puts the file into blocking mode.
    pub fn set_blocking(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
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
                        // Atomic store so that concurrent calls to SetBlocking
                        // do not cause a race condition. isBlocking only ever goes
                        // from 0 to 1 so there is no real race here.
            sync_atomic::store_uint32(self.is_blocking.clone(), Arc::new(Mutex::new(Some(1 as u32))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return syscall::set_nonblock(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false))));
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

    /// Read implements io.Reader.
    pub fn read(&mut self, mut p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
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
        fd_defer_captured.read_unlock();
    }));
            if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // If the caller wanted a zero byte read, return immediately
                // without trying (but after acquiring the readLock).
                // Otherwise syscall.Read returns 0, nil which looks like
                // io.EOF.
                // TODO(bradfitz): make it wait for readability? (Issue 15735)
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, Arc::new(Mutex::new(None)));
    }
    }
                        // If the caller wanted a zero byte read, return immediately
                        // without trying (but after acquiring the readLock).
                        // Otherwise syscall.Read returns 0, nil which looks like
                        // io.EOF.
                        // TODO(bradfitz): make it wait for readability? (Issue 15735)
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            if (*self.is_stream.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1073741824; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (MAX_R_W) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };
    }
            loop {
        let (mut n, mut err) = ignoring_e_i_n_t_r_i_o(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>, __arg1: Arc<Mutex<Option<Vec<u8>>>>| -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) { syscall::read(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>))), Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone());
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
        { let __rhs_holder = self.eof_error(Arc::new(Mutex::new(Some(n))), err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
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

    /// Pread wraps the pread system call.
    pub fn pread(&mut self, mut p: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
                // Call incref, not readLock, because since pread specifies the
                // offset it is independent from other reads.
                // Similarly, using the poller doesn't make sense for pread.
        {
        let mut err = self.incref();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
        if (*self.is_stream.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1073741824; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (MAX_R_W) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };
    }
        let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        loop {
        { let (__tmp_0, __tmp_1) = syscall::pread(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); !__matched } {
        break
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = 0; *n.lock().unwrap() = Some(new_val); };
    }
        self.decref();
        { let __rhs_holder = self.eof_error(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }

    /// ReadFrom wraps the recvfrom network call.
    pub fn read_from(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, Arc::new(Mutex::new(None)), err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.read_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, Arc::new(Mutex::new(None)), err.clone());
    };
        }
    }
            loop {
        let (mut n, mut sa, mut err) = syscall::recvfrom(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), Arc::new(Mutex::new(Some(0))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
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
        { let __rhs_holder = self.eof_error(Arc::new(Mutex::new(Some(n))), err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, sa.clone(), err.clone());
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
                (0 as i32, Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)))
            }
        }
    }

    /// ReadFromInet4 wraps the recvfrom network call for IPv4.
    pub fn read_from_inet4(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, from: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet4>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
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
        fd_defer_captured.read_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            loop {
        let (mut n, mut err) = internal_syscall_unix::recvfrom_inet4(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), Arc::new(Mutex::new(Some(0))), from.clone());
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
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
        { let __rhs_holder = self.eof_error(Arc::new(Mutex::new(Some(n))), err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
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

    /// ReadFromInet6 wraps the recvfrom network call for IPv6.
    pub fn read_from_inet6(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, from: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet6>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
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
        fd_defer_captured.read_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            loop {
        let (mut n, mut err) = internal_syscall_unix::recvfrom_inet6(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), Arc::new(Mutex::new(Some(0))), from.clone());
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
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
        { let __rhs_holder = self.eof_error(Arc::new(Mutex::new(Some(n))), err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
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

    /// ReadMsg wraps the recvmsg network call.
    pub fn read_msg(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, i32, i32, Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, 0, Arc::new(Mutex::new(None)), err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.read_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, 0, Arc::new(Mutex::new(None)), err.clone());
    };
        }
    }
            loop {
        let (mut n, mut oobn, mut sysflags, mut sa, mut err) = syscall::recvmsg(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), oob.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
                // TODO(dfc) should n and oobn be set to 0
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
    }
                // TODO(dfc) should n and oobn be set to 0
        { let __rhs_holder = self.eof_error(Arc::new(Mutex::new(Some(n))), err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, oobn, sysflags, sa.clone(), err.clone());
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
                (0 as i32, 0 as i32, 0 as i32, Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)))
            }
        }
    }

    /// ReadMsgInet4 is ReadMsg, but specialized for syscall.SockaddrInet4.
    pub fn read_msg_inet4(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, sa4: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet4>>>) -> (i32, i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, 0, err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.read_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, 0, err.clone());
    };
        }
    }
            loop {
        let (mut n, mut oobn, mut sysflags, mut err) = internal_syscall_unix::recvmsg_inet4(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), oob.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sa4.clone());
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
                // TODO(dfc) should n and oobn be set to 0
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
    }
                // TODO(dfc) should n and oobn be set to 0
        { let __rhs_holder = self.eof_error(Arc::new(Mutex::new(Some(n))), err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, oobn, sysflags, err.clone());
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
                (0 as i32, 0 as i32, 0 as i32, Arc::new(Mutex::new(None)))
            }
        }
    }

    /// ReadMsgInet6 is ReadMsg, but specialized for syscall.SockaddrInet6.
    pub fn read_msg_inet6(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, sa6: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet6>>>) -> (i32, i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, 0, err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.read_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, 0, err.clone());
    };
        }
    }
            loop {
        let (mut n, mut oobn, mut sysflags, mut err) = internal_syscall_unix::recvmsg_inet6(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), oob.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sa6.clone());
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
                // TODO(dfc) should n and oobn be set to 0
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
    }
                // TODO(dfc) should n and oobn be set to 0
        { let __rhs_holder = self.eof_error(Arc::new(Mutex::new(Some(n))), err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, oobn, sysflags, err.clone());
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
                (0 as i32, 0 as i32, 0 as i32, Arc::new(Mutex::new(None)))
            }
        }
    }

    /// Write implements io.Writer.
    pub fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
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
        fd_defer_captured.write_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            let mut nn: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
            loop {
        let mut max = Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        if (*self.is_stream.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 1073741824; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1073741824; __tmp_x + __tmp_y }; *max.lock().unwrap() = Some(new_val); };
    }
        let (mut n, mut err) = ignoring_e_i_n_t_r_i_o(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>, __arg1: Arc<Mutex<Option<Vec<u8>>>>| -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) { syscall::write(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>))), Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __tmp_x = n; let __tmp_y = { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; __tmp_x > __tmp_y } {
                // This can reportedly happen when using
                // some VPN software. Issue #61060.
                // If we don't check this we will panic
                // with slice bounds out of range.
                // Use a more informative panic.
        std::panic::panic_any(Box::new({ let mut __s = String::new(); __s.push_str(&format!("{}", "invalid return from write: got ".to_string())); __s.push_str(&format!("{}", (*internal_itoa::itoa(Arc::new(Mutex::new(Some(n)))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " from a write of ".to_string())); __s.push_str(&format!("{}", (*internal_itoa::itoa(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })))).lock().unwrap().as_ref().unwrap()))); __s }) as Box<dyn Any + Send + Sync>);
    }
                // This can reportedly happen when using
                // some VPN software. Issue #61060.
                // If we don't check this we will panic
                // with slice bounds out of range.
                // Use a more informative panic.
        { let __rhs = n; let mut guard = nn.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // This can reportedly happen when using
                // some VPN software. Issue #61060.
                // If we don't check this we will panic
                // with slice bounds out of range.
                // Use a more informative panic.
        if { let __tmp_x = ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }
    }
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }
    }
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __return_value_1 = io::ErrUnexpectedEOF.clone(); __return_value_1 });
    }
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

    /// Pwrite wraps the pwrite system call.
    pub fn pwrite(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // Call incref, not writeLock, because since pwrite specifies the
                        // offset it is independent from other writes.
                        // Similarly, using the poller doesn't make sense for pwrite.
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
            let mut nn: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
            loop {
        let mut max = Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        if (*self.is_stream.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 1073741824; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1073741824; __tmp_x + __tmp_y }; *max.lock().unwrap() = Some(new_val); };
    }
        let (mut n, mut err) = syscall::pwrite(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*nn.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }))));
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __rhs = n; let mut guard = nn.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        if { let __tmp_x = ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }
    }
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ({ let __v = (*nn.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __return_value_1 = io::ErrUnexpectedEOF.clone(); __return_value_1 });
    }
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

    /// WriteToInet4 wraps the sendto network call for IPv4 addresses.
    pub fn write_to_inet4(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, sa: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet4>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
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
        fd_defer_captured.write_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            loop {
        let mut err = internal_syscall_unix::sendto_inet4(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), Arc::new(Mutex::new(Some(0))), sa.clone());
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, err.clone());
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, Arc::new(Mutex::new(None)));
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

    /// WriteToInet6 wraps the sendto network call for IPv6 addresses.
    pub fn write_to_inet6(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, sa: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet6>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
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
        fd_defer_captured.write_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            loop {
        let mut err = internal_syscall_unix::sendto_inet6(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), Arc::new(Mutex::new(Some(0))), sa.clone());
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, err.clone());
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, Arc::new(Mutex::new(None)));
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

    /// WriteTo wraps the sendto network call.
    pub fn write_to(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, sa: Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
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
        fd_defer_captured.write_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            loop {
        let mut err = syscall::sendto(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), Arc::new(Mutex::new(Some(0))), sa.clone());
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, err.clone());
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, Arc::new(Mutex::new(None)));
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

    /// WriteMsg wraps the sendmsg network call.
    pub fn write_msg(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, sa: Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.write_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, err.clone());
    };
        }
    }
            loop {
        let (mut n, mut err) = syscall::sendmsg_n(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), oob.clone(), sa.clone(), Arc::new(Mutex::new(Some(0))));
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, 0, err.clone());
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, (*oob.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, err.clone());
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
                (0 as i32, 0 as i32, Arc::new(Mutex::new(None)))
            }
        }
    }

    /// WriteMsgInet4 is WriteMsg specialized for syscall.SockaddrInet4.
    pub fn write_msg_inet4(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, sa: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet4>>>) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.write_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, err.clone());
    };
        }
    }
            loop {
        let (mut n, mut err) = internal_syscall_unix::sendmsg_n_inet4(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), oob.clone(), sa.clone(), Arc::new(Mutex::new(Some(0))));
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, 0, err.clone());
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, (*oob.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, err.clone());
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
                (0 as i32, 0 as i32, Arc::new(Mutex::new(None)))
            }
        }
    }

    /// WriteMsgInet6 is WriteMsg specialized for syscall.SockaddrInet6.
    pub fn write_msg_inet6(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, sa: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet6>>>) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.write_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (0, 0, err.clone());
    };
        }
    }
            loop {
        let (mut n, mut err) = internal_syscall_unix::sendmsg_n_inet6(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone(), oob.clone(), sa.clone(), Arc::new(Mutex::new(Some(0))));
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } && (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, 0, err.clone());
    }
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (n, (*oob.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, err.clone());
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
                (0 as i32, 0 as i32, Arc::new(Mutex::new(None)))
            }
        }
    }

    /// Accept wraps the accept network call.
    pub fn accept(&mut self) -> (i32, Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (-(1), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    };
        }
    }
            let mut fd_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        fd_defer_captured.read_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (-(1), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    };
        }
    }
            loop {
        let (mut s, mut rsa, mut errcall, mut err) = accept(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (s, rsa.clone(), Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
    }
        { let _switch_val = err.clone();
    if { let __err_holder = _switch_val.clone(); let __err_guard = __err_holder.lock().unwrap(); __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false) } {
            continue
        } else if { let __err_holder = _switch_val.clone(); let __err_guard = __err_holder.lock().unwrap(); __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false) } {
            if (*self.pd.lock().unwrap().as_ref().unwrap()).pollable() {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        } else if { let __err_holder = _switch_val.clone(); let __err_guard = __err_holder.lock().unwrap(); __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_C_O_N_N_A_B_O_R_T_E_D as usize)).unwrap_or(false) } {
                        // This means that a socket on the listen
                        // queue was closed before we Accept()ed it;
                        // it's a silly error, so try again.
            continue
        }
    }
                // This means that a socket on the listen
                // queue was closed before we Accept()ed it;
                // it's a silly error, so try again.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (-(1), Arc::new(Mutex::new(None)), { let __owned = errcall.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, err.clone());
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
                (0 as i32, Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(None)))
            }
        }
    }

    /// Fchmod wraps syscall.Fchmod.
    pub fn fchmod(&mut self, mode: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
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
            let mut fd_closure_clone = (*self).clone(); let mode_closure_clone = mode.clone(); {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        syscall::fchmod(Arc::new(Mutex::new(Some({ let __selector_holder = fd_closure_clone.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = mode_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))));
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

    /// Fstat wraps syscall.Fstat
    pub fn fstat(&mut self, s: Arc<Mutex<Option<syscall::ztypes_darwin_arm64::Stat_t>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
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
            let mut fd_closure_clone = (*self).clone(); let s_closure_clone = s.clone(); {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        syscall::fstat(Arc::new(Mutex::new(Some({ let __selector_holder = fd_closure_clone.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), s_closure_clone.clone())
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))));
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

    /// Dup duplicates the file descriptor.
    pub fn dup(&mut self) -> (i32, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
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
        return (-(1), Arc::new(Mutex::new(Some("".to_string()))), err.clone());
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
        return dup_close_on_exec(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
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

    /// WaitWrite waits until data can be written to fd.
    pub fn wait_write(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
    }

    /// WriteOnce is for testing only. It makes a single write call.
    pub fn write_once(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
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
        fd_defer_captured.write_unlock();
    }));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ignoring_e_i_n_t_r_i_o(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>, __arg1: Arc<Mutex<Option<Vec<u8>>>>| -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) { syscall::write(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>))), Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), p.clone());
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

    /// RawRead invokes the user-defined function f for a read operation.
    pub fn raw_read(&mut self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.read_lock();;
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
        fd_defer_captured.read_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            loop {
        if { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize)))) } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(None));
    }
    }
        {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_read(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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

    /// RawWrite invokes the user-defined function f for a write operation.
    pub fn raw_write(&mut self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            {
        let mut err = self.write_lock();;
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
        fd_defer_captured.write_unlock();
    }));
            {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).prepare_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
            loop {
        if { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize)))) } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(None));
    }
    }
        {
        let mut err = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
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
}

#[derive(Clone)]
pub struct FDPtr(pub Arc<Mutex<Option<FD>>>);

impl std::fmt::Display for FDPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl io::r#mod::Closer for FDPtr {
    fn close(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        FD::close(__recv)
    }
    fn __go_clone_box_closer(&self) -> Box<dyn io::r#mod::Closer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Closer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_closer(&self, other: &(dyn io::r#mod::Closer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FDPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::ReadCloser for FDPtr {
    fn __go_clone_box_read_closer(&self) -> Box<dyn io::r#mod::ReadCloser + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::ReadCloser + Send + Sync>
    }
    fn __go_eq_read_closer(&self, other: &(dyn io::r#mod::ReadCloser + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FDPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::Reader for FDPtr {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        FD::read(__recv, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn io::r#mod::Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn io::r#mod::Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FDPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::Writer for FDPtr {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        FD::write(__recv, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn io::r#mod::Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn io::r#mod::Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FDPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// DupCloseOnExec dups fd and marks it close-on-exec.
pub fn dup_close_on_exec(fd: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __tmp_x = syscall::F__D_U_P_F_D__C_L_O_E_X_E_C; let __tmp_y = 0; __tmp_x != __tmp_y } && !(*dupCloexecUnsupported.lock().unwrap().as_mut().unwrap()).load() {
        let (mut r0, mut err) = internal_syscall_unix::fcntl(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(syscall::F__D_U_P_F_D__C_L_O_E_X_E_C))), Arc::new(Mutex::new(Some(0))));
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        return (r0, Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(None)));
    }
        { let _switch_val = err.clone();
    if { let __err_holder = _switch_val.clone(); let __err_guard = __err_holder.lock().unwrap(); __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_V_A_L as usize)).unwrap_or(false) } || { let __err_holder = _switch_val.clone(); let __err_guard = __err_holder.lock().unwrap(); __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_N_O_S_Y_S as usize)).unwrap_or(false) } {
                        // Old kernel, or js/wasm (which returns
                        // ENOSYS). Fall back to the portable way from
                        // now on.
            (*dupCloexecUnsupported.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(true))));
        } else {
            return (-(1), Arc::new(Mutex::new(Some("fcntl".to_string()))), err.clone());
        }
    }
    }
        // Old kernel, or js/wasm (which returns
        // ENOSYS). Fall back to the portable way from
        // now on.
    dup_close_on_exec_old(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// ignoringEINTRIO is like ignoringEINTR, but just for IO calls.
pub fn ignoring_e_i_n_t_r_i_o(r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>, fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    loop {
        let (mut n, mut err) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(fd.clone(), p.clone()) };
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); !__matched } {
        return (n, err.clone());
    }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for FD {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
