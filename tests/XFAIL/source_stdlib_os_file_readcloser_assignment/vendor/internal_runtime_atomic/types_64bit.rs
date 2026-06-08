use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
};

use crate::{atomic_arm64::{load_acq64, store_rel64}, types::{Uint64}};

use std::sync::{Arc, Mutex};

impl crate::types::Uint64 {
    /// LoadAcquire is a partially unsynchronized version
    /// of Load that relaxes ordering constraints. Other threads
    /// may observe operations that precede this operation to
    /// occur after it, but no operation that occurs after it
    /// on this thread can be observed to occur before it.
    ///
    /// WARNING: Use sparingly and with great care.
    ///
    ///go:nosplit
    pub fn load_acquire(&self) -> u64 {
        load_acq64(self.value.clone())
    }

    /// StoreRelease is a partially unsynchronized version
    /// of Store that relaxes ordering constraints. Other threads
    /// may observe operations that occur after this operation to
    /// precede it, but no operation that precedes it
    /// on this thread can be observed to occur after it.
    ///
    /// WARNING: Use sparingly and with great care.
    ///
    ///go:nosplit
    pub fn store_release(&self, value: Arc<Mutex<Option<u64>>>) {
        store_rel64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}