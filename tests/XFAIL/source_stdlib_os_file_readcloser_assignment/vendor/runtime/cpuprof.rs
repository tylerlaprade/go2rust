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
    os_darwin::{osyield},
    proc::{__external_code, __lost_external_code, __lost_s_i_g_p_r_o_f_during_atomic64, __system, prof},
    profbuf::{profBuf},
    runtime2::{mutex},
    time_nofake::{nanotime},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_C_P_U_PROF_STACK: i32 = 64;
pub(crate) const PROF_BUF_WORD_COUNT: i32 = 1 << 17;
pub(crate) const PROF_BUF_TAG_COUNT: i32 = 1 << 14;


#[derive(Clone)]
pub struct cpuProfile {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub on: Arc<Mutex<Option<bool>>>,
    pub log: Arc<Mutex<Option<profBuf>>>,
    pub extra: Arc<Mutex<Option<[usize; 1000]>>>,
    pub num_extra: Arc<Mutex<Option<i32>>>,
    pub lost_extra: Arc<Mutex<Option<u64>>>,
    pub lost_atomic: Arc<Mutex<Option<u64>>>,
}

impl cpuProfile {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.on.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.log.clone();
        let __go_clone_3_0 = { let __guard = self.extra.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.num_extra.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.lost_extra.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.lost_atomic.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            on: __go_clone_1_0,
            log: __go_clone_2_0,
            extra: __go_clone_3_0,
            num_extra: __go_clone_4_0,
            lost_extra: __go_clone_5_0,
            lost_atomic: __go_clone_6_0,
        }
    }
}


