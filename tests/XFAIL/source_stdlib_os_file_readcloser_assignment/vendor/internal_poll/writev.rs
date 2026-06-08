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

use crate::{
    fd::{TestHookDidWritev, consume},
    fd_poll_runtime::{pollDesc},
    fd_unix::{FD},
    fd_writev_libc::{writev},
    iovec_unix::{new_iovec_with_base},
};

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

impl crate::fd_unix::FD {
    /// Writev wraps the writev system call.
    pub fn writev(&mut self, v: Arc<Mutex<Option<Vec<Vec<u8>>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
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
            let mut iovecs: Arc<Mutex<Option<Vec<syscall::ztypes_darwin_arm64::Iovec>>>> = Arc::new(Mutex::new(None));
            if { let __nil_target = (*self.sys_file.lock().unwrap().as_ref().unwrap()).iovecs.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*self.sys_file.lock().unwrap().as_ref().unwrap()).iovecs.clone(); iovecs = new_val; };
    }
                        // TODO: read from sysconf(_SC_IOV_MAX)? The Linux default is
                        // 1024 and this seems conservative enough for now. Darwin's
                        // UIO_MAXIOV also seems to be 1024.
            let mut maxVec = Arc::new(Mutex::new(Some(1024)));
            if { let __tmp_x = "darwin".to_string(); let __tmp_y = "aix".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "solaris".to_string(); __tmp_x == __tmp_y } {
                // IOV_MAX is set to XOPEN_IOV_MAX on AIX and Solaris.
        { let new_val = 16; *maxVec.lock().unwrap() = Some(new_val); };
    }
                        // IOV_MAX is set to XOPEN_IOV_MAX on AIX and Solaris.
            let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
            let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
            while { let __tmp_x = ({ let __slice_holder = v.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = iovecs.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = (0) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); iovecs = new_val; };
        for chunk in &{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = (chunk.len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        continue
    }
        { let new_val = { let __append_target = iovecs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*new_iovec_with_base(GoPtr::slice_elem(GoSliceElemPtr::new(Arc::new(Mutex::new(Some((*chunk).clone()))), (0) as usize))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; iovecs = new_val; };
        if (*self.is_stream.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (chunk.len() as i32); let __tmp_y = 1073741824; __tmp_x > __tmp_y } {
        { let __seq = { let __seq_holder = iovecs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*iovecs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.set_len(Arc::new(Mutex::new(Some(1073741824))));
        break
    }
                // continue chunk on next writev
        { let __seq = { let __seq_holder = iovecs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*iovecs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.set_len(Arc::new(Mutex::new(Some(chunk.len() as i32))));
        if { let __tmp_x = ((*iovecs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*maxVec.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x == __tmp_y } {
        break
    }
    }
                // continue chunk on next writev
        if { let __tmp_x = ((*iovecs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        break
    }
        if { let __nil_target = (*self.sys_file.lock().unwrap().as_ref().unwrap()).iovecs.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(Vec::<syscall::ztypes_darwin_arm64::Iovec>::default()))).clone(); (*self.sys_file.lock().unwrap().as_mut().unwrap()).iovecs = new_val; };
    }
        { let new_val = iovecs.clone(); let __cloned_val = { let __guard = new_val.lock().unwrap(); (*__guard).clone() }; *(*self.sys_file.lock().unwrap().as_ref().unwrap()).iovecs.lock().unwrap() = __cloned_val; };

        let mut wrote: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1) = writev(Arc::new(Mutex::new(Some({ let __selector_holder = self.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), iovecs.clone()); *wrote.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __tmp_x = { let __v = (*wrote.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as usize) as usize; __tmp_x == __tmp_y } {
        { let new_val = 0 as usize; *wrote.lock().unwrap() = Some(new_val); };
    }
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = TestHookDidWritev.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*wrote.lock().unwrap().as_ref().unwrap()) as i32)))) };
        { let __rhs = (*Arc::new(Mutex::new(Some((*wrote.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        consume(
            v.clone(),
            Arc::new(Mutex::new(Some((*wrote.lock().unwrap().as_ref().unwrap()) as i64)))
        );
        { let __clear_holder = iovecs.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = Default::default(); } } };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); __matched } {
        continue
    }
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_A_G_A_I_N as usize)).unwrap_or(false); __matched } {
        {
        { let __rhs_holder = (*self.pd.lock().unwrap().as_ref().unwrap()).wait_write(Arc::new(Mutex::new(Some({ let __selector_holder = self.is_file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            continue;
        }
    }
    }
        break
    }
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let __rhs_holder = io::ErrUnexpectedEOF.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break
    }
    }
                        // continue chunk on next writev
                        // cache
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
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