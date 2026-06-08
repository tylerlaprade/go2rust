use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lockrank::{lockRank}, lockrank_off::{get_lock_rank, lock_with_rank, unlock_with_rank}, mbitmap::{MALLOC_HEADER_SIZE}, mprof::{lockTimer, mLockProfile}, os_darwin::{osyield, semacreate, semasleep, semawakeup}, panic::{throw}, proc::{m0}, rand::{cheaprandn}, runtime2::{g, m, muintptr, mutex, ncpu, sched}, stack::{STACK_PREEMPT}, stubs::{getg, procyield}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const ACTIVE_SPIN: i32 = 4;
pub(crate) const ACTIVE_SPIN_CNT: i32 = 30;


pub(crate) const MUTEX_LOCKED: i32 = 0x001;
pub(crate) const MUTEX_SLEEPING: i32 = 0x002;
pub(crate) const MUTEX_SPINNING: i32 = 0x100;
pub(crate) const MUTEX_STACK_LOCKED: i32 = 0x200;
pub(crate) const MUTEX_M_MASK: i32 = 0x3FF;
pub(crate) const MUTEX_M_OFFSET: i32 = MALLOC_HEADER_SIZE;
pub(crate) const MUTEX_ACTIVE_SPIN_COUNT: i32 = 4;
pub(crate) const MUTEX_ACTIVE_SPIN_SIZE: i32 = 30;
pub(crate) const MUTEX_PASSIVE_SPIN_COUNT: i32 = 1;
pub(crate) const MUTEX_TAIL_WAKE_PERIOD: i32 = 16;


/// mWaitList is part of the M struct, and holds the list of Ms that are waiting
/// for a particular runtime.mutex.
///
/// When an M is unable to immediately obtain a lock, it adds itself to the list
/// of Ms waiting for the lock. It does that via this struct's next field,
/// forming a singly-linked list with the mutex's key field pointing to the head
/// of the list.
#[derive(Debug, Clone)]
pub struct mWaitList {
    pub next: Arc<Mutex<Option<muintptr>>>,
}

impl mWaitList {
    pub fn __go_value_clone(&self) -> Self {
        Self { next: { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mWaitList {
    fn default() -> Self {
        Self { next: Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for mWaitList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.next.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mWaitList {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


///go:nosplit
pub fn key8(p: Arc<Mutex<Option<usize>>>) -> Option<GoArrayElemPtr<u8, 8>> {
    if internal_goarch::BIG_ENDIAN {
        return Some(GoArrayElemPtr::new(Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 8]>(unimplemented!("unsafe.Pointer conversion to [u8; 8]")) } })).clone(), ({ let __tmp_x = { let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 1; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize));
    }
    Some(GoArrayElemPtr::new(Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 8]>(unimplemented!("unsafe.Pointer conversion to [u8; 8]")) } })).clone(), (0) as usize))
}

/// mutexWaitListHead recovers a full muintptr that was missing its low bits.
/// With the exception of the static m0 value, it requires allocating runtime.m
/// values in a size class with a particular minimum alignment. The 2048-byte
/// size class allows recovering the full muintptr value even after overwriting
/// the low 11 bits with flags. We can use those 11 bits as 3 flags and an
/// atomically-swapped byte.
///
///go:nosplit
pub fn mutex_wait_list_head(v: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::runtime2::muintptr>>> {
    {
        let mut highBits = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_M_MASK as usize; __tmp_x & ! __tmp_y })));;
        if { let __tmp_x = { let __v = (*highBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            return Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize)))))));;
        } else {
        let mut m0bits = Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&m0.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize)))))));;
        if { let __tmp_x = { let __v = (*highBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*m0bits.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MUTEX_M_MASK as usize; __tmp_x & ! __tmp_y }; __tmp_x == __tmp_y } {
            return { let __owned = m0bits.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        } else {
            return Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*highBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_M_OFFSET as usize; __tmp_x + __tmp_y } as usize)))))));;
        }
    }
    }
}

/// mutexPreferLowLatency reports if this mutex prefers low latency at the risk
/// of performance collapse. If so, we can allow all waiting threads to spin on
/// the state word rather than go to sleep.
///
/// TODO: We could have the waiting Ms each spin on their own private cache line,
/// especially if we can put a bound on the on-CPU time that would consume.
///
/// TODO: If there's a small set of mutex values with special requirements, they
/// could make use of a more specialized lock2/unlock2 implementation. Otherwise,
/// we're constrained to what we can fit within a single uintptr with no
/// additional storage on the M for each lock held.
///
///go:nosplit
pub fn mutex_prefer_low_latency(l: GoPtr<crate::runtime2::mutex>) -> bool {
    { let _switch_val = l.clone();
    if { let __case = GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()); GoPtr::ptr_eq(&_switch_val, &__case) } {
                        // We often expect sched.lock to pass quickly between Ms in a way that
                        // each M has unique work to do: for instance when we stop-the-world
                        // (bringing each P to idle) or add new netpoller-triggered work to the
                        // global run queue.
            return true;
        } else {
            return false;
        }
    }
}

