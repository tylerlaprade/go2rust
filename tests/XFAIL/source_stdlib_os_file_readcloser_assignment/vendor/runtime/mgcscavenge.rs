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
    float::{is_inf, is_na_n},
    lock_spinbit::{lock, unlock},
    lockrank::{LOCK_RANK_SCAVENGE},
    lockrank_off::{assert_lock_held, assert_world_stopped_or_lock_held, lock_init},
    malloc::{PAGE_SIZE, physHugePageSize, physPageSize},
    mem::{sys_unused},
    mgcpacer::{gcController},
    mheap::{AnonymousStruct15, MAX_PHYS_PAGE_SIZE, mheap_},
    mpagealloc::{LOG_PALLOC_CHUNK_PAGES, PALLOC_CHUNK_BYTES, PALLOC_CHUNK_PAGES, chunkIdx, chunk_base, chunk_index, chunk_page_index, pageAlloc, pallocSum},
    mpallocbits::{pageBits, pallocBits, pallocData},
    mranges::{atomicOffAddr, minOffAddr, offAddr},
    mstats::{consistentHeapStats, heapStatsDelta, memstats, sysMemStat},
    panic::{throw},
    print::{printlock, printunlock},
    proc::{gList, goparkunlock, injectglist},
    runtime2::{WAIT_REASON_G_C_SCAVENGE_WAIT, WAIT_REASON_SLEEP, g, gomaxprocs, mutex},
    stubs::{align_down, align_up, getg, systemstack},
    time::{timer},
    time_nofake::{faketime, nanotime},
    traceruntime::{TRACE_BLOCK_SLEEP, TRACE_BLOCK_SYSTEM_GOROUTINE},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const SCAVENGE_PERCENT: i32 = 1;
pub(crate) const RETAIN_EXTRA_PERCENT: i32 = 10;
pub(crate) const REDUCE_EXTRA_PERCENT: i32 = 5;
pub(crate) const MAX_PAGES_PER_PHYS_PAGE: i32 = MAX_PHYS_PAGE_SIZE / PAGE_SIZE;
pub(crate) const SCAVENGE_COST_RATIO: f64 = 0.7 * 1.0;
pub(crate) const SCAV_CHUNK_HI_OCC_FRAC: f64 = 0.96875;
pub(crate) const SCAV_CHUNK_HI_OCC_PAGES: u16 = ((SCAV_CHUNK_HI_OCC_FRAC * 512.0) as u16);


pub(crate) const STARTING_SCAV_SLEEP_RATIO: f64 = 0.001;
pub(crate) const MIN_SCAV_WORK_TIME: f64 = 1e6;


pub(crate) const SCAV_CHUNK_HAS_FREE: u8 = 1 << 0;
pub(crate) const SCAV_CHUNK_MAX_FLAGS: i32 = 6;
pub(crate) const SCAV_CHUNK_FLAGS_MASK: i32 = (((1 as i32) << (SCAV_CHUNK_MAX_FLAGS as i32)) - (1 as i32));
pub(crate) const LOG_SCAV_CHUNK_IN_USE_MAX: i32 = LOG_PALLOC_CHUNK_PAGES + 1;
pub(crate) const SCAV_CHUNK_IN_USE_MASK: i32 = (((1 as i32) << (LOG_SCAV_CHUNK_IN_USE_MAX as i32)) - (1 as i32));


#[derive(Clone)]
pub struct scavengerState {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub g: Arc<Mutex<Option<g>>>,
    pub timer: Arc<Mutex<Option<timer>>>,
    pub sysmon_wake: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub parked: Arc<Mutex<Option<bool>>>,
    pub print_controller_reset: Arc<Mutex<Option<bool>>>,
    pub target_c_p_u_fraction: Arc<Mutex<Option<f64>>>,
    pub sleep_ratio: Arc<Mutex<Option<f64>>>,
    pub sleep_controller: Arc<Mutex<Option<piController>>>,
    pub controller_cooldown: Arc<Mutex<Option<i64>>>,
    pub sleep_stub: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>>>>,
    pub scavenge: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> (usize, i64) + Send + Sync>>>>,
    pub should_stop: Arc<Mutex<Option<Box<dyn FnMut() -> bool + Send + Sync>>>>,
    pub gomaxprocs: Arc<Mutex<Option<Box<dyn FnMut() -> i32 + Send + Sync>>>>,
}

impl scavengerState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.g.clone();
        let __go_clone_2_0 = self.timer.clone();
        let __go_clone_3_0 = { let __guard = self.sysmon_wake.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.parked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.print_controller_reset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.target_c_p_u_fraction.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.sleep_ratio.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.sleep_controller.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.controller_cooldown.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = self.sleep_stub.clone();
        let __go_clone_11_0 = self.scavenge.clone();
        let __go_clone_12_0 = self.should_stop.clone();
        let __go_clone_13_0 = self.gomaxprocs.clone();
        Self {
            lock: __go_clone_0_0,
            g: __go_clone_1_0,
            timer: __go_clone_2_0,
            sysmon_wake: __go_clone_3_0,
            parked: __go_clone_4_0,
            print_controller_reset: __go_clone_5_0,
            target_c_p_u_fraction: __go_clone_6_0,
            sleep_ratio: __go_clone_7_0,
            sleep_controller: __go_clone_8_0,
            controller_cooldown: __go_clone_9_0,
            sleep_stub: __go_clone_10_0,
            scavenge: __go_clone_11_0,
            should_stop: __go_clone_12_0,
            gomaxprocs: __go_clone_13_0,
        }
    }
}


impl Default for scavengerState {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(piController::default())));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(None));
        let __go_default_11_0 = Arc::new(Mutex::new(None));
        let __go_default_12_0 = Arc::new(Mutex::new(None));
        let __go_default_13_0 = Arc::new(Mutex::new(None));
        Self {
            lock: __go_default_0_0,
            g: __go_default_1_0,
            timer: __go_default_2_0,
            sysmon_wake: __go_default_3_0,
            parked: __go_default_4_0,
            print_controller_reset: __go_default_5_0,
            target_c_p_u_fraction: __go_default_6_0,
            sleep_ratio: __go_default_7_0,
            sleep_controller: __go_default_8_0,
            controller_cooldown: __go_default_9_0,
            sleep_stub: __go_default_10_0,
            scavenge: __go_default_11_0,
            should_stop: __go_default_12_0,
            gomaxprocs: __go_default_13_0,
        }
    }
}

impl std::fmt::Display for scavengerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.g.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", (*self.sysmon_wake.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.parked.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.print_controller_reset.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.target_c_p_u_fraction.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.sleep_ratio.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.sleep_controller.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.controller_cooldown.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", "<func>");
        let __go_fmt_11 = format!("{}", "<func>");
        let __go_fmt_12 = format!("{}", "<func>");
        let __go_fmt_13 = format!("{}", "<func>");
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13
        )
    }
}

impl GoJsonDecode for scavengerState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// scavengeIndex is a structure for efficiently managing which pageAlloc chunks have
/// memory available to scavenge.
#[derive(Clone)]
pub struct scavengeIndex {
    pub chunks: Arc<Mutex<Option<Vec<atomicScavChunkData>>>>,
    pub min: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub max: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub min_heap_idx: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub search_addr_bg: Arc<Mutex<Option<atomicOffAddr>>>,
    pub search_addr_force: Arc<Mutex<Option<atomicOffAddr>>>,
    pub free_h_w_m: Arc<Mutex<Option<offAddr>>>,
    pub gen: Arc<Mutex<Option<u32>>>,
    pub test: Arc<Mutex<Option<bool>>>,
}

impl scavengeIndex {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.chunks.clone();
        let __go_clone_1_0 = { let __guard = self.min.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_1 = { let __guard = self.max.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.min_heap_idx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.search_addr_bg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.search_addr_force.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.free_h_w_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.test.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            chunks: __go_clone_0_0,
            min: __go_clone_1_0,
            max: __go_clone_1_1,
            min_heap_idx: __go_clone_2_0,
            search_addr_bg: __go_clone_3_0,
            search_addr_force: __go_clone_4_0,
            free_h_w_m: __go_clone_5_0,
            gen: __go_clone_6_0,
            test: __go_clone_7_0,
        }
    }
}


impl Default for scavengeIndex {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_1 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(atomicOffAddr::default())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(atomicOffAddr::default())));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(offAddr::default())));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            chunks: __go_default_0_0,
            min: __go_default_1_0,
            max: __go_default_1_1,
            min_heap_idx: __go_default_2_0,
            search_addr_bg: __go_default_3_0,
            search_addr_force: __go_default_4_0,
            free_h_w_m: __go_default_5_0,
            gen: __go_default_6_0,
            test: __go_default_7_0,
        }
    }
}

impl std::fmt::Display for scavengeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.chunks));
        let __go_fmt_1 = format!("{}", (*self.min.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.max.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.min_heap_idx.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.search_addr_bg.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.search_addr_force.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.free_h_w_m.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.gen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.test.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8)
    }
}

impl GoJsonDecode for scavengeIndex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// atomicScavChunkData is an atomic wrapper around a scavChunkData
/// that stores it in its packed form.
#[derive(Clone)]
pub struct atomicScavChunkData {
    pub value: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
}

