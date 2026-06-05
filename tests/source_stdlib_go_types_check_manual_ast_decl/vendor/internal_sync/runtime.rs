use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::hashtriemap::*;
use crate::mutex::*;

use std::sync::{Arc, Mutex as StdMutex};

/// SemacquireMutex is like Semacquire, but for profiling contended
/// Mutexes and RWMutexes.
/// If lifo is true, queue waiter at the head of wait queue.
/// skipframes is the number of frames to omit during tracing, counting from
/// runtime_SemacquireMutex's caller.
/// The different forms of this function just tell the runtime how to present
/// the reason for waiting in a backtrace, and is used to compute some metrics.
/// Otherwise they're functionally identical.
///
///go:linkname runtime_SemacquireMutex
pub fn runtime__semacquire_mutex(s: Arc<StdMutex<Option<u32>>>, lifo: Arc<StdMutex<Option<bool>>>, skipframes: Arc<StdMutex<Option<i32>>>) {
    unimplemented!("Go function declaration has no body");
}


/// Semrelease atomically increments *s and notifies a waiting goroutine
/// if one is blocked in Semacquire.
/// It is intended as a simple wakeup primitive for use by the synchronization
/// library and should not be used directly.
/// If handoff is true, pass count directly to the first waiter.
/// skipframes is the number of frames to omit during tracing, counting from
/// runtime_Semrelease's caller.
///
///go:linkname runtime_Semrelease
pub fn runtime__semrelease(s: Arc<StdMutex<Option<u32>>>, handoff: Arc<StdMutex<Option<bool>>>, skipframes: Arc<StdMutex<Option<i32>>>) {
    unimplemented!("Go function declaration has no body");
}


/// Active spinning runtime support.
/// runtime_canSpin reports whether spinning makes sense at the moment.
///
///go:linkname runtime_canSpin
pub fn runtime_can_spin(i: Arc<StdMutex<Option<i32>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


/// runtime_doSpin does active spinning.
///
///go:linkname runtime_doSpin
pub fn runtime_do_spin() {
    unimplemented!("Go function declaration has no body");
}


///go:linkname runtime_nanotime
pub fn runtime_nanotime() -> i64 {
    unimplemented!("Go function declaration has no body");
}


///go:linkname throw
pub fn throw(__arg0: Arc<StdMutex<Option<String>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:linkname fatal
pub fn fatal(__arg0: Arc<StdMutex<Option<String>>>) {
    unimplemented!("Go function declaration has no body");
}
