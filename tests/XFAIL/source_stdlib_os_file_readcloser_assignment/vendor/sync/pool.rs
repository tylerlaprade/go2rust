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
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_any_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{cond::{noCopy}, mutex::{Mutex}, poolqueue::{poolChain}};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

/// A Pool is a set of temporary objects that may be individually saved and
/// retrieved.
///
/// Any item stored in the Pool may be removed automatically at any time without
/// notification. If the Pool holds the only reference when this happens, the
/// item might be deallocated.
///
/// A Pool is safe for use by multiple goroutines simultaneously.
///
/// Pool's purpose is to cache allocated but unused items for later reuse,
/// relieving pressure on the garbage collector. That is, it makes it easy to
/// build efficient, thread-safe free lists. However, it is not suitable for all
/// free lists.
///
/// An appropriate use of a Pool is to manage a group of temporary items
/// silently shared among and potentially reused by concurrent independent
/// clients of a package. Pool provides a way to amortize allocation overhead
/// across many clients.
///
/// An example of good use of a Pool is in the fmt package, which maintains a
/// dynamically-sized store of temporary output buffers. The store scales under
/// load (when many goroutines are actively printing) and shrinks when
/// quiescent.
///
/// On the other hand, a free list maintained as part of a short-lived object is
/// not a suitable use for a Pool, since the overhead does not amortize well in
/// that scenario. It is more efficient to have such objects implement their own
/// free list.
///
/// A Pool must not be copied after first use.
///
/// In the terminology of [the Go memory model], a call to Put(x) “synchronizes before”
/// a call to [Pool.Get] returning that same value x.
/// Similarly, a call to New returning x “synchronizes before”
/// a call to Get returning that same value x.
///
/// [the Go memory model]: https://go.dev/ref/mem
#[derive(Clone)]
pub struct Pool {
    pub no_copy: Arc<StdMutex<Option<noCopy>>>,
    pub local: Arc<StdMutex<Option<usize>>>,
    pub local_size: Arc<StdMutex<Option<usize>>>,
    pub victim: Arc<StdMutex<Option<usize>>>,
    pub victim_size: Arc<StdMutex<Option<usize>>>,
    pub new: Arc<StdMutex<Option<Box<dyn FnMut() -> Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> + Send + Sync>>>>,
}

impl Pool {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_copy.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.local.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.local_size.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.victim.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.victim_size.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_5_0 = self.new.clone();
        Self {
            no_copy: __go_clone_0_0,
            local: __go_clone_1_0,
            local_size: __go_clone_2_0,
            victim: __go_clone_3_0,
            victim_size: __go_clone_4_0,
            new: __go_clone_5_0,
        }
    }
}


impl Default for Pool {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(StdMutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(StdMutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(StdMutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(StdMutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(StdMutex::new(None));
        Self {
            no_copy: __go_default_0_0,
            local: __go_default_1_0,
            local_size: __go_default_2_0,
            victim: __go_default_3_0,
            victim_size: __go_default_4_0,
            new: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for Pool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_copy.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.local.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.local_size.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.victim.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.victim_size.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", "<func>");
        write!(f, "{{{} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5)
    }
}

impl GoJsonDecode for Pool {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Local per-P Pool appendix.
#[derive(Clone)]
pub struct poolLocalInternal {
    pub private: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>,
    pub shared: Arc<StdMutex<Option<poolChain>>>,
}

impl poolLocalInternal {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.private.clone();
        let __go_clone_1_0 = { let __guard = self.shared.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            private: __go_clone_0_0,
            shared: __go_clone_1_0,
        }
    }
}


impl Default for poolLocalInternal {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(None));
        let __go_default_1_0 = Arc::new(StdMutex::new(Some(poolChain::default())));
        Self {
            private: __go_default_0_0,
            shared: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for poolLocalInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_any(self.private.lock().unwrap().as_ref().unwrap().as_ref()));
        let __go_fmt_1 = format!("{}", (*self.shared.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for poolLocalInternal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct poolLocal {
    pub pool_local_internal: Arc<StdMutex<Option<poolLocalInternal>>>,
    pub pad: Arc<StdMutex<Option<[u8; 96]>>>,
}

impl poolLocal {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.pool_local_internal.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pad.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            pool_local_internal: __go_clone_0_0,
            pad: __go_clone_1_0,
        }
    }
}


impl Default for poolLocal {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(poolLocalInternal::default())));
        let __go_default_1_0 = Arc::new(StdMutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            pool_local_internal: __go_default_0_0,
            pad: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for poolLocal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.pool_local_internal.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.pad));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for poolLocal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static poolRaceHash: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 128]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allPoolsMu: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::mutex::Mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allPools: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<StdMutex<Option<Pool>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static oldPools: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<StdMutex<Option<Pool>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *poolRaceHash.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *allPoolsMu.lock().unwrap() = Some(Default::default());
    *allPools.lock().unwrap() = Some(vec![]);
    *oldPools.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_zero_globals() {
    *poolRaceHash.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *allPoolsMu.lock().unwrap() = Some(Default::default());
    *allPools.lock().unwrap() = Some(vec![]);
    *oldPools.lock().unwrap() = Some(vec![]);
}


impl Pool {
    /// Put adds x to the pool.
    pub fn put(&self, x: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) {
        if { let __nil_result = (*x.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
        if internal_race::ENABLED {
        if { let __tmp_x = runtime_randn(Arc::new(StdMutex::new(Some(4 as u32)))); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // Randomly drop x on floor.
        return;
    }
                // Randomly drop x on floor.
        internal_race::release_merge(pool_race_addr(x.clone()));
        internal_race::disable();
    }
                // Randomly drop x on floor.
        let (mut l, _) = self.pin();
        if { let __nil_target = { let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().private.clone(); __field }); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = x.clone(); l.with_mut(|__ptr_value| { (*__ptr_value.pool_local_internal.lock().unwrap().as_mut().unwrap()).private = new_val; }); };
    } else {
        (*{ let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().shared.clone(); __field }); __ptr_value }.lock().unwrap().as_mut().unwrap()).push_head(x.clone());
    }
        runtime_proc_unpin();
        if internal_race::ENABLED {
        internal_race::enable();
    }
    }

