use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_spinbit::{lock, unlock}, os_darwin_arm64::{cputicks}, panic::{panicking}, proc::{OS_HAS_LOW_RES_CLOCK_INT}, race0::{RACEENABLED, raceacquire, racereleasemerge}, runtime2::{g, m, mutex}, stubs::{getg}, time::{time_sleep}, time_nofake::{nanotime, write}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MIN_TIME_FOR_TICKS_PER_SECOND: i32 = 5_000_000 * (1 - OS_HAS_LOW_RES_CLOCK_INT) + 100_000_000 * OS_HAS_LOW_RES_CLOCK_INT;


#[derive(Clone)]
pub struct ticksType {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub start_ticks: Arc<Mutex<Option<i64>>>,
    pub start_time: Arc<Mutex<Option<i64>>>,
    pub val: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
}

impl ticksType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.start_ticks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.start_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            start_ticks: __go_clone_1_0,
            start_time: __go_clone_2_0,
            val: __go_clone_3_0,
        }
    }
}


impl Default for ticksType {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), start_ticks: Arc::new(Mutex::new(Some(0))), start_time: Arc::new(Mutex::new(Some(0))), val: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for ticksType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.start_ticks.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.start_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.val.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for ticksType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A godebugInc provides access to internal/godebug's IncNonDefault function
/// for a given GODEBUG setting.
/// Calls before internal/godebug registers itself are dropped on the floor.
#[derive(Clone)]
pub struct godebugInc {
    pub name: Arc<Mutex<Option<String>>>,
    pub inc: Arc<Mutex<Option<internal_runtime_atomic::types::Pointer<Box<dyn FnMut() -> () + Send + Sync>>>>>,
}

impl godebugInc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.inc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            inc: __go_clone_1_0,
        }
    }
}


