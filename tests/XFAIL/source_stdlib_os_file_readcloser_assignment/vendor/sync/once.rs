use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_slice, format_slice_values, format_slice_wrapped, go_any_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{cond::{noCopy}, mutex::{Mutex}};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

/// Once is an object that will perform exactly one action.
///
/// A Once must not be copied after first use.
///
/// In the terminology of [the Go memory model],
/// the return from f “synchronizes before”
/// the return from any call of once.Do(f).
///
/// [the Go memory model]: https://go.dev/ref/mem
#[derive(Clone)]
pub struct Once {
    pub __blank_0_0: Arc<StdMutex<Option<noCopy>>>,
    pub done: Arc<StdMutex<Option<sync_atomic::r#type::Uint32>>>,
    pub m: Arc<StdMutex<Option<Mutex>>>,
}

impl Once {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, done: { let __guard = self.done.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, m: { let __guard = self.m.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) } }
    }
}


impl Default for Once {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(StdMutex::new(Some(noCopy::default()))), done: Arc::new(StdMutex::new(Some(Default::default()))), m: Arc::new(StdMutex::new(Some(Mutex::default()))) }
    }
}

impl std::fmt::Display for Once {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.done.lock().unwrap().as_ref().unwrap()), (*self.m.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Once {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Once {
    /// Do calls the function f if and only if Do is being called for the
    /// first time for this instance of [Once]. In other words, given
    ///
    ///	var once Once
    ///
    /// if once.Do(f) is called multiple times, only the first call will invoke f,
    /// even if f has a different value in each invocation. A new instance of
    /// Once is required for each function to execute.
    ///
    /// Do is intended for initialization that must be run exactly once. Since f
    /// is niladic, it may be necessary to use a function literal to capture the
    /// arguments to a function to be invoked by Do:
    ///
    ///	config.once.Do(func() { config.init(filename) })
    ///
    /// Because no call to Do returns until the one call to f returns, if f causes
    /// Do to be called, it will deadlock.
    ///
    /// If f panics, Do considers it to have returned; future calls of Do return
    /// without calling f.
    pub fn r#do(&self, f: Arc<StdMutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
                // Note: Here is an incorrect implementation of Do:
                //
                //	if o.done.CompareAndSwap(0, 1) {
                //		f()
                //	}
                //
                // Do guarantees that when it returns, f has finished.
                // This implementation would not implement that guarantee:
                // given two simultaneous calls, the winner of the cas would
                // call f, and the second would return immediately, without
                // waiting for the first's call to f to complete.
                // This is why the slow path falls back to a mutex, and why
                // the o.done.Store must be delayed until after f returns.
        if { let __tmp_x = (*self.done.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // Outlined slow-path to allow inlining of the fast-path.
        self.do_slow(f.clone());
    }
    }

    pub fn do_slow(&self, f: Arc<StdMutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (*self.m.lock().unwrap().as_ref().unwrap()).lock();
            let mut o_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        (*o_defer_captured.m.lock().unwrap().as_ref().unwrap()).unlock();
    }));
            if { let __tmp_x = (*self.done.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        let mut o_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        (*o_defer_captured.done.lock().unwrap().as_mut().unwrap()).store(Arc::new(StdMutex::new(Some(1 as u32))));
    }));
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
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
                ()
            }
        }
    }
}

impl GoValueClone for Once {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
