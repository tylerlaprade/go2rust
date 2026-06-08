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

use crate::{
    panic::{THROW_TYPE_RUNTIME, THROW_TYPE_USER, throwType},
    r#mod::{envs},
    runtime2::{g, m},
    stack::{STACK_PREEMPT},
    stubs::{getg},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const TRACEBACK_CRASH: i32 = 1 << 0;
pub(crate) const TRACEBACK_ALL: i32 = 1 << 1;
pub(crate) const TRACEBACK_SHIFT: i32 = 2;


#[derive(Clone)]
pub struct dbgVar {
    pub name: Arc<Mutex<Option<String>>>,
    pub value: Arc<Mutex<Option<i32>>>,
    pub atomic: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub def: Arc<Mutex<Option<i32>>>,
}

impl dbgVar {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.value.clone();
        let __go_clone_2_0 = self.atomic.clone();
        let __go_clone_3_0 = { let __guard = self.def.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            value: __go_clone_1_0,
            atomic: __go_clone_2_0,
            def: __go_clone_3_0,
        }
    }
}


impl Default for dbgVar {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            value: __go_default_1_0,
            atomic: __go_default_2_0,
            def: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for dbgVar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.value.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.atomic.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", (*self.def.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


pub(crate) static traceback_cache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceback_env: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static argc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static argv: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static test_z64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static test_x64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static debug: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct25>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static dbgvars: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<dbgVar>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *traceback_cache.lock().unwrap() = Some(0);
    *traceback_env.lock().unwrap() = Some(0);
    *argc.lock().unwrap() = Some(0);
    *argv.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *test_z64.lock().unwrap() = Some(0);
    *test_x64.lock().unwrap() = Some(0);
    *debug.lock().unwrap() = Some(Default::default());
    *dbgvars.lock().unwrap() = Some(vec![]);
    *traceback_cache.lock().unwrap() = Some(((2 as u32) << (TRACEBACK_SHIFT as u32)) as u32);
    {
        let mut __go_slice = Vec::<Arc<Mutex<Option<dbgVar>>>>::with_capacity(30);
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("adaptivestackstart".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).adaptivestackstart.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("asyncpreemptoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("asynctimerchan".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).asynctimerchan.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("cgocheck".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).cgocheck.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("clobberfree".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).clobberfree.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("dataindependenttiming".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).dataindependenttiming.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("disablethp".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).disablethp.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("dontfreezetheworld".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).dontfreezetheworld.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("efence".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).efence.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gccheckmark".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gccheckmark.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcpacertrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcpacertrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcshrinkstackoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcshrinkstackoff.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcstoptheworld".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcstoptheworld.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gctrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gctrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("harddecommit".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).harddecommit.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("inittrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).inittrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("invalidptr".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).invalidptr.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("madvdontneed".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).madvdontneed.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("panicnil".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).panicnil.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("profstackdepth".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone().clone(), def: Arc::new(Mutex::new(Some(128 as i32))), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("runtimecontentionstacks".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).runtime_contention_stacks.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("sbrk".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).sbrk.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("scavtrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).scavtrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("scheddetail".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).scheddetail.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("schedtrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).schedtrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("traceadvanceperiod".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).traceadvanceperiod.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("traceallocfree".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).traceallocfree.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracecheckstackownership".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).trace_check_stack_ownership.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracebackancestors".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).tracebackancestors.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracefpunwindoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).tracefpunwindoff.clone().clone(), ..Default::default() }))));
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *dbgvars.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_zero_globals() {
    *traceback_cache.lock().unwrap() = Some(0);
    *traceback_env.lock().unwrap() = Some(0);
    *argc.lock().unwrap() = Some(0);
    *argv.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *test_z64.lock().unwrap() = Some(0);
    *test_x64.lock().unwrap() = Some(0);
    *debug.lock().unwrap() = Some(Default::default());
    *dbgvars.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_62() {
    *traceback_cache.lock().unwrap() = Some(((2 as u32) << (TRACEBACK_SHIFT as u32)) as u32);
}


pub(crate) fn __go_init_order_63() {
    {
        let mut __go_slice = Vec::<Arc<Mutex<Option<dbgVar>>>>::with_capacity(30);
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("adaptivestackstart".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).adaptivestackstart.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("asyncpreemptoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("asynctimerchan".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).asynctimerchan.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("cgocheck".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).cgocheck.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("clobberfree".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).clobberfree.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("dataindependenttiming".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).dataindependenttiming.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("disablethp".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).disablethp.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("dontfreezetheworld".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).dontfreezetheworld.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("efence".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).efence.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gccheckmark".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gccheckmark.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcpacertrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcpacertrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcshrinkstackoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcshrinkstackoff.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcstoptheworld".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcstoptheworld.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gctrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gctrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("harddecommit".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).harddecommit.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("inittrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).inittrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("invalidptr".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).invalidptr.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("madvdontneed".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).madvdontneed.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("panicnil".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).panicnil.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("profstackdepth".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone().clone(), def: Arc::new(Mutex::new(Some(128 as i32))), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("runtimecontentionstacks".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).runtime_contention_stacks.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("sbrk".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).sbrk.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("scavtrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).scavtrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("scheddetail".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).scheddetail.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("schedtrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).schedtrace.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("traceadvanceperiod".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).traceadvanceperiod.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("traceallocfree".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).traceallocfree.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracecheckstackownership".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).trace_check_stack_ownership.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracebackancestors".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).tracebackancestors.clone().clone(), ..Default::default() }))));
        __go_slice.push(Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracefpunwindoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).tracefpunwindoff.clone().clone(), ..Default::default() }))));
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *dbgvars.lock().unwrap() = Some(__go_slice);
    }
}