impl atomicScavChunkData {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for atomicScavChunkData {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for atomicScavChunkData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for atomicScavChunkData {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// scavChunkData tracks information about a palloc chunk for
/// scavenging. It packs well into 64 bits.
///
/// The zero value always represents a valid newly-grown chunk.
#[derive(Debug, Clone)]
pub struct scavChunkData {
    pub in_use: Arc<Mutex<Option<u16>>>,
    pub last_in_use: Arc<Mutex<Option<u16>>>,
    pub gen: Arc<Mutex<Option<u32>>>,
    pub scav_chunk_flags: Arc<Mutex<Option<scavChunkFlags>>>,
}

impl scavChunkData {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.last_in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.scav_chunk_flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            in_use: __go_clone_0_0,
            last_in_use: __go_clone_1_0,
            gen: __go_clone_2_0,
            scav_chunk_flags: __go_clone_3_0,
        }
    }
}


impl Default for scavChunkData {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(scavChunkFlags(Arc::new(Mutex::new(Some(0)))))));
        Self {
            in_use: __go_default_0_0,
            last_in_use: __go_default_1_0,
            gen: __go_default_2_0,
            scav_chunk_flags: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for scavChunkData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.in_use.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.last_in_use.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.gen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.scav_chunk_flags.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for scavChunkData {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// scavChunkFlags is a set of bit-flags for the scavenger for each palloc chunk.
#[derive(Debug, Clone, Default)]
pub struct scavChunkFlags(pub Arc<Mutex<Option<u8>>>);

impl Display for scavChunkFlags {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for scavChunkFlags {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for scavChunkFlags {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for scavChunkFlags {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for scavChunkFlags {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<scavChunkFlags> for u8 {
    fn eq(&self, other: &scavChunkFlags) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<scavChunkFlags> for u8 {
    fn partial_cmp(&self, other: &scavChunkFlags) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for scavChunkFlags {
    type Output = scavChunkFlags;
    fn add(self, other: Self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn add(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<scavChunkFlags> for u8 {
    type Output = scavChunkFlags;
    fn add(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for scavChunkFlags {
    type Output = scavChunkFlags;
    fn sub(self, other: Self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn sub(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<scavChunkFlags> for u8 {
    type Output = scavChunkFlags;
    fn sub(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for scavChunkFlags {
    type Output = scavChunkFlags;
    fn mul(self, other: Self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn mul(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<scavChunkFlags> for u8 {
    type Output = scavChunkFlags;
    fn mul(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for scavChunkFlags {
    type Output = scavChunkFlags;
    fn div(self, other: Self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn div(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<scavChunkFlags> for u8 {
    type Output = scavChunkFlags;
    fn div(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for scavChunkFlags {
    type Output = scavChunkFlags;
    fn rem(self, other: Self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn rem(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<scavChunkFlags> for u8 {
    type Output = scavChunkFlags;
    fn rem(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for scavChunkFlags {
    type Output = scavChunkFlags;
    fn bitand(self, other: Self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn bitand(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<scavChunkFlags> for u8 {
    type Output = scavChunkFlags;
    fn bitand(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for scavChunkFlags {
    type Output = scavChunkFlags;
    fn bitor(self, other: Self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn bitor(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<scavChunkFlags> for u8 {
    type Output = scavChunkFlags;
    fn bitor(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for scavChunkFlags {
    type Output = scavChunkFlags;
    fn bitxor(self, other: Self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn bitxor(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<scavChunkFlags> for u8 {
    type Output = scavChunkFlags;
    fn bitxor(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for scavChunkFlags {
    type Output = scavChunkFlags;
    fn not(self) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: i32) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: i8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: i16) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: i64) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: u32) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: u16) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: u64) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shl(self, other: usize) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: scavChunkFlags) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: i32) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: i8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: i16) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: i64) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: u32) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: u8) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: u16) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: u64) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for scavChunkFlags {
    type Output = scavChunkFlags;
    fn shr(self, other: usize) -> scavChunkFlags {
        scavChunkFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for scavChunkFlags {}

impl Ord for scavChunkFlags {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct piController {
    pub kp: Arc<Mutex<Option<f64>>>,
    pub ti: Arc<Mutex<Option<f64>>>,
    pub tt: Arc<Mutex<Option<f64>>>,
    pub min: Arc<Mutex<Option<f64>>>,
    pub max: Arc<Mutex<Option<f64>>>,
    pub err_integral: Arc<Mutex<Option<f64>>>,
    pub err_overflow: Arc<Mutex<Option<bool>>>,
    pub input_overflow: Arc<Mutex<Option<bool>>>,
}

impl piController {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.kp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.ti.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.tt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.min.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_1 = { let __guard = self.max.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.err_integral.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.err_overflow.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.input_overflow.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            kp: __go_clone_0_0,
            ti: __go_clone_1_0,
            tt: __go_clone_2_0,
            min: __go_clone_3_0,
            max: __go_clone_3_1,
            err_integral: __go_clone_4_0,
            err_overflow: __go_clone_5_0,
            input_overflow: __go_clone_6_0,
        }
    }
}


impl Default for piController {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_3_1 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            kp: __go_default_0_0,
            ti: __go_default_1_0,
            tt: __go_default_2_0,
            min: __go_default_3_0,
            max: __go_default_3_1,
            err_integral: __go_default_4_0,
            err_overflow: __go_default_5_0,
            input_overflow: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for piController {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.kp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.ti.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.tt.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.min.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.max.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.err_integral.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.err_overflow.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.input_overflow.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7)
    }
}

impl GoJsonDecode for piController {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static scavenge: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct14>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static scavenger: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<scavengerState>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *scavenge.lock().unwrap() = Some(Default::default());
    *scavenger.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *scavenge.lock().unwrap() = Some(Default::default());
    *scavenger.lock().unwrap() = Some(Default::default());
}


impl scavengerState {
    /// init initializes a scavenger state and wires to the current G.
    ///
    /// Must be called from a regular goroutine that can allocate.
    pub fn init(&mut self) {
        if { let __nil_target = self.g.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("scavenger state is already wired".to_string()))));
    }
        lock_init(GoPtr::local(self.lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32))))))));
        { let new_val = getg().clone(); self.g = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(timer::default()))).clone(); self.timer = new_val; };
        let mut f = Arc::new(Mutex::new(Some(Box::new(move |s: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, _: Arc<Mutex<Option<usize>>>, _: Arc<Mutex<Option<i64>>>| {
        { let __recv = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<scavengerState>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).wake(); __result };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync>)));
        (*self.timer.lock().unwrap().as_mut().unwrap()).init(f.clone(), Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn Any + Send + Sync>))));
                // input: fraction of CPU time actually used.
                // setpoint: ideal CPU fraction.
                // output: ratio of time worked to time slept (determines sleep time).
                //
                // The output of this controller is somewhat indirect to what we actually
                // want to achieve: how much time to sleep for. The reason for this definition
                // is to ensure that the controller's outputs have a direct relationship with
                // its inputs (as opposed to an inverse relationship), making it somewhat
                // easier to reason about for tuning purposes.
        { let new_val = piController { kp: Arc::new(Mutex::new(Some(0.3375))), ti: Arc::new(Mutex::new(Some(3.2e+06))), tt: Arc::new(Mutex::new(Some(1e+09))), min: Arc::new(Mutex::new(Some(0.001))), max: Arc::new(Mutex::new(Some(1000.0))), ..Default::default() }; *self.sleep_controller.lock().unwrap() = Some(new_val); };
                // Tuned loosely via Ziegler-Nichols process.
                // 1 second reset time.
                // These ranges seem wide, but we want to give the controller plenty of
                // room to hunt for the optimal value.
                // 1:1000
                // 1000:1
        { let new_val = STARTING_SCAV_SLEEP_RATIO; *self.sleep_ratio.lock().unwrap() = Some(new_val); };
                // Install real functions if stubs aren't present.
        if { let __nil_target = self.scavenge.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Box::new(move |n: Arc<Mutex<Option<usize>>>| -> (usize, i64) {
        let mut start = nanotime();
        let mut r = (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages.lock().unwrap().as_mut().unwrap()).scavenge(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));
        let mut end = nanotime();
        if { let __tmp_x = start; let __tmp_y = end; __tmp_x >= __tmp_y } {
        return (r, 0);
    }
        (*(*scavenge.lock().unwrap().as_ref().unwrap()).background_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = end; let __tmp_y = start; __tmp_x - __tmp_y }))));
        (
            r,
            { let __tmp_x = end; let __tmp_y = start; __tmp_x - __tmp_y }
        )
    }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> (usize, i64) + Send + Sync>; *self.scavenge.lock().unwrap() = Some(new_val); };
    }
        if { let __nil_target = self.should_stop.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Box::new(move || -> bool {
        return {
            let __go_cond_0 = {
                let __tmp_x = heap_retained();
                let __tmp_y = (*(*scavenge.lock().unwrap().as_ref().unwrap()).gc_percent_goal.lock().unwrap().as_mut().unwrap()).load();
                __tmp_x <= __tmp_y
            };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).mapped_ready.lock().unwrap().as_mut().unwrap()).load();
                    let __tmp_y = (*(*scavenge.lock().unwrap().as_ref().unwrap()).memory_limit_goal.lock().unwrap().as_mut().unwrap()).load();
                    __tmp_x <= __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        };
    }) as Box<dyn FnMut() -> bool + Send + Sync>; *self.should_stop.lock().unwrap() = Some(new_val); };
    }
                // If background scavenging is disabled or if there's no work to do just stop.
        if { let __nil_target = self.gomaxprocs.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Box::new(move || -> i32 {
        (*gomaxprocs.lock().unwrap().as_ref().unwrap())
    }) as Box<dyn FnMut() -> i32 + Send + Sync>; *self.gomaxprocs.lock().unwrap() = Some(new_val); };
    }
    }

    /// park parks the scavenger goroutine.
    pub fn park(&mut self) {
        lock(GoPtr::local(self.lock.clone()));
        if { let __left = getg(); let __right = self.g.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("tried to park scavenger from another goroutine".to_string()))));
    }
        { let new_val = true; *self.parked.lock().unwrap() = Some(new_val); };
        goparkunlock(self.lock.clone(), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_G_C_SCAVENGE_WAIT as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SYSTEM_GOROUTINE as u8))))))), Arc::new(Mutex::new(Some(2))));
    }

    /// ready signals to sysmon that the scavenger should be awoken.
    pub fn ready(&self) {
        (*self.sysmon_wake.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(1 as u32))));
    }

    /// wake immediately unparks the scavenger if necessary.
    ///
    /// Safe to run without a P.
    pub fn wake(&mut self) {
        lock(GoPtr::local(self.lock.clone()));
        if (*self.parked.clone().lock().unwrap().as_ref().unwrap()) {
                // Unset sysmonWake, since the scavenger is now being awoken.
        (*self.sysmon_wake.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
                // s.parked is unset to prevent a double wake-up.
        { let new_val = false; *self.parked.lock().unwrap() = Some(new_val); };
                // Ready the goroutine by injecting it. We use injectglist instead
                // of ready or goready in order to allow us to run this function
                // without a P. injectglist also avoids placing the goroutine in
                // the current P's runnext slot, which is desirable to prevent
                // the scavenger from interfering with user goroutine scheduling
                // too much.
        let mut list: Arc<Mutex<Option<gList>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*list.lock().unwrap().as_ref().unwrap()).push(GoPtr::local(self.g.clone()));
        injectglist(list.clone());
    }
                // Unset sysmonWake, since the scavenger is now being awoken.
                // s.parked is unset to prevent a double wake-up.
                // Ready the goroutine by injecting it. We use injectglist instead
                // of ready or goready in order to allow us to run this function
                // without a P. injectglist also avoids placing the goroutine in
                // the current P's runnext slot, which is desirable to prevent
                // the scavenger from interfering with user goroutine scheduling
                // too much.
        unlock(GoPtr::local(self.lock.clone()));
    }

    /// sleep puts the scavenger to sleep based on the amount of time that it worked
    /// in nanoseconds.
    ///
    /// Note that this function should only be called by the scavenger.
    ///
    /// The scavenger may be woken up earlier by a pacing change, and it may not go
    /// to sleep at all if there's a pending pacing change.
    pub fn sleep(&mut self, mut worked: Arc<Mutex<Option<f64>>>) {
        lock(GoPtr::local(self.lock.clone()));
        if { let __left = getg(); let __right = self.g.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("tried to sleep scavenger from another goroutine".to_string()))));
    }
        if { let __tmp_x = { let __v = (*worked.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MIN_SCAV_WORK_TIME as f64; __tmp_x < __tmp_y } {
                // This means there wasn't enough work to actually fill up minScavWorkTime.
                // That's fine; we shouldn't try to do anything with this information
                // because it's going result in a short enough sleep request that things
                // will get messy. Just assume we did at least this much work.
                // All this means is that we'll sleep longer than we otherwise would have.
        { let new_val = MIN_SCAV_WORK_TIME; *worked.lock().unwrap() = Some(new_val); };
    }
                // This means there wasn't enough work to actually fill up minScavWorkTime.
                // That's fine; we shouldn't try to do anything with this information
                // because it's going result in a short enough sleep request that things
                // will get messy. Just assume we did at least this much work.
                // All this means is that we'll sleep longer than we otherwise would have.
                // Multiply the critical time by 1 + the ratio of the costs of using
                // scavenged memory vs. scavenging memory. This forces us to pay down
                // the cost of reusing this memory eagerly by sleeping for a longer period
                // of time and scavenging less frequently. More concretely, we avoid situations
                // where we end up scavenging so often that we hurt allocation performance
                // because of the additional overheads of using scavenged memory.
        { let __rhs = 1.7; let mut guard = worked.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
                // sleepTime is the amount of time we're going to sleep, based on the amount
                // of time we worked, and the sleepRatio.
        let mut sleepTime = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*worked.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.sleep_ratio.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }) as i64)));
        let mut slept: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __nil_target = self.sleep_stub.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
                // Set the timer.
                //
                // This must happen here instead of inside gopark
                // because we can't close over any variables without
                // failing escape analysis.
        let mut start = nanotime();
        (*self.timer.lock().unwrap().as_mut().unwrap()).reset(Arc::new(Mutex::new(Some({ let __tmp_x = start; let __tmp_y = { let __v = (*sleepTime.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(0 as i64))));
                // Mark ourselves as asleep and go to sleep.
        { let new_val = true; *self.parked.lock().unwrap() = Some(new_val); };
        goparkunlock(self.lock.clone(), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SLEEP as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SLEEP as u8))))))), Arc::new(Mutex::new(Some(2))));
                // How long we actually slept for.
        { let new_val = { let __tmp_x = nanotime(); let __tmp_y = start; __tmp_x - __tmp_y }; *slept.lock().unwrap() = Some(new_val); };
        lock(GoPtr::local(self.lock.clone()));
                // Stop the timer here because s.wake is unable to do it for us.
                // We don't really care if we succeed in stopping the timer. One
                // reason we might fail is that we've already woken up, but the timer
                // might be in the process of firing on some other P; essentially we're
                // racing with it. That's totally OK. Double wake-ups are perfectly safe.
        (*self.timer.lock().unwrap().as_mut().unwrap()).stop();
        unlock(GoPtr::local(self.lock.clone()));
    } else {
        unlock(GoPtr::local(self.lock.clone()));
        { let new_val = { let __f_holder = self.sleep_stub.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = sleepTime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) }; *slept.lock().unwrap() = Some(new_val); };
    }
                // Set the timer.
                //
                // This must happen here instead of inside gopark
                // because we can't close over any variables without
                // failing escape analysis.
                // Mark ourselves as asleep and go to sleep.
                // How long we actually slept for.
                // Stop the timer here because s.wake is unable to do it for us.
                // We don't really care if we succeed in stopping the timer. One
                // reason we might fail is that we've already woken up, but the timer
                // might be in the process of firing on some other P; essentially we're
                // racing with it. That's totally OK. Double wake-ups are perfectly safe.
                // Stop here if we're cooling down from the controller.
        if { let __tmp_x = (*self.controller_cooldown.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
                // worked and slept aren't exact measures of time, but it's OK to be a bit
                // sloppy here. We're just hoping we're avoiding some transient bad behavior.
        let mut t = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*slept.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*worked.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.controller_cooldown.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = 0 as i64; *self.controller_cooldown.lock().unwrap() = Some(new_val); };
    } else {
        { let __target = self.controller_cooldown.clone(); let __rhs = (*t.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        return;
    }
                // worked and slept aren't exact measures of time, but it's OK to be a bit
                // sloppy here. We're just hoping we're avoiding some transient bad behavior.
                // idealFraction is the ideal % of overall application CPU time that we
                // spend scavenging.
        let mut idealFraction = Arc::new(Mutex::new(Some({ let __tmp_x = 1.0; let __tmp_y = 100.0; __tmp_x / __tmp_y })));
                // Calculate the CPU time spent.
                //
                // This may be slightly inaccurate with respect to GOMAXPROCS, but we're
                // recomputing this often enough relative to GOMAXPROCS changes in general
                // (it only changes when the world is stopped, and not during a GC) that
                // that small inaccuracy is in the noise.
        let mut cpuFraction = Arc::new(Mutex::new(Some({
            let __tmp_x = { let __v = (*worked.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = ({
                let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*slept.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*worked.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y });
                let __tmp_y = (*Arc::new(Mutex::new(Some({ let __f_holder = self.gomaxprocs.clone(); let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() } as f64))).lock().unwrap().as_ref().unwrap());
                __tmp_x * __tmp_y
            });
            __tmp_x / __tmp_y
        })));
                // Update the critSleepRatio, adjusting until we reach our ideal fraction.
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = (*self.sleep_controller.lock().unwrap().as_mut().unwrap()).next(
            Arc::new(Mutex::new(Some({ let __arg_holder = cpuFraction.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = idealFraction.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*slept.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*worked.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))),
        ); *self.sleep_ratio.lock().unwrap() = Some(__tmp_0); *ok.lock().unwrap() = Some(__tmp_1); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // The core assumption of the controller, that we can get a proportional
                // response, broke down. This may be transient, so temporarily switch to
                // sleeping a fixed, conservative amount.
        { let new_val = STARTING_SCAV_SLEEP_RATIO; *self.sleep_ratio.lock().unwrap() = Some(new_val); };
        { let new_val = 5e9 as i64; *self.controller_cooldown.lock().unwrap() = Some(new_val); };
                // Signal the scav trace printer to output this.
        self.controller_failed();
    }
    }

    /// controllerFailed indicates that the scavenger's scheduling
    /// controller failed.
    pub fn controller_failed(&mut self) {
        lock(GoPtr::local(self.lock.clone()));
        { let new_val = true; *self.print_controller_reset.lock().unwrap() = Some(new_val); };
        unlock(GoPtr::local(self.lock.clone()));
    }

    /// run is the body of the main scavenging loop.
    ///
    /// Returns the number of bytes released and the estimated time spent
    /// releasing those bytes.
    ///
    /// Must be run on the scavenger goroutine.
    pub fn run(&self) -> (usize, f64) {
    let mut released: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut worked: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));

        lock(GoPtr::local(self.lock.clone()));
        if { let __left = getg(); let __right = self.g.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("tried to run scavenger from another goroutine".to_string()))));
    }
        unlock(GoPtr::local(self.lock.clone()));
        while { let __tmp_x = { let __v = (*worked.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MIN_SCAV_WORK_TIME as f64; __tmp_x < __tmp_y } {
                // If something from outside tells us to stop early, stop.
        if { let __f_holder = self.should_stop.clone(); let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() } {
        break
    }

                // scavengeQuantum is the amount of memory we try to scavenge
                // in one go. A smaller value means the scavenger is more responsive
                // to the scheduler in case of e.g. preemption. A larger value means
                // that the overheads of scavenging are better amortized, so better
                // scavenging throughput.
                //
                // The current value is chosen assuming a cost of ~10µs/physical page
                // (this is somewhat pessimistic), which implies a worst-case latency of
                // about 160µs for 4 KiB physical pages. The current value is biased
                // toward latency over throughput.
        const scavengeQuantum: i32 = 64 << 10;


                // Accumulate the amount of time spent scavenging.
        let (mut r, mut duration) = { let __f_holder = self.scavenge.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> (usize, i64) + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> (usize, i64) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(scavengeQuantum as usize)))) };

                // On some platforms we may see end >= start if the time it takes to scavenge
                // memory is less than the minimum granularity of its clock (e.g. Windows) or
                // due to clock bugs.
                //
                // In this case, just assume scavenging takes 10 µs per regular physical page
                // (determined empirically), and conservatively ignore the impact of huge pages
                // on timing.
        const approxWorkedNSPerPhysicalPage: f64 = 10e3;

        if { let __tmp_x = duration; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let __rhs = { let __tmp_x = approxWorkedNSPerPhysicalPage as f64; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = r; let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = worked.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
                // TODO(mknyszek): If duration is small compared to worked, it could be
                // rounded down to zero. Probably not a problem in practice because the
                // values are all within a few orders of magnitude of each other but maybe
                // worth worrying about.
        { let __rhs = (*Arc::new(Mutex::new(Some(duration as f64))).lock().unwrap().as_ref().unwrap()); let mut guard = worked.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // TODO(mknyszek): If duration is small compared to worked, it could be
                // rounded down to zero. Probably not a problem in practice because the
                // values are all within a few orders of magnitude of each other but maybe
                // worth worrying about.
        { let __rhs = r; let mut guard = released.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

                // scavenge does not return until it either finds the requisite amount of
                // memory to scavenge, or exhausts the heap. If we haven't found enough
                // to scavenge, then the heap must be exhausted.
        if { let __tmp_x = r; let __tmp_y = scavengeQuantum as usize; __tmp_x < __tmp_y } {
        break
    }

                // When using fake time just do one loop.
        if { let __tmp_x = (*faketime.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        break
    }
    }
                // If something from outside tells us to stop early, stop.
                // scavengeQuantum is the amount of memory we try to scavenge
                // in one go. A smaller value means the scavenger is more responsive
                // to the scheduler in case of e.g. preemption. A larger value means
                // that the overheads of scavenging are better amortized, so better
                // scavenging throughput.
                //
                // The current value is chosen assuming a cost of ~10µs/physical page
                // (this is somewhat pessimistic), which implies a worst-case latency of
                // about 160µs for 4 KiB physical pages. The current value is biased
                // toward latency over throughput.
                // Accumulate the amount of time spent scavenging.
                // On some platforms we may see end >= start if the time it takes to scavenge
                // memory is less than the minimum granularity of its clock (e.g. Windows) or
                // due to clock bugs.
                //
                // In this case, just assume scavenging takes 10 µs per regular physical page
                // (determined empirically), and conservatively ignore the impact of huge pages
                // on timing.
                // TODO(mknyszek): If duration is small compared to worked, it could be
                // rounded down to zero. Probably not a problem in practice because the
                // values are all within a few orders of magnitude of each other but maybe
                // worth worrying about.
                // scavenge does not return until it either finds the requisite amount of
                // memory to scavenge, or exhausts the heap. If we haven't found enough
                // to scavenge, then the heap must be exhausted.
                // When using fake time just do one loop.
        if { let __tmp_x = { let __v = (*released.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*released.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // If this happens, it means that we may have attempted to release part
                // of a physical page, but the likely effect of that is that it released
                // the whole physical page, some of which may have still been in-use.
                // This could lead to memory corruption. Throw.
        throw(Arc::new(Mutex::new(Some("released less than one physical page of memory".to_string()))));
    }
                // If this happens, it means that we may have attempted to release part
                // of a physical page, but the likely effect of that is that it released
                // the whole physical page, some of which may have still been in-use.
                // This could lead to memory corruption. Throw.
        return ((*released.lock().unwrap().as_ref().unwrap()), (*worked.lock().unwrap().as_ref().unwrap()));
    }
}

impl crate::mpagealloc::pageAlloc {
    /// scavenge scavenges nbytes worth of free pages, starting with the
    /// highest address first. Successive calls continue from where it left
    /// off until the heap is exhausted. force makes all memory available to
    /// scavenge, ignoring huge page heuristics.
    ///
    /// Returns the amount of memory scavenged in bytes.
    ///
    /// scavenge always tries to scavenge nbytes worth of memory, and will
    /// only fail to do so if the heap is exhausted for now.
    pub fn scavenge(&mut self, nbytes: Arc<Mutex<Option<usize>>>, shouldStop: Arc<Mutex<Option<Box<dyn FnMut() -> bool + Send + Sync>>>>, force: Arc<Mutex<Option<bool>>>) -> usize {
        let mut released = Arc::new(Mutex::new(Some(0 as usize)));
        while { let __tmp_x = { let __v = (*released.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nbytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let (mut ci, mut pageIdx) = (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_ref().unwrap()).find(Arc::new(Mutex::new(Some({ let __arg_holder = force.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*ci.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mpagealloc::chunkIdx(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        break
    }
        let ci_closure_clone = ci.clone(); let nbytes_closure_clone = nbytes.clone(); let mut p_closure_clone = (*self).clone(); let pageIdx_closure_clone = pageIdx.clone(); let mut released_closure_clone = released.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let __rhs = p_closure_clone.scavenge_one(
            Arc::new(Mutex::new(Some({ let __arg_holder = ci_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pageIdx_closure_clone))),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*nbytes_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*released_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))),
        ); let mut guard = released_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if { let __nil_result = (*shouldStop.lock().unwrap()).is_some(); __nil_result } && { let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = { let mut __f_guard = shouldStop.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() } {
        break
    }
    }
        return { let __v = (*released.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// scavengeOne walks over the chunk at chunk index ci and searches for
    /// a contiguous run of pages to scavenge. It will try to scavenge
    /// at most max bytes at once, but may scavenge more to avoid
    /// breaking huge pages. Once it scavenges some memory it returns
    /// how much it scavenged in bytes.
    ///
    /// searchIdx is the page index to start searching from in ci.
    ///
    /// Returns the number of bytes scavenged.
    ///
    /// Must run on the systemstack because it acquires p.mheapLock.
    ///
    ///go:systemstack
    pub fn scavenge_one(&mut self, ci: Arc<Mutex<Option<chunkIdx>>>, searchIdx: Arc<Mutex<Option<u64>>>, max: Arc<Mutex<Option<usize>>>) -> usize {
                // Calculate the maximum number of pages to scavenge.
                //
                // This should be alignUp(max, pageSize) / pageSize but max can and will
                // be ^uintptr(0), so we need to be very careful not to overflow here.
                // Rather than use alignUp, calculate the number of pages rounded down
                // first, then add back one if necessary.
        let mut maxPages = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let mut guard = maxPages.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Calculate the minimum number of pages we can scavenge.
                //
                // Because we can only scavenge whole physical pages, we must
                // ensure that we scavenge at least minPages each time, aligned
                // to minPages*pageSize.
        let mut minPages = Arc::new(Mutex::new(Some({ let __tmp_x = (*physPageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*minPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x < __tmp_y } {
        { let new_val = 1 as usize; *minPages.lock().unwrap() = Some(new_val); };
    }
        lock(GoPtr::local(self.mheap_lock.clone()));
        if {
            let __tmp_x = crate::mpagealloc::pallocSum::max(&({ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }[(*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone()));
            let __tmp_y = (*Arc::new(Mutex::new(Some((*minPages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap());
            __tmp_x >= __tmp_y
        } {
                // We only bother looking for a candidate if there at least
                // minPages free pages at all.
        let (mut base, mut npages) = { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).find_scavenge_candidate(Arc::new(Mutex::new(Some({ let __arg_holder = searchIdx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = minPages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = maxPages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
                // If we found something, scavenge it and return!
        if { let __tmp_x = npages; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
                // Compute the full address for the start of the range.
        let mut addr = Arc::new(Mutex::new(Some({ let __tmp_x = chunk_base(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(base as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
                // Mark the range we're about to scavenge as allocated, because
                // we don't want any allocating goroutines to grab it while
                // the scavenging is in progress. Be careful here -- just do the
                // bare minimum to avoid stepping on our own scavenging stats.
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).alloc_range(Arc::new(Mutex::new(Some(base))), Arc::new(Mutex::new(Some(npages)))); __result };
        self.update(
            Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(npages as usize))),
            Arc::new(Mutex::new(Some(true))),
            Arc::new(Mutex::new(Some(true))),
        );
                // With that done, it's safe to unlock.
        unlock(GoPtr::local(self.mheap_lock.clone()));
        if !(*self.test.clone().lock().unwrap().as_ref().unwrap()) {
                // Only perform sys* operations if we're not in a test.
                // It's dangerous to do so otherwise.
        sys_unused(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(npages as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }))));
                // Update global accounting only when not in test, otherwise
                // the runtime's accounting will be wrong.
        let mut nbytes = Arc::new(Mutex::new(Some(({ let __tmp_x = npages; let __tmp_y = PAGE_SIZE as u64; __tmp_x * __tmp_y }) as i64)));
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_released.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = nbytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_free.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(-((*nbytes.lock().unwrap().as_ref().unwrap()))))));
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).committed.clone(), Arc::new(Mutex::new(Some(-((*nbytes.lock().unwrap().as_ref().unwrap()))))));
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).released.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = nbytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
    }
                // Only perform sys* operations if we're not in a test.
                // It's dangerous to do so otherwise.
                // Update global accounting only when not in test, otherwise
                // the runtime's accounting will be wrong.
                // Relock the heap, because now we need to make these pages
                // available allocation. Free them back to the page allocator.
        lock(GoPtr::local(self.mheap_lock.clone()));
        {
        let mut b = Arc::new(Mutex::new(Some((offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))));;
        if (*b.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = self.search_addr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
            { let new_val = b.lock().unwrap().as_ref().unwrap().clone(); *self.search_addr.lock().unwrap() = Some(new_val); };;
        }
    }
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).free(Arc::new(Mutex::new(Some(base))), Arc::new(Mutex::new(Some(npages)))); __result };
        self.update(
            Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(npages as usize))),
            Arc::new(Mutex::new(Some(true))),
            Arc::new(Mutex::new(Some(false))),
        );
                // Mark the range as scavenged.
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __field = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.clone(); let __result = (*__field.lock().unwrap().as_mut().unwrap()).set_range(Arc::new(Mutex::new(Some(base))), Arc::new(Mutex::new(Some(npages)))); __result };
        unlock(GoPtr::local(self.mheap_lock.clone()));
        return { let __tmp_x = (*Arc::new(Mutex::new(Some(npages as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y };
    }
    }
                // We only bother looking for a candidate if there at least
                // minPages free pages at all.
                // If we found something, scavenge it and return!
                // Compute the full address for the start of the range.
                // Mark the range we're about to scavenge as allocated, because
                // we don't want any allocating goroutines to grab it while
                // the scavenging is in progress. Be careful here -- just do the
                // bare minimum to avoid stepping on our own scavenging stats.
                // With that done, it's safe to unlock.
                // Only perform sys* operations if we're not in a test.
                // It's dangerous to do so otherwise.
                // Update global accounting only when not in test, otherwise
                // the runtime's accounting will be wrong.
                // Relock the heap, because now we need to make these pages
                // available allocation. Free them back to the page allocator.
                // Mark the range as scavenged.
                // Mark this chunk as having no free pages.
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_ref().unwrap()).set_empty(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        unlock(GoPtr::local(self.mheap_lock.clone()));
        0
    }
}

impl crate::mpallocbits::pallocData {
    /// findScavengeCandidate returns a start index and a size for this pallocData
    /// segment which represents a contiguous region of free and unscavenged memory.
    ///
    /// searchIdx indicates the page index within this chunk to start the search, but
    /// note that findScavengeCandidate searches backwards through the pallocData. As
    /// a result, it will return the highest scavenge candidate in address order.
    ///
    /// min indicates a hard minimum size and alignment for runs of pages. That is,
    /// findScavengeCandidate will not return a region smaller than min pages in size,
    /// or that is min pages or greater in size but not aligned to min. min must be
    /// a non-zero power of 2 <= maxPagesPerPhysPage.
    ///
    /// max is a hint for how big of a region is desired. If max >= pallocChunkPages, then
    /// findScavengeCandidate effectively returns entire free and unscavenged regions.
    /// If max < pallocChunkPages, it may truncate the returned region such that size is
    /// max. However, findScavengeCandidate may still return a larger region if, for
    /// example, it chooses to preserve huge pages, or if max is not aligned to min (it
    /// will round up). That is, even if max is small, the returned size is not guaranteed
    /// to be equal to max. max is allowed to be less than min, in which case it is as if
    /// max == min.
    pub fn find_scavenge_candidate(&self, searchIdx: Arc<Mutex<Option<u64>>>, minimum: Arc<Mutex<Option<usize>>>, mut max: Arc<Mutex<Option<usize>>>) -> (u64, u64) {
        if { let __tmp_x = { let __tmp_x = { let __v = (*minimum.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*minimum.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*minimum.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: min = ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*minimum.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        throw(Arc::new(Mutex::new(Some("min must be a non-zero power of 2".to_string()))));
    } else if { let __tmp_x = { let __v = (*minimum.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_PAGES_PER_PHYS_PAGE as usize; __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: min = ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*minimum.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        throw(Arc::new(Mutex::new(Some("min too large".to_string()))));
    }
                // max may not be min-aligned, so we might accidentally truncate to
                // a max value which causes us to return a non-min-aligned value.
                // To prevent this, align max up to a multiple of min (which is always
                // a power of 2). This also prevents max from ever being less than
                // min, unless it's zero, so handle that explicitly.
        if { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = minimum.lock().unwrap().as_ref().unwrap().clone(); *max.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = max.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = minimum.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *max.lock().unwrap() = Some(new_val); };
    }
        let mut i = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*searchIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x / __tmp_y }) as i32)));
                // Start by quickly skipping over blocks of non-free or scavenged pages.
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
                // 1s are scavenged OR non-free => 0s are unscavenged AND free
        let mut x = fill_aligned(Arc::new(Mutex::new(Some({
            let __tmp_x = { let __seq_holder = { let __named_array = (*self.scavenged.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = { let __seq_holder = { let __named_array = (*self.palloc_bits.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            __tmp_x | __tmp_y
        }))), Arc::new(Mutex::new(Some((*minimum.lock().unwrap().as_ref().unwrap()) as u64))));
        if { let __tmp_x = x; let __tmp_y = !(0 as u64) as u64; __tmp_x != __tmp_y } {
        break
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // 1s are scavenged OR non-free => 0s are unscavenged AND free
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
                // Failed to find any free/unscavenged pages.
        return (0, 0);
    }
                // Failed to find any free/unscavenged pages.
                // We have something in the 64-bit chunk at i, but it could
                // extend further. Loop until we find the extent of it.
                // 1s are scavenged OR non-free => 0s are unscavenged AND free
        let mut x = fill_aligned(Arc::new(Mutex::new(Some({
            let __tmp_x = { let __seq_holder = { let __named_array = (*self.scavenged.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = { let __seq_holder = { let __named_array = (*self.palloc_bits.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            __tmp_x | __tmp_y
        }))), Arc::new(Mutex::new(Some((*minimum.lock().unwrap().as_ref().unwrap()) as u64))));
        let mut z1 = Arc::new(Mutex::new(Some(internal_runtime_sys::leading_zeros64(Arc::new(Mutex::new(Some(!x)))) as u64)));
        let (mut run, mut end) = (Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 64 as u64; __tmp_x * __tmp_y }; let __tmp_y = ({ let __tmp_x = 64 as u64; let __tmp_y = { let __v = (*z1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x + __tmp_y }))));
        if { let __tmp_x = { let __tmp_x = x; let __tmp_y = { let __v = (*z1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
                // After shifting out z1 bits, we still have 1s,
                // so the run ends inside this word.
        { let new_val = Arc::new(Mutex::new(Some(internal_runtime_sys::leading_zeros64(Arc::new(Mutex::new(Some({ let __tmp_x = x; let __tmp_y = { let __v = (*z1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y })))) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *run.lock().unwrap() = __moved_val; };
    } else {
                // After shifting out z1 bits, we have no more 1s.
                // This means the run extends to the bottom of the
                // word so it may extend into further words.
        { let new_val = { let __tmp_x = 64 as u64; let __tmp_y = { let __v = (*z1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *run.lock().unwrap() = Some(new_val); };
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        let mut x = fill_aligned(Arc::new(Mutex::new(Some({
            let __tmp_x = { let __seq_holder = { let __named_array = (*self.scavenged.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = { let __seq_holder = { let __named_array = (*self.palloc_bits.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); let __seq_inner_holder_0 = __seq.0.clone(); let __seq_inner_guard_0 = __seq_inner_holder_0.lock().unwrap(); let __seq = __seq_inner_guard_0.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            __tmp_x | __tmp_y
        }))), Arc::new(Mutex::new(Some((*minimum.lock().unwrap().as_ref().unwrap()) as u64))));
        { let __rhs = (*Arc::new(Mutex::new(Some(internal_runtime_sys::leading_zeros64(Arc::new(Mutex::new(Some(x)))) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = run.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = x; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
                // The run stopped in this word.
        break
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
                // After shifting out z1 bits, we still have 1s,
                // so the run ends inside this word.
                // After shifting out z1 bits, we have no more 1s.
                // This means the run extends to the bottom of the
                // word so it may extend into further words.
                // The run stopped in this word.
                // Split the run we found if it's larger than max but hold on to
                // our original length, since we may need it later.
        let mut size = Arc::new(Mutex::new(Some(std::cmp::min(({ let __v = (*run.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64), ((*Arc::new(Mutex::new(Some((*max.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()) as u64)))));
        let mut start = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
                // Each huge page is guaranteed to fit in a single palloc chunk.
                //
                // TODO(mknyszek): Support larger huge page sizes.
                // TODO(mknyszek): Consider taking pages-per-huge-page as a parameter
                // so we can write tests for this.
        if { let __tmp_x = (*physHugePageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x > __tmp_y } && { let __tmp_x = (*physHugePageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
                // We have huge pages, so let's ensure we don't break one by scavenging
                // over a huge page boundary. If the range [start, start+size) overlaps with
                // a free-and-unscavenged huge page, we want to grow the region we scavenge
                // to include that huge page.
                // Compute the huge page boundary above our candidate.
        let mut pagesPerHugePage = Arc::new(Mutex::new(Some({ let __tmp_x = (*physHugePageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y })));
        let mut hugePageAbove = Arc::new(Mutex::new(Some(align_up(Arc::new(Mutex::new(Some((*start.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = pagesPerHugePage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)));
                // If that boundary is within our current candidate, then we may be breaking
                // a huge page.
        if { let __tmp_x = { let __v = (*hugePageAbove.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
                // Compute the huge page boundary below our candidate.
        let mut hugePageBelow = Arc::new(Mutex::new(Some(align_down(Arc::new(Mutex::new(Some((*start.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = pagesPerHugePage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)));
        if { let __tmp_x = { let __v = (*hugePageBelow.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*run.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; __tmp_x >= __tmp_y } {
                // We're in danger of breaking apart a huge page since start+size crosses
                // a huge page boundary and rounding down start to the nearest huge
                // page boundary is included in the full run we found. Include the entire
                // huge page in the bound by rounding down to the huge page size.
        { let new_val = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hugePageBelow.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x + __tmp_y }; *size.lock().unwrap() = Some(new_val); };
        { let new_val = hugePageBelow.lock().unwrap().as_ref().unwrap().clone(); *start.lock().unwrap() = Some(new_val); };
    }
    }
    }
                // We have huge pages, so let's ensure we don't break one by scavenging
                // over a huge page boundary. If the range [start, start+size) overlaps with
                // a free-and-unscavenged huge page, we want to grow the region we scavenge
                // to include that huge page.
                // Compute the huge page boundary above our candidate.
                // If that boundary is within our current candidate, then we may be breaking
                // a huge page.
                // Compute the huge page boundary below our candidate.
                // We're in danger of breaking apart a huge page since start+size crosses
                // a huge page boundary and rounding down start to the nearest huge
                // page boundary is included in the full run we found. Include the entire
                // huge page in the bound by rounding down to the huge page size.
        return ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
}

impl scavengeIndex {
    /// init initializes the scavengeIndex.
    ///
    /// Returns the amount added to sysStat.
    pub fn init(&mut self, test: Arc<Mutex<Option<bool>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> usize {
        (*self.search_addr_bg.lock().unwrap().as_ref().unwrap()).clear();
        (*self.search_addr_force.lock().unwrap().as_ref().unwrap()).clear();
        { let new_val = minOffAddr.lock().unwrap().as_ref().unwrap().clone(); *self.free_h_w_m.lock().unwrap() = Some(new_val); };
        { let new_val = test.lock().unwrap().as_ref().unwrap().clone(); *self.test.lock().unwrap() = Some(new_val); };
        self.sys_init(Arc::new(Mutex::new(Some({ let __arg_holder = test.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sysStat.clone())
    }

    /// sysGrow updates the index's backing store in response to a heap growth.
    ///
    /// Returns the amount of memory added to sysStat.
    pub fn grow(&self, base: Arc<Mutex<Option<usize>>>, limit: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> usize {
                // Update minHeapIdx. Note that even if there's no mapping work to do,
                // we may still have a new, lower minimum heap address.
        let mut minHeapIdx = (*self.min_heap_idx.lock().unwrap().as_mut().unwrap()).load();
        {
        let mut baseIdx = Arc::new(Mutex::new(Some((*(*chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize)));;
        if { let __tmp_x = minHeapIdx; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*baseIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = minHeapIdx; __tmp_x < __tmp_y } {
            (*self.min_heap_idx.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = baseIdx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
        self.sys_grow(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sysStat.clone())
    }

    /// find returns the highest chunk index that may contain pages available to scavenge.
    /// It also returns an offset to start searching in the highest chunk.
    pub fn find(&self, force: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<crate::mpagealloc::chunkIdx>>>, u64) {
        let mut cursor = self.search_addr_bg.clone();
        if { let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = self.search_addr_force.clone().clone(); cursor = new_val; };
    }
        let (mut searchAddr, mut marked) = { let __recv = cursor.clone(); let __recv_ptr: *const crate::mranges::atomicOffAddr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mranges::atomicOffAddr }; let __result = unsafe { &*__recv_ptr }.load(); __result };
        if { let __tmp_x = searchAddr; let __tmp_y = (*minOffAddr.lock().unwrap().as_ref().unwrap()).addr(); __tmp_x == __tmp_y } {
                // We got a cleared search addr.
        return (Arc::new(Mutex::new(Some(crate::mpagealloc::chunkIdx(Arc::new(Mutex::new(Some(0 as u64))))))), 0);
    }
                // We got a cleared search addr.
                // Starting from searchAddr's chunk, iterate until we find a chunk with pages to scavenge.
        let mut gen = Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut min = Arc::new(Mutex::new(Some(crate::mpagealloc::chunkIdx(Arc::new(Mutex::new(Some((*self.min_heap_idx.lock().unwrap().as_mut().unwrap()).load() as u64)))))));
        let mut start = chunk_index(Arc::new(Mutex::new(Some(searchAddr))));
                // N.B. We'll never map the 0'th chunk, so minHeapIdx ensures this loop overflow.
        let mut i = { let __owned = start.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = (*i.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*min.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x >= __tmp_y } {
                // Skip over chunks.
        if !{ let __recv = { let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.load(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).should_scavenge(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = force.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - 1 as u64); }; continue
    }

                // We're still scavenging this chunk.
        if { let __tmp_x = (*i.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*start.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return (
            { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) },
            chunk_page_index(Arc::new(Mutex::new(Some(searchAddr))))
        );
    }

                // Try to reduce searchAddr to newSearchAddr.
        let mut newSearchAddr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = chunk_base(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x + __tmp_y }; let __tmp_y = PAGE_SIZE as usize; __tmp_x - __tmp_y })));
        if marked {
                // Attempt to be the first one to decrease the searchAddr
                // after an increase. If we fail, that means there was another
                // increase, or somebody else got to it before us. Either way,
                // it doesn't matter. We may lose some performance having an
                // incorrect search address, but it's far more important that
                // we don't miss updates.
        { let __recv = cursor.clone(); let __recv_ptr: *const crate::mranges::atomicOffAddr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mranges::atomicOffAddr }; let __result = unsafe { &*__recv_ptr }.store_unmark(Arc::new(Mutex::new(Some(searchAddr))), Arc::new(Mutex::new(Some({ let __arg_holder = newSearchAddr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    } else {
                // Decrease searchAddr.
        { let __recv = cursor.clone(); let __recv_ptr: *const crate::mranges::atomicOffAddr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mranges::atomicOffAddr }; let __result = unsafe { &*__recv_ptr }.store_min(Arc::new(Mutex::new(Some({ let __arg_holder = newSearchAddr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }
                // Attempt to be the first one to decrease the searchAddr
                // after an increase. If we fail, that means there was another
                // increase, or somebody else got to it before us. Either way,
                // it doesn't matter. We may lose some performance having an
                // incorrect search address, but it's far more important that
                // we don't miss updates.
                // Decrease searchAddr.
        return (
            { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) },
            ((PALLOC_CHUNK_PAGES as u64) - (1 as u64)) as u64
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - 1 as u64); }
    }
                // Skip over chunks.
                // We're still scavenging this chunk.
                // Try to reduce searchAddr to newSearchAddr.
                // Attempt to be the first one to decrease the searchAddr
                // after an increase. If we fail, that means there was another
                // increase, or somebody else got to it before us. Either way,
                // it doesn't matter. We may lose some performance having an
                // incorrect search address, but it's far more important that
                // we don't miss updates.
                // Decrease searchAddr.
                // Clear searchAddr, because we've exhausted the heap.
        { let __recv = cursor.clone(); let __recv_ptr: *const crate::mranges::atomicOffAddr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mranges::atomicOffAddr }; let __result = unsafe { &*__recv_ptr }.clear(); __result };
        (Arc::new(Mutex::new(Some(crate::mpagealloc::chunkIdx(Arc::new(Mutex::new(Some(0 as u64))))))), 0)
    }

    /// alloc updates metadata for chunk at index ci with the fact that
    /// an allocation of npages occurred. It also eagerly attempts to collapse
    /// the chunk's memory into hugepage if the chunk has become sufficiently
    /// dense and we're not allocating the whole chunk at once (which suggests
    /// the allocation is part of a bigger one and it's probably not worth
    /// eagerly collapsing).
    ///
    /// alloc may only run concurrently with find.
    pub fn alloc(&self, ci: Arc<Mutex<Option<chunkIdx>>>, npages: Arc<Mutex<Option<u64>>>) {
        let mut sc = { let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.load();
        (*sc.lock().unwrap().as_mut().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
                // TODO(mknyszek): Consider eagerly backing memory with huge pages
                // here and track whether we believe this chunk is backed by huge pages.
                // In the past we've attempted to use sysHugePageCollapse (which uses
                // MADV_COLLAPSE on Linux, and is unsupported elswhere) for this purpose,
                // but that caused performance issues in production environments.
        { let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.store(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// free updates metadata for chunk at index ci with the fact that
    /// a free of npages occurred.
    ///
    /// free may only run concurrently with find.
    pub fn free(&mut self, ci: Arc<Mutex<Option<chunkIdx>>>, page: Arc<Mutex<Option<u64>>>, npages: Arc<Mutex<Option<u64>>>) {
        let mut sc = { let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.load();
        (*sc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.store(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Update scavenge search addresses.
        let mut addr = Arc::new(Mutex::new(Some({ let __tmp_x = chunk_base(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __v = (*page.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        if (*self.free_h_w_m.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))) {
        { let new_val = offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }; *self.free_h_w_m.lock().unwrap() = Some(new_val); };
    }
                // N.B. Because free is serialized, it's not necessary to do a
                // full CAS here. free only ever increases searchAddr, while
                // find only ever decreases it. Since we only ever race with
                // decreases, even if the value we loaded is stale, the actual
                // value will never be larger.
        let (mut searchAddr, _) = (*self.search_addr_force.lock().unwrap().as_ref().unwrap()).load();
        if (offAddr { a: Arc::new(Mutex::new(Some(searchAddr))), ..Default::default() }).less_than(Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))) {
        (*self.search_addr_force.lock().unwrap().as_ref().unwrap()).store_marked(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }

    /// nextGen moves the scavenger forward one generation. Must be called
    /// once per GC cycle, but may be called more often to force more memory
    /// to be released.
    ///
    /// nextGen may only run concurrently with find.
    pub fn next_gen(&mut self) {
        { let __target = self.gen.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let (mut searchAddr, _) = (*self.search_addr_bg.lock().unwrap().as_ref().unwrap()).load();
        if (offAddr { a: Arc::new(Mutex::new(Some(searchAddr))), ..Default::default() }).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = self.free_h_w_m.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        (*self.search_addr_bg.lock().unwrap().as_ref().unwrap()).store_marked(Arc::new(Mutex::new(Some((*self.free_h_w_m.lock().unwrap().as_ref().unwrap()).addr()))));
    }
        { let new_val = minOffAddr.lock().unwrap().as_ref().unwrap().clone(); *self.free_h_w_m.lock().unwrap() = Some(new_val); };
    }

    /// setEmpty marks that the scavenger has finished looking at ci
    /// for now to prevent the scavenger from getting stuck looking
    /// at the same chunk.
    ///
    /// setEmpty may only run concurrently with find.
    pub fn set_empty(&self, ci: Arc<Mutex<Option<chunkIdx>>>) {
        let mut val = { let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.load();
        (*val.lock().unwrap().as_mut().unwrap()).set_empty();
        { let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.store(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

impl atomicScavChunkData {
    /// load loads and unpacks a scavChunkData.
    pub fn load(&self) -> Arc<Mutex<Option<scavChunkData>>> {
        unpack_scav_chunk_data(Arc::new(Mutex::new(Some((*self.value.lock().unwrap().as_mut().unwrap()).load()))))
    }

    /// store packs and writes a new scavChunkData. store must be serialized
    /// with other calls to store.
    pub fn store(&self, ssc: Arc<Mutex<Option<scavChunkData>>>) {
        (*self.value.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some((*ssc.lock().unwrap().as_ref().unwrap()).pack()))));
    }
}

impl scavChunkData {
    /// pack returns sc packed into a uint64.
    pub fn pack(&self) -> u64 {
        return {
            let __go_binary_0 = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.in_use.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.last_in_use.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_2 = 16;
            let __go_binary_3 = __go_binary_1 << __go_binary_2;
            let __go_binary_4 = __go_binary_0 | __go_binary_3;
            let __go_binary_5 = (*Arc::new(Mutex::new(Some((*(*self.scav_chunk_flags.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_6 = 16;
            let __go_binary_7 = LOG_SCAV_CHUNK_IN_USE_MAX;
            let __go_binary_8 = __go_binary_6 + __go_binary_7;
            let __go_binary_9 = __go_binary_5 << __go_binary_8;
            let __go_binary_10 = __go_binary_4 | __go_binary_9;
            let __go_binary_11 = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_12 = 32;
            let __go_binary_13 = __go_binary_11 << __go_binary_12;
            let __go_binary_14 = __go_binary_10 | __go_binary_13;
            __go_binary_14
        };
    }

    /// shouldScavenge returns true if the corresponding chunk should be interrogated
    /// by the scavenger.
    pub fn should_scavenge(&self, currGen: Arc<Mutex<Option<u32>>>, force: Arc<Mutex<Option<bool>>>) -> bool {
        if { let __promoted_recv = self.scav_chunk_flags.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.is_empty(); __result } {
                // Nothing to scavenge.
        return false;
    }
                // Nothing to scavenge.
        if { let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // We're forcing the memory to be scavenged.
        return true;
    }
                // We're forcing the memory to be scavenged.
        if { let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*currGen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
                // In the current generation, if either the current or last generation
                // is dense, then skip scavenging. Inverting that, we should scavenge
                // if both the current and last generation were not dense.
        return { let __tmp_x = (*self.in_use.lock().unwrap().as_ref().unwrap()); let __tmp_y = SCAV_CHUNK_HI_OCC_PAGES as u16; __tmp_x < __tmp_y } && { let __tmp_x = (*self.last_in_use.lock().unwrap().as_ref().unwrap()); let __tmp_y = SCAV_CHUNK_HI_OCC_PAGES as u16; __tmp_x < __tmp_y };
    }
                // In the current generation, if either the current or last generation
                // is dense, then skip scavenging. Inverting that, we should scavenge
                // if both the current and last generation were not dense.
                // If we're one or more generations ahead, we know inUse represents the current
                // state of the chunk, since otherwise it would've been updated already.
        return { let __tmp_x = (*self.in_use.lock().unwrap().as_ref().unwrap()); let __tmp_y = SCAV_CHUNK_HI_OCC_PAGES as u16; __tmp_x < __tmp_y };
    }

    /// alloc updates sc given that npages were allocated in the corresponding chunk.
    pub fn alloc(&mut self, npages: Arc<Mutex<Option<u64>>>, newGen: Arc<Mutex<Option<u32>>>) {
        if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.in_use.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = PALLOC_CHUNK_PAGES as u64; __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: inUse=".to_string());
            let __go_print_arg_1 = format!("{}", (*self.in_use.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " npages=".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        throw(Arc::new(Mutex::new(Some("too many pages allocated in chunk?".to_string()))));
    }
        if { let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*newGen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let new_val = { let __selector_holder = self.in_use.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.last_in_use.lock().unwrap() = Some(new_val); };
        { let new_val = newGen.lock().unwrap().as_ref().unwrap().clone(); *self.gen.lock().unwrap() = Some(new_val); };
    }
        { let __target = self.in_use.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u16))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = (*self.in_use.lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_CHUNK_PAGES as u16; __tmp_x == __tmp_y } {
                // There's nothing for the scavenger to take from here.
        { let __promoted_recv = self.scav_chunk_flags.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.set_empty(); __result };
    }
    }

    /// free updates sc given that npages was freed in the corresponding chunk.
    pub fn free(&mut self, npages: Arc<Mutex<Option<u64>>>, newGen: Arc<Mutex<Option<u32>>>) {
        if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.in_use.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: inUse=".to_string());
            let __go_print_arg_1 = format!("{}", (*self.in_use.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " npages=".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        throw(Arc::new(Mutex::new(Some("allocated pages below zero?".to_string()))));
    }
        if { let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*newGen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let new_val = { let __selector_holder = self.in_use.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.last_in_use.lock().unwrap() = Some(new_val); };
        { let new_val = newGen.lock().unwrap().as_ref().unwrap().clone(); *self.gen.lock().unwrap() = Some(new_val); };
    }
        { let __target = self.in_use.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u16))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
                // The scavenger can no longer be done with this chunk now that
                // new memory has been freed into it.
        { let __promoted_recv = self.scav_chunk_flags.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.set_non_empty(); __result };
    }

    pub fn is_empty(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.scav_chunk_flags.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_empty()
    }

    pub fn set_empty(&mut self) {
        // Forward to embedded type's method
        let embedded = self.scav_chunk_flags.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_empty()
    }

    pub fn set_non_empty(&mut self) {
        // Forward to embedded type's method
        let embedded = self.scav_chunk_flags.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_non_empty()
    }
}

impl scavChunkFlags {
    /// isEmpty returns true if the hasFree flag is unset.
    pub fn is_empty(&self) -> bool {
        return {
            let __tmp_x = scavChunkFlags(Arc::new(Mutex::new(Some((((*(*self).clone().0.lock().unwrap().as_ref().unwrap())) & SCAV_CHUNK_HAS_FREE as u8)))));
            let __tmp_y = scavChunkFlags(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x == __tmp_y
        };
    }

    /// setEmpty clears the hasFree flag.
    pub fn set_empty(&mut self) {
        { let __rhs = SCAV_CHUNK_HAS_FREE as u8; let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
    }

    /// setNonEmpty sets the hasFree flag.
    pub fn set_non_empty(&mut self) {
        { let __rhs = SCAV_CHUNK_HAS_FREE as u8; let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
}

impl piController {
    /// next provides a new sample to the controller.
    ///
    /// input is the sample, setpoint is the desired point, and period is how much
    /// time (in whatever unit makes the most sense) has passed since the last sample.
    ///
    /// Returns a new value for the variable it's controlling, and whether the operation
    /// completed successfully. One reason this might fail is if error has been growing
    /// in an unbounded manner, to the point of overflow.
    ///
    /// In the specific case of an error overflow occurs, the errOverflow field will be
    /// set and the rest of the controller's internal state will be fully reset.
    pub fn next(&mut self, input: Arc<Mutex<Option<f64>>>, setpoint: Arc<Mutex<Option<f64>>>, period: Arc<Mutex<Option<f64>>>) -> (f64, bool) {
                // Compute the raw output value.
        let mut prop = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.kp.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __v = (*setpoint.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*input.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x * __tmp_y })));
        let mut rawOutput = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*prop.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.err_integral.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
                // Clamp rawOutput into output.
        let mut output = { let __owned = rawOutput.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = output.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = output.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // The input had a large enough magnitude that either it was already
                // overflowed, or some operation with it overflowed.
                // Set a flag and reset. That's the safest thing to do.
        self.reset();
        { let new_val = true; *self.input_overflow.lock().unwrap() = Some(new_val); };
        return ((*self.min.lock().unwrap().as_ref().unwrap()), false);
    }
                // The input had a large enough magnitude that either it was already
                // overflowed, or some operation with it overflowed.
                // Set a flag and reset. That's the safest thing to do.
        if { let __tmp_x = { let __v = (*output.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.min.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = self.min.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *output.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*output.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.max.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = { let __selector_holder = self.max.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *output.lock().unwrap() = Some(new_val); };
    }
                // Update the controller's state.
        if { let __tmp_x = (*self.ti.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0.0; __tmp_x != __tmp_y } && { let __tmp_x = (*self.tt.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0.0; __tmp_x != __tmp_y } {
        { let __target = self.err_integral.clone(); let __rhs = {
            let __tmp_x = {
                let __tmp_x = ({
                    let __tmp_x = { let __tmp_x = (*self.kp.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*period.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y };
                    let __tmp_y = (*self.ti.lock().unwrap().as_ref().unwrap());
                    __tmp_x / __tmp_y
                });
                let __tmp_y = ({ let __tmp_x = { let __v = (*setpoint.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*input.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y });
                __tmp_x * __tmp_y
            };
            let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*period.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.tt.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __v = (*output.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rawOutput.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x * __tmp_y };
            __tmp_x + __tmp_y
        }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if is_inf(Arc::new(Mutex::new(Some({ let __selector_holder = self.err_integral.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) || is_na_n(Arc::new(Mutex::new(Some({ let __selector_holder = self.err_integral.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
                // So much error has accumulated that we managed to overflow.
                // The assumptions around the controller have likely broken down.
                // Set a flag and reset. That's the safest thing to do.
        self.reset();
        { let new_val = true; *self.err_overflow.lock().unwrap() = Some(new_val); };
        return ((*self.min.lock().unwrap().as_ref().unwrap()), false);
    }
    }
                // So much error has accumulated that we managed to overflow.
                // The assumptions around the controller have likely broken down.
                // Set a flag and reset. That's the safest thing to do.
        return ({ let __v = (*output.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }

    /// reset resets the controller state, except for controller error flags.
    pub fn reset(&mut self) {
        { let new_val = 0.0; *self.err_integral.lock().unwrap() = Some(new_val); };
    }
}

/// heapRetained returns an estimate of the current heap RSS.
pub fn heap_retained() -> u64 {
    return {
        let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_in_use.lock().unwrap().as_ref().unwrap()).load();
        let __tmp_y = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_free.lock().unwrap().as_ref().unwrap()).load();
        __tmp_x + __tmp_y
    };
}

/// gcPaceScavenger updates the scavenger's pacing, particularly
/// its rate and RSS goal. For this, it requires the current heapGoal,
/// and the heapGoal for the previous GC cycle.
///
/// The RSS goal is based on the current heap goal with a small overhead
/// to accommodate non-determinism in the allocator.
///
/// The pacing is based on scavengePageRate, which applies to both regular and
/// huge pages. See that constant for more information.
///
/// Must be called whenever GC pacing is updated.
///
/// mheap_.lock must be held or the world must be stopped.
pub fn gc_pace_scavenger(memoryLimit: Arc<Mutex<Option<i64>>>, heapGoal: Arc<Mutex<Option<u64>>>, lastHeapGoal: Arc<Mutex<Option<u64>>>) {
    assert_world_stopped_or_lock_held((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone());

        // As described at the top of this file, there are two scavenge goals here: one
        // for gcPercent and one for memoryLimit. Let's handle the latter first because
        // it's simpler.
        // We want to target retaining (100-reduceExtraPercent)% of the heap.
    let mut memoryLimitGoal = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*memoryLimit.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0.95; __tmp_x * __tmp_y }) as u64)));

        // mappedReady is comparable to memoryLimit, and represents how much total memory
        // the Go runtime has committed now (estimated).
    let mut mappedReady = (*(*gcController.lock().unwrap().as_ref().unwrap()).mapped_ready.lock().unwrap().as_mut().unwrap()).load();

        // If we're below the goal already indicate that we don't need the background
        // scavenger for the memory limit. This may seems worrisome at first, but note
        // that the allocator will assist the background scavenger in the face of a memory
        // limit, so we'll be safe even if we stop the scavenger when we shouldn't have.
    if { let __tmp_x = mappedReady; let __tmp_y = { let __v = (*memoryLimitGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        (*(*scavenge.lock().unwrap().as_ref().unwrap()).memory_limit_goal.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(!(0 as u64) as u64))));
    } else {
        (*(*scavenge.lock().unwrap().as_ref().unwrap()).memory_limit_goal.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = memoryLimitGoal.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Now handle the gcPercent goal.
        // If we're called before the first GC completed, disable scavenging.
        // We never scavenge before the 2nd GC cycle anyway (we don't have enough
        // information about the heap yet) so this is fine, and avoids a fault
        // or garbage data later.
    if { let __tmp_x = { let __v = (*lastHeapGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        (*(*scavenge.lock().unwrap().as_ref().unwrap()).gc_percent_goal.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(!(0 as u64) as u64))));
        return;
    }

        // Compute our scavenging goal.
    let mut goalRatio = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*heapGoal.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*lastHeapGoal.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y })));
    let mut gcPercentGoal = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*memstats.lock().unwrap().as_ref().unwrap()).last_heap_in_use.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*goalRatio.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as u64)));

        // Add retainExtraPercent overhead to retainedGoal. This calculation
        // looks strange but the purpose is to arrive at an integer division
        // (e.g. if retainExtraPercent = 12.5, then we get a divisor of 8)
        // that also avoids the overflow from a multiplication.
    { let __rhs = { let __tmp_x = { let __v = (*gcPercentGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = 1.0; let __tmp_y = 0.1; __tmp_x / __tmp_y }) as u64; __tmp_x / __tmp_y }; let mut guard = gcPercentGoal.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

        // Align it to a physical page boundary to make the following calculations
        // a bit more exact.
    { let new_val = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*gcPercentGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*physPageSize.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*physPageSize.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & ! __tmp_y }; *gcPercentGoal.lock().unwrap() = Some(new_val); };

        // Represents where we are now in the heap's contribution to RSS in bytes.
        //
        // Guaranteed to always be a multiple of physPageSize on systems where
        // physPageSize <= pageSize since we map new heap memory at a size larger than
        // any physPageSize and released memory in multiples of the physPageSize.
        //
        // However, certain functions recategorize heap memory as other stats (e.g.
        // stacks) and this happens in multiples of pageSize, so on systems
        // where physPageSize > pageSize the calculations below will not be exact.
        // Generally this is OK since we'll be off by at most one regular
        // physical page.
    let mut heapRetainedNow = heap_retained();

        // If we're already below our goal, or within one page of our goal, then indicate
        // that we don't need the background scavenger for maintaining a memory overhead
        // proportional to the heap goal.
    if { let __tmp_x = heapRetainedNow; let __tmp_y = { let __v = (*gcPercentGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } || { let __tmp_x = { let __tmp_x = heapRetainedNow; let __tmp_y = { let __v = (*gcPercentGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*physPageSize.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        (*(*scavenge.lock().unwrap().as_ref().unwrap()).gc_percent_goal.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(!(0 as u64) as u64))));
    } else {
        (*(*scavenge.lock().unwrap().as_ref().unwrap()).gc_percent_goal.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = gcPercentGoal.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

/// printScavTrace prints a scavenge trace line to standard error.
///
/// released should be the amount of memory released since the last time this
/// was called, and forced indicates whether the scavenge was forced by the
/// application.
///
/// scavenger.lock must be held.
pub fn print_scav_trace(releasedBg: Arc<Mutex<Option<usize>>>, releasedEager: Arc<Mutex<Option<usize>>>, forced: Arc<Mutex<Option<bool>>>) {
    assert_lock_held(GoPtr::local((*scavenger.lock().unwrap().as_ref().unwrap()).lock.clone()));

    printlock();
    {
            let __go_print_arg_0 = format!("{}", "scav ".to_string());
            let __go_print_arg_1 = format!("{}", { let __tmp_x = { let __v = (*releasedBg.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x >> __tmp_y });
            let __go_print_arg_2 = format!("{}", " KiB work (bg), ".to_string());
            let __go_print_arg_3 = format!("{}", { let __tmp_x = { let __v = (*releasedEager.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x >> __tmp_y });
            let __go_print_arg_4 = format!("{}", " KiB work (eager), ".to_string());
            let __go_print_arg_5 = format!("{}", { let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_released.lock().unwrap().as_ref().unwrap()).load(); let __tmp_y = 10; __tmp_x >> __tmp_y });
            let __go_print_arg_6 = format!("{}", " KiB now, ".to_string());
            let __go_print_arg_7 = format!("{}", {
                let __tmp_x = ({ let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_in_use.lock().unwrap().as_ref().unwrap()).load(); let __tmp_y = 100 as u64; __tmp_x * __tmp_y });
                let __tmp_y = heap_retained();
                __tmp_x / __tmp_y
            });
            let __go_print_arg_8 = format!("{}", "% util".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
    if { let __v = (*forced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", " (forced)".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    } else if (*{ let __field = (*scavenger.lock().unwrap().as_ref().unwrap()).print_controller_reset.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
            let __go_print_arg_0 = format!("{}", " [controller reset]".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        { let new_val = false; *(*scavenger.lock().unwrap().as_ref().unwrap()).print_controller_reset.lock().unwrap() = Some(new_val); };
    }
    eprintln!();
    printunlock();
}

/// fillAligned returns x but with all zeroes in m-aligned
/// groups of m bits set to 1 if any bit in the group is non-zero.
///
/// For example, fillAligned(0x0100a3, 8) == 0xff00ff.
///
/// Note that if m == 1, this is a no-op.
///
/// m must be a power of 2 <= maxPagesPerPhysPage.
pub fn fill_aligned(mut x: Arc<Mutex<Option<u64>>>, m: Arc<Mutex<Option<u64>>>) -> u64 {
    let mut apply = Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<u64>>>, c: Arc<Mutex<Option<u64>>>| -> u64 {
        !({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }); let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y })
    }) as Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync>)));

        // The technique used it here is derived from
        // https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord
        // and extended for more than just bytes (like nibbles
        // and uint16s) by using an appropriate constant.
        //
        // To summarize the technique, quoting from that page:
        // "[It] works by first zeroing the high bits of the [8]
        // bytes in the word. Subsequently, it adds a number that
        // will result in an overflow to the high bit of a byte if
        // any of the low bits were initially set. Next the high
        // bits of the original word are ORed with these values;
        // thus, the high bit of a byte is set iff any bit in the
        // byte was set. Finally, we determine if any of these high
        // bits are zero by ORing with ones everywhere except the
        // high bits and inverting the result."
        // Transform x to contain a 1 bit at the top of each m-aligned
        // group of m zero bits.
    { let _switch_val = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (1 as u64) {
            return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
        } else if _switch_val == (2 as u64) {
            { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> = { let mut __f_guard = apply.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone(), Arc::new(Mutex::new(Some(0x5555555555555555 as u64)))) }; *x.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (4 as u64) {
            { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> = { let mut __f_guard = apply.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone(), Arc::new(Mutex::new(Some(0x7777777777777777 as u64)))) }; *x.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (8 as u64) {
            { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> = { let mut __f_guard = apply.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone(), Arc::new(Mutex::new(Some(0x7f7f7f7f7f7f7f7f as u64)))) }; *x.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (16 as u64) {
            { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> = { let mut __f_guard = apply.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone(), Arc::new(Mutex::new(Some(0x7fff7fff7fff7fff as u64)))) }; *x.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (32 as u64) {
            { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> = { let mut __f_guard = apply.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone(), Arc::new(Mutex::new(Some(0x7fffffff7fffffff as u64)))) }; *x.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (64 as u64) {
            { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> = { let mut __f_guard = apply.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u64>>>, Arc<Mutex<Option<u64>>>) -> u64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone(), Arc::new(Mutex::new(Some(0x7fffffffffffffff as u64)))) }; *x.lock().unwrap() = Some(new_val); };
        } else {
            throw(Arc::new(Mutex::new(Some("bad m value".to_string()))));
        }
    }

        // == maxPagesPerPhysPage
        // Now, the top bit of each m-aligned group in x is set
        // that group was all zero in the original x.
        // From each group of m bits subtract 1.
        // Because we know only the top bits of each
        // m-aligned group are set, we know this will
        // set each group to have all the bits set except
        // the top bit, so just OR with the original
        // result to set all the bits.
    !({ let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }); __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y })
}

/// unpackScavChunkData unpacks a scavChunkData from a uint64.
pub fn unpack_scav_chunk_data(sc: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<scavChunkData>>> {
    Arc::new(Mutex::new(Some(scavChunkData {
        in_use: Arc::new(Mutex::new(Some((*sc.lock().unwrap().as_ref().unwrap()) as u16))),
        last_in_use: Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y }) as u16))).lock().unwrap().as_ref().unwrap()); let __tmp_y = SCAV_CHUNK_IN_USE_MASK as u16; __tmp_x & __tmp_y }))),
        gen: Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }) as u32))),
        scav_chunk_flags: Arc::new(Mutex::new(Some(scavChunkFlags(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = 16; let __tmp_y = LOG_SCAV_CHUNK_IN_USE_MAX; __tmp_x + __tmp_y }); __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = SCAV_CHUNK_FLAGS_MASK as u8; __tmp_x & __tmp_y } as u8))))))),
        ..Default::default()
    })))
}

#[derive(Clone)]
pub struct AnonymousStruct14 {
    pub gc_percent_goal: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub memory_limit_goal: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub assist_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub background_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
}
impl AnonymousStruct14 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.gc_percent_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.memory_limit_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.background_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            gc_percent_goal: __go_clone_0_0,
            memory_limit_goal: __go_clone_1_0,
            assist_time: __go_clone_2_0,
            background_time: __go_clone_3_0,
        }
    }
}


impl Default for AnonymousStruct14 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            gc_percent_goal: __go_default_0_0,
            memory_limit_goal: __go_default_1_0,
            assist_time: __go_default_2_0,
            background_time: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.gc_percent_goal.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.memory_limit_goal.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.assist_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.background_time.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for AnonymousStruct14 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for scavengerState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for scavengeIndex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for atomicScavChunkData {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for scavChunkData {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for piController {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
