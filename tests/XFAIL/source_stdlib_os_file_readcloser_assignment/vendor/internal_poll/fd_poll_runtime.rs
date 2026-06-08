use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{errno_unix::{errno_err}, fd::{ErrDeadlineExceeded, ErrNoDeadline, ErrNotPollable, err_closing}, fd_unix::{FD}};

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const POLL_NO_ERROR: i32 = 0;
pub(crate) const POLL_ERR_CLOSING: i32 = 1;
pub(crate) const POLL_ERR_TIMEOUT: i32 = 2;
pub(crate) const POLL_ERR_NOT_POLLABLE: i32 = 3;


#[derive(Debug, Clone)]
pub struct pollDesc {
    pub runtime_ctx: Arc<Mutex<Option<usize>>>,
}

impl pollDesc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.runtime_ctx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            runtime_ctx: __go_clone_0_0,
        }
    }
}


impl Default for pollDesc {
    fn default() -> Self {
        Self { runtime_ctx: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for pollDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.runtime_ctx.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for pollDesc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static serverInit: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::once::Once>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *serverInit.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *serverInit.lock().unwrap() = Some(Default::default());
}


impl pollDesc {
    pub fn init(&mut self, fd: Arc<Mutex<Option<FD>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        { let __once = (*serverInit.lock().unwrap().as_ref().unwrap()).clone(); __once.r#do(Arc::new(Mutex::new(Some(Box::new(move || { runtime_poll_server_init() }) as Box<dyn FnMut() -> () + Send + Sync>)))) };
        let (mut ctx, mut errno) = runtime_poll_open(Arc::new(Mutex::new(Some({ let __selector_holder = (*fd.lock().unwrap().as_ref().unwrap()).sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))));
        if { let __tmp_x = errno; let __tmp_y = 0; __tmp_x != __tmp_y } {
        return errno_err(Arc::new(Mutex::new(Some(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(errno as usize))))))));
    }
        { let new_val = ctx; *self.runtime_ctx.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(None));
    }

    pub fn close(&mut self) {
        if { let __tmp_x = (*self.runtime_ctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return;
    }
        runtime_poll_close(Arc::new(Mutex::new(Some({ let __selector_holder = self.runtime_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as usize; *self.runtime_ctx.lock().unwrap() = Some(new_val); };
    }

    /// Evict evicts fd from the pending list, unblocking any I/O running on fd.
    pub fn evict(&self) {
        if { let __tmp_x = (*self.runtime_ctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return;
    }
        runtime_poll_unblock(Arc::new(Mutex::new(Some({ let __selector_holder = self.runtime_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }

    pub fn prepare(&self, mode: Arc<Mutex<Option<i32>>>, isFile: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = (*self.runtime_ctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        let mut res = runtime_poll_reset(Arc::new(Mutex::new(Some({ let __selector_holder = self.runtime_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        convert_err(Arc::new(Mutex::new(Some(res))), Arc::new(Mutex::new(Some({ let __arg_holder = isFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn prepare_read(&self, isFile: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.prepare(Arc::new(Mutex::new(Some(('r' as i32)))), Arc::new(Mutex::new(Some({ let __arg_holder = isFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn prepare_write(&self, isFile: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.prepare(Arc::new(Mutex::new(Some(('w' as i32)))), Arc::new(Mutex::new(Some({ let __arg_holder = isFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn wait(&self, mode: Arc<Mutex<Option<i32>>>, isFile: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = (*self.runtime_ctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return errors::new(Arc::new(Mutex::new(Some("waiting for unsupported file type".to_string()))));
    }
        let mut res = runtime_poll_wait(Arc::new(Mutex::new(Some({ let __selector_holder = self.runtime_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        convert_err(Arc::new(Mutex::new(Some(res))), Arc::new(Mutex::new(Some({ let __arg_holder = isFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn wait_read(&self, isFile: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.wait(Arc::new(Mutex::new(Some(('r' as i32)))), Arc::new(Mutex::new(Some({ let __arg_holder = isFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn wait_write(&self, isFile: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.wait(Arc::new(Mutex::new(Some(('w' as i32)))), Arc::new(Mutex::new(Some({ let __arg_holder = isFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn wait_canceled(&self, mode: Arc<Mutex<Option<i32>>>) {
        if { let __tmp_x = (*self.runtime_ctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return;
    }
        runtime_poll_wait_canceled(Arc::new(Mutex::new(Some({ let __selector_holder = self.runtime_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    pub fn pollable(&self) -> bool {
        return { let __tmp_x = (*self.runtime_ctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
    }
}

impl crate::fd_unix::FD {
    /// SetDeadline sets the read and write deadlines associated with fd.
    pub fn set_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        set_deadline_impl(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(233))))
    }

    /// SetReadDeadline sets the read deadline associated with fd.
    pub fn set_read_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        set_deadline_impl(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('r' as i32) as i32))))
    }

    /// SetWriteDeadline sets the write deadline associated with fd.
    pub fn set_write_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        set_deadline_impl(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('w' as i32) as i32))))
    }
}

pub fn runtime_poll_server_init() {
    unimplemented!("Go function declaration has no body");
}


pub fn runtime_poll_open(fd: Arc<Mutex<Option<usize>>>) -> (usize, i32) {
    unimplemented!("Go function declaration has no body");
}


pub fn runtime_poll_close(ctx: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn runtime_poll_wait(ctx: Arc<Mutex<Option<usize>>>, mode: Arc<Mutex<Option<i32>>>) -> i32 {
    unimplemented!("Go function declaration has no body");
}


pub fn runtime_poll_wait_canceled(ctx: Arc<Mutex<Option<usize>>>, mode: Arc<Mutex<Option<i32>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn runtime_poll_reset(ctx: Arc<Mutex<Option<usize>>>, mode: Arc<Mutex<Option<i32>>>) -> i32 {
    unimplemented!("Go function declaration has no body");
}


pub fn runtime_poll_set_deadline(ctx: Arc<Mutex<Option<usize>>>, d: Arc<Mutex<Option<i64>>>, mode: Arc<Mutex<Option<i32>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn runtime_poll_unblock(ctx: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn convert_err(res: Arc<Mutex<Option<i32>>>, isFile: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    { let _switch_val = { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return Arc::new(Mutex::new(None));
        } else if _switch_val == (1) {
            return err_closing(Arc::new(Mutex::new(Some({ let __arg_holder = isFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        } else if _switch_val == (2) {
            return ErrDeadlineExceeded.clone();
        } else if _switch_val == (3) {
            return ErrNotPollable.clone();
        }
    }
    eprintln!("{} {}", format!("{}", "unreachable: ".to_string()), format!("{}", { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
}

pub fn set_deadline_impl(fd: Arc<Mutex<Option<FD>>>, t: Arc<Mutex<Option<time::r#mod::Time>>>, mode: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut d: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        if !(*t.lock().unwrap().as_ref().unwrap()).is_zero() {
        { let new_val = Arc::new(Mutex::new(Some((*time::until(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).as_nanos() as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *d.lock().unwrap() = __moved_val; };
        if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = -1 as i64; *d.lock().unwrap() = Some(new_val); };
    }
    }
                // don't confuse deadline right now with no deadline
        {
        let mut err = { let __recv = fd.clone(); let __recv_ptr: *const crate::fd_unix::FD = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::fd_unix::FD }; let __result = unsafe { &*__recv_ptr }.incref(); __result };;
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
        let fd_defer_captured = fd.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = fd_defer_captured.clone(); let __recv_ptr: *mut crate::fd_unix::FD = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::fd_unix::FD }; let __result = unsafe { &mut *__recv_ptr }.decref(); __result };
    }));
        if { let __tmp_x = (*(*(*fd.lock().unwrap().as_ref().unwrap()).pd.lock().unwrap().as_ref().unwrap()).runtime_ctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ErrNoDeadline.clone();
    }
    }
        runtime_poll_set_deadline(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*fd.lock().unwrap().as_ref().unwrap()).pd.lock().unwrap().as_ref().unwrap()).runtime_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(None));
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

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for pollDesc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
