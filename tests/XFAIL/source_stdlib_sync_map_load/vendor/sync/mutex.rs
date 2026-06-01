use go2rust_stdlib_stubs::*;

use crate::{GoAtomicPointer, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_slice, format_slice_values, format_slice_wrapped};

use crate::cond::*;
use crate::hashtriemap::*;
use crate::once::*;
use crate::oncefunc::*;
use crate::pool::*;
use crate::poolqueue::*;
use crate::runtime::*;
use crate::runtime2::*;
use crate::rwmutex::*;
use crate::waitgroup::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

/// A Mutex is a mutual exclusion lock.
/// The zero value for a Mutex is an unlocked mutex.
///
/// A Mutex must not be copied after first use.
///
/// In the terminology of [the Go memory model],
/// the n'th call to [Mutex.Unlock] “synchronizes before” the m'th call to [Mutex.Lock]
/// for any n < m.
/// A successful call to [Mutex.TryLock] is equivalent to a call to Lock.
/// A failed call to TryLock does not establish any “synchronizes before”
/// relation at all.
///
/// [the Go memory model]: https://go.dev/ref/mem
#[derive(Clone)]
pub struct Mutex {
    pub __blank_0_0: Arc<StdMutex<Option<noCopy>>>,
    pub mu: Arc<StdMutex<Option<internal_sync::Mutex>>>,
}

impl Mutex {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, mu: { let __guard = self.mu.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) } }
    }
}


impl Default for Mutex {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(StdMutex::new(Some(noCopy::default()))), mu: Arc::new(StdMutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for Mutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.mu.lock().unwrap().as_ref().unwrap()))
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
    /// If the lock is already in use, the calling goroutine
    /// blocks until the mutex is available.
    pub fn lock(&self) {
        (*self.mu.lock().unwrap().as_ref().unwrap()).lock();
    }

    /// TryLock tries to lock m and reports whether it succeeded.
    ///
    /// Note that while correct uses of TryLock do exist, they are rare,
    /// and use of TryLock is often a sign of a deeper problem
    /// in a particular use of mutexes.
    pub fn try_lock(&self) -> bool {
        (*self.mu.lock().unwrap().as_ref().unwrap()).try_lock()
    }

    /// Unlock unlocks m.
    /// It is a run-time error if m is not locked on entry to Unlock.
    ///
    /// A locked [Mutex] is not associated with a particular goroutine.
    /// It is allowed for one goroutine to lock a Mutex and then
    /// arrange for another goroutine to unlock it.
    pub fn unlock(&self) {
        (*self.mu.lock().unwrap().as_ref().unwrap()).unlock();
    }
}