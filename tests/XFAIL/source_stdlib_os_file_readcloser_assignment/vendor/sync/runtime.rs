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

use crate::{runtime2::{notifyList}};

use std::sync::{Arc, Mutex as StdMutex};

/// Semacquire(RW)Mutex(R) is like Semacquire, but for profiling contended
/// Mutexes and RWMutexes.
/// If lifo is true, queue waiter at the head of wait queue.
/// skipframes is the number of frames to omit during tracing, counting from
/// runtime_SemacquireMutex's caller.
/// The different forms of this function just tell the runtime how to present
/// the reason for waiting in a backtrace, and is used to compute some metrics.
/// Otherwise they're functionally identical.
pub fn runtime__semacquire_r_w_mutex_r(s: Arc<StdMutex<Option<u32>>>, lifo: Arc<StdMutex<Option<bool>>>, skipframes: Arc<StdMutex<Option<i32>>>) {
    loop {
        let __acquired = {
            let mut __sem_guard = s.lock().unwrap();
            let __sem = __sem_guard.as_mut().unwrap();
            if *__sem > 0 {
                *__sem -= 1;
                true
            } else {
                false
            }
        };
        if __acquired {
            break;
        }
        std::thread::yield_now();
    }
}


pub fn runtime__semacquire_r_w_mutex(s: Arc<StdMutex<Option<u32>>>, lifo: Arc<StdMutex<Option<bool>>>, skipframes: Arc<StdMutex<Option<i32>>>) {
    loop {
        let __acquired = {
            let mut __sem_guard = s.lock().unwrap();
            let __sem = __sem_guard.as_mut().unwrap();
            if *__sem > 0 {
                *__sem -= 1;
                true
            } else {
                false
            }
        };
        if __acquired {
            break;
        }
        std::thread::yield_now();
    }
}


/// Semrelease atomically increments *s and notifies a waiting goroutine
/// if one is blocked in Semacquire.
/// It is intended as a simple wakeup primitive for use by the synchronization
/// library and should not be used directly.
/// If handoff is true, pass count directly to the first waiter.
/// skipframes is the number of frames to omit during tracing, counting from
/// runtime_Semrelease's caller.
pub fn runtime__semrelease(s: Arc<StdMutex<Option<u32>>>, handoff: Arc<StdMutex<Option<bool>>>, skipframes: Arc<StdMutex<Option<i32>>>) {
    {
        let mut __sem_guard = s.lock().unwrap();
        let __sem = __sem_guard.as_mut().unwrap();
        *__sem = __sem.saturating_add(1);
    }
    let _ = handoff;
    let _ = skipframes;
}


/// Ensure that sync and runtime agree on size of notifyList.
pub fn runtime_notify_list_check(size: Arc<StdMutex<Option<usize>>>) {
    let _ = size;
}


fn __go_init_0() {
    let mut n: Arc<StdMutex<Option<notifyList>>> = Arc::new(StdMutex::new(Some(Default::default())));
    runtime_notify_list_check(Arc::new(StdMutex::new(Some(std::mem::size_of::<crate::runtime2::notifyList>()))));
}

pub fn fatal(__arg0: Arc<StdMutex<Option<String>>>) {
    let __message = { let __arg_holder = __arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() };
    panic!("{}", __message);
}


pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_0();
}