pub fn lock(l: GoPtr<crate::runtime2::mutex>) {
    lock_with_rank(l.clone(), get_lock_rank(l.clone()));
}

pub fn lock2(l: GoPtr<crate::runtime2::mutex>) {
    let mut gp = getg();
    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime\u{b7}lock: lock count".to_string()))));
    }
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

    let mut k8: Option<GoArrayElemPtr<u8, 8>> = key8({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone());

        // Speculative grab for lock.
    let mut v8 = internal_runtime_atomic::xchg8(unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"), Arc::new(Mutex::new(Some(MUTEX_LOCKED as u8))));
    if { let __tmp_x = { let __tmp_x = v8; let __tmp_y = MUTEX_LOCKED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __tmp_x = v8; let __tmp_y = MUTEX_SLEEPING as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        internal_runtime_atomic::or8(unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"), Arc::new(Mutex::new(Some(MUTEX_SLEEPING as u8))));
    }
        return;
    }
    semacreate({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field });

    let mut timer = Arc::new(Mutex::new(Some(crate::mprof::lockTimer { lock: l.clone(), ..Default::default() })));
    { let __recv = timer.clone(); let __recv_ptr: *mut crate::mprof::lockTimer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mprof::lockTimer }; let __result = unsafe { &mut *__recv_ptr }.begin(); __result };

        // On uniprocessors, no point spinning.
        // On multiprocessors, spin for mutexActiveSpinCount attempts.
    let mut spin = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = (*ncpu.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x > __tmp_y } {
        { let new_val = 4; *spin.lock().unwrap() = Some(new_val); };
    }

    let mut weSpin: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));let mut atTail: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut v = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone()));
    let mut i = Arc::new(Mutex::new(Some(0)));
    'try_acquire: loop {
        if { let __tmp_x = { let __tmp_x = v; let __tmp_y = MUTEX_LOCKED as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        if { let __v = (*weSpin.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut next = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = v; let __tmp_y = MUTEX_SPINNING as usize; __tmp_x & ! __tmp_y }); let __tmp_y = MUTEX_SLEEPING as usize; __tmp_x | __tmp_y }; let __tmp_y = MUTEX_LOCKED as usize; __tmp_x | __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_M_MASK as usize; __tmp_x & ! __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // The fast-path Xchg8 may have cleared mutexSleeping. Fix
                // the hint so unlock2 knows when to use its slow path.
        { let new_val = { let __tmp_x = { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_SLEEPING as usize; __tmp_x & ! __tmp_y }; *next.lock().unwrap() = Some(new_val); };
    }
                // The fast-path Xchg8 may have cleared mutexSleeping. Fix
                // the hint so unlock2 knows when to use its slow path.
        if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let __recv = timer.clone(); let __recv_ptr: *const crate::mprof::lockTimer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mprof::lockTimer }; let __result = unsafe { &*__recv_ptr }.end(); __result };
        return;
    }
    } else {
        let mut prev8 = internal_runtime_atomic::xchg8(unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"), Arc::new(Mutex::new(Some(((MUTEX_LOCKED as u8) | (MUTEX_SLEEPING as u8)) as u8))));
        if { let __tmp_x = { let __tmp_x = prev8; let __tmp_y = MUTEX_LOCKED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let __recv = timer.clone(); let __recv_ptr: *const crate::mprof::lockTimer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mprof::lockTimer }; let __result = unsafe { &*__recv_ptr }.end(); __result };
        return;
    }
    }
                // The fast-path Xchg8 may have cleared mutexSleeping. Fix
                // the hint so unlock2 knows when to use its slow path.
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone())); v = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'try_acquire
    }

                // The fast-path Xchg8 may have cleared mutexSleeping. Fix
                // the hint so unlock2 knows when to use its slow path.
        if !{ let __v = (*weSpin.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = { let __tmp_x = v; let __tmp_y = MUTEX_SPINNING as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } && internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some({ let __tmp_x = v; let __tmp_y = MUTEX_SPINNING as usize; __tmp_x | __tmp_y })))) {
        { let __rhs = MUTEX_SPINNING as usize; v = v | __rhs; };
        { let new_val = true; *weSpin.lock().unwrap() = Some(new_val); };
    }

        if { let __v = (*weSpin.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __v = (*atTail.lock().unwrap().as_ref().unwrap()).clone(); __v } || mutex_prefer_low_latency(l.clone()) {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*spin.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        procyield(Arc::new(Mutex::new(Some(MUTEX_ACTIVE_SPIN_SIZE as u32))));
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone())); v = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'try_acquire
    } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*spin.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x < __tmp_y } {
        osyield();
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone())); v = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'try_acquire
    }
    }

                // TODO: Consider removing this step. See https://go.dev/issue/69268.
                // Go to sleep
        if { let __tmp_x = { let __tmp_x = v; let __tmp_y = MUTEX_LOCKED as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime\u{b7}lock: sleeping while lock is available".to_string()))));
    }

                // Store the current head of the list of sleeping Ms in our gp.m.mWaitList.next field
        { let new_val = mutex_wait_list_head(Arc::new(Mutex::new(Some(v)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).m_wait_list.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = __moved_val; };

                // Pack a (partial) pointer to this M with the current lock state bits
        let mut next = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*gp.lock().unwrap().as_ref().unwrap()).m.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MUTEX_M_MASK as usize; __tmp_x & ! __tmp_y }); let __tmp_y = { let __tmp_x = v; let __tmp_y = MUTEX_M_MASK as usize; __tmp_x & __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = MUTEX_SLEEPING as usize; __tmp_x | __tmp_y })));
        if { let __v = (*weSpin.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = { let __tmp_x = { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MUTEX_SPINNING as usize; __tmp_x & ! __tmp_y }; *next.lock().unwrap() = Some(new_val); };
    }

        if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = false; *weSpin.lock().unwrap() = Some(new_val); };
                // We've pushed ourselves onto the stack of waiters. Wait.
        semasleep(Arc::new(Mutex::new(Some(-1 as i64))));
        { let new_val = { let __tmp_x = { let __selector_holder = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).m_wait_list.lock().unwrap().as_ref().unwrap()).next.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y }; *atTail.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    }

                // We've pushed ourselves onto the stack of waiters. Wait.
                // we were at risk of starving
        { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize)))); *(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).m_wait_list.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = Some(new_val); };
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone())); v = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn unlock(l: GoPtr<crate::runtime2::mutex>) {
    unlock_with_rank(l.clone());
}

