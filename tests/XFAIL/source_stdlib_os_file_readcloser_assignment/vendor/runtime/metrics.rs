use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_spinbit::{lock, unlock}, mfixalloc::{fixalloc}, mgc::{work}, mgcpacer::{gcController}, mheap::{mheap_}, mstats::{consistentHeapStats, cpuStats, heapStatsDelta, memstats, sysMemStat}, runtime2::{mutex}, sizeclasses::{class_to_size}, stubs::{systemstack}};

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const HEAP_STATS_DEP: u64 = 0;
pub(crate) const SYS_STATS_DEP: u64 = 1;
pub(crate) const CPU_STATS_DEP: u64 = 2;
pub(crate) const GC_STATS_DEP: u64 = 3;
pub(crate) const NUM_STATS_DEPS: u64 = 4;


pub(crate) const METRIC_KIND_BAD: i32 = 0;
pub(crate) const METRIC_KIND_UINT64: i32 = 1;
pub(crate) const METRIC_KIND_FLOAT64: i32 = 2;
pub(crate) const METRIC_KIND_FLOAT64_HISTOGRAM: i32 = 3;


#[derive(Clone)]
pub struct metricData {
    pub deps: Arc<Mutex<Option<statDepSet>>>,
    pub compute: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<statAggregate>>>, Arc<Mutex<Option<metricValue>>>) -> () + Send + Sync>>>>,
}

impl metricData {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.deps.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.compute.clone();
        Self {
            deps: __go_clone_0_0,
            compute: __go_clone_1_0,
        }
    }
}


impl Default for metricData {
    fn default() -> Self {
        Self { deps: Arc::new(Mutex::new(Some(statDepSet(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))))), compute: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for metricData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.deps.lock().unwrap().as_ref().unwrap()), "<func>")
    }
}