/// gotraceback returns the current traceback settings.
///
/// If level is 0, suppress all tracebacks.
/// If level is 1, show tracebacks, but exclude runtime frames.
/// If level is 2, show tracebacks including runtime frames.
/// If all is set, print all goroutine stacks. Otherwise, print just the current goroutine.
/// If crash is set, crash (core dump, etc) after tracebacking.
///
///go:nosplit
pub fn gotraceback() -> (i32, bool, bool) {
    let mut level: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut all: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut crash: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut gp = getg();
    let mut t = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local(traceback_cache.clone()));
    { let new_val = { let __tmp_x = { let __tmp_x = t; let __tmp_y = TRACEBACK_CRASH as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y }; *crash.lock().unwrap() = Some(new_val); };
    { let new_val = {
        let __go_cond_0 = {
            let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::panic::throwType(Arc::new(Mutex::new(Some(THROW_TYPE_USER as u32))));
            __tmp_x >= __tmp_y
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = { let __tmp_x = { let __tmp_x = t; let __tmp_y = TRACEBACK_ALL as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
            __go_cond_1
        }
    }; *all.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *level.lock().unwrap() = __moved_val; };
    } else if {
        let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::panic::throwType(Arc::new(Mutex::new(Some(THROW_TYPE_RUNTIME as u32))));
        __tmp_x >= __tmp_y
    } {
        { let new_val = 2 as i32; *level.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = t; let __tmp_y = TRACEBACK_SHIFT; __tmp_x >> __tmp_y }) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *level.lock().unwrap() = __moved_val; };
    }
        // Always include runtime frames in runtime throws unless
        // otherwise overridden by m.traceback.
    return ((*level.lock().unwrap().as_ref().unwrap()), (*all.lock().unwrap().as_ref().unwrap()), (*crash.lock().unwrap().as_ref().unwrap()));
}

pub fn environ() -> Arc<Mutex<Option<Vec<String>>>> {
    envs.clone()
}

///go:nosplit
pub fn acquirem() -> Arc<Mutex<Option<crate::runtime2::m>>> {
    let mut gp = getg();
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return (*gp.lock().unwrap().as_ref().unwrap()).m.clone();
}