/// We might not be holding a p in this code.
///
///go:nowritebarrier
pub fn unlock2(l: GoPtr<crate::runtime2::mutex>) {
    let mut gp = getg();

    let mut prev8 = internal_runtime_atomic::xchg8(unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"), Arc::new(Mutex::new(Some(0 as u8))));
    if { let __tmp_x = { let __tmp_x = prev8; let __tmp_y = MUTEX_LOCKED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("unlock of unlocked lock".to_string()))));
    }

    if { let __tmp_x = { let __tmp_x = prev8; let __tmp_y = MUTEX_SLEEPING as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        unlock2_wake(l.clone());
    }

    (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).m_lock_profile.lock().unwrap().as_mut().unwrap()).record_unlock(l.clone());
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime\u{b7}unlock: lock count".to_string()))));
    }
    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = STACK_PREEMPT as usize; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    }
}

/// unlock2Wake updates the list of Ms waiting on l, waking an M if necessary.
///
///go:nowritebarrier
pub fn unlock2_wake(l: GoPtr<crate::runtime2::mutex>) {
    let mut v = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone()));

        // On occasion, seek out and wake the M at the bottom of the stack so it
        // doesn't starve.
    let mut antiStarve = Arc::new(Mutex::new(Some({ let __tmp_x = cheaprandn(Arc::new(Mutex::new(Some(MUTEX_TAIL_WAKE_PERIOD as u32)))); let __tmp_y = 0 as u32; __tmp_x == __tmp_y })));
    if !({ let __v = (*antiStarve.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = { let __tmp_x = v; let __tmp_y = MUTEX_SPINNING as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || mutex_prefer_low_latency(l.clone())) {
        return;
    }

        // no spinners means we must wake
        // prefer waiters be awake as much as possible
    loop {
        if { let __tmp_x = { let __tmp_x = v; let __tmp_y = MUTEX_M_MASK as usize; __tmp_x & ! __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = v; let __tmp_y = MUTEX_STACK_LOCKED as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // No waiting Ms means nothing to do.
                //
                // If the stack lock is unavailable, its owner would make the same
                // wake decisions that we would, so there's nothing for us to do.
                //
                // Although: This thread may have a different call stack, which
                // would result in a different entry in the mutex contention profile
                // (upon completion of go.dev/issue/66999). That could lead to weird
                // results if a slow critical section ends but another thread
                // quickly takes the lock, finishes its own critical section,
                // releases the lock, and then grabs the stack lock. That quick
                // thread would then take credit (blame) for the delay that this
                // slow thread caused. The alternative is to have more expensive
                // atomic operations (a CAS) on the critical path of unlock2.
        return;
    }

                // No waiting Ms means nothing to do.
                //
                // If the stack lock is unavailable, its owner would make the same
                // wake decisions that we would, so there's nothing for us to do.
                //
                // Although: This thread may have a different call stack, which
                // would result in a different entry in the mutex contention profile
                // (upon completion of go.dev/issue/66999). That could lead to weird
                // results if a slow critical section ends but another thread
                // quickly takes the lock, finishes its own critical section,
                // releases the lock, and then grabs the stack lock. That quick
                // thread would then take credit (blame) for the delay that this
                // slow thread caused. The alternative is to have more expensive
                // atomic operations (a CAS) on the critical path of unlock2.
                // Other M's are waiting for the lock.
                // Obtain the stack lock, and pop off an M.
        let mut next = Arc::new(Mutex::new(Some({ let __tmp_x = v; let __tmp_y = MUTEX_STACK_LOCKED as usize; __tmp_x | __tmp_y })));
        if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        break
    }
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone())); v = new_val; };
    }

        // No waiting Ms means nothing to do.
        //
        // If the stack lock is unavailable, its owner would make the same
        // wake decisions that we would, so there's nothing for us to do.
        //
        // Although: This thread may have a different call stack, which
        // would result in a different entry in the mutex contention profile
        // (upon completion of go.dev/issue/66999). That could lead to weird
        // results if a slow critical section ends but another thread
        // quickly takes the lock, finishes its own critical section,
        // releases the lock, and then grabs the stack lock. That quick
        // thread would then take credit (blame) for the delay that this
        // slow thread caused. The alternative is to have more expensive
        // atomic operations (a CAS) on the critical path of unlock2.
        // Other M's are waiting for the lock.
        // Obtain the stack lock, and pop off an M.
        // We own the mutexStackLocked flag. New Ms may push themselves onto the
        // stack concurrently, but we're now the only thread that can remove or
        // modify the Ms that are sleeping in the list.
    let mut committed: GoPtr<crate::runtime2::m> = GoPtr::nil();
    loop {
        let mut headM = Arc::new(Mutex::new(Some({ let __tmp_x = v; let __tmp_y = MUTEX_M_MASK as usize; __tmp_x & ! __tmp_y })));
        let mut flags = Arc::new(Mutex::new(Some({ let __tmp_x = v; let __tmp_y = ((MUTEX_M_MASK as usize) & ! (MUTEX_STACK_LOCKED as usize)) as usize; __tmp_x & __tmp_y })));

        let mut mp: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*mutex_wait_list_head(Arc::new(Mutex::new(Some(v)))).lock().unwrap().as_ref().unwrap()));
        let mut wakem: GoPtr<crate::runtime2::m> = committed.clone();
        if committed.is_nil() {
        if { let __tmp_x = { let __tmp_x = v; let __tmp_y = MUTEX_SPINNING as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || mutex_prefer_low_latency(l.clone()) {
        wakem = mp.clone();
    }
        if { let __v = (*antiStarve.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Wake the M at the bottom of the stack of waiters. (This is
                // O(N) with the number of waiters.)
        wakem = mp.clone();
        let mut prev: GoPtr<crate::runtime2::m> = mp.clone();
        loop {
        let mut next: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*(*{ let __ptr_value = wakem.with_mut(|__ptr_value| __ptr_value.m_wait_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()));
        if next.is_nil() {
        break
    }
        { let __tmp_0 = wakem.clone(); let __tmp_1 = next.clone(); prev = __tmp_0.clone(); wakem = __tmp_1.clone(); };
    }
        if { let __left_addr = wakem.addr(); let __right_addr = mp.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some((*(*(*{ let __ptr_value = wakem.with_mut(|__ptr_value| __ptr_value.m_wait_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*{ let __ptr_value = prev.with_mut(|__ptr_value| __ptr_value.m_wait_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = Some(new_val); };
        committed = wakem.clone();
    }
    }
    }

                // Wake the M at the bottom of the stack of waiters. (This is
                // O(N) with the number of waiters.)
        if { let __left_addr = wakem.addr(); let __right_addr = mp.addr(); let __eq = __left_addr == __right_addr; __eq } {
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*(*(*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.m_wait_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = MUTEX_M_MASK as usize; __tmp_x & ! __tmp_y }; *headM.lock().unwrap() = Some(new_val); };
    }

        let mut next = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*headM.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y })));
        if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if !wakem.is_nil() {
                // Claimed an M. Wake it.
        semawakeup(wakem.clone());
    }
                // Claimed an M. Wake it.
        break
    }

                // Claimed an M. Wake it.
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local({ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone())); v = new_val; };
    }
}

impl GoValueClone for mWaitList {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