impl GoJsonDecode for metricData {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// statDep is a dependency on a group of statistics
/// that a metric might have.
#[derive(Debug, Clone, Default)]
pub struct statDep(pub Arc<Mutex<Option<u64>>>);

impl Display for statDep {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for statDep {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for statDep {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for statDep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for statDep {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<statDep> for u64 {
    fn eq(&self, other: &statDep) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<statDep> for u64 {
    fn partial_cmp(&self, other: &statDep) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for statDep {
    type Output = statDep;
    fn add(self, other: Self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for statDep {
    type Output = statDep;
    fn add(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<statDep> for u64 {
    type Output = statDep;
    fn add(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for statDep {
    type Output = statDep;
    fn sub(self, other: Self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for statDep {
    type Output = statDep;
    fn sub(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<statDep> for u64 {
    type Output = statDep;
    fn sub(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for statDep {
    type Output = statDep;
    fn mul(self, other: Self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for statDep {
    type Output = statDep;
    fn mul(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<statDep> for u64 {
    type Output = statDep;
    fn mul(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for statDep {
    type Output = statDep;
    fn div(self, other: Self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for statDep {
    type Output = statDep;
    fn div(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<statDep> for u64 {
    type Output = statDep;
    fn div(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for statDep {
    type Output = statDep;
    fn rem(self, other: Self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for statDep {
    type Output = statDep;
    fn rem(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<statDep> for u64 {
    type Output = statDep;
    fn rem(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for statDep {
    type Output = statDep;
    fn bitand(self, other: Self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for statDep {
    type Output = statDep;
    fn bitand(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<statDep> for u64 {
    type Output = statDep;
    fn bitand(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for statDep {
    type Output = statDep;
    fn bitor(self, other: Self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for statDep {
    type Output = statDep;
    fn bitor(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<statDep> for u64 {
    type Output = statDep;
    fn bitor(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for statDep {
    type Output = statDep;
    fn bitxor(self, other: Self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for statDep {
    type Output = statDep;
    fn bitxor(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<statDep> for u64 {
    type Output = statDep;
    fn bitxor(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for statDep {
    type Output = statDep;
    fn not(self) -> statDep {
        statDep(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for statDep {
    type Output = statDep;
    fn shl(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for statDep {
    type Output = statDep;
    fn shl(self, other: i32) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for statDep {
    type Output = statDep;
    fn shl(self, other: i8) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for statDep {
    type Output = statDep;
    fn shl(self, other: i16) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for statDep {
    type Output = statDep;
    fn shl(self, other: i64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for statDep {
    type Output = statDep;
    fn shl(self, other: u32) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for statDep {
    type Output = statDep;
    fn shl(self, other: u8) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for statDep {
    type Output = statDep;
    fn shl(self, other: u16) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for statDep {
    type Output = statDep;
    fn shl(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for statDep {
    type Output = statDep;
    fn shl(self, other: usize) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for statDep {
    type Output = statDep;
    fn shr(self, other: statDep) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for statDep {
    type Output = statDep;
    fn shr(self, other: i32) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for statDep {
    type Output = statDep;
    fn shr(self, other: i8) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for statDep {
    type Output = statDep;
    fn shr(self, other: i16) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for statDep {
    type Output = statDep;
    fn shr(self, other: i64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for statDep {
    type Output = statDep;
    fn shr(self, other: u32) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for statDep {
    type Output = statDep;
    fn shr(self, other: u8) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for statDep {
    type Output = statDep;
    fn shr(self, other: u16) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for statDep {
    type Output = statDep;
    fn shr(self, other: u64) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for statDep {
    type Output = statDep;
    fn shr(self, other: usize) -> statDep {
        statDep(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for statDep {}

impl Ord for statDep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// statDepSet represents a set of statDeps.
///
/// Under the hood, it's a bitmap.
#[derive(Debug, Clone)]
pub struct statDepSet(pub Arc<Mutex<Option<[u64; 1]>>>);

impl Default for statDepSet {
    fn default() -> Self {
        statDepSet(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl Display for statDepSet {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


/// heapStatsAggregate represents memory stats obtained from the
/// runtime. This set of stats is grouped together because they
/// depend on each other in some way to make sense of the runtime's
/// current heap memory use. They're also sharded across Ps, so it
/// makes sense to grab them all at once.
#[derive(Debug, Clone)]
pub struct heapStatsAggregate {
    pub heap_stats_delta: Arc<Mutex<Option<heapStatsDelta>>>,
    pub in_objects: Arc<Mutex<Option<u64>>>,
    pub num_objects: Arc<Mutex<Option<u64>>>,
    pub total_allocated: Arc<Mutex<Option<u64>>>,
    pub total_freed: Arc<Mutex<Option<u64>>>,
    pub total_allocs: Arc<Mutex<Option<u64>>>,
    pub total_frees: Arc<Mutex<Option<u64>>>,
}

impl heapStatsAggregate {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.heap_stats_delta.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.in_objects.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.num_objects.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.total_allocated.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.total_freed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.total_allocs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.total_frees.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            heap_stats_delta: __go_clone_0_0,
            in_objects: __go_clone_1_0,
            num_objects: __go_clone_2_0,
            total_allocated: __go_clone_3_0,
            total_freed: __go_clone_4_0,
            total_allocs: __go_clone_5_0,
            total_frees: __go_clone_6_0,
        }
    }
}


impl Default for heapStatsAggregate {
    fn default() -> Self {
        Self { heap_stats_delta: Arc::new(Mutex::new(Some(heapStatsDelta::default()))), in_objects: Arc::new(Mutex::new(Some(0))), num_objects: Arc::new(Mutex::new(Some(0))), total_allocated: Arc::new(Mutex::new(Some(0))), total_freed: Arc::new(Mutex::new(Some(0))), total_allocs: Arc::new(Mutex::new(Some(0))), total_frees: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for heapStatsAggregate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.heap_stats_delta.lock().unwrap().as_ref().unwrap()), (*self.in_objects.lock().unwrap().as_ref().unwrap()), (*self.num_objects.lock().unwrap().as_ref().unwrap()), (*self.total_allocated.lock().unwrap().as_ref().unwrap()), (*self.total_freed.lock().unwrap().as_ref().unwrap()), (*self.total_allocs.lock().unwrap().as_ref().unwrap()), (*self.total_frees.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for heapStatsAggregate {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// sysStatsAggregate represents system memory stats obtained
/// from the runtime. This set of stats is grouped together because
/// they're all relatively cheap to acquire and generally independent
/// of one another and other runtime memory stats. The fact that they
/// may be acquired at different times, especially with respect to
/// heapStatsAggregate, means there could be some skew, but because of
/// these stats are independent, there's no real consistency issue here.
#[derive(Debug, Clone)]
pub struct sysStatsAggregate {
    pub stacks_sys: Arc<Mutex<Option<u64>>>,
    pub m_span_sys: Arc<Mutex<Option<u64>>>,
    pub m_span_in_use: Arc<Mutex<Option<u64>>>,
    pub m_cache_sys: Arc<Mutex<Option<u64>>>,
    pub m_cache_in_use: Arc<Mutex<Option<u64>>>,
    pub buck_hash_sys: Arc<Mutex<Option<u64>>>,
    pub gc_misc_sys: Arc<Mutex<Option<u64>>>,
    pub other_sys: Arc<Mutex<Option<u64>>>,
    pub heap_goal: Arc<Mutex<Option<u64>>>,
    pub gc_cycles_done: Arc<Mutex<Option<u64>>>,
    pub gc_cycles_forced: Arc<Mutex<Option<u64>>>,
}

impl sysStatsAggregate {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.stacks_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.m_span_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.m_span_in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.m_cache_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.m_cache_in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.buck_hash_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.gc_misc_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.other_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.heap_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.gc_cycles_done.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.gc_cycles_forced.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            stacks_sys: __go_clone_0_0,
            m_span_sys: __go_clone_1_0,
            m_span_in_use: __go_clone_2_0,
            m_cache_sys: __go_clone_3_0,
            m_cache_in_use: __go_clone_4_0,
            buck_hash_sys: __go_clone_5_0,
            gc_misc_sys: __go_clone_6_0,
            other_sys: __go_clone_7_0,
            heap_goal: __go_clone_8_0,
            gc_cycles_done: __go_clone_9_0,
            gc_cycles_forced: __go_clone_10_0,
        }
    }
}


impl Default for sysStatsAggregate {
    fn default() -> Self {
        Self { stacks_sys: Arc::new(Mutex::new(Some(0))), m_span_sys: Arc::new(Mutex::new(Some(0))), m_span_in_use: Arc::new(Mutex::new(Some(0))), m_cache_sys: Arc::new(Mutex::new(Some(0))), m_cache_in_use: Arc::new(Mutex::new(Some(0))), buck_hash_sys: Arc::new(Mutex::new(Some(0))), gc_misc_sys: Arc::new(Mutex::new(Some(0))), other_sys: Arc::new(Mutex::new(Some(0))), heap_goal: Arc::new(Mutex::new(Some(0))), gc_cycles_done: Arc::new(Mutex::new(Some(0))), gc_cycles_forced: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for sysStatsAggregate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {}}}", (*self.stacks_sys.lock().unwrap().as_ref().unwrap()), (*self.m_span_sys.lock().unwrap().as_ref().unwrap()), (*self.m_span_in_use.lock().unwrap().as_ref().unwrap()), (*self.m_cache_sys.lock().unwrap().as_ref().unwrap()), (*self.m_cache_in_use.lock().unwrap().as_ref().unwrap()), (*self.buck_hash_sys.lock().unwrap().as_ref().unwrap()), (*self.gc_misc_sys.lock().unwrap().as_ref().unwrap()), (*self.other_sys.lock().unwrap().as_ref().unwrap()), (*self.heap_goal.lock().unwrap().as_ref().unwrap()), (*self.gc_cycles_done.lock().unwrap().as_ref().unwrap()), (*self.gc_cycles_forced.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for sysStatsAggregate {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// cpuStatsAggregate represents CPU stats obtained from the runtime
/// acquired together to avoid skew and inconsistencies.
#[derive(Debug, Clone)]
pub struct cpuStatsAggregate {
    pub cpu_stats: Arc<Mutex<Option<cpuStats>>>,
}

impl cpuStatsAggregate {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cpu_stats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            cpu_stats: __go_clone_0_0,
        }
    }
}


impl Default for cpuStatsAggregate {
    fn default() -> Self {
        Self { cpu_stats: Arc::new(Mutex::new(Some(cpuStats::default()))) }
    }
}

impl std::fmt::Display for cpuStatsAggregate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.cpu_stats.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for cpuStatsAggregate {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// gcStatsAggregate represents various GC stats obtained from the runtime
/// acquired together to avoid skew and inconsistencies.
#[derive(Debug, Clone)]
pub struct gcStatsAggregate {
    pub heap_scan: Arc<Mutex<Option<u64>>>,
    pub stack_scan: Arc<Mutex<Option<u64>>>,
    pub globals_scan: Arc<Mutex<Option<u64>>>,
    pub total_scan: Arc<Mutex<Option<u64>>>,
}

impl gcStatsAggregate {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.heap_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.stack_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.globals_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.total_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            heap_scan: __go_clone_0_0,
            stack_scan: __go_clone_1_0,
            globals_scan: __go_clone_2_0,
            total_scan: __go_clone_3_0,
        }
    }
}


impl Default for gcStatsAggregate {
    fn default() -> Self {
        Self { heap_scan: Arc::new(Mutex::new(Some(0))), stack_scan: Arc::new(Mutex::new(Some(0))), globals_scan: Arc::new(Mutex::new(Some(0))), total_scan: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for gcStatsAggregate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.heap_scan.lock().unwrap().as_ref().unwrap()), (*self.stack_scan.lock().unwrap().as_ref().unwrap()), (*self.globals_scan.lock().unwrap().as_ref().unwrap()), (*self.total_scan.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gcStatsAggregate {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// statAggregate is the main driver of the metrics implementation.
///
/// It contains multiple aggregates of runtime statistics, as well
/// as a set of these aggregates that it has populated. The aggregates
/// are populated lazily by its ensure method.
#[derive(Debug, Clone)]
pub struct statAggregate {
    pub ensured: Arc<Mutex<Option<statDepSet>>>,
    pub heap_stats: Arc<Mutex<Option<heapStatsAggregate>>>,
    pub sys_stats: Arc<Mutex<Option<sysStatsAggregate>>>,
    pub cpu_stats: Arc<Mutex<Option<cpuStatsAggregate>>>,
    pub gc_stats: Arc<Mutex<Option<gcStatsAggregate>>>,
}

impl statAggregate {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.ensured.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.heap_stats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.sys_stats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.cpu_stats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.gc_stats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            ensured: __go_clone_0_0,
            heap_stats: __go_clone_1_0,
            sys_stats: __go_clone_2_0,
            cpu_stats: __go_clone_3_0,
            gc_stats: __go_clone_4_0,
        }
    }
}


impl Default for statAggregate {
    fn default() -> Self {
        Self { ensured: Arc::new(Mutex::new(Some(statDepSet(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))))), heap_stats: Arc::new(Mutex::new(Some(heapStatsAggregate::default()))), sys_stats: Arc::new(Mutex::new(Some(sysStatsAggregate::default()))), cpu_stats: Arc::new(Mutex::new(Some(cpuStatsAggregate::default()))), gc_stats: Arc::new(Mutex::new(Some(gcStatsAggregate::default()))) }
    }
}

impl std::fmt::Display for statAggregate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.ensured.lock().unwrap().as_ref().unwrap()), (*self.heap_stats.lock().unwrap().as_ref().unwrap()), (*self.sys_stats.lock().unwrap().as_ref().unwrap()), (*self.cpu_stats.lock().unwrap().as_ref().unwrap()), (*self.gc_stats.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for statAggregate {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// metricKind is a runtime copy of runtime/metrics.ValueKind and
/// must be kept structurally identical to that type.
#[derive(Debug, Clone, Default)]
pub struct metricKind(pub Arc<Mutex<Option<i32>>>);

impl Display for metricKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for metricKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for metricKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for metricKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for metricKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<metricKind> for i32 {
    fn eq(&self, other: &metricKind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<metricKind> for i32 {
    fn partial_cmp(&self, other: &metricKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for metricKind {
    type Output = metricKind;
    fn add(self, other: Self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for metricKind {
    type Output = metricKind;
    fn add(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<metricKind> for i32 {
    type Output = metricKind;
    fn add(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for metricKind {
    type Output = metricKind;
    fn sub(self, other: Self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for metricKind {
    type Output = metricKind;
    fn sub(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<metricKind> for i32 {
    type Output = metricKind;
    fn sub(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for metricKind {
    type Output = metricKind;
    fn mul(self, other: Self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for metricKind {
    type Output = metricKind;
    fn mul(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<metricKind> for i32 {
    type Output = metricKind;
    fn mul(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for metricKind {
    type Output = metricKind;
    fn div(self, other: Self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for metricKind {
    type Output = metricKind;
    fn div(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<metricKind> for i32 {
    type Output = metricKind;
    fn div(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for metricKind {
    type Output = metricKind;
    fn neg(self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for metricKind {
    type Output = metricKind;
    fn rem(self, other: Self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for metricKind {
    type Output = metricKind;
    fn rem(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<metricKind> for i32 {
    type Output = metricKind;
    fn rem(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for metricKind {
    type Output = metricKind;
    fn bitand(self, other: Self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for metricKind {
    type Output = metricKind;
    fn bitand(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<metricKind> for i32 {
    type Output = metricKind;
    fn bitand(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for metricKind {
    type Output = metricKind;
    fn bitor(self, other: Self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for metricKind {
    type Output = metricKind;
    fn bitor(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<metricKind> for i32 {
    type Output = metricKind;
    fn bitor(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for metricKind {
    type Output = metricKind;
    fn bitxor(self, other: Self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for metricKind {
    type Output = metricKind;
    fn bitxor(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<metricKind> for i32 {
    type Output = metricKind;
    fn bitxor(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for metricKind {
    type Output = metricKind;
    fn not(self) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for metricKind {
    type Output = metricKind;
    fn shl(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for metricKind {
    type Output = metricKind;
    fn shl(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for metricKind {
    type Output = metricKind;
    fn shl(self, other: i8) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for metricKind {
    type Output = metricKind;
    fn shl(self, other: i16) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for metricKind {
    type Output = metricKind;
    fn shl(self, other: i64) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for metricKind {
    type Output = metricKind;
    fn shl(self, other: u32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for metricKind {
    type Output = metricKind;
    fn shl(self, other: u8) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for metricKind {
    type Output = metricKind;
    fn shl(self, other: u16) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for metricKind {
    type Output = metricKind;
    fn shl(self, other: u64) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for metricKind {
    type Output = metricKind;
    fn shl(self, other: usize) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for metricKind {
    type Output = metricKind;
    fn shr(self, other: metricKind) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for metricKind {
    type Output = metricKind;
    fn shr(self, other: i32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for metricKind {
    type Output = metricKind;
    fn shr(self, other: i8) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for metricKind {
    type Output = metricKind;
    fn shr(self, other: i16) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for metricKind {
    type Output = metricKind;
    fn shr(self, other: i64) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for metricKind {
    type Output = metricKind;
    fn shr(self, other: u32) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for metricKind {
    type Output = metricKind;
    fn shr(self, other: u8) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for metricKind {
    type Output = metricKind;
    fn shr(self, other: u16) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for metricKind {
    type Output = metricKind;
    fn shr(self, other: u64) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for metricKind {
    type Output = metricKind;
    fn shr(self, other: usize) -> metricKind {
        metricKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for metricKind {}

impl Ord for metricKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// metricValue is a runtime copy of runtime/metrics.Sample and
/// must be kept structurally identical to that type.
#[derive(Debug, Clone)]
pub struct metricValue {
    pub kind: Arc<Mutex<Option<metricKind>>>,
    pub scalar: Arc<Mutex<Option<u64>>>,
    pub pointer: Arc<Mutex<Option<usize>>>,
}

impl metricValue {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.scalar.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.pointer.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            kind: __go_clone_0_0,
            scalar: __go_clone_1_0,
            pointer: __go_clone_2_0,
        }
    }
}


impl Default for metricValue {
    fn default() -> Self {
        Self { kind: Arc::new(Mutex::new(Some(metricKind(Arc::new(Mutex::new(Some(0))))))), scalar: Arc::new(Mutex::new(Some(0))), pointer: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for metricValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.kind.lock().unwrap().as_ref().unwrap()), (*self.scalar.lock().unwrap().as_ref().unwrap()), (*self.pointer.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for metricValue {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// metricFloat64Histogram is a runtime copy of runtime/metrics.Float64Histogram
/// and must be kept structurally identical to that type.
#[derive(Debug, Clone, Default)]
pub struct metricFloat64Histogram {
    pub counts: Arc<Mutex<Option<Vec<u64>>>>,
    pub buckets: Arc<Mutex<Option<Vec<f64>>>>,
}

impl metricFloat64Histogram {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.counts.clone();
        let __go_clone_1_0 = self.buckets.clone();
        Self {
            counts: __go_clone_0_0,
            buckets: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for metricFloat64Histogram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice(&self.counts), format_slice(&self.buckets))
    }
}

impl GoJsonDecode for metricFloat64Histogram {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static metricsSema: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static metricsInit: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static metrics: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<String, Arc<Mutex<Option<metricData>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sizeClassBuckets: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<f64>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static timeHistBuckets: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<f64>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static agg: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<statAggregate>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *metricsSema.lock().unwrap() = Some(0);
    *metricsInit.lock().unwrap() = Some(false);
    *metrics.lock().unwrap() = Some(BTreeMap::new());
    *sizeClassBuckets.lock().unwrap() = Some(vec![]);
    *timeHistBuckets.lock().unwrap() = Some(vec![]);
    *agg.lock().unwrap() = Some(Default::default());
    *metricsSema.lock().unwrap() = Some(1 as u32);
}


pub(crate) fn __go_zero_globals() {
    *metricsSema.lock().unwrap() = Some(0);
    *metricsInit.lock().unwrap() = Some(false);
    *metrics.lock().unwrap() = Some(BTreeMap::new());
    *sizeClassBuckets.lock().unwrap() = Some(vec![]);
    *timeHistBuckets.lock().unwrap() = Some(vec![]);
    *agg.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_25() {
    *metricsSema.lock().unwrap() = Some(1 as u32);
}


impl statDepSet {
    /// difference returns set difference of s from b as a new set.
    pub fn difference(&self, b: Arc<Mutex<Option<statDepSet>>>) -> Arc<Mutex<Option<statDepSet>>> {
        let mut c: Arc<Mutex<Option<statDepSet>>> = Arc::new(Mutex::new(Some(statDepSet(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))))))));
        for i in 0..(({ let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*{ let __named_array = (*c.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(i) as usize].clone() }; let __tmp_y = { let __seq_holder = { let __named_array = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(i) as usize].clone() }; __tmp_x & ! __tmp_y };
    }
        return { let __owned = c.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// union returns the union of the two sets as a new set.
    pub fn union(&self, b: Arc<Mutex<Option<statDepSet>>>) -> Arc<Mutex<Option<statDepSet>>> {
        let mut c: Arc<Mutex<Option<statDepSet>>> = Arc::new(Mutex::new(Some(statDepSet(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))))))));
        for i in 0..(({ let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*{ let __named_array = (*c.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(i) as usize].clone() }; let __tmp_y = { let __seq_holder = { let __named_array = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(i) as usize].clone() }; __tmp_x | __tmp_y };
    }
        return { let __owned = c.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// empty returns true if there are no dependencies in the set.
    pub fn empty(&self) -> bool {
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter().copied() {
        if { let __tmp_x = c; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return false;
    }
    } }
        true
    }

    /// has returns true if the set contains a given statDep.
    pub fn has(&self, d: Arc<Mutex<Option<statDep>>>) -> bool {
        return { let __tmp_x = { let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[((*{ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) / 64) as usize].clone() }; let __tmp_y = ({ let __tmp_x = (1 as u64); let __tmp_y = (((*{ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) % 64)); __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y };
    }
}

impl heapStatsAggregate {
    /// compute populates the heapStatsAggregate with values from the runtime.
    pub fn compute(&mut self) {
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).read(self.heap_stats_delta.clone());
                // Calculate derived stats.
        { let new_val = { let __selector_holder = (*self.heap_stats_delta.lock().unwrap().as_ref().unwrap()).large_alloc_count.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.total_allocs.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*self.heap_stats_delta.lock().unwrap().as_ref().unwrap()).large_free_count.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.total_frees.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*self.heap_stats_delta.lock().unwrap().as_ref().unwrap()).large_alloc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.total_allocated.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*self.heap_stats_delta.lock().unwrap().as_ref().unwrap()).large_free.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.total_freed.lock().unwrap() = Some(new_val); };
        for i in 0..(({ let __range_holder = (*self.heap_stats_delta.lock().unwrap().as_ref().unwrap()).small_alloc_count.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut na = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.heap_stats_delta.lock().unwrap().as_ref().unwrap()).small_alloc_count.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() })));
        let mut nf = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.heap_stats_delta.lock().unwrap().as_ref().unwrap()).small_free_count.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() })));
        { let __target = self.total_allocs.clone(); let __rhs = (*na.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.total_frees.clone(); let __rhs = (*nf.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.total_allocated.clone(); let __rhs = { let __tmp_x = { let __v = (*na.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = class_to_size.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.total_freed.clone(); let __rhs = { let __tmp_x = { let __v = (*nf.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = class_to_size.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let new_val = { let __tmp_x = (*self.total_allocated.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.total_freed.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; *self.in_objects.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = (*self.total_allocs.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.total_frees.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; *self.num_objects.lock().unwrap() = Some(new_val); };
    }

    pub fn merge(&mut self, b: GoPtr<crate::mstats::heapStatsDelta>) {
        // Forward to embedded type's method
        let embedded = self.heap_stats_delta.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.merge(b)
    }
}

impl sysStatsAggregate {
    /// compute populates the sysStatsAggregate with values from the runtime.
    pub fn compute(&mut self) {
        { let new_val = (*(*memstats.lock().unwrap().as_ref().unwrap()).stacks_sys.lock().unwrap().as_ref().unwrap()).load(); *self.stacks_sys.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*memstats.lock().unwrap().as_ref().unwrap()).buckhash_sys.lock().unwrap().as_ref().unwrap()).load(); *self.buck_hash_sys.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.lock().unwrap().as_ref().unwrap()).load(); *self.gc_misc_sys.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*memstats.lock().unwrap().as_ref().unwrap()).other_sys.lock().unwrap().as_ref().unwrap()).load(); *self.other_sys.lock().unwrap() = Some(new_val); };
        { let new_val = (*gcController.lock().unwrap().as_ref().unwrap()).heap_goal(); *self.heap_goal.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*memstats.lock().unwrap().as_ref().unwrap()).numgc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.gc_cycles_done.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*memstats.lock().unwrap().as_ref().unwrap()).numforcedgc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.gc_cycles_forced.lock().unwrap() = __moved_val; };
        let mut a_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = (*(*memstats.lock().unwrap().as_ref().unwrap()).mspan_sys.lock().unwrap().as_ref().unwrap()).load(); *a_closure_clone.m_span_sys.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*mheap_.lock().unwrap().as_ref().unwrap()).spanalloc.lock().unwrap().as_ref().unwrap()).inuse.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *a_closure_clone.m_span_in_use.lock().unwrap() = __moved_val; };
        { let new_val = (*(*memstats.lock().unwrap().as_ref().unwrap()).mcache_sys.lock().unwrap().as_ref().unwrap()).load(); *a_closure_clone.m_cache_sys.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*mheap_.lock().unwrap().as_ref().unwrap()).cachealloc.lock().unwrap().as_ref().unwrap()).inuse.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *a_closure_clone.m_cache_in_use.lock().unwrap() = __moved_val; };
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
}

impl cpuStatsAggregate {
    /// compute populates the cpuStatsAggregate with values from the runtime.
    pub fn compute(&mut self) {
        { let new_val = { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).cpu_stats.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.cpu_stats.lock().unwrap() = Some(new_val); };
    }

    pub fn accumulate(&mut self, now: Arc<Mutex<Option<i64>>>, gcMarkPhase: Arc<Mutex<Option<bool>>>) {
        // Forward to embedded type's method
        let embedded = self.cpu_stats.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.accumulate(now, gcMarkPhase)
    }

    pub fn accumulate_g_c_pause_time(&mut self, dt: Arc<Mutex<Option<i64>>>, maxProcs: Arc<Mutex<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.cpu_stats.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.accumulate_g_c_pause_time(dt, maxProcs)
    }
}

impl gcStatsAggregate {
    /// compute populates the gcStatsAggregate with values from the runtime.
    pub fn compute(&mut self) {
        { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_scan.lock().unwrap().as_mut().unwrap()).load(); *self.heap_scan.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).last_stack_scan.lock().unwrap().as_mut().unwrap()).load(); *self.stack_scan.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).globals_scan.lock().unwrap().as_mut().unwrap()).load(); *self.globals_scan.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __tmp_x = (*self.heap_scan.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.stack_scan.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*self.globals_scan.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *self.total_scan.lock().unwrap() = Some(new_val); };
    }
}

impl statAggregate {
    /// ensure populates statistics aggregates determined by deps if they
    /// haven't yet been populated.
    pub fn ensure(&mut self, deps: Arc<Mutex<Option<statDepSet>>>) {
        let mut missing = { let __recv = deps.clone(); let __recv_ptr: *const statDepSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const statDepSet }; let __result = unsafe { &*__recv_ptr }.difference(Arc::new(Mutex::new(Some({ let __selector_holder = self.ensured.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };
        if (*missing.lock().unwrap().as_ref().unwrap()).empty() {
        return;
    }
        let mut i = Arc::new(Mutex::new(Some(statDep(Arc::new(Mutex::new(Some(0 as u64)))))));
    while { let __tmp_x = (*i.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = statDep(Arc::new(Mutex::new(Some(NUM_STATS_DEPS as u64)))); __tmp_x < __tmp_y } {
        if !(*missing.lock().unwrap().as_ref().unwrap()).has(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u64); }; continue
    }
        { let _switch_val = (*i.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (statDep(Arc::new(Mutex::new(Some(HEAP_STATS_DEP as u64))))) {
            (*self.heap_stats.lock().unwrap().as_mut().unwrap()).compute();
        } else if _switch_val == (statDep(Arc::new(Mutex::new(Some(SYS_STATS_DEP as u64))))) {
            (*self.sys_stats.lock().unwrap().as_mut().unwrap()).compute();
        } else if _switch_val == (statDep(Arc::new(Mutex::new(Some(CPU_STATS_DEP as u64))))) {
            (*self.cpu_stats.lock().unwrap().as_mut().unwrap()).compute();
        } else if _switch_val == (statDep(Arc::new(Mutex::new(Some(GC_STATS_DEP as u64))))) {
            (*self.gc_stats.lock().unwrap().as_mut().unwrap()).compute();
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u64); }
    }
        { let new_val = (*self.ensured.lock().unwrap().as_ref().unwrap()).union(Arc::new(Mutex::new(Some({ let __arg_holder = missing.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.ensured.lock().unwrap() = __moved_val; };
    }
}

impl metricValue {
    /// float64HistOrInit tries to pull out an existing float64Histogram
    /// from the value, but if none exists, then it allocates one with
    /// the given buckets.
    pub fn float64_hist_or_init(&mut self, buckets: Arc<Mutex<Option<Vec<f64>>>>) -> GoPtr<metricFloat64Histogram> {
        let mut hist: GoPtr<metricFloat64Histogram> = GoPtr::nil();
        if { let __tmp_x = { let __selector_holder = self.kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = metricKind(Arc::new(Mutex::new(Some(METRIC_KIND_FLOAT64_HISTOGRAM as i32)))); __tmp_x == __tmp_y } && { let __nil_target = self.pointer.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        hist = GoPtr::raw({ let __ptr = self.pointer.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    } else {
        { let new_val = metricKind(Arc::new(Mutex::new(Some(METRIC_KIND_FLOAT64_HISTOGRAM as i32)))); *self.kind.lock().unwrap() = Some(new_val); };
        hist = GoPtr::local(Arc::new(Mutex::new(Some(metricFloat64Histogram::default()))));
        { let new_val = Arc::new(Mutex::new(Some(hist.addr()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.pointer.lock().unwrap() = __moved_val; };
    }
        { let new_val = buckets.clone(); hist.with_mut(|__ptr_value| { __ptr_value.buckets = new_val; }); };
        if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = hist.with_mut(|__ptr_value| __ptr_value.counts.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = hist.with_mut(|__ptr_value| __ptr_value.buckets.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(vec![0; ({ let __tmp_x = ((*buckets.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize]))); hist.with_mut(|__ptr_value| { __ptr_value.counts = new_val; }); };
    }
        hist.clone()
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for metricData {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for heapStatsAggregate {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for sysStatsAggregate {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for cpuStatsAggregate {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gcStatsAggregate {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for statAggregate {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for metricValue {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for metricFloat64Histogram {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