///go:nosplit
pub fn releasem(mp: GoPtr<crate::runtime2::m>) {
    let mut gp = getg();
    { let __target = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.locks.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    if { let __tmp_x = (*{ let __ptr_value = mp.borrow(); __ptr_value.as_ref().unwrap().locks.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // restore the preemption request in case we've cleared it in newstack
        { let new_val = STACK_PREEMPT as usize; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    }
}

#[derive(Clone)]
pub struct AnonymousStruct25 {
    pub cgocheck: Arc<Mutex<Option<i32>>>,
    pub clobberfree: Arc<Mutex<Option<i32>>>,
    pub disablethp: Arc<Mutex<Option<i32>>>,
    pub dontfreezetheworld: Arc<Mutex<Option<i32>>>,
    pub efence: Arc<Mutex<Option<i32>>>,
    pub gccheckmark: Arc<Mutex<Option<i32>>>,
    pub gcpacertrace: Arc<Mutex<Option<i32>>>,
    pub gcshrinkstackoff: Arc<Mutex<Option<i32>>>,
    pub gcstoptheworld: Arc<Mutex<Option<i32>>>,
    pub gctrace: Arc<Mutex<Option<i32>>>,
    pub invalidptr: Arc<Mutex<Option<i32>>>,
    pub madvdontneed: Arc<Mutex<Option<i32>>>,
    pub runtime_contention_stacks: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub scavtrace: Arc<Mutex<Option<i32>>>,
    pub scheddetail: Arc<Mutex<Option<i32>>>,
    pub schedtrace: Arc<Mutex<Option<i32>>>,
    pub tracebackancestors: Arc<Mutex<Option<i32>>>,
    pub asyncpreemptoff: Arc<Mutex<Option<i32>>>,
    pub harddecommit: Arc<Mutex<Option<i32>>>,
    pub adaptivestackstart: Arc<Mutex<Option<i32>>>,
    pub tracefpunwindoff: Arc<Mutex<Option<i32>>>,
    pub traceadvanceperiod: Arc<Mutex<Option<i32>>>,
    pub trace_check_stack_ownership: Arc<Mutex<Option<i32>>>,
    pub profstackdepth: Arc<Mutex<Option<i32>>>,
    pub dataindependenttiming: Arc<Mutex<Option<i32>>>,
    pub malloc: Arc<Mutex<Option<bool>>>,
    pub inittrace: Arc<Mutex<Option<i32>>>,
    pub sbrk: Arc<Mutex<Option<i32>>>,
    pub traceallocfree: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub panicnil: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub asynctimerchan: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct25 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cgocheck.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.clobberfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.disablethp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.dontfreezetheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.efence.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.gccheckmark.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.gcpacertrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.gcshrinkstackoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.gcstoptheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.gctrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.invalidptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.madvdontneed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.runtime_contention_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.scavtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.scheddetail.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.schedtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.tracebackancestors.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.asyncpreemptoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.harddecommit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.adaptivestackstart.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.tracefpunwindoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.traceadvanceperiod.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = { let __guard = self.trace_check_stack_ownership.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_23_0 = { let __guard = self.profstackdepth.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_24_0 = { let __guard = self.dataindependenttiming.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_25_0 = { let __guard = self.malloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_0 = { let __guard = self.inittrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_27_0 = { let __guard = self.sbrk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_0 = { let __guard = self.traceallocfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_29_0 = { let __guard = self.panicnil.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_30_0 = { let __guard = self.asynctimerchan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            cgocheck: __go_clone_0_0,
            clobberfree: __go_clone_1_0,
            disablethp: __go_clone_2_0,
            dontfreezetheworld: __go_clone_3_0,
            efence: __go_clone_4_0,
            gccheckmark: __go_clone_5_0,
            gcpacertrace: __go_clone_6_0,
            gcshrinkstackoff: __go_clone_7_0,
            gcstoptheworld: __go_clone_8_0,
            gctrace: __go_clone_9_0,
            invalidptr: __go_clone_10_0,
            madvdontneed: __go_clone_11_0,
            runtime_contention_stacks: __go_clone_12_0,
            scavtrace: __go_clone_13_0,
            scheddetail: __go_clone_14_0,
            schedtrace: __go_clone_15_0,
            tracebackancestors: __go_clone_16_0,
            asyncpreemptoff: __go_clone_17_0,
            harddecommit: __go_clone_18_0,
            adaptivestackstart: __go_clone_19_0,
            tracefpunwindoff: __go_clone_20_0,
            traceadvanceperiod: __go_clone_21_0,
            trace_check_stack_ownership: __go_clone_22_0,
            profstackdepth: __go_clone_23_0,
            dataindependenttiming: __go_clone_24_0,
            malloc: __go_clone_25_0,
            inittrace: __go_clone_26_0,
            sbrk: __go_clone_27_0,
            traceallocfree: __go_clone_28_0,
            panicnil: __go_clone_29_0,
            asynctimerchan: __go_clone_30_0,
        }
    }
}


impl Default for AnonymousStruct25 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_19_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_21_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_22_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_23_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_24_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_25_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_26_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_27_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_28_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_29_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_30_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            cgocheck: __go_default_0_0,
            clobberfree: __go_default_1_0,
            disablethp: __go_default_2_0,
            dontfreezetheworld: __go_default_3_0,
            efence: __go_default_4_0,
            gccheckmark: __go_default_5_0,
            gcpacertrace: __go_default_6_0,
            gcshrinkstackoff: __go_default_7_0,
            gcstoptheworld: __go_default_8_0,
            gctrace: __go_default_9_0,
            invalidptr: __go_default_10_0,
            madvdontneed: __go_default_11_0,
            runtime_contention_stacks: __go_default_12_0,
            scavtrace: __go_default_13_0,
            scheddetail: __go_default_14_0,
            schedtrace: __go_default_15_0,
            tracebackancestors: __go_default_16_0,
            asyncpreemptoff: __go_default_17_0,
            harddecommit: __go_default_18_0,
            adaptivestackstart: __go_default_19_0,
            tracefpunwindoff: __go_default_20_0,
            traceadvanceperiod: __go_default_21_0,
            trace_check_stack_ownership: __go_default_22_0,
            profstackdepth: __go_default_23_0,
            dataindependenttiming: __go_default_24_0,
            malloc: __go_default_25_0,
            inittrace: __go_default_26_0,
            sbrk: __go_default_27_0,
            traceallocfree: __go_default_28_0,
            panicnil: __go_default_29_0,
            asynctimerchan: __go_default_30_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct25 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.cgocheck.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.clobberfree.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.disablethp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.dontfreezetheworld.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.efence.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.gccheckmark.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.gcpacertrace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.gcshrinkstackoff.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.gcstoptheworld.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.gctrace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.invalidptr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.madvdontneed.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.runtime_contention_stacks.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.scavtrace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.scheddetail.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.schedtrace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.tracebackancestors.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.asyncpreemptoff.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.harddecommit.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", (*self.adaptivestackstart.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_20 = format!("{}", (*self.tracefpunwindoff.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_21 = format!("{}", (*self.traceadvanceperiod.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_22 = format!("{}", (*self.trace_check_stack_ownership.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_23 = format!("{}", (*self.profstackdepth.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_24 = format!("{}", (*self.dataindependenttiming.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_25 = format!("{}", (*self.malloc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_26 = format!("{}", (*self.inittrace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_27 = format!("{}", (*self.sbrk.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_28 = format!("{}", (*self.traceallocfree.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_29 = format!("{}", (*self.panicnil.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_30 = format!("{}", (*self.asynctimerchan.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22,
            __go_fmt_23,
            __go_fmt_24,
            __go_fmt_25,
            __go_fmt_26,
            __go_fmt_27,
            __go_fmt_28,
            __go_fmt_29,
            __go_fmt_30
        )
    }
}


pub(crate) type debug = AnonymousStruct25;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for dbgVar {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
