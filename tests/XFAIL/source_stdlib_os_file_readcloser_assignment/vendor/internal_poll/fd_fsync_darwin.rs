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

use crate::{fd_posix::{ignoring_e_i_n_t_r}, fd_unix::{FD}};

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

impl crate::fd_unix::FD {
    /// Fsync invokes SYS_FCNTL with SYS_FULLFSYNC because
    /// on OS X, SYS_FSYNC doesn't fully flush contents to disk.
    /// See Issue #26650 as well as the man page for fsync on OS X.
    pub fn fsync(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
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
            let mut fd_closure_clone = (*self).clone(); {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (_, mut err) = internal_syscall_unix::fcntl(Arc::new(Mutex::new(Some({ let __selector_holder = fd_closure_clone.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(syscall::F__F_U_L_L_F_S_Y_N_C))), Arc::new(Mutex::new(Some(0))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } && errors::is(err.clone(), Arc::new(Mutex::new(Some(syscall::E_N_O_T_S_U_P)))) {
        { let __rhs_holder = syscall::fsync(Arc::new(Mutex::new(Some({ let __selector_holder = fd_closure_clone.sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        return err.clone();
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
}