impl Default for cpuProfile {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            lock: __go_default_0_0,
            on: __go_default_1_0,
            log: __go_default_2_0,
            extra: __go_default_3_0,
            num_extra: __go_default_4_0,
            lost_extra: __go_default_5_0,
            lost_atomic: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for cpuProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.on.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.log.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", format_slice(&self.extra));
        let __go_fmt_4 = format!("{}", (*self.num_extra.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.lost_extra.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.lost_atomic.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}

impl GoJsonDecode for cpuProfile {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static cpuprof: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<cpuProfile>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *cpuprof.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *cpuprof.lock().unwrap() = Some(Default::default());
}


impl cpuProfile {
    /// add adds the stack trace to the profile.
    /// It is called from signal handlers and other limited environments
    /// and cannot allocate memory or acquire locks that might be
    /// held at the time of the signal, nor can it use substantial amounts
    /// of stack.
    ///
    ///go:nowritebarrierrec
    pub fn add(&mut self, tagPtr: Arc<Mutex<Option<usize>>>, stk: Arc<Mutex<Option<Vec<usize>>>>) {
                // Simple cas-lock to coordinate with setcpuprofilerate.
        while !(*(*prof.lock().unwrap().as_ref().unwrap()).signal_lock.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as u32))), Arc::new(Mutex::new(Some(1 as u32)))) {
                // TODO: Is it safe to osyield here? https://go.dev/issue/52672
        osyield();
    }
                // TODO: Is it safe to osyield here? https://go.dev/issue/52672
        if { let __tmp_x = (*(*prof.lock().unwrap().as_ref().unwrap()).hz.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        if { let __tmp_x = (*self.num_extra.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } || { let __tmp_x = (*self.lost_extra.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } || { let __tmp_x = (*self.lost_atomic.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        self.add_extra();
    }
        let mut hdr = Arc::new(Mutex::new(Some([1 as u64])));
                // Note: write "knows" that the argument is &gp.labels,
                // because otherwise its write barrier behavior may not
                // be correct. See the long comment there before
                // changing the argument here.
        (*(*cpuprof.lock().unwrap().as_ref().unwrap()).log.lock().unwrap().as_mut().unwrap()).write(
            tagPtr.clone(),
            Arc::new(Mutex::new(Some(nanotime()))),
            Arc::new(Mutex::new(Some({
                let __seq_holder = hdr.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = __seq.len();
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))),
            stk.clone(),
        );
    }
                // Note: write "knows" that the argument is &gp.labels,
                // because otherwise its write barrier behavior may not
                // be correct. See the long comment there before
                // changing the argument here.
        (*(*prof.lock().unwrap().as_ref().unwrap()).signal_lock.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
    }

    /// addNonGo adds the non-Go stack trace to the profile.
    /// It is called from a non-Go thread, so we cannot use much stack at all,
    /// nor do anything that needs a g or an m.
    /// In particular, we can't call cpuprof.log.write.
    /// Instead, we copy the stack into cpuprof.extra,
    /// which will be drained the next time a Go thread
    /// gets the signal handling event.
    ///
    ///go:nosplit
    ///go:nowritebarrierrec
    pub fn add_non_go(&self, stk: Arc<Mutex<Option<Vec<usize>>>>) {
                // Simple cas-lock to coordinate with SetCPUProfileRate.
                // (Other calls to add or addNonGo should be blocked out
                // by the fact that only one SIGPROF can be handled by the
                // process at a time. If not, this lock will serialize those too.
                // The use of timer_create(2) on Linux to request process-targeted
                // signals may have changed this.)
        while !(*(*prof.lock().unwrap().as_ref().unwrap()).signal_lock.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as u32))), Arc::new(Mutex::new(Some(1 as u32)))) {
                // TODO: Is it safe to osyield here? https://go.dev/issue/52672
        osyield();
    }
                // TODO: Is it safe to osyield here? https://go.dev/issue/52672
        if {
            let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (*{ let __field = (*cpuprof.lock().unwrap().as_ref().unwrap()).num_extra.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y } as i32);
            let __tmp_y = 1000;
            __tmp_x < __tmp_y
        } {
        let mut i = Arc::new(Mutex::new(Some({ let __selector_holder = (*cpuprof.lock().unwrap().as_ref().unwrap()).num_extra.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        (*(*cpuprof.lock().unwrap().as_ref().unwrap()).extra.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = 1; let __tmp_y = ((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()).clone();
        {
            let _dst_start = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize;
            let _dst_len = (*(*cpuprof.lock().unwrap().as_ref().unwrap()).extra.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = stk.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*(*cpuprof.lock().unwrap().as_ref().unwrap()).extra.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        };
        { let __target = (*cpuprof.lock().unwrap().as_ref().unwrap()).num_extra.clone(); let __rhs = { let __tmp_x = 1; let __tmp_y = ((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
        { let __target = (*cpuprof.lock().unwrap().as_ref().unwrap()).lost_extra.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        (*(*prof.lock().unwrap().as_ref().unwrap()).signal_lock.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
    }

    /// addExtra adds the "extra" profiling events,
    /// queued by addNonGo, to the profile log.
    /// addExtra is called either from a signal handler on a Go thread
    /// or from an ordinary goroutine; either way it can use stack
    /// and has a g. The world may be stopped, though.
    pub fn add_extra(&mut self) {
                // Copy accumulated non-Go profile events.
        let mut hdr = Arc::new(Mutex::new(Some([1 as u64])));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.num_extra.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        (*self.log.lock().unwrap().as_mut().unwrap()).write(
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Some(0 as i64))),
            Arc::new(Mutex::new(Some({
                let __seq_holder = hdr.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = __seq.len();
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))),
            Arc::new(Mutex::new(Some({
                let __seq_holder = self.extra.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize;
                let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.extra.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize;
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))),
        );
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.extra.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let new_val = 0; *self.num_extra.lock().unwrap() = Some(new_val); };
                // Report any lost events.
        if { let __tmp_x = (*self.lost_extra.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        let mut hdr = Arc::new(Mutex::new(Some([{ let __selector_holder = self.lost_extra.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }])));
        let mut lostStk = Arc::new(Mutex::new(Some([{ let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__lost_external_code.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y }, { let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__external_code.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y }])));
        (*self.log.lock().unwrap().as_mut().unwrap()).write(
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Some(0 as i64))),
            Arc::new(Mutex::new(Some({
                let __seq_holder = hdr.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = __seq.len();
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))),
            Arc::new(Mutex::new(Some({
                let __seq_holder = lostStk.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = __seq.len();
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))),
        );
        { let new_val = 0 as u64; *self.lost_extra.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = (*self.lost_atomic.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        let mut hdr = Arc::new(Mutex::new(Some([{ let __selector_holder = self.lost_atomic.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }])));
        let mut lostStk = Arc::new(Mutex::new(Some([{ let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__lost_s_i_g_p_r_o_f_during_atomic64.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y }, { let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__system.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y }])));
        (*self.log.lock().unwrap().as_mut().unwrap()).write(
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Some(0 as i64))),
            Arc::new(Mutex::new(Some({
                let __seq_holder = hdr.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = __seq.len();
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))),
            Arc::new(Mutex::new(Some({
                let __seq_holder = lostStk.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = __seq.len();
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))),
        );
        { let new_val = 0 as u64; *self.lost_atomic.lock().unwrap() = Some(new_val); };
    }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for cpuProfile {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
