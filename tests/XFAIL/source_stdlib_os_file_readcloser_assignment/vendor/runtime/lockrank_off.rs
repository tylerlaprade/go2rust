use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_spinbit::{lock2, unlock2}, lockrank::{lockRank}, mgc::{AnonymousStruct12}, runtime1::{acquirem, releasem}, runtime2::{g, m, mutex}, stubs::{getg}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const STATIC_LOCK_RANKING: bool = false;


/// // lockRankStruct is embedded in mutex, but is empty when staticklockranking is
/// disabled (the default)
#[derive(Debug, Clone, Default)]
pub struct lockRankStruct {
}

impl lockRankStruct {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for lockRankStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for lockRankStruct {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub fn lock_init(l: GoPtr<crate::runtime2::mutex>, rank: Arc<Mutex<Option<lockRank>>>) {
}

pub fn get_lock_rank(l: GoPtr<crate::runtime2::mutex>) -> Arc<Mutex<Option<crate::lockrank::lockRank>>> {
    Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(0 as i32)))))))
}

pub fn lock_with_rank(l: GoPtr<crate::runtime2::mutex>, rank: Arc<Mutex<Option<lockRank>>>) {
    lock2(l.clone());
}

/// This function may be called in nosplit context and thus must be nosplit.
///
///go:nosplit
pub fn acquire_lock_rank_and_m(rank: Arc<Mutex<Option<lockRank>>>) {
    acquirem();
}

pub fn unlock_with_rank(l: GoPtr<crate::runtime2::mutex>) {
    unlock2(l.clone());
}

/// This function may be called in nosplit context and thus must be nosplit.
///
///go:nosplit
pub fn release_lock_rank_and_m(rank: Arc<Mutex<Option<lockRank>>>) {
    releasem(GoPtr::local((*getg().lock().unwrap().as_ref().unwrap()).m.clone()));
}

/// This function may be called in nosplit context and thus must be nosplit.
///
///go:nosplit
pub fn lock_with_rank_may_acquire(l: Arc<Mutex<Option<mutex>>>, rank: Arc<Mutex<Option<lockRank>>>) {
}

///go:nosplit
pub fn assert_lock_held(l: GoPtr<crate::runtime2::mutex>) {
}

///go:nosplit
pub fn world_stopped() {
}

///go:nosplit
pub fn world_started() {
}

///go:nosplit
pub fn assert_world_stopped() {
}

///go:nosplit
pub fn assert_world_stopped_or_lock_held(l: Arc<Mutex<Option<mutex>>>) {
}

impl GoValueClone for lockRankStruct {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
