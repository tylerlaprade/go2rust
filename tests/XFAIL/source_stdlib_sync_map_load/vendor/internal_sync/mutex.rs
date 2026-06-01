use go2rust_stdlib_stubs::*;

use crate::{GoAtomicPointer, format_slice, format_slice_values, format_slice_wrapped};

use crate::hashtriemap::*;
use crate::runtime::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MUTEX_LOCKED: i32 = 1 << 0;
pub(crate) const MUTEX_WOKEN: i32 = 1 << 1;
pub(crate) const MUTEX_STARVING: i32 = 1 << 2;
pub(crate) const MUTEX_WAITER_SHIFT: i32 = 3;
pub(crate) const STARVATION_THRESHOLD_NS: f64 = 1e6;


/// A Mutex is a mutual exclusion lock.
///
/// See package [sync.Mutex] documentation.
#[derive(Debug, Clone)]
pub struct Mutex {
    pub state: Arc<Mutex<Option<i32>>>,
    pub sema: Arc<Mutex<Option<u32>>>,
}

impl Mutex {
    pub fn __go_value_clone(&self) -> Self {
        Self { state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sema: { let __guard = self.sema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Mutex {
    fn default() -> Self {
        Self { state: Arc::new(Mutex::new(Some(0))), sema: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Mutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.state.lock().unwrap().as_ref().unwrap()), (*self.sema.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Mutex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Mutex {
    /// Lock locks m.
    ///
    /// See package [sync.Mutex] documentation.
    pub fn lock(&self) {
                // Fast path: grab unlocked mutex.
        if atomic::compare_and_swap_int32(self.state.clone(), 0 as i32, MUTEX_LOCKED as i32) {
        if race::ENABLED {
        race::acquire(Arc::new(Mutex::new(Some(self as *const _ as usize))));
    }
        return;
    }
                // Slow path (outlined so that the fast path can be inlined)
        self.lock_slow();
    }

    /// TryLock tries to lock m and reports whether it succeeded.
    ///
    /// See package [sync.Mutex] documentation.
    pub fn try_lock(&self) -> bool {
        let mut old = Arc::new(Mutex::new(Some({ let __selector_holder = self.state.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = MUTEX_LOCKED; let __tmp_y = MUTEX_STARVING; __tmp_x | __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        return false;
    }
                // There may be a goroutine waiting for the mutex, but we are
                // running now and can try to grab the mutex before that
                // goroutine wakes up.
        if !atomic::compare_and_swap_int32(self.state.clone(), old.clone(), { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_LOCKED as i32; __tmp_x | __tmp_y }) {
        return false;
    }
        if race::ENABLED {
        race::acquire(Arc::new(Mutex::new(Some(self as *const _ as usize))));
    }
        true
    }

    pub fn lock_slow(&self) {
        let mut waitStartTime: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        let mut starving = Arc::new(Mutex::new(Some(false)));
        let mut awoke = Arc::new(Mutex::new(Some(false)));
        let mut iter = Arc::new(Mutex::new(Some(0)));
        let mut old = Arc::new(Mutex::new(Some({ let __selector_holder = self.state.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        loop {
                // Don't spin in starvation mode, ownership is handed off to waiters
                // so we won't be able to acquire the mutex anyway.
        if { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = MUTEX_LOCKED; let __tmp_y = MUTEX_STARVING; __tmp_x | __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = MUTEX_LOCKED as i32; __tmp_x == __tmp_y } && runtime_can_spin(Arc::new(Mutex::new(Some({ let __arg_holder = iter.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // Active spinning makes sense.
                // Try to set mutexWoken flag to inform Unlock
                // to not wake other blocked goroutines.
        if !{ let __v = (*awoke.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_WOKEN as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_WAITER_SHIFT as i32; __tmp_x >> __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && atomic::compare_and_swap_int32(self.state.clone(), old.clone(), { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_WOKEN as i32; __tmp_x | __tmp_y }) {
        { let new_val = true; *awoke.lock().unwrap() = Some(new_val); };
    }
        runtime_do_spin();
        { let mut guard = iter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = { let __selector_holder = self.state.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *old.lock().unwrap() = Some(new_val); };
        continue
    }
                // Active spinning makes sense.
                // Try to set mutexWoken flag to inform Unlock
                // to not wake other blocked goroutines.
        let mut new = { let __owned = old.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };

                // Don't try to acquire starving mutex, new arriving goroutines must queue.
        if { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_STARVING as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        { let __rhs = MUTEX_LOCKED as i32; let mut guard = new.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = MUTEX_LOCKED; let __tmp_y = MUTEX_STARVING; __tmp_x | __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        { let __rhs = ((1 as i32) << (MUTEX_WAITER_SHIFT as i32)) as i32; let mut guard = new.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

                // The current goroutine switches mutex to starvation mode.
                // But if the mutex is currently unlocked, don't do the switch.
                // Unlock expects that starving mutex has waiters, which will not
                // be true in this case.
        if { let __v = (*starving.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_LOCKED as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        { let __rhs = MUTEX_STARVING as i32; let mut guard = new.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        if { let __v = (*awoke.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // The goroutine has been woken from sleep,
                // so we need to reset the flag in either case.
        if { let __tmp_x = { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_WOKEN as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("sync: inconsistent mutex state".to_string()))));
    }
        { let __rhs = MUTEX_WOKEN as i32; let mut guard = new.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
    }
                // The goroutine has been woken from sleep,
                // so we need to reset the flag in either case.
        if atomic::compare_and_swap_int32(self.state.clone(), old.clone(), new.clone()) {
        if { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = MUTEX_LOCKED; let __tmp_y = MUTEX_STARVING; __tmp_x | __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        break
    }
                // locked the mutex with CAS
                // If we were already waiting before, queue at the front of the queue.
        let mut queueLifo = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*waitStartTime.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y })));
        if { let __tmp_x = { let __v = (*waitStartTime.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = runtime_nanotime(); *waitStartTime.lock().unwrap() = Some(new_val); };
    }
        runtime__semacquire_mutex(self.sema.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = queueLifo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2))));
        { let new_val = { let __v = (*starving.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __tmp_x = runtime_nanotime(); let __tmp_y = { let __v = (*waitStartTime.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = STARVATION_THRESHOLD_NS as i64; __tmp_x > __tmp_y }; *starving.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.state.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *old.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_STARVING as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
                // If this goroutine was woken and mutex is in starvation mode,
                // ownership was handed off to us but mutex is in somewhat
                // inconsistent state: mutexLocked is not set and we are still
                // accounted as waiter. Fix that.
        if { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = MUTEX_LOCKED; let __tmp_y = MUTEX_WOKEN; __tmp_x | __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_WAITER_SHIFT as i32; __tmp_x >> __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("sync: inconsistent mutex state".to_string()))));
    }
        let mut delta = Arc::new(Mutex::new(Some(({ let __tmp_x = MUTEX_LOCKED; let __tmp_y = { let __tmp_x = 1; let __tmp_y = MUTEX_WAITER_SHIFT; __tmp_x << __tmp_y }; __tmp_x - __tmp_y }) as i32)));
        if !{ let __v = (*starving.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_WAITER_SHIFT as i32; __tmp_x >> __tmp_y }; let __tmp_y = 1 as i32; __tmp_x == __tmp_y } {
                // Exit starvation mode.
                // Critical to do it here and consider wait time.
                // Starvation mode is so inefficient, that two goroutines
                // can go lock-step infinitely once they switch mutex
                // to starvation mode.
        { let __rhs = MUTEX_STARVING as i32; let mut guard = delta.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // Exit starvation mode.
                // Critical to do it here and consider wait time.
                // Starvation mode is so inefficient, that two goroutines
                // can go lock-step infinitely once they switch mutex
                // to starvation mode.
        atomic::add_int32(self.state.clone(), delta.clone());
        break
    }
                // If this goroutine was woken and mutex is in starvation mode,
                // ownership was handed off to us but mutex is in somewhat
                // inconsistent state: mutexLocked is not set and we are still
                // accounted as waiter. Fix that.
                // Exit starvation mode.
                // Critical to do it here and consider wait time.
                // Starvation mode is so inefficient, that two goroutines
                // can go lock-step infinitely once they switch mutex
                // to starvation mode.
        { let new_val = true; *awoke.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *iter.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __selector_holder = self.state.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *old.lock().unwrap() = Some(new_val); };
    }
    }
                // Don't spin in starvation mode, ownership is handed off to waiters
                // so we won't be able to acquire the mutex anyway.
                // Active spinning makes sense.
                // Try to set mutexWoken flag to inform Unlock
                // to not wake other blocked goroutines.
                // Don't try to acquire starving mutex, new arriving goroutines must queue.
                // The current goroutine switches mutex to starvation mode.
                // But if the mutex is currently unlocked, don't do the switch.
                // Unlock expects that starving mutex has waiters, which will not
                // be true in this case.
                // The goroutine has been woken from sleep,
                // so we need to reset the flag in either case.
                // locked the mutex with CAS
                // If we were already waiting before, queue at the front of the queue.
                // If this goroutine was woken and mutex is in starvation mode,
                // ownership was handed off to us but mutex is in somewhat
                // inconsistent state: mutexLocked is not set and we are still
                // accounted as waiter. Fix that.
                // Exit starvation mode.
                // Critical to do it here and consider wait time.
                // Starvation mode is so inefficient, that two goroutines
                // can go lock-step infinitely once they switch mutex
                // to starvation mode.
        if race::ENABLED {
        race::acquire(Arc::new(Mutex::new(Some(self as *const _ as usize))));
    }
    }

    /// Unlock unlocks m.
    ///
    /// See package [sync.Mutex] documentation.
    pub fn unlock(&self) {
        if race::ENABLED {
        let _ = self.state.clone();
        race::release(Arc::new(Mutex::new(Some(self as *const _ as usize))));
    }
                // Fast path: drop lock bit.
        let mut new = atomic::add_int32(self.state.clone(), -MUTEX_LOCKED as i32);
        if { let __tmp_x = new; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
                // Outlined slow path to allow inlining the fast path.
                // To hide unlockSlow during tracing we skip one extra frame when tracing GoUnblock.
        self.unlock_slow(Arc::new(Mutex::new(Some(new))));
    }
    }

    pub fn unlock_slow(&self, mut new: Arc<Mutex<Option<i32>>>) {
        if { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_LOCKED as i32; __tmp_x + __tmp_y }); let __tmp_y = MUTEX_LOCKED as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        fatal(Arc::new(Mutex::new(Some("sync: unlock of unlocked mutex".to_string()))));
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_STARVING as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        let mut old = { let __owned = new.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        loop {
                // If there are no waiters or a goroutine has already
                // been woken or grabbed the lock, no need to wake anyone.
                // In starvation mode ownership is directly handed off from unlocking
                // goroutine to the next waiter. We are not part of this chain,
                // since we did not observe mutexStarving when we unlocked the mutex above.
                // So get off the way.
        if { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_WAITER_SHIFT as i32; __tmp_x >> __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __tmp_x = MUTEX_LOCKED; let __tmp_y = MUTEX_WOKEN; __tmp_x | __tmp_y }; let __tmp_y = MUTEX_STARVING; __tmp_x | __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        return;
    }

                // Grab the right to wake someone.
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as i32) << (MUTEX_WAITER_SHIFT as i32)) as i32; __tmp_x - __tmp_y }); let __tmp_y = MUTEX_WOKEN as i32; __tmp_x | __tmp_y }; *new.lock().unwrap() = Some(new_val); };
        if atomic::compare_and_swap_int32(self.state.clone(), old.clone(), new.clone()) {
        runtime__semrelease(self.sema.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(2))));
        return;
    }
        { let new_val = { let __selector_holder = self.state.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *old.lock().unwrap() = Some(new_val); };
    }
    } else {
                // Starving mode: handoff mutex ownership to the next waiter, and yield
                // our time slice so that the next waiter can start to run immediately.
                // Note: mutexLocked is not set, the waiter will set it after wakeup.
                // But mutex is still considered locked if mutexStarving is set,
                // so new coming goroutines won't acquire it.
        runtime__semrelease(self.sema.clone(), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(2))));
    }
    }
}