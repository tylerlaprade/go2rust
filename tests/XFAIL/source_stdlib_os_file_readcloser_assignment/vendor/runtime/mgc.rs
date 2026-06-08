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
    arena::{liveUserArenaChunk, userArenaState},
    atomic_pointer::{atomicstorep},
    lfstack::{lfstack},
    lock_spinbit::{lock, unlock},
    lockrank_off::{assert_world_stopped},
    malloc::{MIN_HEAP_FOR_METADATA_HUGE_PAGES},
    mcache::{mcache},
    mcheckmark::{end_checkmarks, start_checkmarks},
    mgclimit::{LIMITER_EVENT_IDLE_MARK_WORK, gcCPULimiter, limiterEvent, limiterEventType},
    mgcmark::{gcDrainFlags, gc_drain, gc_drain_mark_worker_dedicated, gc_drain_mark_worker_fractional, gc_drain_mark_worker_idle, gc_mark_root_check, gc_mark_root_prepare, gc_mark_tiny_allocs, gc_wake_all_assists},
    mgcpacer::{gcController, gc_controller_commit},
    mgcscavenge::{scavenge, scavengeIndex},
    mgcsweep::{activeSweep, finishsweep_m, sweep, sweepClass, sweepLocker, sweepone},
    mgcwork::{free_some_wbufs, gcWork, prepare_free_workbufs, workbuf},
    mheap::{AnonymousStruct15, arenaIdx, gc_wake_all_strong_from_weak, heapArena, mSpanList, mheap_, mspan},
    mpagealloc::{pageAlloc},
    mpagecache::{pageCache},
    mprof::{m_prof__flush, m_prof__next_cycle},
    mstats::{cpuStats, memstats, sysMemStat},
    mwbbuf::{wbBuf, wb_buf_flush1},
    panic::{panicking, throw},
    pinner::{pinner},
    print::{hex, printlock, printunlock},
    proc::{STW_G_C_MARK_TERM, STW_G_C_SWEEP_TERM, cas_g_to_waiting_for_suspend_g, casgstatus, for_each_g, for_each_p, forcegcperiod, gList, gQueue, gcsema, globrunqputbatch, gopark, gosched, injectglist, ready, runqdrain, runtimeInitTime, sched_enable_user, start_the_world_with_sema, stop_the_world_with_sema, worldStop, worldsema},
    runtime1::{acquirem, debug, releasem},
    runtime2::{WAIT_REASON_FLUSH_PROC_CACHES, WAIT_REASON_GARBAGE_COLLECTION, WAIT_REASON_G_C_MARK_TERMINATION, WAIT_REASON_G_C_WORKER_ACTIVE, WAIT_REASON_G_C_WORKER_IDLE, __GRUNNING, __GWAITING, __PIDLE, _defer, allp, g, gcBgMarkWorkerCount, gcBgMarkWorkerPool, gomaxprocs, guintptr, lfnode, m, muintptr, mutex, ncpu, p, puintptr, sched, sudog},
    sema::{semacquire, semrelease},
    stack::{free_stack_spans, gc_compute_starting_stack_size},
    stubs::{getg, systemstack},
    synctest::{synctestGroup},
    time_nofake::{nanotime},
    timestub::{time_now},
    traceruntime::{TRACE_BLOCK_SYSTEM_GOROUTINE, traceLocker, trace_acquire, trace_release},
};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) const __DEBUG_G_C: i32 = 0;
pub(crate) const __FIN_BLOCK_SIZE: i32 = 4 * 1024;
pub(crate) const CONCURRENT_SWEEP: bool = true;
pub(crate) const DEBUG_SCAN_CONSERVATIVE: bool = false;
pub(crate) const SWEEP_MIN_HEAP_DISTANCE: i32 = 1024 * 1024;


pub(crate) const __G_COFF: i32 = 0;
pub(crate) const __G_CMARK: i32 = 1;
pub(crate) const __G_CMARKTERMINATION: i32 = 2;


pub(crate) const GC_MARK_WORKER_NOT_WORKER: i32 = 0;
pub(crate) const GC_MARK_WORKER_DEDICATED_MODE: i32 = 1;
pub(crate) const GC_MARK_WORKER_FRACTIONAL_MODE: i32 = 2;
pub(crate) const GC_MARK_WORKER_IDLE_MODE: i32 = 3;


pub(crate) const GC_BACKGROUND_MODE: i32 = 0;
pub(crate) const GC_FORCE_MODE: i32 = 1;
pub(crate) const GC_FORCE_BLOCK_MODE: i32 = 2;


pub(crate) const GC_TRIGGER_HEAP: i32 = 0;
pub(crate) const GC_TRIGGER_TIME: i32 = 1;
pub(crate) const GC_TRIGGER_CYCLE: i32 = 2;


/// gcMarkWorkerMode represents the mode that a concurrent mark worker
/// should operate in.
///
/// Concurrent marking happens through four different mechanisms. One
/// is mutator assists, which happen in response to allocations and are
/// not scheduled. The other three are variations in the per-P mark
/// workers and are distinguished by gcMarkWorkerMode.
#[derive(Debug, Clone, Default)]
pub struct gcMarkWorkerMode(pub Arc<Mutex<Option<i32>>>);