    /// Get selects an arbitrary item from the [Pool], removes it from the
    /// Pool, and returns it to the caller.
    /// Get may choose to ignore the pool and treat it as empty.
    /// Callers should not assume any relation between values passed to [Pool.Put] and
    /// the values returned by Get.
    ///
    /// If Get would otherwise return nil and p.New is non-nil, Get returns
    /// the result of calling p.New.
    pub fn get(&self) -> Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> {
        if internal_race::ENABLED {
        internal_race::disable();
    }
        let (mut l, mut pid) = self.pin();
        let mut x = { let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().private.clone(); __field }); __ptr_value }.clone();
        *{ let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().private.clone(); __field }); __ptr_value }.lock().unwrap() = None;
        if { let __nil_result = (*x.lock().unwrap()).is_none(); __nil_result } {
                // Try to pop the head of the local shard. We prefer
                // the head over the tail for temporal locality of
                // reuse.
        {
            let (__tmp_0, __tmp_1) = (*{ let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().shared.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).pop_head();
            let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_0;
        };
        if { let __nil_result = (*x.lock().unwrap()).is_none(); __nil_result } {
        x = self.get_slow(Arc::new(StdMutex::new(Some(pid)))).clone();
    }
    }
                // Try to pop the head of the local shard. We prefer
                // the head over the tail for temporal locality of
                // reuse.
        runtime_proc_unpin();
        if internal_race::ENABLED {
        internal_race::enable();
        if { let __nil_result = (*x.lock().unwrap()).is_some(); __nil_result } {
        internal_race::acquire(pool_race_addr(x.clone()));
    }
    }
        if { let __nil_result = (*x.lock().unwrap()).is_none(); __nil_result } && { let __nil_target = self.new.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        x = { let __f_holder = self.new.clone(); let __f_ptr: *mut Box<dyn FnMut() -> Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone();
    }
        return x.clone();
    }

    pub fn get_slow(&self, pid: Arc<StdMutex<Option<i32>>>) -> Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> {
                // See the comment in pin regarding ordering of the loads.
        let mut size = runtime__load_acquintptr(self.local_size.clone());
        let mut locals = Arc::new(StdMutex::new(Some({ let __selector_holder = self.local.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
                // Try to steal one element from other procs.
        let mut i = Arc::new(StdMutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(StdMutex::new(Some(size as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut l: GoPtr<poolLocal> = index_local(
            Arc::new(StdMutex::new(Some({ let __arg_holder = locals.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(StdMutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*pid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }); let __tmp_y = (*Arc::new(StdMutex::new(Some(size as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y })))
        );
        {
        let (mut x, _) = (*{ let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().shared.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).pop_tail();;
        if { let __nil_result = (*x.lock().unwrap()).is_some(); __nil_result } {
            return x.clone();;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Try the victim cache. We do this after attempting to steal
                // from all primary caches because we want objects in the
                // victim cache to age out if at all possible.
        { let new_val = sync_atomic::load_uintptr(self.victim_size.clone()); size = new_val; };
        if { let __tmp_x = (*Arc::new(StdMutex::new(Some((*pid.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = size; __tmp_x >= __tmp_y } {
        return Arc::new(StdMutex::new(None));
    }
        { let new_val = { let __selector_holder = self.victim.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *locals.lock().unwrap() = Some(new_val); };
        let mut l: GoPtr<poolLocal> = index_local(Arc::new(StdMutex::new(Some({ let __arg_holder = locals.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(StdMutex::new(Some({ let __arg_holder = pid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
        let mut x = { let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().private.clone(); __field }); __ptr_value }.clone();;
        if { let __nil_result = (*x.lock().unwrap()).is_some(); __nil_result } {
            *{ let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().private.clone(); __field }); __ptr_value }.lock().unwrap() = None;;
            return x.clone();;
        }
    }
        let mut i = Arc::new(StdMutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(StdMutex::new(Some(size as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut l: GoPtr<poolLocal> = index_local(
            Arc::new(StdMutex::new(Some({ let __arg_holder = locals.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(StdMutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*pid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = (*Arc::new(StdMutex::new(Some(size as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y })))
        );
        {
        let (mut x, _) = (*{ let __ptr_value = l.with_mut(|__ptr_value| { let __field = __ptr_value.pool_local_internal.lock().unwrap().as_ref().unwrap().shared.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).pop_tail();;
        if { let __nil_result = (*x.lock().unwrap()).is_some(); __nil_result } {
            return x.clone();;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Mark the victim cache as empty for future gets don't bother
                // with it.
        sync_atomic::store_uintptr(self.victim_size.clone(), Arc::new(StdMutex::new(Some(0 as usize))));
        return Arc::new(StdMutex::new(None));
    }

    /// pin pins the current goroutine to P, disables preemption and
    /// returns poolLocal pool for the P and the P's id.
    /// Caller must call runtime_procUnpin() when done with the pool.
    pub fn pin(&self) -> (GoPtr<poolLocal>, i32) {
                // Check whether p is nil to get a panic.
                // Otherwise the nil dereference happens while the m is pinned,
                // causing a fatal error rather than a panic.
        if false {
        std::panic::panic_any(Box::new("nil Pool".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut pid = runtime_proc_pin();
                // In pinSlow we store to local and then to localSize, here we load in opposite order.
                // Since we've disabled preemption, GC cannot happen in between.
                // Thus here we must observe local at least as large localSize.
                // We can observe a newer/larger local, it is fine (we must observe its zero-initialized-ness).
        let mut s = runtime__load_acquintptr(self.local_size.clone());
        let mut l = Arc::new(StdMutex::new(Some({ let __selector_holder = self.local.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*Arc::new(StdMutex::new(Some(pid as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = s; __tmp_x < __tmp_y } {
        return (
            index_local(Arc::new(StdMutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(StdMutex::new(Some(pid)))),
            pid
        );
    }
        self.pin_slow()
    }

    pub fn pin_slow(&self) -> (GoPtr<poolLocal>, i32) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // Retry under the mutex.
                        // Can not lock the mutex while pinned.
            runtime_proc_unpin();
            (*allPoolsMu.lock().unwrap().as_ref().unwrap()).lock();
            __defer_stack.push(Box::new(move || {
        (*allPoolsMu.lock().unwrap().as_ref().unwrap()).unlock();
    }));
            let mut pid = runtime_proc_pin();
                        // poolCleanup won't be called while we are pinned.
            let mut s = Arc::new(StdMutex::new(Some({ let __selector_holder = self.local_size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            let mut l = Arc::new(StdMutex::new(Some({ let __selector_holder = self.local.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            if { let __tmp_x = (*Arc::new(StdMutex::new(Some(pid as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (
            index_local(Arc::new(StdMutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(StdMutex::new(Some(pid)))),
            pid
        );
    }
    }
            if { let __nil_target = self.local.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = { let __collection_holder = { let __append_target = allPools.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(StdMutex::new(Some(self.clone())))); __append_target.clone() }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *allPools.lock().unwrap() = new_val; };
    }
                        // If GOMAXPROCS changes between GCs, we re-allocate the array and lose the old one.
            let mut size = runtime::g_o_m_a_x_p_r_o_c_s(Arc::new(StdMutex::new(Some(0))));
            let mut local: Arc<StdMutex<Option<Vec<poolLocal>>>> = Arc::new(StdMutex::new(Some(vec![Default::default(); (size) as usize])));
            sync_atomic::store_pointer(self.local.clone(), Arc::new(StdMutex::new(Some({ let __seq_holder = local.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))));
            runtime__store_reluintptr(
                self.local_size.clone(),
                Arc::new(StdMutex::new(Some(size as usize)))
            );
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (
            GoPtr::slice_elem(GoSliceElemPtr::new(local.clone(), (pid) as usize)),
            pid
        );
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
                (GoPtr::nil(), 0 as i32)
            }
        }
    }
}

impl poolLocal {
}

/// from runtime
///
///go:linkname runtime_randn runtime.randn
pub fn runtime_randn(n: Arc<StdMutex<Option<u32>>>) -> u32 {
    let _ = n;
    0
}


/// poolRaceAddr returns an address to use as the synchronization point
/// for race detector logic. We don't use the actual pointer stored in x
/// directly, for fear of conflicting with other synchronization on that address.
/// Instead, we hash the pointer to get an index into poolRaceHash.
/// See discussion on golang.org/cl/31589.
pub fn pool_race_addr(x: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<StdMutex<Option<usize>>> {
    let mut ptr = Arc::new(StdMutex::new(Some({ let __seq = { let __seq_holder = Arc::new(StdMutex::new({ let __ptr = Arc::new(StdMutex::new(Some(Arc::as_ptr(&x.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[usize; 2]>(unimplemented!("unsafe.Pointer conversion to [usize; 2]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as usize)));
    let mut h = Arc::new(StdMutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*Arc::new(StdMutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as u32 as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0x85ebca6b as u64; __tmp_x * __tmp_y }); let __tmp_y = 16; __tmp_x >> __tmp_y }) as u32)));
    return Arc::new(StdMutex::new(Some({ let __seq_holder = poolRaceHash.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(StdMutex::new(Some((*poolRaceHash.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y }) as usize] as *const _ as usize })));
}

/// poolCleanup should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/gopkg
///   - github.com/songzhibin97/gkit
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname poolCleanup
pub fn pool_cleanup() {
        // This function is called with the world stopped, at the beginning of a garbage collection.
        // It must not allocate and probably should not call any runtime functions.
        // Because the world is stopped, no pool user can be in a
        // pinned section (in effect, this has all Ps pinned).
        // Drop victim caches from all pools.
    { let __range_holder = oldPools.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        *(*p.lock().unwrap().as_ref().unwrap()).victim.lock().unwrap() = None;
        { let new_val = 0 as usize; *(*p.lock().unwrap().as_ref().unwrap()).victim_size.lock().unwrap() = Some(new_val); };
    } }

        // Move primary cache to victim cache.
    { let __range_holder = allPools.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        { let new_val = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).local.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*p.lock().unwrap().as_ref().unwrap()).victim.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).local_size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*p.lock().unwrap().as_ref().unwrap()).victim_size.lock().unwrap() = Some(new_val); };
        *(*p.lock().unwrap().as_ref().unwrap()).local.lock().unwrap() = None;
        { let new_val = 0 as usize; *(*p.lock().unwrap().as_ref().unwrap()).local_size.lock().unwrap() = Some(new_val); };
    } }

        // The pools with non-empty primary caches now have non-empty
        // victim caches and no pools have primary caches.
    {
        let __tmp_0 = allPools.clone();
        let __tmp_1 = None;
        *oldPools.lock().unwrap() = __tmp_0.lock().unwrap().take();
        *allPools.lock().unwrap() = __tmp_1;
    };
}

fn __go_init_0() {
    runtime_register_pool_cleanup(Arc::new(StdMutex::new(Some(Box::new(move || { pool_cleanup() }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

pub fn index_local(l: Arc<StdMutex<Option<usize>>>, i: Arc<StdMutex<Option<i32>>>) -> GoPtr<poolLocal> {
    let mut lp = Arc::new(StdMutex::new(Some({
        let __tmp_x = (*Arc::new(StdMutex::new(Some((*l.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
        let __tmp_y = { let __tmp_x = (*Arc::new(StdMutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(StdMutex::new(Some(std::mem::size_of::<poolLocal>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y };
        __tmp_x + __tmp_y
    })));
    return GoPtr::raw({ let __ptr = lp.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
}

/// Implemented in runtime.
pub fn runtime_register_pool_cleanup(cleanup: Arc<StdMutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
    let _ = cleanup;
}


pub fn runtime_proc_pin() -> i32 {
    0
}


pub fn runtime_proc_unpin() {
}


///go:linkname runtime_LoadAcquintptr internal/runtime/atomic.LoadAcquintptr
pub fn runtime__load_acquintptr(ptr: Arc<StdMutex<Option<usize>>>) -> usize {
    let __value = (*ptr.lock().unwrap().as_ref().unwrap()).clone();
    __value
}


///go:linkname runtime_StoreReluintptr internal/runtime/atomic.StoreReluintptr
pub fn runtime__store_reluintptr(ptr: Arc<StdMutex<Option<usize>>>, val: Arc<StdMutex<Option<usize>>>) -> usize {
    let __stored = (*val.lock().unwrap().as_ref().unwrap()).clone();
    *ptr.lock().unwrap().as_mut().unwrap() = __stored;
    __stored
}


pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for Pool {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for poolLocalInternal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for poolLocal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