impl Default for godebugInc {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), inc: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for godebugInc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.inc.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for godebugInc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static ticks: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<ticksType>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static envs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static argslice: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static godebugDefault: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static godebugUpdate: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Pointer<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static godebugEnv: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Pointer<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static godebugNewIncNonDefault: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Pointer<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> + Send + Sync>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static crashFD: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uintptr>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static auxv: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<usize>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zeroVal: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 1024]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ticks.lock().unwrap() = Some(Default::default());
    *envs.lock().unwrap() = Some(vec![]);
    *argslice.lock().unwrap() = Some(vec![]);
    *godebugDefault.lock().unwrap() = Some(String::new());
    *godebugUpdate.lock().unwrap() = Some(Default::default());
    *godebugEnv.lock().unwrap() = Some(Default::default());
    *godebugNewIncNonDefault.lock().unwrap() = Some(Default::default());
    *crashFD.lock().unwrap() = Some(Default::default());
    *auxv.lock().unwrap() = Some(vec![]);
    *zeroVal.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_zero_globals() {
    *ticks.lock().unwrap() = Some(Default::default());
    *envs.lock().unwrap() = Some(vec![]);
    *argslice.lock().unwrap() = Some(vec![]);
    *godebugDefault.lock().unwrap() = Some(String::new());
    *godebugUpdate.lock().unwrap() = Some(Default::default());
    *godebugEnv.lock().unwrap() = Some(Default::default());
    *godebugNewIncNonDefault.lock().unwrap() = Some(Default::default());
    *crashFD.lock().unwrap() = Some(Default::default());
    *auxv.lock().unwrap() = Some(vec![]);
    *zeroVal.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


impl ticksType {
    /// init initializes ticks to maximize the chance that we have a good ticksPerSecond reference.
    ///
    /// Must not run concurrently with ticksPerSecond.
    pub fn init(&mut self) {
        lock(GoPtr::local((*ticks.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = nanotime(); *self.start_time.lock().unwrap() = Some(new_val); };
        { let new_val = cputicks(); *self.start_ticks.lock().unwrap() = Some(new_val); };
        unlock(GoPtr::local((*ticks.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
}

impl godebugInc {
    pub fn inc_non_default(&self) {
        let mut inc = (*self.inc.lock().unwrap().as_ref().unwrap()).load();
        if { let __nil_result = (*inc.lock().unwrap()).is_none(); __nil_result } {
        let mut newInc = (*godebugNewIncNonDefault.lock().unwrap().as_ref().unwrap()).load();
        if { let __nil_result = (*newInc.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
        { let new_val = Arc::new(Mutex::new(None)).clone(); inc = new_val; };
        { let new_val = { let __f_holder = newInc.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *inc.lock().unwrap() = __moved_val; };
        if RACEENABLED {
        racereleasemerge(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.inc.clone()) as usize))));
    }
        if !(*self.inc.lock().unwrap().as_ref().unwrap()).compare_and_swap(Arc::new(Mutex::new(None)), internal_runtime_atomic::GoPtr::local(inc.clone())) {
        { let new_val = (*self.inc.lock().unwrap().as_ref().unwrap()).load().clone(); inc = new_val; };
    }
    }
        if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.inc.clone()) as usize))));
    }
        { let __f_holder = inc.clone(); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
}

/// ticksPerSecond returns a conversion rate between the cputicks clock and the nanotime clock.
///
/// Note: Clocks are hard. Using this as an actual conversion rate for timestamps is ill-advised
/// and should be avoided when possible. Use only for durations, where a tiny error term isn't going
/// to make a meaningful difference in even a 1ms duration. If an accurate timestamp is needed,
/// use nanotime instead. (The entire Windows platform is a broad exception to this rule, where nanotime
/// produces timestamps on such a coarse granularity that the error from this conversion is actually
/// preferable.)
///
/// The strategy for computing the conversion rate is to write down nanotime and cputicks as
/// early in process startup as possible. From then, we just need to wait until we get values
/// from nanotime that we can use (some platforms have a really coarse system time granularity).
/// We require some amount of time to pass to ensure that the conversion rate is fairly accurate
/// in aggregate. But because we compute this rate lazily, there's a pretty good chance a decent
/// amount of time has passed by the time we get here.
///
/// Must be called from a normal goroutine context (running regular goroutine with a P).
///
/// Called by runtime/pprof in addition to runtime code.
///
/// TODO(mknyszek): This doesn't account for things like CPU frequency scaling. Consider
/// a more sophisticated and general approach in the future.
pub fn ticks_per_second() -> i64 {
        // Get the conversion rate if we've already computed it.
    let mut r = (*(*ticks.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).load();
    if { let __tmp_x = r; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        return r;
    }

        // Compute the conversion rate.
    loop {
        lock(GoPtr::local((*ticks.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = (*(*ticks.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).load(); r = new_val; };
        if { let __tmp_x = r; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        unlock(GoPtr::local((*ticks.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return r;
    }

                // Grab the current time in both clocks.
        let mut nowTime = nanotime();
        let mut nowTicks = cputicks();

                // See if we can use these times.
        if { let __tmp_x = nowTicks; let __tmp_y = (*{ let __field = (*ticks.lock().unwrap().as_ref().unwrap()).start_ticks.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } && { let __tmp_x = { let __tmp_x = nowTime; let __tmp_y = (*{ let __field = (*ticks.lock().unwrap().as_ref().unwrap()).start_time.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = MIN_TIME_FOR_TICKS_PER_SECOND as i64; __tmp_x > __tmp_y } {
                // Perform the calculation with floats. We don't want to risk overflow.
        { let new_val = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = nowTicks; let __tmp_y = (*{ let __field = (*ticks.lock().unwrap().as_ref().unwrap()).start_ticks.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1e+09; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = nowTime; let __tmp_y = (*{ let __field = (*ticks.lock().unwrap().as_ref().unwrap()).start_time.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); r = new_val; };
        if { let __tmp_x = r; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // Zero is both a sentinel value and it would be bad if callers used this as
                // a divisor. We tried out best, so just make it 1.
        { r += 1; }
    }
                // Zero is both a sentinel value and it would be bad if callers used this as
                // a divisor. We tried out best, so just make it 1.
        (*(*ticks.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(r))));
        unlock(GoPtr::local((*ticks.lock().unwrap().as_ref().unwrap()).lock.clone()));
        break
    }
                // Perform the calculation with floats. We don't want to risk overflow.
                // Zero is both a sentinel value and it would be bad if callers used this as
                // a divisor. We tried out best, so just make it 1.
        unlock(GoPtr::local((*ticks.lock().unwrap().as_ref().unwrap()).lock.clone()));

                // Sleep in one millisecond increments until we have a reliable time.
        time_sleep(Arc::new(Mutex::new(Some(1_000_000 as i64))));
    }
        // Grab the current time in both clocks.
        // See if we can use these times.
        // Perform the calculation with floats. We don't want to risk overflow.
        // Zero is both a sentinel value and it would be bad if callers used this as
        // a divisor. We tried out best, so just make it 1.
        // Sleep in one millisecond increments until we have a reliable time.
    r
}

/// writeErrStr writes a string to descriptor 2.
/// If SetCrashOutput(f) was called, it also writes to f.
///
///go:nosplit
pub fn write_err_str(s: Arc<Mutex<Option<String>>>) {
    write_err_data(GoPtr::local({ let __go_unsafe_result: Arc<Mutex<Option<u8>>> = unimplemented!("unsafe.StringData requires unsafe intrinsic support"); __go_unsafe_result }), Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32))));
}

/// writeErrData is the common parts of writeErr{,Str}.
///
///go:nosplit
pub fn write_err_data(data: GoPtr<u8>, n: Arc<Mutex<Option<i32>>>) {
    write(Arc::new(Mutex::new(Some(2 as usize))), Arc::new(Mutex::new(Some(data.addr()))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // If crashing, print a copy to the SetCrashOutput fd.
    let mut gp = getg();
    if { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).dying.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } || { let __nil_result = (*gp.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        {
        let mut fd = (*crashFD.lock().unwrap().as_mut().unwrap()).load();;
        if { let __tmp_x = fd; let __tmp_y = !(0 as usize) as usize; __tmp_x != __tmp_y } {
            write(Arc::new(Mutex::new(Some(fd))), Arc::new(Mutex::new(Some(data.addr()))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for ticksType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for godebugInc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