impl Display for gcMarkWorkerMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for gcMarkWorkerMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for gcMarkWorkerMode {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for gcMarkWorkerMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for gcMarkWorkerMode {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<gcMarkWorkerMode> for i32 {
    fn eq(&self, other: &gcMarkWorkerMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<gcMarkWorkerMode> for i32 {
    fn partial_cmp(&self, other: &gcMarkWorkerMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn add(self, other: Self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn add(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<gcMarkWorkerMode> for i32 {
    type Output = gcMarkWorkerMode;
    fn add(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn sub(self, other: Self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn sub(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<gcMarkWorkerMode> for i32 {
    type Output = gcMarkWorkerMode;
    fn sub(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn mul(self, other: Self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn mul(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<gcMarkWorkerMode> for i32 {
    type Output = gcMarkWorkerMode;
    fn mul(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn div(self, other: Self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn div(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<gcMarkWorkerMode> for i32 {
    type Output = gcMarkWorkerMode;
    fn div(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn neg(self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn rem(self, other: Self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn rem(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<gcMarkWorkerMode> for i32 {
    type Output = gcMarkWorkerMode;
    fn rem(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn bitand(self, other: Self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn bitand(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<gcMarkWorkerMode> for i32 {
    type Output = gcMarkWorkerMode;
    fn bitand(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn bitor(self, other: Self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn bitor(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<gcMarkWorkerMode> for i32 {
    type Output = gcMarkWorkerMode;
    fn bitor(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn bitxor(self, other: Self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn bitxor(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<gcMarkWorkerMode> for i32 {
    type Output = gcMarkWorkerMode;
    fn bitxor(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn not(self) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: i8) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: i16) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: i64) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: u32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: u8) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: u16) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: u64) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shl(self, other: usize) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: gcMarkWorkerMode) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: i32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: i8) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: i16) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: i64) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: u32) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: u8) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: u16) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: u64) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for gcMarkWorkerMode {
    type Output = gcMarkWorkerMode;
    fn shr(self, other: usize) -> gcMarkWorkerMode {
        gcMarkWorkerMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for gcMarkWorkerMode {}

impl Ord for gcMarkWorkerMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct workType {
    pub full: Arc<Mutex<Option<lfstack>>>,
    pub __blank_1_0: Arc<Mutex<Option<internal_cpu::r#mod::CacheLinePad>>>,
    pub empty: Arc<Mutex<Option<lfstack>>>,
    pub __blank_3_0: Arc<Mutex<Option<internal_cpu::r#mod::CacheLinePad>>>,
    pub wbuf_spans: Arc<Mutex<Option<AnonymousStruct6>>>,
    pub __blank_5_0: Arc<Mutex<Option<u32>>>,
    pub bytes_marked: Arc<Mutex<Option<u64>>>,
    pub markroot_next: Arc<Mutex<Option<u32>>>,
    pub markroot_jobs: Arc<Mutex<Option<u32>>>,
    pub nproc: Arc<Mutex<Option<u32>>>,
    pub tstart: Arc<Mutex<Option<i64>>>,
    pub nwait: Arc<Mutex<Option<u32>>>,
    pub n_data_roots: Arc<Mutex<Option<i32>>>,
    pub n_b_s_s_roots: Arc<Mutex<Option<i32>>>,
    pub n_span_roots: Arc<Mutex<Option<i32>>>,
    pub n_stack_roots: Arc<Mutex<Option<i32>>>,
    pub base_data: Arc<Mutex<Option<u32>>>,
    pub base_b_s_s: Arc<Mutex<Option<u32>>>,
    pub base_spans: Arc<Mutex<Option<u32>>>,
    pub base_stacks: Arc<Mutex<Option<u32>>>,
    pub base_end: Arc<Mutex<Option<u32>>>,
    pub stack_roots: Arc<Mutex<Option<Vec<Arc<Mutex<Option<g>>>>>>>,
    pub start_sema: Arc<Mutex<Option<u32>>>,
    pub mark_done_sema: Arc<Mutex<Option<u32>>>,
    pub bg_mark_done: Arc<Mutex<Option<u32>>>,
    pub mode: Arc<Mutex<Option<gcMode>>>,
    pub user_forced: Arc<Mutex<Option<bool>>>,
    pub initial_heap_live: Arc<Mutex<Option<u64>>>,
    pub assist_queue: Arc<Mutex<Option<AnonymousStruct7>>>,
    pub sweep_waiters: Arc<Mutex<Option<AnonymousStruct8>>>,
    pub strong_from_weak: Arc<Mutex<Option<AnonymousStruct9>>>,
    pub cycles: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub stwprocs: Arc<Mutex<Option<i32>>>,
    pub maxprocs: Arc<Mutex<Option<i32>>>,
    pub t_sweep_term: Arc<Mutex<Option<i64>>>,
    pub t_mark: Arc<Mutex<Option<i64>>>,
    pub t_mark_term: Arc<Mutex<Option<i64>>>,
    pub t_end: Arc<Mutex<Option<i64>>>,
    pub pause_n_s: Arc<Mutex<Option<i64>>>,
    pub heap0: Arc<Mutex<Option<u64>>>,
    pub heap1: Arc<Mutex<Option<u64>>>,
    pub heap2: Arc<Mutex<Option<u64>>>,
    pub cpu_stats: Arc<Mutex<Option<cpuStats>>>,
}

impl workType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.full.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.empty.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.__blank_3_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.wbuf_spans.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.__blank_5_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.bytes_marked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.markroot_next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.markroot_jobs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.nproc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.tstart.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.nwait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.n_data_roots.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_1 = { let __guard = self.n_b_s_s_roots.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_2 = { let __guard = self.n_span_roots.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_3 = { let __guard = self.n_stack_roots.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.base_data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_1 = { let __guard = self.base_b_s_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_2 = { let __guard = self.base_spans.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_3 = { let __guard = self.base_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_4 = { let __guard = self.base_end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = self.stack_roots.clone();
        let __go_clone_15_0 = { let __guard = self.start_sema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.mark_done_sema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.bg_mark_done.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.user_forced.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.initial_heap_live.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.assist_queue.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = { let __guard = self.sweep_waiters.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_23_0 = { let __guard = self.strong_from_weak.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_24_0 = { let __guard = self.cycles.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_25_0 = { let __guard = self.stwprocs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_25_1 = { let __guard = self.maxprocs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_0 = { let __guard = self.t_sweep_term.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_1 = { let __guard = self.t_mark.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_2 = { let __guard = self.t_mark_term.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_3 = { let __guard = self.t_end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_27_0 = { let __guard = self.pause_n_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_0 = { let __guard = self.heap0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_1 = { let __guard = self.heap1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_2 = { let __guard = self.heap2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_29_0 = { let __guard = self.cpu_stats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            full: __go_clone_0_0,
            __blank_1_0: __go_clone_1_0,
            empty: __go_clone_2_0,
            __blank_3_0: __go_clone_3_0,
            wbuf_spans: __go_clone_4_0,
            __blank_5_0: __go_clone_5_0,
            bytes_marked: __go_clone_6_0,
            markroot_next: __go_clone_7_0,
            markroot_jobs: __go_clone_8_0,
            nproc: __go_clone_9_0,
            tstart: __go_clone_10_0,
            nwait: __go_clone_11_0,
            n_data_roots: __go_clone_12_0,
            n_b_s_s_roots: __go_clone_12_1,
            n_span_roots: __go_clone_12_2,
            n_stack_roots: __go_clone_12_3,
            base_data: __go_clone_13_0,
            base_b_s_s: __go_clone_13_1,
            base_spans: __go_clone_13_2,
            base_stacks: __go_clone_13_3,
            base_end: __go_clone_13_4,
            stack_roots: __go_clone_14_0,
            start_sema: __go_clone_15_0,
            mark_done_sema: __go_clone_16_0,
            bg_mark_done: __go_clone_17_0,
            mode: __go_clone_18_0,
            user_forced: __go_clone_19_0,
            initial_heap_live: __go_clone_20_0,
            assist_queue: __go_clone_21_0,
            sweep_waiters: __go_clone_22_0,
            strong_from_weak: __go_clone_23_0,
            cycles: __go_clone_24_0,
            stwprocs: __go_clone_25_0,
            maxprocs: __go_clone_25_1,
            t_sweep_term: __go_clone_26_0,
            t_mark: __go_clone_26_1,
            t_mark_term: __go_clone_26_2,
            t_end: __go_clone_26_3,
            pause_n_s: __go_clone_27_0,
            heap0: __go_clone_28_0,
            heap1: __go_clone_28_1,
            heap2: __go_clone_28_2,
            cpu_stats: __go_clone_29_0,
        }
    }
}


impl Default for workType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(AnonymousStruct6::default())));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_12_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_12_2 = Arc::new(Mutex::new(Some(0)));
        let __go_default_12_3 = Arc::new(Mutex::new(Some(0)));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_13_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_13_2 = Arc::new(Mutex::new(Some(0)));
        let __go_default_13_3 = Arc::new(Mutex::new(Some(0)));
        let __go_default_13_4 = Arc::new(Mutex::new(Some(0)));
        let __go_default_14_0 = Arc::new(Mutex::new(None));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(gcMode(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_19_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_21_0 = Arc::new(Mutex::new(Some(AnonymousStruct7::default())));
        let __go_default_22_0 = Arc::new(Mutex::new(Some(AnonymousStruct8::default())));
        let __go_default_23_0 = Arc::new(Mutex::new(Some(AnonymousStruct9::default())));
        let __go_default_24_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_25_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_25_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_26_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_26_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_26_2 = Arc::new(Mutex::new(Some(0)));
        let __go_default_26_3 = Arc::new(Mutex::new(Some(0)));
        let __go_default_27_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_28_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_28_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_28_2 = Arc::new(Mutex::new(Some(0)));
        let __go_default_29_0 = Arc::new(Mutex::new(Some(cpuStats::default())));
        Self {
            full: __go_default_0_0,
            __blank_1_0: __go_default_1_0,
            empty: __go_default_2_0,
            __blank_3_0: __go_default_3_0,
            wbuf_spans: __go_default_4_0,
            __blank_5_0: __go_default_5_0,
            bytes_marked: __go_default_6_0,
            markroot_next: __go_default_7_0,
            markroot_jobs: __go_default_8_0,
            nproc: __go_default_9_0,
            tstart: __go_default_10_0,
            nwait: __go_default_11_0,
            n_data_roots: __go_default_12_0,
            n_b_s_s_roots: __go_default_12_1,
            n_span_roots: __go_default_12_2,
            n_stack_roots: __go_default_12_3,
            base_data: __go_default_13_0,
            base_b_s_s: __go_default_13_1,
            base_spans: __go_default_13_2,
            base_stacks: __go_default_13_3,
            base_end: __go_default_13_4,
            stack_roots: __go_default_14_0,
            start_sema: __go_default_15_0,
            mark_done_sema: __go_default_16_0,
            bg_mark_done: __go_default_17_0,
            mode: __go_default_18_0,
            user_forced: __go_default_19_0,
            initial_heap_live: __go_default_20_0,
            assist_queue: __go_default_21_0,
            sweep_waiters: __go_default_22_0,
            strong_from_weak: __go_default_23_0,
            cycles: __go_default_24_0,
            stwprocs: __go_default_25_0,
            maxprocs: __go_default_25_1,
            t_sweep_term: __go_default_26_0,
            t_mark: __go_default_26_1,
            t_mark_term: __go_default_26_2,
            t_end: __go_default_26_3,
            pause_n_s: __go_default_27_0,
            heap0: __go_default_28_0,
            heap1: __go_default_28_1,
            heap2: __go_default_28_2,
            cpu_stats: __go_default_29_0,
        }
    }
}

impl std::fmt::Display for workType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.full.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.__blank_1_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.empty.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.__blank_3_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.wbuf_spans.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.__blank_5_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.bytes_marked.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.markroot_next.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.markroot_jobs.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.nproc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.tstart.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.nwait.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.n_data_roots.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.n_b_s_s_roots.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.n_span_roots.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.n_stack_roots.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.base_data.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.base_b_s_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.base_spans.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", (*self.base_stacks.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_20 = format!("{}", (*self.base_end.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_21 = format!("{}", format_slice_wrapped(&self.stack_roots));
        let __go_fmt_22 = format!("{}", (*self.start_sema.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_23 = format!("{}", (*self.mark_done_sema.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_24 = format!("{}", (*self.bg_mark_done.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_25 = format!("{}", (*self.mode.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_26 = format!("{}", (*self.user_forced.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_27 = format!("{}", (*self.initial_heap_live.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_28 = format!("{}", (*self.assist_queue.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_29 = format!("{}", (*self.sweep_waiters.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_30 = format!("{}", (*self.strong_from_weak.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_31 = format!("{}", (*self.cycles.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_32 = format!("{}", (*self.stwprocs.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_33 = format!("{}", (*self.maxprocs.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_34 = format!("{}", (*self.t_sweep_term.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_35 = format!("{}", (*self.t_mark.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_36 = format!("{}", (*self.t_mark_term.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_37 = format!("{}", (*self.t_end.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_38 = format!("{}", (*self.pause_n_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_39 = format!("{}", (*self.heap0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_40 = format!("{}", (*self.heap1.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_41 = format!("{}", (*self.heap2.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_42 = format!("{}", (*self.cpu_stats.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
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
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22,
            __go_fmt_23,
            __go_fmt_24,
            __go_fmt_25,
            __go_fmt_26,
            __go_fmt_27,
            __go_fmt_28,
            __go_fmt_29,
            __go_fmt_30,
            __go_fmt_31,
            __go_fmt_32,
            __go_fmt_33,
            __go_fmt_34,
            __go_fmt_35,
            __go_fmt_36,
            __go_fmt_37,
            __go_fmt_38,
            __go_fmt_39,
            __go_fmt_40,
            __go_fmt_41,
            __go_fmt_42
        )
    }
}

impl GoJsonDecode for workType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// gcMode indicates how concurrent a GC cycle should be.
#[derive(Debug, Clone, Default)]
pub struct gcMode(pub Arc<Mutex<Option<i32>>>);

impl Display for gcMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for gcMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for gcMode {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for gcMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for gcMode {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<gcMode> for i32 {
    fn eq(&self, other: &gcMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<gcMode> for i32 {
    fn partial_cmp(&self, other: &gcMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for gcMode {
    type Output = gcMode;
    fn add(self, other: Self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for gcMode {
    type Output = gcMode;
    fn add(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<gcMode> for i32 {
    type Output = gcMode;
    fn add(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for gcMode {
    type Output = gcMode;
    fn sub(self, other: Self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for gcMode {
    type Output = gcMode;
    fn sub(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<gcMode> for i32 {
    type Output = gcMode;
    fn sub(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for gcMode {
    type Output = gcMode;
    fn mul(self, other: Self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for gcMode {
    type Output = gcMode;
    fn mul(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<gcMode> for i32 {
    type Output = gcMode;
    fn mul(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for gcMode {
    type Output = gcMode;
    fn div(self, other: Self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for gcMode {
    type Output = gcMode;
    fn div(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<gcMode> for i32 {
    type Output = gcMode;
    fn div(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for gcMode {
    type Output = gcMode;
    fn neg(self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for gcMode {
    type Output = gcMode;
    fn rem(self, other: Self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for gcMode {
    type Output = gcMode;
    fn rem(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<gcMode> for i32 {
    type Output = gcMode;
    fn rem(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for gcMode {
    type Output = gcMode;
    fn bitand(self, other: Self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for gcMode {
    type Output = gcMode;
    fn bitand(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<gcMode> for i32 {
    type Output = gcMode;
    fn bitand(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for gcMode {
    type Output = gcMode;
    fn bitor(self, other: Self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for gcMode {
    type Output = gcMode;
    fn bitor(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<gcMode> for i32 {
    type Output = gcMode;
    fn bitor(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for gcMode {
    type Output = gcMode;
    fn bitxor(self, other: Self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for gcMode {
    type Output = gcMode;
    fn bitxor(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<gcMode> for i32 {
    type Output = gcMode;
    fn bitxor(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for gcMode {
    type Output = gcMode;
    fn not(self) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for gcMode {
    type Output = gcMode;
    fn shl(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for gcMode {
    type Output = gcMode;
    fn shl(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for gcMode {
    type Output = gcMode;
    fn shl(self, other: i8) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for gcMode {
    type Output = gcMode;
    fn shl(self, other: i16) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for gcMode {
    type Output = gcMode;
    fn shl(self, other: i64) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for gcMode {
    type Output = gcMode;
    fn shl(self, other: u32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for gcMode {
    type Output = gcMode;
    fn shl(self, other: u8) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for gcMode {
    type Output = gcMode;
    fn shl(self, other: u16) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for gcMode {
    type Output = gcMode;
    fn shl(self, other: u64) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for gcMode {
    type Output = gcMode;
    fn shl(self, other: usize) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for gcMode {
    type Output = gcMode;
    fn shr(self, other: gcMode) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for gcMode {
    type Output = gcMode;
    fn shr(self, other: i32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for gcMode {
    type Output = gcMode;
    fn shr(self, other: i8) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for gcMode {
    type Output = gcMode;
    fn shr(self, other: i16) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for gcMode {
    type Output = gcMode;
    fn shr(self, other: i64) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for gcMode {
    type Output = gcMode;
    fn shr(self, other: u32) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for gcMode {
    type Output = gcMode;
    fn shr(self, other: u8) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for gcMode {
    type Output = gcMode;
    fn shr(self, other: u16) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for gcMode {
    type Output = gcMode;
    fn shr(self, other: u64) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for gcMode {
    type Output = gcMode;
    fn shr(self, other: usize) -> gcMode {
        gcMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for gcMode {}

impl Ord for gcMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A gcTrigger is a predicate for starting a GC cycle. Specifically,
/// it is an exit condition for the _GCoff phase.
#[derive(Debug, Clone)]
pub struct gcTrigger {
    pub kind: Arc<Mutex<Option<gcTriggerKind>>>,
    pub now: Arc<Mutex<Option<i64>>>,
    pub n: Arc<Mutex<Option<u32>>>,
}

impl gcTrigger {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.now.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            kind: __go_clone_0_0,
            now: __go_clone_1_0,
            n: __go_clone_2_0,
        }
    }
}


impl Default for gcTrigger {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(gcTriggerKind(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            kind: __go_default_0_0,
            now: __go_default_1_0,
            n: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for gcTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.kind.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.now.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for gcTrigger {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct gcTriggerKind(pub Arc<Mutex<Option<i32>>>);

impl Display for gcTriggerKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for gcTriggerKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for gcTriggerKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for gcTriggerKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for gcTriggerKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<gcTriggerKind> for i32 {
    fn eq(&self, other: &gcTriggerKind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<gcTriggerKind> for i32 {
    fn partial_cmp(&self, other: &gcTriggerKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for gcTriggerKind {
    type Output = gcTriggerKind;
    fn add(self, other: Self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn add(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<gcTriggerKind> for i32 {
    type Output = gcTriggerKind;
    fn add(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for gcTriggerKind {
    type Output = gcTriggerKind;
    fn sub(self, other: Self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn sub(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<gcTriggerKind> for i32 {
    type Output = gcTriggerKind;
    fn sub(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for gcTriggerKind {
    type Output = gcTriggerKind;
    fn mul(self, other: Self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn mul(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<gcTriggerKind> for i32 {
    type Output = gcTriggerKind;
    fn mul(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for gcTriggerKind {
    type Output = gcTriggerKind;
    fn div(self, other: Self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn div(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<gcTriggerKind> for i32 {
    type Output = gcTriggerKind;
    fn div(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for gcTriggerKind {
    type Output = gcTriggerKind;
    fn neg(self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for gcTriggerKind {
    type Output = gcTriggerKind;
    fn rem(self, other: Self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn rem(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<gcTriggerKind> for i32 {
    type Output = gcTriggerKind;
    fn rem(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for gcTriggerKind {
    type Output = gcTriggerKind;
    fn bitand(self, other: Self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn bitand(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<gcTriggerKind> for i32 {
    type Output = gcTriggerKind;
    fn bitand(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for gcTriggerKind {
    type Output = gcTriggerKind;
    fn bitor(self, other: Self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn bitor(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<gcTriggerKind> for i32 {
    type Output = gcTriggerKind;
    fn bitor(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for gcTriggerKind {
    type Output = gcTriggerKind;
    fn bitxor(self, other: Self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn bitxor(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<gcTriggerKind> for i32 {
    type Output = gcTriggerKind;
    fn bitxor(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for gcTriggerKind {
    type Output = gcTriggerKind;
    fn not(self) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: i8) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: i16) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: i64) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: u32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: u8) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: u16) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: u64) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shl(self, other: usize) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: gcTriggerKind) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: i32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: i8) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: i16) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: i64) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: u32) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: u8) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: u16) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: u64) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for gcTriggerKind {
    type Output = gcTriggerKind;
    fn shr(self, other: usize) -> gcTriggerKind {
        gcTriggerKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for gcTriggerKind {}

impl Ord for gcTriggerKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// gcBgMarkWorkerNode is an entry in the gcBgMarkWorkerPool. It points to a single
/// gcBgMarkWorker goroutine.
#[derive(Debug, Clone)]
pub struct gcBgMarkWorkerNode {
    pub node: Arc<Mutex<Option<lfnode>>>,
    pub gp: Arc<Mutex<Option<guintptr>>>,
    pub m: Arc<Mutex<Option<muintptr>>>,
}

impl gcBgMarkWorkerNode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.node.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.gp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            node: __go_clone_0_0,
            gp: __go_clone_1_0,
            m: __go_clone_2_0,
        }
    }
}


impl Default for gcBgMarkWorkerNode {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(lfnode::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0)))))));
        Self {
            node: __go_default_0_0,
            gp: __go_default_1_0,
            m: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for gcBgMarkWorkerNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.node.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.gp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.m.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for gcBgMarkWorkerNode {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct6 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: Arc<Mutex<Option<mSpanList>>>,
    pub busy: Arc<Mutex<Option<mSpanList>>>,
}
impl AnonymousStruct6 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.busy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            free: __go_clone_1_0,
            busy: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(mSpanList::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(mSpanList::default())));
        Self {
            lock: __go_default_0_0,
            free: __go_default_1_0,
            busy: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.free.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.busy.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for AnonymousStruct6 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct7 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub q: Arc<Mutex<Option<gQueue>>>,
}
impl AnonymousStruct7 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            q: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(gQueue::default())));
        Self {
            lock: __go_default_0_0,
            q: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.q.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct7 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct8 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub list: Arc<Mutex<Option<gList>>>,
}
impl AnonymousStruct8 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            list: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct8 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(gList::default())));
        Self {
            lock: __go_default_0_0,
            list: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.list.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct9 {
    pub block: Arc<Mutex<Option<bool>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub q: Arc<Mutex<Option<gQueue>>>,
}
impl AnonymousStruct9 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.block.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            block: __go_clone_0_0,
            lock: __go_clone_1_0,
            q: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct9 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(gQueue::default())));
        Self {
            block: __go_default_0_0,
            lock: __go_default_1_0,
            q: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.block.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.q.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for AnonymousStruct9 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static gcphase: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static writeBarrier: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct10>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcBlackenEnabled: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcMarkWorkerModeStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 4]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static work: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<workType>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcMarkDoneFlushed: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcDebugMarkDone: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct11>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static poolcleanup: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static boringCaches: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<usize>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static uniqueMapCleanup: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoChannel<AnonymousStruct12>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *gcphase.lock().unwrap() = Some(0);
    *writeBarrier.lock().unwrap() = Some(Default::default());
    *gcBlackenEnabled.lock().unwrap() = Some(0);
    *gcMarkWorkerModeStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *work.lock().unwrap() = Some(Default::default());
    *gcMarkDoneFlushed.lock().unwrap() = Some(0);
    *gcDebugMarkDone.lock().unwrap() = Some(Default::default());
    *boringCaches.lock().unwrap() = Some(vec![]);
    *uniqueMapCleanup.lock().unwrap() = Some(Default::default());
    {
        let mut __go_array = Vec::<String>::with_capacity(4);
        __go_array.push("Not worker".to_string());
        __go_array.push("GC (dedicated)".to_string());
        __go_array.push("GC (fractional)".to_string());
        __go_array.push("GC (idle)".to_string());
        let __go_array: [String; 4] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *gcMarkWorkerModeStrings.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *gcphase.lock().unwrap() = Some(0);
    *writeBarrier.lock().unwrap() = Some(Default::default());
    *gcBlackenEnabled.lock().unwrap() = Some(0);
    *gcMarkWorkerModeStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *work.lock().unwrap() = Some(Default::default());
    *gcMarkDoneFlushed.lock().unwrap() = Some(0);
    *gcDebugMarkDone.lock().unwrap() = Some(Default::default());
    *boringCaches.lock().unwrap() = Some(vec![]);
    *uniqueMapCleanup.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_27() {
    {
        let mut __go_array = Vec::<String>::with_capacity(4);
        __go_array.push("Not worker".to_string());
        __go_array.push("GC (dedicated)".to_string());
        __go_array.push("GC (fractional)".to_string());
        __go_array.push("GC (idle)".to_string());
        let __go_array: [String; 4] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *gcMarkWorkerModeStrings.lock().unwrap() = Some(__go_array);
    }
}


impl gcTrigger {
    /// test reports whether the trigger condition is satisfied, meaning
    /// that the exit condition for the _GCoff phase has been met. The exit
    /// condition should be tested when allocating.
    pub fn test(&self) -> bool {
        if !(*{ let __field = (*memstats.lock().unwrap().as_ref().unwrap()).enablegc.clone(); __field }.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } || { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x != __tmp_y } {
        return false;
    }
        { let _switch_val = { let __selector_holder = self.kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_HEAP as i32))))) {
            let (mut trigger, _) = (*gcController.lock().unwrap().as_ref().unwrap()).trigger();
            return { let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_live.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = trigger; __tmp_x >= __tmp_y };
        } else if _switch_val == (gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_TIME as i32))))) {
            if { let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).gc_percent.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        return false;
    }
            let mut lastgc = Arc::new(Mutex::new(Some(internal_runtime_atomic::load64((*memstats.lock().unwrap().as_ref().unwrap()).last_gc_nanotime.clone()) as i64)));
            return { let __tmp_x = { let __v = (*lastgc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && { let __tmp_x = { let __tmp_x = (*self.now.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*lastgc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = (*forcegcperiod.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y };
        } else if _switch_val == (gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_CYCLE as i32))))) {
                        // t.n > work.cycles, but accounting for wraparound.
            return {
                let __tmp_x = (*Arc::new(Mutex::new(Some(({
                    let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*(*work.lock().unwrap().as_ref().unwrap()).cycles.lock().unwrap().as_mut().unwrap()).load();
                    __tmp_x - __tmp_y
                }) as i32))).lock().unwrap().as_ref().unwrap());
                let __tmp_y = 0 as i32;
                __tmp_x > __tmp_y
            };
        }
    }
                // t.n > work.cycles, but accounting for wraparound.
        true
    }
}

impl workType {
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

///go:nosplit
pub fn set_g_c_phase(x: Arc<Mutex<Option<u32>>>) {
    internal_runtime_atomic::store(gcphase.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARK as u32; __tmp_x == __tmp_y } || { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARKTERMINATION as u32; __tmp_x == __tmp_y }; *(*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.lock().unwrap() = Some(new_val); };
}

/// pollFractionalWorkerExit reports whether a fractional mark worker
/// should self-preempt. It assumes it is called from the fractional
/// worker.
pub fn poll_fractional_worker_exit() -> bool {
        // This should be kept in sync with the fractional worker
        // scheduler logic in findRunnableGCWorker.
    let mut now = nanotime();
    let mut delta = Arc::new(Mutex::new(Some({ let __tmp_x = now; let __tmp_y = (*{ let __field = (*gcController.lock().unwrap().as_ref().unwrap()).mark_start_time.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return true;
    }
    let mut p: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    let mut selfTime = Arc::new(Mutex::new(Some({
        let __tmp_x = (*{ let __ptr_value = p.borrow(); __ptr_value.as_ref().unwrap().gc_fractional_mark_time.clone() }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = ({ let __tmp_x = now; let __tmp_y = (*{ let __ptr_value = p.borrow(); __ptr_value.as_ref().unwrap().gc_mark_worker_start_time.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y });
        __tmp_x + __tmp_y
    })));

        // Add some slack to the utilization goal so that the
        // fractional worker isn't behind again the instant it exits.
    return {
        let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*selfTime.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*delta.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y };
        let __tmp_y = { let __tmp_x = 1.2; let __tmp_y = (*{ let __field = (*gcController.lock().unwrap().as_ref().unwrap()).fractional_utilization_goal.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
        __tmp_x > __tmp_y
    };
}

/// gcStart starts the GC. It transitions from _GCoff to _GCmark (if
/// debug.gcstoptheworld == 0) or performs all of GC (if
/// debug.gcstoptheworld != 0).
///
/// This may return without performing this transition in some cases,
/// such as when called on a system stack or with locks held.
pub fn gc_start(trigger: Arc<Mutex<Option<gcTrigger>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Since this is called from malloc and malloc is called in
                // the guts of a number of libraries that might be holding
                // locks, don't attempt to start GC in non-preemptible or
                // potentially unstable situations.
        let mut mp = acquirem();
        {
        let mut gp = getg();;
        if { let __left = gp.clone(); let __right = (*mp.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } || { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x > __tmp_y } || { let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            releasem(GoPtr::local(mp.clone()));;
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    };
        }
    }
        releasem(GoPtr::local(mp.clone()));
        *mp.lock().unwrap() = None;

        {
        let mut gp = getg();;
        if { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
            let mut sg = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone();;
            *(*gp.lock().unwrap().as_ref().unwrap()).sync_group.lock().unwrap() = None;;
            let gp_defer_captured = gp.clone(); let sg_defer_captured = sg.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = sg_defer_captured.clone(); (*gp_defer_captured.lock().unwrap().as_mut().unwrap()).sync_group = new_val; };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));;
        }
    }

                // Disassociate the G from its synctest bubble while allocating.
                // This is less elegant than incrementing the group's active count,
                // but avoids any contamination between GC and synctest.
                // Pick up the remaining unswept/not being swept spans concurrently
                //
                // This shouldn't happen if we're being invoked in background
                // mode since proportional sweep should have just finished
                // sweeping everything, but rounding errors, etc, may leave a
                // few spans unswept. In forced mode, this is necessary since
                // GC can be forced at any point in the sweeping cycle.
                //
                // We check the transition condition continuously here in case
                // this G gets delayed in to the next GC cycle.
        while (*trigger.lock().unwrap().as_ref().unwrap()).test() && { let __tmp_x = sweepone(); let __tmp_y = !(0 as usize) as usize; __tmp_x != __tmp_y } {
    }

                // Perform GC initialization and the sweep termination
                // transition.
        semacquire(GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).start_sema.clone()));

                // Re-check transition condition under transition lock.
        if !(*trigger.lock().unwrap().as_ref().unwrap()).test() {
        semrelease(GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).start_sema.clone()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

                // In gcstoptheworld debug mode, upgrade the mode accordingly.
                // We do this after re-checking the transition condition so
                // that multiple goroutines that detect the heap trigger don't
                // start multiple STW GCs.
        let mut mode = Arc::new(Mutex::new(Some(gcMode(Arc::new(Mutex::new(Some(GC_BACKGROUND_MODE as i32)))))));
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gcstoptheworld.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x == __tmp_y } {
        { let new_val = gcMode(Arc::new(Mutex::new(Some(GC_FORCE_MODE as i32)))); *mode.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gcstoptheworld.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as i32; __tmp_x == __tmp_y } {
        { let new_val = gcMode(Arc::new(Mutex::new(Some(GC_FORCE_BLOCK_MODE as i32)))); *mode.lock().unwrap() = Some(new_val); };
    }

                // Ok, we're doing it! Stop everybody else
        semacquire(GoPtr::local(gcsema.clone()));
        semacquire(GoPtr::local(worldsema.clone()));

                // For stats, check if this GC was forced by the user.
                // Update it under gcsema to avoid gctrace getting wrong values.
        { let new_val = {
            let __tmp_x = { let __selector_holder = (*trigger.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_CYCLE as i32))));
            __tmp_x == __tmp_y
        }; *(*work.lock().unwrap().as_ref().unwrap()).user_forced.lock().unwrap() = Some(new_val); };

        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_start();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

                // Check that all Ps have finished deferred mcache flushes.
        { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        {
        let mut fg = (*{ let __ptr_value = (*p.lock().unwrap().as_ref().unwrap()).mcache.with_mut(|__ptr_value| __ptr_value.flush_gen.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load();;
        if { let __tmp_x = fg; let __tmp_y = (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
            {
            let __go_print_arg_0 = format!("{}", "runtime: p".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).id.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "flushGen".to_string());
            let __go_print_arg_3 = format!("{}", fg);
            let __go_print_arg_4 = format!("{}", "!= sweepgen".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5)
        };;
            throw(Arc::new(Mutex::new(Some("p mcache not flushed".to_string()))));;
        }
    }
    } }

        gc_bg_mark_start_workers();

        systemstack(Arc::new(Mutex::new(Some(Box::new(move || { gc_reset_mark_state() }) as Box<dyn FnMut() -> () + Send + Sync>))));

        {
            let __tmp_0 = (*gomaxprocs.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = (*gomaxprocs.lock().unwrap().as_ref().unwrap()).clone();
            *(*work.lock().unwrap().as_ref().unwrap()).stwprocs.lock().unwrap() = Some(__tmp_0);
            *(*work.lock().unwrap().as_ref().unwrap()).maxprocs.lock().unwrap() = Some(__tmp_1);
        };
        if { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).stwprocs.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*ncpu.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
                // This is used to compute CPU time of the STW phases,
                // so it can't be more than ncpu, even if GOMAXPROCS is.
        { let new_val = ncpu.lock().unwrap().as_ref().unwrap().clone(); *(*work.lock().unwrap().as_ref().unwrap()).stwprocs.lock().unwrap() = Some(new_val); };
    }
                // This is used to compute CPU time of the STW phases,
                // so it can't be more than ncpu, even if GOMAXPROCS is.
        { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_live.lock().unwrap().as_mut().unwrap()).load(); *(*work.lock().unwrap().as_ref().unwrap()).heap0.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as i64; *(*work.lock().unwrap().as_ref().unwrap()).pause_n_s.lock().unwrap() = Some(new_val); };
        { let new_val = mode.lock().unwrap().as_ref().unwrap().clone(); *(*work.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };

        let mut now = nanotime();
        { let new_val = now; *(*work.lock().unwrap().as_ref().unwrap()).t_sweep_term.lock().unwrap() = Some(new_val); };
        let mut stw: Arc<Mutex<Option<worldStop>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut stw_closure_clone = stw.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = stop_the_world_with_sema(Arc::new(Mutex::new(Some(crate::proc::stwReason(Arc::new(Mutex::new(Some(STW_G_C_SWEEP_TERM as u8)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *stw_closure_clone.lock().unwrap() = __moved_val; };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

                // Accumulate fine-grained stopping time.
        (*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_mut().unwrap()).accumulate_g_c_pause_time(Arc::new(Mutex::new(Some({ let __selector_holder = (*stw.lock().unwrap().as_ref().unwrap()).stopping_c_p_u_time.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(1 as i32))));

                // Finish sweep before we start concurrent scan.
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        finishsweep_m();
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

                // clearpools before we start the GC. If we wait the memory will not be
                // reclaimed until the next GC cycle.
        clearpools();

        (*(*work.lock().unwrap().as_ref().unwrap()).cycles.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));

                // Assists and workers can start the moment we start
                // the world.
        (*gcController.lock().unwrap().as_mut().unwrap()).start_cycle(
            Arc::new(Mutex::new(Some(now))),
            Arc::new(Mutex::new(Some((*gomaxprocs.lock().unwrap().as_ref().unwrap()) as i32))),
            Arc::new(Mutex::new(Some({ let __arg_holder = trigger.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        );

                // Notify the CPU limiter that assists may begin.
        (*gcCPULimiter.lock().unwrap().as_mut().unwrap()).start_g_c_transition(Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(now))));

                // In STW mode, disable scheduling of user Gs. This may also
                // disable scheduling of this goroutine, so it may block as
                // soon as we start the world again.
        if { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = gcMode(Arc::new(Mutex::new(Some(GC_BACKGROUND_MODE as i32)))); __tmp_x != __tmp_y } {
        sched_enable_user(Arc::new(Mutex::new(Some(false))));
    }

                // Enter concurrent mark phase and enable
                // write barriers.
                //
                // Because the world is stopped, all Ps will
                // observe that write barriers are enabled by
                // the time we start the world and begin
                // scanning.
                //
                // Write barriers must be enabled before assists are
                // enabled because they must be enabled before
                // any non-leaf heap objects are marked. Since
                // allocations are blocked until assists can
                // happen, we want to enable assists as early as
                // possible.
        set_g_c_phase(Arc::new(Mutex::new(Some(__G_CMARK as u32))));

        gc_bg_mark_prepare();
        gc_mark_root_prepare();

                // Mark all active tinyalloc blocks. Since we're
                // allocating from these, they need to be black like
                // other allocations. The alternative is to blacken
                // the tiny block on every allocation from it, which
                // would slow down the tiny allocator.
        gc_mark_tiny_allocs();

                // At this point all Ps have enabled the write
                // barrier, thus maintaining the no white to
                // black invariant. Enable mutator assists to
                // put back-pressure on fast allocating
                // mutators.
        internal_runtime_atomic::store(gcBlackenEnabled.clone(), Arc::new(Mutex::new(Some(1 as u32))));

                // In STW mode, we could block the instant systemstack
                // returns, so make sure we're not preemptible.
        { let new_val = acquirem().clone(); mp = new_val; };

                // Update the CPU stats pause time.
                //
                // Use maxprocs instead of stwprocs here because the total time
                // computed in the CPU stats is based on maxprocs, and we want them
                // to be comparable.
        (*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_mut().unwrap()).accumulate_g_c_pause_time(
            Arc::new(Mutex::new(Some({ let __tmp_x = nanotime(); let __tmp_y = (*{ let __field = (*stw.lock().unwrap().as_ref().unwrap()).finished_stopping.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).maxprocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
        );

                // Concurrent mark.
        let mut now_closure_clone = now.clone(); let stw_closure_clone = stw.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = start_the_world_with_sema(Arc::new(Mutex::new(Some(0 as i64))), Arc::new(Mutex::new(Some({ let __arg_holder = stw_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); now = new_val; };
        { let __target = (*work.lock().unwrap().as_ref().unwrap()).pause_n_s.clone(); let __rhs = { let __tmp_x = now_closure_clone; let __tmp_y = (*{ let __field = (*stw_closure_clone.lock().unwrap().as_ref().unwrap()).started_stopping.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = now_closure_clone; *(*work.lock().unwrap().as_ref().unwrap()).t_mark.lock().unwrap() = Some(new_val); };
        (*gcCPULimiter.lock().unwrap().as_mut().unwrap()).finish_g_c_transition(Arc::new(Mutex::new(Some(now_closure_clone))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

                // Release the CPU limiter.
                // Release the world sema before Gosched() in STW mode
                // because we will need to reacquire it later but before
                // this goroutine becomes runnable again, and we could
                // self-deadlock otherwise.
        semrelease(GoPtr::local(worldsema.clone()));
        releasem(GoPtr::local(mp.clone()));

                // Make sure we block instead of returning to user code
                // in STW mode.
        if { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = gcMode(Arc::new(Mutex::new(Some(GC_BACKGROUND_MODE as i32)))); __tmp_x != __tmp_y } {
        gosched();
    }

        semrelease(GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).start_sema.clone()));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
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
            ()
        }
    }
}

/// gcMarkDone transitions the GC from mark to mark termination if all
/// reachable objects have been marked (that is, there are no grey
/// objects and can be no more in the future). Otherwise, it flushes
/// all local work to the global queues where it can be discovered by
/// other workers.
///
/// This should be called when all local mark work has been drained and
/// there are no remaining workers. Specifically, when
///
///	work.nwait == work.nproc && !gcMarkWorkAvailable(p)
///
/// The calling context must be preemptible.
///
/// Flushing local work is important because idle Ps may have local
/// work queued. This is the only way to make that work visible and
/// drive GC to completion.
///
/// It is explicitly okay to have write barriers in this function. If
/// it does transition to mark termination, then all reachable objects
/// have been marked, so the write barrier cannot shade any more
/// objects.
pub fn gc_mark_done() {
        // Ensure only one thread is running the ragged barrier at a
        // time.
    semacquire(GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).mark_done_sema.clone()));

    'top: loop {
                // Re-check transition condition under transition lock.
                //
                // It's critical that this checks the global work queues are
                // empty before performing the ragged barrier. Otherwise,
                // there could be global work that a P could take after the P
                // has passed the ragged barrier.
        if !({ let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARK as u32; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nwait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && !gc_mark_work_available(GoPtr::nil())) {
        semrelease(GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).mark_done_sema.clone()));
        return;
    }

                // forEachP needs worldsema to execute, and we'll need it to
                // stop the world later, so acquire worldsema now.
        semacquire(GoPtr::local(worldsema.clone()));

                // Prevent weak->strong conversions from generating additional
                // GC work. forEachP will guarantee that it is observed globally.
        { let new_val = true; *(*(*work.lock().unwrap().as_ref().unwrap()).strong_from_weak.lock().unwrap().as_ref().unwrap()).block.lock().unwrap() = Some(new_val); };

                // Flush all local buffers and collect flushedWork flags.
        { let new_val = 0 as u32; *gcMarkDoneFlushed.lock().unwrap() = Some(new_val); };
        for_each_p(
            Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_G_C_MARK_TERMINATION as u8))))))),
            Arc::new(Mutex::new(Some(Box::new(move |pp: GoPtr<crate::runtime2::p>| {
        wb_buf_flush1(pp.clone());
        (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).dispose();
        if (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).flushed_work.lock().unwrap().as_ref().unwrap()) {
        internal_runtime_atomic::xadd(internal_runtime_atomic::GoPtr::local(gcMarkDoneFlushed.clone()), Arc::new(Mutex::new(Some(1 as i32))));
        { let new_val = false; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).flushed_work.lock().unwrap() = Some(new_val); };
    }
    }) as Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync>)))
        );

                // Flush the write barrier buffer, since this may add
                // work to the gcWork.
                // Flush the gcWork, since this may create global work
                // and set the flushedWork flag.
                //
                // TODO(austin): Break up these workbufs to
                // better distribute work.
                // Collect the flushedWork flag.
        if { let __tmp_x = (*gcMarkDoneFlushed.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
                // More grey objects were discovered since the
                // previous termination check, so there may be more
                // work to do. Keep going. It's possible the
                // transition condition became true again during the
                // ragged barrier, so re-check it.
        semrelease(GoPtr::local(worldsema.clone()));
        continue 'top;
    }

                // More grey objects were discovered since the
                // previous termination check, so there may be more
                // work to do. Keep going. It's possible the
                // transition condition became true again during the
                // ragged barrier, so re-check it.
                // For debugging/testing.
        while (*(*gcDebugMarkDone.lock().unwrap().as_ref().unwrap()).spin_after_ragged_barrier.lock().unwrap().as_ref().unwrap()).load() {
    }

                // There was no global work, no local work, and no Ps
                // communicated work since we took markDoneSema. Therefore
                // there are no grey objects and no more objects can be
                // shaded. Transition to mark termination.
        let mut now = nanotime();
        { let new_val = now; *(*work.lock().unwrap().as_ref().unwrap()).t_mark_term.lock().unwrap() = Some(new_val); };
        { let new_val = "gcing".to_string(); *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).preemptoff.lock().unwrap() = Some(new_val); };
        let mut stw: Arc<Mutex<Option<worldStop>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut stw_closure_clone = stw.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = stop_the_world_with_sema(Arc::new(Mutex::new(Some(crate::proc::stwReason(Arc::new(Mutex::new(Some(STW_G_C_MARK_TERM as u8)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *stw_closure_clone.lock().unwrap() = __moved_val; };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

                // The gcphase is _GCmark, it will transition to _GCmarktermination
                // below. The important thing is that the wb remains active until
                // all marking is complete. This includes writes made by the GC.
                // Accumulate fine-grained stopping time.
        (*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_mut().unwrap()).accumulate_g_c_pause_time(Arc::new(Mutex::new(Some({ let __selector_holder = (*stw.lock().unwrap().as_ref().unwrap()).stopping_c_p_u_time.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(1 as i32))));

                // There is sometimes work left over when we enter mark termination due
                // to write barriers performed after the completion barrier above.
                // Detect this and resume concurrent mark. This is obviously
                // unfortunate.
                //
                // See issue #27993 for details.
                //
                // Switch to the system stack to call wbBufFlush1, though in this case
                // it doesn't matter because we're non-preemptible anyway.
        let mut restart = Arc::new(Mutex::new(Some(false)));
        let mut restart_closure_clone = restart.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        wb_buf_flush1(GoPtr::local(p.clone()));
        if !(*(*p.lock().unwrap().as_ref().unwrap()).gcw.lock().unwrap().as_ref().unwrap()).empty() {
        { let new_val = true; *restart_closure_clone.lock().unwrap() = Some(new_val); };
        break
    }
    } }
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if { let __v = (*restart.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = true; *(*gcDebugMarkDone.lock().unwrap().as_ref().unwrap()).restarted_due_to27993.lock().unwrap() = Some(new_val); };
        { let new_val = "".to_string(); *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).preemptoff.lock().unwrap() = Some(new_val); };
        let stw_closure_clone = stw.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        (*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_mut().unwrap()).accumulate_g_c_pause_time(
            Arc::new(Mutex::new(Some({ let __tmp_x = nanotime(); let __tmp_y = (*{ let __field = (*stw_closure_clone.lock().unwrap().as_ref().unwrap()).finished_stopping.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).maxprocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
        );
        let mut now = start_the_world_with_sema(Arc::new(Mutex::new(Some(0 as i64))), Arc::new(Mutex::new(Some({ let __arg_holder = stw_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __target = (*work.lock().unwrap().as_ref().unwrap()).pause_n_s.clone(); let __rhs = { let __tmp_x = now; let __tmp_y = (*{ let __field = (*stw_closure_clone.lock().unwrap().as_ref().unwrap()).started_stopping.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
                // Accumulate the time we were stopped before we had to start again.
                // Start the world again.
        semrelease(GoPtr::local(worldsema.clone()));
        continue 'top;
    }

                // Accumulate the time we were stopped before we had to start again.
                // Start the world again.
        gc_compute_starting_stack_size();

                // Disable assists and background workers. We must do
                // this before waking blocked assists.
        internal_runtime_atomic::store(gcBlackenEnabled.clone(), Arc::new(Mutex::new(Some(0 as u32))));

                // Notify the CPU limiter that GC assists will now cease.
        (*gcCPULimiter.lock().unwrap().as_mut().unwrap()).start_g_c_transition(Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(now))));

                // Wake all blocked assists. These will run when we
                // start the world again.
        gc_wake_all_assists();

                // Wake all blocked weak->strong conversions. These will run
                // when we start the world again.
        { let new_val = false; *(*(*work.lock().unwrap().as_ref().unwrap()).strong_from_weak.lock().unwrap().as_ref().unwrap()).block.lock().unwrap() = Some(new_val); };
        gc_wake_all_strong_from_weak();

                // Likewise, release the transition lock. Blocked
                // workers and assists will run when we start the
                // world again.
        semrelease(GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).mark_done_sema.clone()));

                // In STW mode, re-enable user goroutines. These will be
                // queued to run after we start the world.
        sched_enable_user(Arc::new(Mutex::new(Some(true))));

                // endCycle depends on all gcWork cache stats being flushed.
                // The termination algorithm above ensured that up to
                // allocations since the ragged barrier.
        (*gcController.lock().unwrap().as_mut().unwrap()).end_cycle(
            Arc::new(Mutex::new(Some(now))),
            Arc::new(Mutex::new(Some((*gomaxprocs.lock().unwrap().as_ref().unwrap()) as i32))),
            Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).user_forced.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
        );

                // Perform mark termination. This will restart the world.
        gc_mark_termination(Arc::new(Mutex::new(Some({ let __arg_holder = stw.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        break 'top;
    };
}

/// World must be stopped and mark assists and background workers must be
/// disabled.
pub fn gc_mark_termination(stw: Arc<Mutex<Option<worldStop>>>) {
        // Start marktermination (write barrier remains enabled for now).
    set_g_c_phase(Arc::new(Mutex::new(Some(__G_CMARKTERMINATION as u32))));

    { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_live.lock().unwrap().as_mut().unwrap()).load(); *(*work.lock().unwrap().as_ref().unwrap()).heap1.lock().unwrap() = Some(new_val); };
    let mut startTime = nanotime();

    let mut mp = acquirem();
    { let new_val = "gcing".to_string(); *(*mp.lock().unwrap().as_ref().unwrap()).preemptoff.lock().unwrap() = Some(new_val); };
    { let new_val = 2 as u8; *(*mp.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap() = Some(new_val); };
    let mut curgp: GoPtr<crate::runtime2::g> = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone();

        // N.B. The execution tracer is not aware of this status
        // transition and handles it specially based on the
        // wait reason.
    cas_g_to_waiting_for_suspend_g(curgp.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_GARBAGE_COLLECTION as u8))))))));

        // Run gc on the g0 stack. We do this so that the g stack
        // we're currently running on will no longer change. Cuts
        // the root set down a bit (g0 stacks are not scanned, and
        // we don't need to scan gc's internal state).  We also
        // need to switch to g0 so we can shrink the stack.
    let startTime_closure_clone = startTime.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        gc_mark(Arc::new(Mutex::new(Some(startTime_closure_clone))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Must return immediately.
        // The outer function's stack may have moved
        // during gcMark (it shrinks stacks, including the
        // outer function's stack), so we must not refer
        // to any of its variables. Return back to the
        // non-system stack to pick up the new addresses
        // before continuing.
    let mut stwSwept: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut stwSwept_closure_clone = stwSwept.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).bytes_marked.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*work.lock().unwrap().as_ref().unwrap()).heap2.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gccheckmark.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        start_checkmarks();
        gc_reset_mark_state();
        let mut gcw = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().gcw.clone() }.clone();
        gc_drain(gcw.clone(), Arc::new(Mutex::new(Some(crate::mgcmark::gcDrainFlags(Arc::new(Mutex::new(Some(0 as i32))))))));
        wb_buf_flush1(crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())));
        { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.dispose(); __result };
        end_checkmarks();
    }
        set_g_c_phase(Arc::new(Mutex::new(Some(__G_COFF as u32))));
        { let new_val = gc_sweep(Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); *stwSwept_closure_clone.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Run a full non-parallel, stop-the-world
        // mark using checkmark bits, to check that we
        // didn't forget to mark anything during the
        // concurrent mark process.
        // marking is complete so we can turn the write barrier off
    { let new_val = 0 as u8; *(*mp.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap() = Some(new_val); };
    casgstatus(curgp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));

    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_done();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // all done
    { let new_val = "".to_string(); *(*mp.lock().unwrap().as_ref().unwrap()).preemptoff.lock().unwrap() = Some(new_val); };

    if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("gc done but gcphase != _GCoff".to_string()))));
    }

        // Record heapInUse for scavenger.
    { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_in_use.lock().unwrap().as_ref().unwrap()).load(); *(*memstats.lock().unwrap().as_ref().unwrap()).last_heap_in_use.lock().unwrap() = Some(new_val); };

        // Update GC trigger and pacing, as well as downstream consumers
        // of this pacing information, for the next cycle.
    systemstack(Arc::new(Mutex::new(Some(Box::new(move || { gc_controller_commit() }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Update timing memstats
    let mut now = nanotime();
    let (mut sec, mut nsec, _) = time_now();
    let mut unixNow = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = sec; let __tmp_y = 1e9 as i64; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(nsec as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
    { let __target = (*work.lock().unwrap().as_ref().unwrap()).pause_n_s.clone(); let __rhs = { let __tmp_x = now; let __tmp_y = (*{ let __field = (*stw.lock().unwrap().as_ref().unwrap()).started_stopping.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let new_val = now; *(*work.lock().unwrap().as_ref().unwrap()).t_end.lock().unwrap() = Some(new_val); };
    internal_runtime_atomic::store64((*memstats.lock().unwrap().as_ref().unwrap()).last_gc_unix.clone(), Arc::new(Mutex::new(Some((*unixNow.lock().unwrap().as_ref().unwrap()) as u64))));
    internal_runtime_atomic::store64((*memstats.lock().unwrap().as_ref().unwrap()).last_gc_nanotime.clone(), Arc::new(Mutex::new(Some(now as u64))));
    (*(*memstats.lock().unwrap().as_ref().unwrap()).pause_ns.lock().unwrap().as_mut().unwrap())[({
        let __tmp_x = (*{ let __field = (*memstats.lock().unwrap().as_ref().unwrap()).numgc.clone(); __field }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = (*Arc::new(Mutex::new(Some((*(*memstats.lock().unwrap().as_ref().unwrap()).pause_ns.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
        __tmp_x % __tmp_y
    }) as usize] = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).pause_n_s.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()).clone();
    (*(*memstats.lock().unwrap().as_ref().unwrap()).pause_end.lock().unwrap().as_mut().unwrap())[({
        let __tmp_x = (*{ let __field = (*memstats.lock().unwrap().as_ref().unwrap()).numgc.clone(); __field }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = (*Arc::new(Mutex::new(Some((*(*memstats.lock().unwrap().as_ref().unwrap()).pause_end.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
        __tmp_x % __tmp_y
    }) as usize] = (*Arc::new(Mutex::new(Some((*unixNow.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()).clone();
    { let __target = (*memstats.lock().unwrap().as_ref().unwrap()).pause_total_ns.clone(); let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).pause_n_s.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

        // Accumulate CPU stats.
        //
        // Use maxprocs instead of stwprocs for GC pause time because the total time
        // computed in the CPU stats is based on maxprocs, and we want them to be
        // comparable.
        //
        // Pass gcMarkPhase=true to accumulate so we can get all the latest GC CPU stats
        // in there too.
    (*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_mut().unwrap()).accumulate_g_c_pause_time(
        Arc::new(Mutex::new(Some({ let __tmp_x = now; let __tmp_y = (*{ let __field = (*stw.lock().unwrap().as_ref().unwrap()).finished_stopping.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).maxprocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
    );
    (*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_mut().unwrap()).accumulate(Arc::new(Mutex::new(Some(now))), Arc::new(Mutex::new(Some(true))));

        // Compute overall GC CPU utilization.
        // Omit idle marking time from the overall utilization here since it's "free".
    { let new_val = {
        let __tmp_x = (*Arc::new(Mutex::new(Some(({
            let __tmp_x = (*(*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_ref().unwrap()).g_c_total_time.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_ref().unwrap()).g_c_idle_time.lock().unwrap().as_ref().unwrap());
            __tmp_x - __tmp_y
        }) as f64))).lock().unwrap().as_ref().unwrap());
        let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*work.lock().unwrap().as_ref().unwrap()).cpu_stats.lock().unwrap().as_ref().unwrap()).total_time.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as f64))).lock().unwrap().as_ref().unwrap());
        __tmp_x / __tmp_y
    }; *(*memstats.lock().unwrap().as_ref().unwrap()).gc_cpu_fraction.lock().unwrap() = Some(new_val); };

        // Reset assist time and background time stats.
        //
        // Do this now, instead of at the start of the next GC cycle, because
        // these two may keep accumulating even if the GC is not active.
    (*(*scavenge.lock().unwrap().as_ref().unwrap()).assist_time.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
    (*(*scavenge.lock().unwrap().as_ref().unwrap()).background_time.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));

        // Reset idle time stat.
    (*(*sched.lock().unwrap().as_ref().unwrap()).idle_time.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));

    if (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).user_forced.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __target = (*memstats.lock().unwrap().as_ref().unwrap()).numforcedgc.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Bump GC cycle count and wake goroutines waiting on sweep.
    lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).sweep_waiters.lock().unwrap().as_ref().unwrap()).lock.clone()));
    { let __target = (*memstats.lock().unwrap().as_ref().unwrap()).numgc.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    injectglist((*(*work.lock().unwrap().as_ref().unwrap()).sweep_waiters.lock().unwrap().as_ref().unwrap()).list.clone());
    unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).sweep_waiters.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Increment the scavenge generation now.
        //
        // This moment represents peak heap in use because we're
        // about to start sweeping.
    (*(*(*(*mheap_.lock().unwrap().as_ref().unwrap()).pages.lock().unwrap().as_ref().unwrap()).scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).next_gen();

        // Release the CPU limiter.
    (*gcCPULimiter.lock().unwrap().as_mut().unwrap()).finish_g_c_transition(Arc::new(Mutex::new(Some(now))));

        // Finish the current heap profiling cycle and start a new
        // heap profiling cycle. We do this before starting the world
        // so events don't leak into the wrong cycle.
    m_prof__next_cycle();

        // There may be stale spans in mcaches that need to be swept.
        // Those aren't tracked in any sweep lists, so we need to
        // count them against sweep completion until we ensure all
        // those spans have been forced out.
        //
        // If gcSweep fully swept the heap (for example if the sweep
        // is not concurrent due to a GODEBUG setting), then we expect
        // the sweepLocker to be invalid, since sweeping is done.
        //
        // N.B. Below we might duplicate some work from gcSweep; this is
        // fine as all that work is idempotent within a GC cycle, and
        // we're still holding worldsema so a new cycle can't start.
    let mut sl = (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).begin();
    if !{ let __v = (*stwSwept.lock().unwrap().as_ref().unwrap()).clone(); __v } && !(*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).valid.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("failed to set sweep barrier".to_string()))));
    } else if { let __v = (*stwSwept.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).valid.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("non-concurrent sweep failed to drain all sweep queues".to_string()))));
    }

    let now_closure_clone = now.clone(); let stw_closure_clone = stw.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        start_the_world_with_sema(Arc::new(Mutex::new(Some(now_closure_clone))), Arc::new(Mutex::new(Some({ let __arg_holder = stw_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // The memstats updated above must be updated with the world
        // stopped to ensure consistency of some values, such as
        // sched.idleTime and sched.totaltime. memstats also include
        // the pause time (work,pauseNS), forcing computation of the
        // total pause time before the pause actually ends.
        //
        // Here we reuse the same now for start the world so that the
        // time added to /sched/pauses/total/gc:seconds will be
        // consistent with the value in memstats.
        // Flush the heap profile so we can start a new cycle next GC.
        // This is relatively expensive, so we don't do it with the
        // world stopped.
    m_prof__flush();

        // Prepare workbufs for freeing by the sweeper. We do this
        // asynchronously because it can take non-trivial time.
    prepare_free_workbufs();

        // Free stack spans. This must be done between GC cycles.
    systemstack(Arc::new(Mutex::new(Some(Box::new(move || { free_stack_spans() }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Ensure all mcaches are flushed. Each P will flush its own
        // mcache before allocating, but idle Ps may not. Since this
        // is necessary to sweep all spans, we need to ensure all
        // mcaches are flushed before we start the next GC cycle.
        //
        // While we're here, flush the page cache for idle Ps to avoid
        // having pages get stuck on them. These pages are hidden from
        // the scavenger, so in small idle heaps a significant amount
        // of additional memory might be held onto.
        //
        // Also, flush the pinner cache, to avoid leaking that memory
        // indefinitely.
    for_each_p(
        Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_FLUSH_PROC_CACHES as u8))))))),
        Arc::new(Mutex::new(Some(Box::new(move |pp: GoPtr<crate::runtime2::p>| {
        { let __recv_field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mcache.clone()); __ptr_value }.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.prepare_for_sweep()); __result };
        if { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PIDLE as u32; __tmp_x == __tmp_y } {
        let pp_closure_clone = pp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*{ let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.pcache.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).flush((*mheap_.lock().unwrap().as_ref().unwrap()).pages.clone());
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
        *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.pinner_cache.clone()); __ptr_value }.lock().unwrap() = None;
    }) as Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync>)))
    );
    if (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).valid.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // Now that we've swept stale spans in mcaches, they don't
                // count against unswept spans.
                //
                // Note: this sweepLocker may not be valid if sweeping had
                // already completed during the STW. See the corresponding
                // begin() call that produced sl.
        (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).end(Arc::new(Mutex::new(Some({ let __arg_holder = sl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Now that we've swept stale spans in mcaches, they don't
        // count against unswept spans.
        //
        // Note: this sweepLocker may not be valid if sweeping had
        // already completed during the STW. See the corresponding
        // begin() call that produced sl.
        // Print gctrace before dropping worldsema. As soon as we drop
        // worldsema another cycle could start and smash the stats
        // we're trying to print.
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gctrace.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        let mut util = Arc::new(Mutex::new(Some(({ let __tmp_x = (*{ let __field = (*memstats.lock().unwrap().as_ref().unwrap()).gc_cpu_fraction.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 100.0; __tmp_x * __tmp_y }) as i32)));
        let mut sbuf: Arc<Mutex<Option<[u8; 24]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        printlock();
        {
            let __go_print_arg_0 = format!("{}", "gc ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*memstats.lock().unwrap().as_ref().unwrap()).numgc.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " @".to_string());
            let __go_print_arg_3 = format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*itoa_div(
                Arc::new(Mutex::new(Some({
                    let __seq_holder = sbuf.clone();
                    let __seq_guard = __seq_holder.lock().unwrap();
                    let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                    let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                    drop(__seq_guard);
                    let __low = 0;
                    let __high = __seq.len();
                    let __max = __source_cap;
                    let _slice = &__seq[__low..__high];
                    let mut _v = Vec::with_capacity((__max - __low) as usize);
                    _v.extend_from_slice(_slice);
                    _v
                }))),
                Arc::new(Mutex::new(Some({
                    let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).t_sweep_term.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*runtimeInitTime.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap());
                    let __tmp_y = 1e6 as u64;
                    __tmp_x / __tmp_y
                }))),
                Arc::new(Mutex::new(Some(3)))
            ).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", "s ".to_string());
            let __go_print_arg_5 = format!("{}", { let __v = (*util.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_6 = format!("{}", "%: ".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        let mut prev = Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).t_sweep_term.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        for (i, ns) in vec![{ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).t_mark.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).t_mark_term.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).t_end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }].iter().copied().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "+".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*fmt_n_s_as_m_s(
                Arc::new(Mutex::new(Some({
                    let __seq_holder = sbuf.clone();
                    let __seq_guard = __seq_holder.lock().unwrap();
                    let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                    let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                    drop(__seq_guard);
                    let __low = 0;
                    let __high = __seq.len();
                    let __max = __source_cap;
                    let _slice = &__seq[__low..__high];
                    let mut _v = Vec::with_capacity((__max - __low) as usize);
                    _v.extend_from_slice(_slice);
                    _v
                }))),
                Arc::new(Mutex::new(Some(({ let __tmp_x = ns; let __tmp_y = { let __v = (*prev.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64)))
            ).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };
        { let new_val = ns.clone(); *prev.lock().unwrap() = Some(new_val); };
    }
        {
            let __go_print_arg_0 = format!("{}", " ms clock, ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        for (i, ns) in vec![
            {
                let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).stwprocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap());
                let __tmp_y = ({ let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).t_mark.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).t_sweep_term.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y });
                __tmp_x * __tmp_y
            },
            (*(*gcController.lock().unwrap().as_ref().unwrap()).assist_time.lock().unwrap().as_mut().unwrap()).load(),
            {
                let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).dedicated_mark_time.lock().unwrap().as_mut().unwrap()).load();
                let __tmp_y = (*(*gcController.lock().unwrap().as_ref().unwrap()).fractional_mark_time.lock().unwrap().as_mut().unwrap()).load();
                __tmp_x + __tmp_y
            },
            (*(*gcController.lock().unwrap().as_ref().unwrap()).idle_mark_time.lock().unwrap().as_mut().unwrap()).load(),
            {
                let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).stwprocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap());
                let __tmp_y = ({ let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).t_end.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).t_mark_term.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y });
                __tmp_x * __tmp_y
            },
        ].iter().copied().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 2; __tmp_x == __tmp_y } || { let __tmp_x = i as i32; let __tmp_y = 3; __tmp_x == __tmp_y } {
                // Separate mark time components with /.
        {
            let __go_print_arg_0 = format!("{}", "/".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    } else if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "+".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
                // Separate mark time components with /.
        {
            let __go_print_arg_0 = format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*fmt_n_s_as_m_s(
                Arc::new(Mutex::new(Some({
                    let __seq_holder = sbuf.clone();
                    let __seq_guard = __seq_holder.lock().unwrap();
                    let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                    let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                    drop(__seq_guard);
                    let __low = 0;
                    let __high = __seq.len();
                    let __max = __source_cap;
                    let _slice = &__seq[__low..__high];
                    let mut _v = Vec::with_capacity((__max - __low) as usize);
                    _v.extend_from_slice(_slice);
                    _v
                }))),
                Arc::new(Mutex::new(Some(ns as u64)))
            ).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };
    }
                // Separate mark time components with /.
        {
            let __go_print_arg_0 = format!("{}", " ms cpu, ".to_string());
            let __go_print_arg_1 = format!("{}", { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).heap0.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 20; __tmp_x >> __tmp_y });
            let __go_print_arg_2 = format!("{}", "->".to_string());
            let __go_print_arg_3 = format!("{}", { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).heap1.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 20; __tmp_x >> __tmp_y });
            let __go_print_arg_4 = format!("{}", "->".to_string());
            let __go_print_arg_5 = format!("{}", { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).heap2.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 20; __tmp_x >> __tmp_y });
            let __go_print_arg_6 = format!("{}", " MB, ".to_string());
            let __go_print_arg_7 = format!("{}", { let __tmp_x = (*{ let __field = (*gcController.lock().unwrap().as_ref().unwrap()).last_heap_goal.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 20; __tmp_x >> __tmp_y });
            let __go_print_arg_8 = format!("{}", " MB goal, ".to_string());
            let __go_print_arg_9 = format!("{}", { let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).last_stack_scan.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 20; __tmp_x >> __tmp_y });
            let __go_print_arg_10 = format!("{}", " MB stacks, ".to_string());
            let __go_print_arg_11 = format!("{}", { let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).globals_scan.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 20; __tmp_x >> __tmp_y });
            let __go_print_arg_12 = format!("{}", " MB globals, ".to_string());
            let __go_print_arg_13 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).maxprocs.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_14 = format!("{}", " P".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8, __go_print_arg_9, __go_print_arg_10, __go_print_arg_11, __go_print_arg_12, __go_print_arg_13, __go_print_arg_14)
        };
        if (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).user_forced.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
            let __go_print_arg_0 = format!("{}", " (forced)".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        printunlock();
    }

        // Separate mark time components with /.
        // Set any arena chunks that were deferred to fault.
    lock(GoPtr::local((*userArenaState.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut faultList = (*userArenaState.lock().unwrap().as_ref().unwrap()).fault.clone();
    *(*userArenaState.lock().unwrap().as_ref().unwrap()).fault.lock().unwrap() = None;
    unlock(GoPtr::local((*userArenaState.lock().unwrap().as_ref().unwrap()).lock.clone()));
    { let __range_holder = faultList.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for lc in __range_values.iter() {
        { let __recv_field = lc.mspan.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.set_user_arena_chunk_to_fault()); __result };
    } }

        // Enable huge pages on some metadata if we cross a heap threshold.
    if { let __tmp_x = (*gcController.lock().unwrap().as_ref().unwrap()).heap_goal(); let __tmp_y = MIN_HEAP_FOR_METADATA_HUGE_PAGES as u64; __tmp_x > __tmp_y } {
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        (*mheap_.lock().unwrap().as_mut().unwrap()).enable_metadata_huge_pages();
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }

    semrelease(GoPtr::local(worldsema.clone()));
    semrelease(GoPtr::local(gcsema.clone()));

        // Careful: another GC cycle may start now.
    releasem(GoPtr::local(mp.clone()));
    *mp.lock().unwrap() = None;

        // now that gc is done, kick off finalizer thread if needed
    if !CONCURRENT_SWEEP {
                // give the queued finalizers, if any, a chance to run
        gosched();
    }
}

/// gcBgMarkStartWorkers prepares background mark worker goroutines. These
/// goroutines will not run until the mark phase, but they must be started while
/// the work is not stopped and from a regular G stack. The caller must hold
/// worldsema.
pub fn gc_bg_mark_start_workers() {
        // Background marking is performed by per-P G's. Ensure that each P has
        // a background GC G.
        //
        // Worker Gs don't exit if gomaxprocs is reduced. If it is raised
        // again, we can reuse the old workers; no need to create new workers.
    if { let __tmp_x = (*gcBgMarkWorkerCount.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*gomaxprocs.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return;
    }

        // Increment mp.locks when allocating. We are called within gcStart,
        // and thus must not trigger another gcStart via an allocation. gcStart
        // bails when allocating with locks held, so simulate that for these
        // allocations.
        //
        // TODO(prattmic): cleanup gcStart to use a more explicit "in gcStart"
        // check for bailing.
    let mut mp = acquirem();
    let mut ready = GoChannel::<AnonymousStruct12>::new_buffered(1 as usize);
    releasem(GoPtr::local(mp.clone()));

    while { let __tmp_x = (*gcBgMarkWorkerCount.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*gomaxprocs.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut mp = acquirem();
        let ready_thread = ready.clone(); std::thread::spawn(move || {
        gc_bg_mark_worker(ready_thread.clone());
    });
        releasem(GoPtr::local(mp.clone()));

                // N.B. we intentionally wait on each goroutine individually
                // rather than starting all in a batch and then waiting once
                // afterwards. By running one goroutine at a time, we can take
                // advantage of runnext to bounce back and forth between
                // workers and this goroutine. In an overloaded application,
                // this can reduce GC start latency by prioritizing these
                // goroutines rather than waiting on the end of the run queue.
        ready.recv().unwrap_or_default();

                // The worker is now guaranteed to be added to the pool before
                // its P's next findRunnableGCWorker.
        { let mut guard = gcBgMarkWorkerCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

/// gcBgMarkPrepare sets up state for background marking.
/// Mutator assists must not yet be enabled.
pub fn gc_bg_mark_prepare() {
        // Background marking will stop when the work queues are empty
        // and there are no more workers (note that, since this is
        // concurrent, this may be a transient state, but mark
        // termination will clean it up). Between background workers
        // and assists, we don't really know how many workers there
        // will be, so we pretend to have an arbitrarily large number
        // of workers, almost all of which are "waiting". While a
        // worker is working it decrements nwait. If nproc == nwait,
        // there are no workers.
    { let new_val = !(0 as u32) as u32; *(*work.lock().unwrap().as_ref().unwrap()).nproc.lock().unwrap() = Some(new_val); };
    { let new_val = !(0 as u32) as u32; *(*work.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap() = Some(new_val); };
}

pub fn gc_bg_mark_worker(ready: GoChannel<AnonymousStruct12>) {
    let mut gp = getg();

        // We pass node to a gopark unlock function, so it can't be on
        // the stack (see gopark). Prevent deadlock from recursively
        // starting GC by disabling preemption.
    { let new_val = "GC worker init".to_string(); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).preemptoff.lock().unwrap() = Some(new_val); };
    let mut node = Arc::new(Mutex::new(Some(gcBgMarkWorkerNode::default())));
    { let new_val = "".to_string(); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).preemptoff.lock().unwrap() = Some(new_val); };

    (*(*node.lock().unwrap().as_ref().unwrap()).gp.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(gp.clone()));

    (*(*node.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(acquirem()));

    ready.send(AnonymousStruct12 {  });

        // After this point, the background mark worker is generally scheduled
        // cooperatively by gcController.findRunnableGCWorker. While performing
        // work on the P, preemption is disabled because we are working on
        // P-local work buffers. When the preempt flag is set, this puts itself
        // into _Gwaiting to be woken up by gcController.findRunnableGCWorker
        // at the appropriate time.
        //
        // When preemption is enabled (e.g., while in gcMarkDone), this worker
        // may be preempted and schedule as a _Grunnable G from a runq. That is
        // fine; it will eventually gopark again for further scheduling via
        // findRunnableGCWorker.
        //
        // Since we disable preemption before notifying ready, we guarantee that
        // this G will be in the worker pool for the next findRunnableGCWorker.
        // This isn't strictly necessary, but it reduces latency between
        // _GCmark starting and the workers starting.
    loop {
                // Go to sleep until woken by
                // gcController.findRunnableGCWorker.
        gopark(
            Arc::new(Mutex::new(Some(Box::new(move |g: Arc<Mutex<Option<g>>>, nodep: Arc<Mutex<Option<usize>>>| -> bool {
        let mut node = Arc::new(Mutex::new({ let __ptr = nodep.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<gcBgMarkWorkerNode>(unimplemented!("unsafe.Pointer conversion to gcBgMarkWorkerNode")) } }));
        {
        let mut mp: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*(*node.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()));;
        if !mp.is_nil() {
            releasem(mp.clone());;
        }
    }
        (*gcBgMarkWorkerPool.lock().unwrap().as_ref().unwrap()).push((*node.lock().unwrap().as_ref().unwrap()).node.clone());
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>))),
            Arc::new(Mutex::new(Some(Arc::as_ptr(&node) as usize))),
            Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_G_C_WORKER_IDLE as u8))))))),
            Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SYSTEM_GOROUTINE as u8))))))),
            Arc::new(Mutex::new(Some(0)))
        );

                // The worker G is no longer running; release
                // the M.
                //
                // N.B. it is _safe_ to release the M as soon
                // as we are no longer performing P-local mark
                // work.
                //
                // However, since we cooperatively stop work
                // when gp.preempt is set, if we releasem in
                // the loop then the following call to gopark
                // would immediately preempt the G. This is
                // also safe, but inefficient: the G must
                // schedule again only to enter gopark and park
                // again. Thus, we defer the release until
                // after parking the G.
                // Release this G to the pool.
                // Note that at this point, the G may immediately be
                // rescheduled and may be running.
                // Preemption must not occur here, or another G might see
                // p.gcMarkWorkerMode.
                // Disable preemption so we can use the gcw. If the
                // scheduler wants to preempt us, we'll stop draining,
                // dispose the gcw, and then preempt.
        (*(*node.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(acquirem()));
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));

        if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "worker mode".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().gc_mark_worker_mode.clone() }.lock().unwrap().as_ref().unwrap()).clone());
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };
        throw(Arc::new(Mutex::new(Some("gcBgMarkWorker: blackening not enabled".to_string()))));
    }

        if {
            let __tmp_x = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_NOT_WORKER as i32))));
            __tmp_x == __tmp_y
        } {
        throw(Arc::new(Mutex::new(Some("gcBgMarkWorker: mode not set".to_string()))));
    }

        let mut startTime = nanotime();
        { let new_val = startTime; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_start_time.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        let mut trackLimiterEvent: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        if {
            let __tmp_x = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_IDLE_MODE as i32))));
            __tmp_x == __tmp_y
        } {
        { let new_val = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.limiter_event.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).start(Arc::new(Mutex::new(Some(crate::mgclimit::limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_IDLE_MARK_WORK as u8))))))), Arc::new(Mutex::new(Some(startTime)))); *trackLimiterEvent.lock().unwrap() = Some(new_val); };
    }

        let mut decnwait = internal_runtime_atomic::xadd(internal_runtime_atomic::GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).nwait.clone()), Arc::new(Mutex::new(Some(-1 as i32))));
        if { let __tmp_x = decnwait; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: work.nwait=".to_string());
            let __go_print_arg_1 = format!("{}", decnwait);
            let __go_print_arg_2 = format!("{}", "work.nproc=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        throw(Arc::new(Mutex::new(Some("work.nwait was > work.nproc".to_string()))));
    }

        let gp_closure_clone = gp.clone(); let pp_closure_clone = pp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        cas_g_to_waiting_for_suspend_g(GoPtr::local(gp_closure_clone.clone()), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_G_C_WORKER_ACTIVE as u8))))))));
        { let _switch_val = { let __selector_holder = { let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_DEDICATED_MODE as i32))))) {
            gc_drain_mark_worker_dedicated({ let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some(true))));
            if (*{ let __field = (*gp_closure_clone.lock().unwrap().as_ref().unwrap()).preempt.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
        let (mut drainQ, mut n) = runqdrain(pp_closure_clone.clone());;
        if { let __tmp_x = n; let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
            lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));;
            globrunqputbatch(
                drainQ.clone(),
                Arc::new(Mutex::new(Some(n as i32)))
            );;
            unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));;
        }
    }
    }
            gc_drain_mark_worker_dedicated({ let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some(false))));
        } else if _switch_val == (gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_FRACTIONAL_MODE as i32))))) {
            gc_drain_mark_worker_fractional({ let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.clone());
        } else if _switch_val == (gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_IDLE_MODE as i32))))) {
            gc_drain_mark_worker_idle({ let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.clone());
        } else {
            throw(Arc::new(Mutex::new(Some("gcBgMarkWorker: unexpected gcMarkWorkerMode".to_string()))));
        }
    }
        casgstatus(GoPtr::local(gp_closure_clone.clone()), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

                // Mark our goroutine preemptible so its stack
                // can be scanned or observed by the execution
                // tracer. This, for example, lets two mark workers
                // scan each other (otherwise, they would
                // deadlock). We must not modify anything on
                // the G stack. However, stack shrinking is
                // disabled for mark workers, so it is safe to
                // read from the G stack.
                //
                // N.B. The execution tracer is not aware of this status
                // transition and handles it specially based on the
                // wait reason.
                // We were preempted. This is
                // a useful signal to kick
                // everything out of the run
                // queue so it can run
                // somewhere else.
                // Go back to draining, this time
                // without preemption.
                // Account for time and mark us as stopped.
        let mut now = nanotime();
        let mut duration = Arc::new(Mutex::new(Some({ let __tmp_x = now; let __tmp_y = startTime; __tmp_x - __tmp_y })));
        (*gcController.lock().unwrap().as_ref().unwrap()).mark_worker_stop(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = duration.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __v = (*trackLimiterEvent.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.limiter_event.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).stop(Arc::new(Mutex::new(Some(crate::mgclimit::limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_IDLE_MARK_WORK as u8))))))), Arc::new(Mutex::new(Some(now))));
    }
        if {
            let __tmp_x = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_FRACTIONAL_MODE as i32))));
            __tmp_x == __tmp_y
        } {
        internal_runtime_atomic::xaddint64({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_fractional_mark_time.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = duration.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

                // Was this the last worker and did we run out
                // of work?
        let mut incnwait = internal_runtime_atomic::xadd(internal_runtime_atomic::GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).nwait.clone()), Arc::new(Mutex::new(Some(1 as i32))));
        if { let __tmp_x = incnwait; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: p.gcMarkWorkerMode=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().gc_mark_worker_mode.clone() }.lock().unwrap().as_ref().unwrap()).clone());
            let __go_print_arg_2 = format!("{}", "work.nwait=".to_string());
            let __go_print_arg_3 = format!("{}", incnwait);
            let __go_print_arg_4 = format!("{}", "work.nproc=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5)
        };
        throw(Arc::new(Mutex::new(Some("work.nwait > work.nproc".to_string()))));
    }

                // We'll releasem after this point and thus this P may run
                // something else. We must clear the worker mode to avoid
                // attributing the mode to a different (non-worker) G in
                // traceGoStart.
        { let new_val = gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_NOT_WORKER as i32)))); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

                // If this worker reached a background mark completion
                // point, signal the main GC goroutine.
        if { let __tmp_x = incnwait; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && !gc_mark_work_available(GoPtr::nil()) {
                // We don't need the P-local buffers here, allow
                // preemption because we may schedule like a regular
                // goroutine in gcMarkDone (block on locks, etc).
        releasem(crate::runtime2::muintptr::ptr(&(*(*node.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap())));
        (*(*node.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_mut().unwrap()).set(GoPtr::nil());
        gc_mark_done();
    }
    }
}

/// gcMarkWorkAvailable reports whether executing a mark worker
/// on p is potentially useful. p may be nil, in which case it only
/// checks the global sources of work.
pub fn gc_mark_work_available(p: GoPtr<crate::runtime2::p>) -> bool {
    if !p.is_nil() && !(*{ let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).empty() {
        return true;
    }
    if !(*(*work.lock().unwrap().as_ref().unwrap()).full.lock().unwrap().as_ref().unwrap()).empty() {
        return true;
    }
        // global work available
    if { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return true;
    }
        // root scan work available
    false
}

/// gcMark runs the mark (or, for concurrent GC, mark termination)
/// All gcWork caches must be empty.
/// STW is in effect at this point.
pub fn gc_mark(startTime: Arc<Mutex<Option<i64>>>) {
    if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARKTERMINATION as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("in gcMark expecting to see gcphase as _GCmarktermination".to_string()))));
    }
    { let new_val = startTime.lock().unwrap().as_ref().unwrap().clone(); *(*work.lock().unwrap().as_ref().unwrap()).tstart.lock().unwrap() = Some(new_val); };

        // Check that there's no marking work remaining.
    if {
        let __go_cond_0 = {
            let __tmp_x = { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).full.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0 as u64))));
            __tmp_x != __tmp_y
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
            __go_cond_1
        }
    } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: full=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*(*(*work.lock().unwrap().as_ref().unwrap()).full.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", " next=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " jobs=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", " nDataRoots=".to_string());
            let __go_print_arg_7 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_data_roots.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_8 = format!("{}", " nBSSRoots=".to_string());
            let __go_print_arg_9 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_b_s_s_roots.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_10 = format!("{}", " nSpanRoots=".to_string());
            let __go_print_arg_11 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_span_roots.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_12 = format!("{}", " nStackRoots=".to_string());
            let __go_print_arg_13 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_stack_roots.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_14 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8, __go_print_arg_9, __go_print_arg_10, __go_print_arg_11, __go_print_arg_12, __go_print_arg_13, __go_print_arg_14)
        };
        std::panic::panic_any(Box::new("non-empty mark queue after concurrent mark".to_string()) as Box<dyn Any + Send + Sync>);
    }

    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gccheckmark.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
                // This is expensive when there's a large number of
                // Gs, so only do it if checkmark is also enabled.
        gc_mark_root_check();
    }

        // This is expensive when there's a large number of
        // Gs, so only do it if checkmark is also enabled.
        // Drop allg snapshot. allgs may have grown, in which case
        // this is the only reference to the old backing store and
        // there's no need to keep it around.
    *(*work.lock().unwrap().as_ref().unwrap()).stack_roots.lock().unwrap() = None;

        // Clear out buffers and double-check that all gcWork caches
        // are empty. This should be ensured by gcMarkDone before we
        // enter mark termination.
        //
        // TODO: We could clear out buffers just before mark if this
        // has a non-negligible impact on STW time.
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
                // The write barrier may have buffered pointers since
                // the gcMarkDone barrier. However, since the barrier
                // ensured all reachable objects were marked, all of
                // these must be pointers to black objects. Hence we
                // can just discard the write barrier buffer.
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gccheckmark.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
                // For debugging, flush the buffer and make
                // sure it really was all marked.
        wb_buf_flush1(GoPtr::local(p.clone()));
    } else {
        (*(*p.lock().unwrap().as_ref().unwrap()).wb_buf.lock().unwrap().as_mut().unwrap()).reset();
    }
                // For debugging, flush the buffer and make
                // sure it really was all marked.
        let mut gcw = (*p.lock().unwrap().as_ref().unwrap()).gcw.clone();
        if !{ let __recv = gcw.clone(); let __recv_ptr: *const crate::mgcwork::gcWork = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mgcwork::gcWork }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        printlock();
        {
            let __go_print_arg_0 = format!("{}", "runtime: P ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).id.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " flushedWork ".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).flushed_work.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        if { let __ptr_field = (*gcw.lock().unwrap().as_ref().unwrap()).wbuf1.clone(); __ptr_field.is_nil() } {
        {
            let __go_print_arg_0 = format!("{}", " wbuf1=<nil>".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", " wbuf1.n=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = (*gcw.lock().unwrap().as_ref().unwrap()).wbuf1.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    }
        if { let __ptr_field = (*gcw.lock().unwrap().as_ref().unwrap()).wbuf2.clone(); __ptr_field.is_nil() } {
        {
            let __go_print_arg_0 = format!("{}", " wbuf2=<nil>".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", " wbuf2.n=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = (*gcw.lock().unwrap().as_ref().unwrap()).wbuf2.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        throw(Arc::new(Mutex::new(Some("P has cached GC work at end of mark termination".to_string()))));
    }
                // There may still be cached empty buffers, which we
                // need to flush since we're going to free them. Also,
                // there may be non-zero stats because we allocated
                // black after the gcMarkDone barrier.
        { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.dispose(); __result };
    } }

        // The write barrier may have buffered pointers since
        // the gcMarkDone barrier. However, since the barrier
        // ensured all reachable objects were marked, all of
        // these must be pointers to black objects. Hence we
        // can just discard the write barrier buffer.
        // For debugging, flush the buffer and make
        // sure it really was all marked.
        // There may still be cached empty buffers, which we
        // need to flush since we're going to free them. Also,
        // there may be non-zero stats because we allocated
        // black after the gcMarkDone barrier.
        // Flush scanAlloc from each mcache since we're about to modify
        // heapScan directly. If we were to flush this later, then scanAlloc
        // might have incorrect information.
        //
        // Note that it's not important to retain this information; we know
        // exactly what heapScan is at this point via scanWork.
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let mut c: GoPtr<crate::mcache::mcache> = (*p.lock().unwrap().as_ref().unwrap()).mcache.clone();
        if c.is_nil() {
        continue
    }
        { let new_val = 0 as usize; *{ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.scan_alloc.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    } }

        // Reset controller state.
    (*gcController.lock().unwrap().as_mut().unwrap()).reset_live(Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).bytes_marked.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
}

/// gcSweep must be called on the system stack because it acquires the heap
/// lock. See mheap for details.
///
/// Returns true if the heap was fully swept by this function.
///
/// The world must be stopped.
///
///go:systemstack
pub fn gc_sweep(mode: Arc<Mutex<Option<gcMode>>>) -> bool {
    assert_world_stopped();

    if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("gcSweep being done but phase is not GCoff".to_string()))));
    }

    lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    { let __target = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __rhs = 2 as u32; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).reset();
    (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_swept.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u64))));
    { let new_val = (*mheap_.lock().unwrap().as_ref().unwrap()).all_arenas.clone(); (*mheap_.lock().unwrap().as_mut().unwrap()).sweep_arenas = new_val; };
    (*(*mheap_.lock().unwrap().as_ref().unwrap()).reclaim_index.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u64))));
    (*(*mheap_.lock().unwrap().as_ref().unwrap()).reclaim_credit.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as usize))));
    unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));

    (*(*sweep.lock().unwrap().as_ref().unwrap()).central_index.lock().unwrap().as_ref().unwrap()).clear();

    if !CONCURRENT_SWEEP || { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = gcMode(Arc::new(Mutex::new(Some(GC_FORCE_BLOCK_MODE as i32)))); __tmp_x == __tmp_y } {
                // Special case synchronous sweep.
                // Record that no proportional sweeping has to happen.
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = 0.0; *(*mheap_.lock().unwrap().as_ref().unwrap()).sweep_pages_per_byte.lock().unwrap() = Some(new_val); };
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
                // Flush all mcaches.
        { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pp in __range_values.iter() {
        { let __recv_field = (*pp.lock().unwrap().as_ref().unwrap()).mcache.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.prepare_for_sweep()); __result };
    } }
                // Sweep all spans eagerly.
        while { let __tmp_x = sweepone(); let __tmp_y = !(0 as usize) as usize; __tmp_x != __tmp_y } {
    }
                // Free workbufs eagerly.
        prepare_free_workbufs();
        while free_some_wbufs(Arc::new(Mutex::new(Some(false)))) {
    }
                // All "free" events for this mark/sweep cycle have
                // now happened, so we can make this profile cycle
                // available immediately.
        m_prof__next_cycle();
        m_prof__flush();
        return true;
    }

        // Special case synchronous sweep.
        // Record that no proportional sweeping has to happen.
        // Flush all mcaches.
        // Sweep all spans eagerly.
        // Free workbufs eagerly.
        // All "free" events for this mark/sweep cycle have
        // now happened, so we can make this profile cycle
        // available immediately.
        // Background sweep.
    lock(GoPtr::local((*sweep.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if (*{ let __field = (*sweep.lock().unwrap().as_ref().unwrap()).parked.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = false; *(*sweep.lock().unwrap().as_ref().unwrap()).parked.lock().unwrap() = Some(new_val); };
        ready(GoPtr::local((*sweep.lock().unwrap().as_ref().unwrap()).g.clone()), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(true))));
    }
    unlock(GoPtr::local((*sweep.lock().unwrap().as_ref().unwrap()).lock.clone()));
    false
}

/// gcResetMarkState resets global state prior to marking (concurrent
/// or STW) and resets the stack scan state of all Gs.
///
/// This is safe to do without the world stopped because any Gs created
/// during or after this will start out in the reset state.
///
/// gcResetMarkState must be called on the system stack because it acquires
/// the heap lock. See mheap for details.
///
///go:systemstack
pub fn gc_reset_mark_state() {
        // This may be called during a concurrent phase, so lock to make sure
        // allgs doesn't change.
    for_each_g(Arc::new(Mutex::new(Some(Box::new(move |gp: Arc<Mutex<Option<g>>>| {
        { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).gcscandone.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as i64; *(*gp.lock().unwrap().as_ref().unwrap()).gc_assist_bytes.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>))));

        // set to true in gcphasework
        // Clear page marks. This is just 1MB per 64GB of heap, so the
        // time here is pretty trivial.
    lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut arenas = (*mheap_.lock().unwrap().as_ref().unwrap()).all_arenas.clone();
    unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    { let __range_holder = arenas.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ai in __range_values.iter().cloned() {
        let mut ha = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l1(&(ai))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l2(&(ai))) as usize].clone() }.clone();
        { let __clear_start = 0usize; let __clear_end = { let __clear_len_holder = (*ha.lock().unwrap().as_ref().unwrap()).page_marks.clone(); let __clear_len_guard = __clear_len_holder.lock().unwrap(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = (*ha.lock().unwrap().as_ref().unwrap()).page_marks.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = 0; } } };
    } }

    { let new_val = 0 as u64; *(*work.lock().unwrap().as_ref().unwrap()).bytes_marked.lock().unwrap() = Some(new_val); };
    { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_live.lock().unwrap().as_mut().unwrap()).load(); *(*work.lock().unwrap().as_ref().unwrap()).initial_heap_live.lock().unwrap() = Some(new_val); };
}

pub fn clearpools() {
        // clear sync.Pools
    if { let __nil_result = (*poolcleanup.lock().unwrap()).is_some(); __nil_result } {
        {
            let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
                let mut __f_guard = poolcleanup.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        };
    }

        // clear boringcrypto caches
    { let __range_holder = boringCaches.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        atomicstorep(Arc::new(Mutex::new(Some(p.clone()))), Arc::new(Mutex::new(None)));
    } }

        // clear unique maps
    if !{ let __channel = uniqueMapCleanup.lock().unwrap().as_ref().unwrap().clone(); __channel }.is_nil() {
        loop {
        if { let __channel = uniqueMapCleanup.lock().unwrap().as_ref().unwrap().clone(); __channel }.try_send(AnonymousStruct12 {  }) {
            break;
        }
        break;
    }
    }

        // Clear central sudog cache.
        // Leave per-P caches alone, they have strictly bounded size.
        // Disconnect cached list before dropping it on the floor,
        // so that a dangling ref to one entry does not pin all of them.
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).sudoglock.clone()));
    let mut sg: Arc<Mutex<Option<sudog>>> = Arc::new(Mutex::new(None));let mut sgnext: Arc<Mutex<Option<sudog>>> = Arc::new(Mutex::new(None));
    { let new_val = (*sched.lock().unwrap().as_ref().unwrap()).sudogcache.clone(); sg = new_val; };
    while { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*sg.lock().unwrap().as_ref().unwrap()).next.clone(); sgnext = new_val; };
        *(*sg.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        { let new_val = sgnext.clone(); sg = new_val; };
    }
    *(*sched.lock().unwrap().as_ref().unwrap()).sudogcache.lock().unwrap() = None;
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).sudoglock.clone()));

        // Clear central defer pool.
        // Leave per-P pools alone, they have strictly bounded size.
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).deferlock.clone()));

        // disconnect cached list before dropping it on the floor,
        // so that a dangling ref to one entry does not pin all of them.
    let mut d: Arc<Mutex<Option<_defer>>> = Arc::new(Mutex::new(None));let mut dlink: Arc<Mutex<Option<_defer>>> = Arc::new(Mutex::new(None));
    { let new_val = (*sched.lock().unwrap().as_ref().unwrap()).deferpool.clone(); d = new_val; };
    while { let __nil_result = (*d.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*d.lock().unwrap().as_ref().unwrap()).link.clone(); dlink = new_val; };
        *(*d.lock().unwrap().as_ref().unwrap()).link.lock().unwrap() = None;
        { let new_val = dlink.clone(); d = new_val; };
    }
    *(*sched.lock().unwrap().as_ref().unwrap()).deferpool.lock().unwrap() = None;
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).deferlock.clone()));
}

/// itoaDiv formats val/(10**dec) into buf.
pub fn itoa_div(buf_local: Arc<Mutex<Option<Vec<u8>>>>, mut val: Arc<Mutex<Option<u64>>>, dec: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*buf_local.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    let mut idec = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*dec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x >= __tmp_y } || { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*idec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        (*buf_local.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x % __tmp_y }; let __tmp_y = ('0' as u64); __tmp_x + __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*idec.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        (*buf_local.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('.' as i32) as u8;
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let __rhs = 10 as u64; let mut guard = val.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
    }
    (*buf_local.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as u64); __tmp_x + __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
    return Arc::new(Mutex::new(Some({
        let __seq_holder = buf_local.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        drop(__seq_guard);
        let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    })));
}

/// fmtNSAsMS nicely formats ns nanoseconds as milliseconds.
pub fn fmt_n_s_as_m_s(buf_local: Arc<Mutex<Option<Vec<u8>>>>, ns: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10e6 as u64; __tmp_x >= __tmp_y } {
                // Format as whole milliseconds.
        return itoa_div(
            buf_local.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e6 as u64; __tmp_x / __tmp_y }))),
            Arc::new(Mutex::new(Some(0)))
        );
    }

        // Format as whole milliseconds.
        // Format two digits of precision, with at most three decimal places.
    let mut x = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e3 as u64; __tmp_x / __tmp_y })));
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        (*buf_local.lock().unwrap().as_mut().unwrap())[(0) as usize] = ('0' as i32) as u8;
        return Arc::new(Mutex::new(Some({
            let __seq_holder = buf_local.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = (1) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        })));
    }
    let mut dec = Arc::new(Mutex::new(Some(3)));
    while { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100 as u64; __tmp_x >= __tmp_y } {
        { let __rhs = 10 as u64; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
        { let mut guard = dec.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return itoa_div(buf_local.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct10 {
    pub enabled: Arc<Mutex<Option<bool>>>,
    pub pad: Arc<Mutex<Option<[u8; 3]>>>,
    pub alignme: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct10 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.alignme.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            enabled: __go_clone_0_0,
            pad: __go_clone_1_0,
            alignme: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct10 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            enabled: __go_default_0_0,
            pad: __go_default_1_0,
            alignme: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.enabled.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.pad));
        let __go_fmt_2 = format!("{}", (*self.alignme.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for AnonymousStruct10 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct11 {
    pub spin_after_ragged_barrier: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub restarted_due_to27993: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct11 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.spin_after_ragged_barrier.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.restarted_due_to27993.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            spin_after_ragged_barrier: __go_clone_0_0,
            restarted_due_to27993: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct11 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            spin_after_ragged_barrier: __go_default_0_0,
            restarted_due_to27993: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.spin_after_ragged_barrier.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.restarted_due_to27993.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct11 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct12 {
}
impl AnonymousStruct12 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for AnonymousStruct12 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type gcDebugMarkDone = AnonymousStruct11;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for workType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gcTrigger {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gcBgMarkWorkerNode {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
