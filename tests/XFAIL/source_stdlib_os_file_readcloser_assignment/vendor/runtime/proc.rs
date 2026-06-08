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
    asan0::{ASANENABLED, asanwrite},
    atomic_pointer::{atomicstorep},
    cgo::{_cgo_thread_start, cgoHasExtraM, cgo_yield, iscgo},
    cgocall::{callback_update_system_stack, cgoCallers},
    chan::{hchan},
    cpuprof::{MAX_C_P_U_PROF_STACK, cpuprof},
    histogram::{timeHistogram},
    lock_sema::{before_idle, check_timeouts, noteclear, notesleep, notetsleep, notewakeup},
    lock_spinbit::{lock, unlock},
    lockrank::{LOCK_RANK_FORCEGC, LOCK_RANK_GSCAN, LOCK_RANK_TIMERS},
    lockrank_off::{STATIC_LOCK_RANKING, acquire_lock_rank_and_m, assert_lock_held, assert_world_stopped, assert_world_stopped_or_lock_held, lock_init, lock_with_rank_may_acquire, release_lock_rank_and_m, world_started, world_stopped},
    mcache::{allocmcache, freemcache, mcache},
    mfinal::{FING_WAIT, FING_WAKE, fingStatus, wakefing},
    mfixalloc::{fixalloc},
    mgc::{GC_MARK_WORKER_IDLE_MODE, GC_TRIGGER_TIME, __G_COFF, gcBgMarkWorkerNode, gcBlackenEnabled, gcMarkWorkerMode, gcTrigger, gc_mark_work_available, gc_start, gcphase},
    mgclimit::{LIMITER_EVENT_IDLE, gcCPULimiter, limiterEvent, limiterEventType},
    mgcpacer::{gcController},
    mgcwork::{gcWork},
    mheap::{mheap_, mspan},
    mpagealloc::{pageAlloc},
    mpagecache::{pageCache},
    mprof::{MAX_SKIP, goroutineProfile, mLockProfile, try_record_goroutine_profile, try_record_goroutine_profile_w_b},
    msan0::{MSANENABLED, msanwrite},
    mwbbuf::{wbBuf, wb_buf_flush1},
    netpoll::{netpoll_adjust_waiters, netpoll_any_waiters, netpollinited},
    netpoll_kqueue::{netpoll, netpoll_break},
    note_other::{note},
    os_darwin::{minit, mpreinit, newosproc, osyield, osyield_no_g, set_thread_c_p_u_profiler, sigset, unminit},
    panic::{deadlock, fatal, panicking, throw, throwType},
    pinner::{pinner},
    preempt::{can_preempt_m},
    print::{hex},
    r#extern::{G_O_A_R_C_H, G_O_O_S},
    r#mod::{write_err_str},
    race0::{RACEENABLED, racectxend, raceproccreate, raceprocdestroy},
    rand::{cheaprand, cheaprandn, mrandinit, randn},
    runtime1::{acquirem, debug, releasem},
    runtime2::{AnonymousStruct26, AnonymousStruct27, AnonymousStruct28, AnonymousStruct29, FREE_M_STACK, FREE_M_WAIT, G_TRACKING_PERIOD, WAIT_REASON_FORCE_G_C_IDLE, WAIT_REASON_PREEMPTED, WAIT_REASON_SLEEP, WAIT_REASON_STOPPING_THE_WORLD, __GDEAD, __GPREEMPTED, __GRUNNABLE, __GRUNNING, __GSCAN, __GSCANPREEMPTED, __GSCANRUNNABLE, __GSCANRUNNING, __GSCANSYSCALL, __GSCANWAITING, __GSYSCALL, __GWAITING, __PDEAD, __PGCSTOP, __PIDLE, __PRUNNING, __PSYSCALL, _defer, allm, allp, allpLock, forcegc, g, gcBgMarkWorkerPool, getcallerfp, goarm, gobuf, gomaxprocs, guintptr, idlepMask, isarchive, islibrary, m, muintptr, mutex, newprocs, p, puintptr, sched, set_g_no_w_b, set_m_no_w_b, stack, sudog, timerpMask, waitReason},
    rwmutex::{rwmutex},
    sema::{semacquire, semrelease, semrelease1},
    signal_unix::{PREEMPT_M_SUPPORTED, msigrestore, preempt_m, sigblock, sigsave},
    stack::{STACK_GUARD, STACK_PREEMPT, STACK_SYSTEM, round2, stackalloc, stackfree},
    stubs::{add, asmcgocall, asminit, getg, gogo, mcall, procyield, setg, systemstack},
    symtab::{findfunc, firstmoduledata, funcInfo, funcname},
    synctest::{synctestGroup},
    sys_darwin::{exit, usleep, usleep_no_g},
    time::{MAX_WHEN, time_sleep_until, timerWhen, timers},
    time_nofake::{faketime, nanotime},
    tls_stub::{os_setup_t_l_s},
    trace::{trace_reader, trace_reader_available},
    traceback::{UNWIND_JUMP_STACK, UNWIND_SILENT_ERRORS, UNWIND_TRAP, callers_1, is_system_goroutine, traceback_p_cs, unwindFlags, unwinder},
    tracecpu::{trace_c_p_u_sample},
    traceruntime::{TRACE_BLOCK_PREEMPTED, TRACE_BLOCK_SYSTEM_GOROUTINE, gTraceState, traceBlockReason, traceLocker, trace_acquire, trace_enabled, trace_exited_syscall, trace_exiting_syscall, trace_release, trace_shutting_down, trace_thread_destroy},
    vdso_in_none::{in_v_d_s_o_page},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) const CRASH_STACK_IMPLEMENTED: bool = !go_const_str_eq(G_O_O_S, "windows");


pub(crate) const __GOID_CACHE_BATCH: i32 = 16;


pub(crate) const OS_HAS_LOW_RES_TIMER: bool = go_const_str_eq(G_O_O_S, "windows") || go_const_str_eq(G_O_O_S, "openbsd") || go_const_str_eq(G_O_O_S, "netbsd");
pub(crate) const OS_HAS_LOW_RES_CLOCK_INT: i32 = internal_goos::IS_WINDOWS as i32;
pub(crate) const OS_HAS_LOW_RES_CLOCK: bool = OS_HAS_LOW_RES_CLOCK_INT > 0;


pub(crate) const FREEZE_STOP_WAIT: i32 = 0x7fffffff;


pub(crate) const STW_UNKNOWN: u8 = 0;
pub(crate) const STW_G_C_MARK_TERM: u8 = 1;
pub(crate) const STW_G_C_SWEEP_TERM: u8 = 2;
pub(crate) const STW_WRITE_HEAP_DUMP: u8 = 3;
pub(crate) const STW_GOROUTINE_PROFILE: u8 = 4;
pub(crate) const STW_GOROUTINE_PROFILE_CLEANUP: u8 = 5;
pub(crate) const STW_ALL_GOROUTINES_STACK: u8 = 6;
pub(crate) const STW_READ_MEM_STATS: u8 = 7;
pub(crate) const STW_ALL_THREADS_SYSCALL: u8 = 8;
pub(crate) const STW_G_O_M_A_X_P_R_O_C_S: u8 = 9;
pub(crate) const STW_START_TRACE: u8 = 10;
pub(crate) const STW_STOP_TRACE: u8 = 11;
pub(crate) const STW_FOR_TEST_COUNT_PAGES_IN_USE: u8 = 12;
pub(crate) const STW_FOR_TEST_READ_METRICS_SLOW: u8 = 13;
pub(crate) const STW_FOR_TEST_READ_MEM_STATS_SLOW: u8 = 14;
pub(crate) const STW_FOR_TEST_PAGE_CACHE_PAGES_LEAKED: u8 = 15;
pub(crate) const STW_FOR_TEST_RESET_DEBUG_LOG: u8 = 16;


pub(crate) const FAILTHREADCREATE: &'static str = "runtime: failed to create new OS thread\n";
pub(crate) const FAILALLOCATESTACK: &'static str = "runtime: failed to allocate stack for the new OS thread\n";


pub(crate) const HAVE_SYSMON: bool = !go_const_str_eq(G_O_A_R_C_H, "wasm");


pub(crate) const FORCE_PREEMPT_N_S: i32 = 10 * 1000 * 1000;


pub(crate) const RANDOMIZE_SCHEDULER: bool = RACEENABLED;


/// stwReason is an enumeration of reasons the world is stopping.
#[derive(Debug, Clone, Default)]
pub struct stwReason(pub Arc<Mutex<Option<u8>>>);

impl Display for stwReason {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for stwReason {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for stwReason {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for stwReason {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for stwReason {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<stwReason> for u8 {
    fn eq(&self, other: &stwReason) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<stwReason> for u8 {
    fn partial_cmp(&self, other: &stwReason) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for stwReason {
    type Output = stwReason;
    fn add(self, other: Self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for stwReason {
    type Output = stwReason;
    fn add(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<stwReason> for u8 {
    type Output = stwReason;
    fn add(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for stwReason {
    type Output = stwReason;
    fn sub(self, other: Self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for stwReason {
    type Output = stwReason;
    fn sub(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<stwReason> for u8 {
    type Output = stwReason;
    fn sub(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for stwReason {
    type Output = stwReason;
    fn mul(self, other: Self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for stwReason {
    type Output = stwReason;
    fn mul(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<stwReason> for u8 {
    type Output = stwReason;
    fn mul(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for stwReason {
    type Output = stwReason;
    fn div(self, other: Self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for stwReason {
    type Output = stwReason;
    fn div(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<stwReason> for u8 {
    type Output = stwReason;
    fn div(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for stwReason {
    type Output = stwReason;
    fn rem(self, other: Self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for stwReason {
    type Output = stwReason;
    fn rem(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<stwReason> for u8 {
    type Output = stwReason;
    fn rem(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for stwReason {
    type Output = stwReason;
    fn bitand(self, other: Self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for stwReason {
    type Output = stwReason;
    fn bitand(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<stwReason> for u8 {
    type Output = stwReason;
    fn bitand(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for stwReason {
    type Output = stwReason;
    fn bitor(self, other: Self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for stwReason {
    type Output = stwReason;
    fn bitor(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<stwReason> for u8 {
    type Output = stwReason;
    fn bitor(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for stwReason {
    type Output = stwReason;
    fn bitxor(self, other: Self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for stwReason {
    type Output = stwReason;
    fn bitxor(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<stwReason> for u8 {
    type Output = stwReason;
    fn bitxor(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for stwReason {
    type Output = stwReason;
    fn not(self) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for stwReason {
    type Output = stwReason;
    fn shl(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for stwReason {
    type Output = stwReason;
    fn shl(self, other: i32) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for stwReason {
    type Output = stwReason;
    fn shl(self, other: i8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for stwReason {
    type Output = stwReason;
    fn shl(self, other: i16) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for stwReason {
    type Output = stwReason;
    fn shl(self, other: i64) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for stwReason {
    type Output = stwReason;
    fn shl(self, other: u32) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for stwReason {
    type Output = stwReason;
    fn shl(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for stwReason {
    type Output = stwReason;
    fn shl(self, other: u16) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for stwReason {
    type Output = stwReason;
    fn shl(self, other: u64) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for stwReason {
    type Output = stwReason;
    fn shl(self, other: usize) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for stwReason {
    type Output = stwReason;
    fn shr(self, other: stwReason) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for stwReason {
    type Output = stwReason;
    fn shr(self, other: i32) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for stwReason {
    type Output = stwReason;
    fn shr(self, other: i8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for stwReason {
    type Output = stwReason;
    fn shr(self, other: i16) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for stwReason {
    type Output = stwReason;
    fn shr(self, other: i64) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for stwReason {
    type Output = stwReason;
    fn shr(self, other: u32) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for stwReason {
    type Output = stwReason;
    fn shr(self, other: u8) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for stwReason {
    type Output = stwReason;
    fn shr(self, other: u16) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for stwReason {
    type Output = stwReason;
    fn shr(self, other: u64) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for stwReason {
    type Output = stwReason;
    fn shr(self, other: usize) -> stwReason {
        stwReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for stwReason {}

impl Ord for stwReason {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// worldStop provides context from the stop-the-world required by the
/// start-the-world.
#[derive(Debug, Clone)]
pub struct worldStop {
    pub reason: Arc<Mutex<Option<stwReason>>>,
    pub started_stopping: Arc<Mutex<Option<i64>>>,
    pub finished_stopping: Arc<Mutex<Option<i64>>>,
    pub stopping_c_p_u_time: Arc<Mutex<Option<i64>>>,
}

impl worldStop {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.reason.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.started_stopping.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.finished_stopping.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.stopping_c_p_u_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            reason: __go_clone_0_0,
            started_stopping: __go_clone_1_0,
            finished_stopping: __go_clone_2_0,
            stopping_c_p_u_time: __go_clone_3_0,
        }
    }
}


impl Default for worldStop {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(stwReason(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            reason: __go_default_0_0,
            started_stopping: __go_default_1_0,
            finished_stopping: __go_default_2_0,
            stopping_c_p_u_time: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for worldStop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.reason.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.started_stopping.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.finished_stopping.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.stopping_c_p_u_time.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for worldStop {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct cgothreadstart {
    pub g: Arc<Mutex<Option<guintptr>>>,
    pub tls: Arc<Mutex<Option<u64>>>,
    pub r#fn: Arc<Mutex<Option<usize>>>,
}

impl cgothreadstart {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.g.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.tls.clone();
        let __go_clone_2_0 = { let __guard = self.r#fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            g: __go_clone_0_0,
            tls: __go_clone_1_0,
            r#fn: __go_clone_2_0,
        }
    }
}


impl Default for cgothreadstart {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            g: __go_default_0_0,
            tls: __go_default_1_0,
            r#fn: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for cgothreadstart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.g.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.tls.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.r#fn.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for cgothreadstart {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct sysmontick {
    pub schedtick: Arc<Mutex<Option<u32>>>,
    pub syscalltick: Arc<Mutex<Option<u32>>>,
    pub schedwhen: Arc<Mutex<Option<i64>>>,
    pub syscallwhen: Arc<Mutex<Option<i64>>>,
}

impl sysmontick {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.schedtick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.syscalltick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.schedwhen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.syscallwhen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            schedtick: __go_clone_0_0,
            syscalltick: __go_clone_1_0,
            schedwhen: __go_clone_2_0,
            syscallwhen: __go_clone_3_0,
        }
    }
}


impl Default for sysmontick {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            schedtick: __go_default_0_0,
            syscalltick: __go_default_1_0,
            schedwhen: __go_default_2_0,
            syscallwhen: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for sysmontick {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.schedtick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.syscalltick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.schedwhen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.syscallwhen.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for sysmontick {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pMask is an atomic bitstring with one bit per P.
#[derive(Debug, Clone, Default)]
pub struct pMask(pub Arc<Mutex<Option<Vec<u32>>>>);

impl Display for pMask {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


/// A gQueue is a dequeue of Gs linked through g.schedlink. A G can only
/// be on one gQueue or gList at a time.
#[derive(Debug, Clone)]
pub struct gQueue {
    pub head: Arc<Mutex<Option<guintptr>>>,
    pub tail: Arc<Mutex<Option<guintptr>>>,
}

impl gQueue {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.head.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.tail.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            head: __go_clone_0_0,
            tail: __go_clone_1_0,
        }
    }
}


impl Default for gQueue {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0)))))));
        Self {
            head: __go_default_0_0,
            tail: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for gQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.head.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.tail.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for gQueue {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A gList is a list of Gs linked through g.schedlink. A G can only be
/// on one gQueue or gList at a time.
#[derive(Debug, Clone)]
pub struct gList {
    pub head: Arc<Mutex<Option<guintptr>>>,
}

impl gList {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.head.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            head: __go_clone_0_0,
        }
    }
}


impl Default for gList {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0)))))));
        Self {
            head: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for gList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.head.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for gList {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// randomOrder/randomEnum are helper types for randomized work stealing.
/// They allow to enumerate all Ps in different pseudo-random orders without repetitions.
/// The algorithm is based on the fact that if we have X such that X and GOMAXPROCS
/// are coprime, then a sequences of (i + X) % GOMAXPROCS gives the required enumeration.
#[derive(Debug, Clone)]
pub struct randomOrder {
    pub count: Arc<Mutex<Option<u32>>>,
    pub coprimes: Arc<Mutex<Option<Vec<u32>>>>,
}

impl randomOrder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.coprimes.clone();
        Self {
            count: __go_clone_0_0,
            coprimes: __go_clone_1_0,
        }
    }
}


impl Default for randomOrder {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            count: __go_default_0_0,
            coprimes: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for randomOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.count.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.coprimes));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for randomOrder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct randomEnum {
    pub i: Arc<Mutex<Option<u32>>>,
    pub count: Arc<Mutex<Option<u32>>>,
    pub pos: Arc<Mutex<Option<u32>>>,
    pub inc: Arc<Mutex<Option<u32>>>,
}

impl randomEnum {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.inc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            i: __go_clone_0_0,
            count: __go_clone_1_0,
            pos: __go_clone_2_0,
            inc: __go_clone_3_0,
        }
    }
}


impl Default for randomEnum {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            i: __go_default_0_0,
            count: __go_default_1_0,
            pos: __go_default_2_0,
            inc: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for randomEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.i.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.count.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.pos.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.inc.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for randomEnum {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An initTask represents the set of initializations that need to be done for a package.
/// Keep in sync with ../../test/noinit.go:initTask
#[derive(Debug, Clone)]
pub struct initTask {
    pub state: Arc<Mutex<Option<u32>>>,
    pub nfns: Arc<Mutex<Option<u32>>>,
}

impl initTask {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.nfns.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            state: __go_clone_0_0,
            nfns: __go_clone_1_0,
        }
    }
}


impl Default for initTask {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            state: __go_default_0_0,
            nfns: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for initTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.state.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.nfns.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for initTask {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct tracestat {
    pub active: Arc<Mutex<Option<bool>>>,
    pub id: Arc<Mutex<Option<u64>>>,
    pub allocs: Arc<Mutex<Option<u64>>>,
    pub bytes: Arc<Mutex<Option<u64>>>,
}

impl tracestat {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.active.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.allocs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.bytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            active: __go_clone_0_0,
            id: __go_clone_1_0,
            allocs: __go_clone_2_0,
            bytes: __go_clone_3_0,
        }
    }
}


impl Default for tracestat {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            active: __go_default_0_0,
            id: __go_default_1_0,
            allocs: __go_default_2_0,
            bytes: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for tracestat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.active.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.id.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.allocs.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.bytes.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for tracestat {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static modinfo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static m0: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::m>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static g0: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::g>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mcache0: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::mcache::mcache>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static raceprocctx0: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static raceFiniLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static runtime_inittasks: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<initTask>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static main_init_done: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoChannel<bool>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mainStarted: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static runtimeInitTime: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static initSigmask: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::os_darwin::sigset>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcrash: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::g>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static crashingG: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Pointer<crate::runtime2::g>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allglock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allgs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<crate::runtime2::g>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allglen: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allgptr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Arc<Mutex<Option<crate::runtime2::g>>>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static freezing: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static casgstatusAlwaysTrack: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stwReasonStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 17]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stopTheWorldContext: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<worldStop>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static worldsema: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcsema: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cgoThreadStart: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static extraM: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uintptr>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static extraMLength: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static extraMWaiters: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static extraMInUse: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allocmLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::rwmutex::rwmutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static execLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::rwmutex::rwmutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static newmHandoff: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct22>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static inForkedChild: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pendingPreemptSignals: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Int32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static prof: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct23>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static forcegcperiod: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static needSysmonWorkaround: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static starttime: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stealOrder: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<randomOrder>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static inittrace: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<tracestat>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *modinfo.lock().unwrap() = Some(String::new());
    *m0.lock().unwrap() = Some(Default::default());
    *g0.lock().unwrap() = Some(Default::default());
    *mcache0.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *raceprocctx0.lock().unwrap() = Some(0);
    *raceFiniLock.lock().unwrap() = Some(Default::default());
    *runtime_inittasks.lock().unwrap() = Some(vec![]);
    *main_init_done.lock().unwrap() = Some(Default::default());
    *mainStarted.lock().unwrap() = Some(false);
    *runtimeInitTime.lock().unwrap() = Some(0);
    *initSigmask.lock().unwrap() = Some(crate::os_darwin::sigset(Arc::new(Mutex::new(Some(0)))));
    *gcrash.lock().unwrap() = Some(Default::default());
    *crashingG.lock().unwrap() = Some(Default::default());
    *allglock.lock().unwrap() = Some(Default::default());
    *allgs.lock().unwrap() = Some(vec![]);
    *allglen.lock().unwrap() = Some(0);
    *allgptr.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *freezing.lock().unwrap() = Some(Default::default());
    *casgstatusAlwaysTrack.lock().unwrap() = Some(false);
    *stwReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *stopTheWorldContext.lock().unwrap() = Some(Default::default());
    *worldsema.lock().unwrap() = Some(0);
    *gcsema.lock().unwrap() = Some(0);
    *cgoThreadStart.lock().unwrap() = Some(0);
    *extraM.lock().unwrap() = Some(Default::default());
    *extraMLength.lock().unwrap() = Some(Default::default());
    *extraMWaiters.lock().unwrap() = Some(Default::default());
    *extraMInUse.lock().unwrap() = Some(Default::default());
    *allocmLock.lock().unwrap() = Some(Default::default());
    *execLock.lock().unwrap() = Some(Default::default());
    *newmHandoff.lock().unwrap() = Some(Default::default());
    *inForkedChild.lock().unwrap() = Some(false);
    *pendingPreemptSignals.lock().unwrap() = Some(Default::default());
    *prof.lock().unwrap() = Some(Default::default());
    *forcegcperiod.lock().unwrap() = Some(0);
    *needSysmonWorkaround.lock().unwrap() = Some(false);
    *starttime.lock().unwrap() = Some(0);
    *stealOrder.lock().unwrap() = Some(Default::default());
    *inittrace.lock().unwrap() = Some(Default::default());
    *casgstatusAlwaysTrack.lock().unwrap() = Some(false);
    {
        let mut __go_array = Vec::<String>::with_capacity(17);
        __go_array.push("unknown".to_string());
        __go_array.push("GC mark termination".to_string());
        __go_array.push("GC sweep termination".to_string());
        __go_array.push("write heap dump".to_string());
        __go_array.push("goroutine profile".to_string());
        __go_array.push("goroutine profile cleanup".to_string());
        __go_array.push("all goroutines stack trace".to_string());
        __go_array.push("read mem stats".to_string());
        __go_array.push("AllThreadsSyscall".to_string());
        __go_array.push("GOMAXPROCS".to_string());
        __go_array.push("start trace".to_string());
        __go_array.push("stop trace".to_string());
        __go_array.push("CountPagesInUse (test)".to_string());
        __go_array.push("ReadMetricsSlow (test)".to_string());
        __go_array.push("ReadMemStatsSlow (test)".to_string());
        __go_array.push("PageCachePagesLeaked (test)".to_string());
        __go_array.push("ResetDebugLog (test)".to_string());
        let __go_array: [String; 17] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *stwReasonStrings.lock().unwrap() = Some(__go_array);
    }
    *worldsema.lock().unwrap() = Some(1 as u32);
    *gcsema.lock().unwrap() = Some(1 as u32);
    *forcegcperiod.lock().unwrap() = Some({ let __tmp_x = 120.0; let __tmp_y = 1e+09; __tmp_x * __tmp_y } as i64);
    *needSysmonWorkaround.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *modinfo.lock().unwrap() = Some(String::new());
    *m0.lock().unwrap() = Some(Default::default());
    *g0.lock().unwrap() = Some(Default::default());
    *mcache0.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *raceprocctx0.lock().unwrap() = Some(0);
    *raceFiniLock.lock().unwrap() = Some(Default::default());
    *runtime_inittasks.lock().unwrap() = Some(vec![]);
    *main_init_done.lock().unwrap() = Some(Default::default());
    *mainStarted.lock().unwrap() = Some(false);
    *runtimeInitTime.lock().unwrap() = Some(0);
    *initSigmask.lock().unwrap() = Some(crate::os_darwin::sigset(Arc::new(Mutex::new(Some(0)))));
    *gcrash.lock().unwrap() = Some(Default::default());
    *crashingG.lock().unwrap() = Some(Default::default());
    *allglock.lock().unwrap() = Some(Default::default());
    *allgs.lock().unwrap() = Some(vec![]);
    *allglen.lock().unwrap() = Some(0);
    *allgptr.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *freezing.lock().unwrap() = Some(Default::default());
    *casgstatusAlwaysTrack.lock().unwrap() = Some(false);
    *stwReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *stopTheWorldContext.lock().unwrap() = Some(Default::default());
    *worldsema.lock().unwrap() = Some(0);
    *gcsema.lock().unwrap() = Some(0);
    *cgoThreadStart.lock().unwrap() = Some(0);
    *extraM.lock().unwrap() = Some(Default::default());
    *extraMLength.lock().unwrap() = Some(Default::default());
    *extraMWaiters.lock().unwrap() = Some(Default::default());
    *extraMInUse.lock().unwrap() = Some(Default::default());
    *allocmLock.lock().unwrap() = Some(Default::default());
    *execLock.lock().unwrap() = Some(Default::default());
    *newmHandoff.lock().unwrap() = Some(Default::default());
    *inForkedChild.lock().unwrap() = Some(false);
    *pendingPreemptSignals.lock().unwrap() = Some(Default::default());
    *prof.lock().unwrap() = Some(Default::default());
    *forcegcperiod.lock().unwrap() = Some(0);
    *needSysmonWorkaround.lock().unwrap() = Some(false);
    *starttime.lock().unwrap() = Some(0);
    *stealOrder.lock().unwrap() = Some(Default::default());
    *inittrace.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_56() {
    *casgstatusAlwaysTrack.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_57() {
    {
        let mut __go_array = Vec::<String>::with_capacity(17);
        __go_array.push("unknown".to_string());
        __go_array.push("GC mark termination".to_string());
        __go_array.push("GC sweep termination".to_string());
        __go_array.push("write heap dump".to_string());
        __go_array.push("goroutine profile".to_string());
        __go_array.push("goroutine profile cleanup".to_string());
        __go_array.push("all goroutines stack trace".to_string());
        __go_array.push("read mem stats".to_string());
        __go_array.push("AllThreadsSyscall".to_string());
        __go_array.push("GOMAXPROCS".to_string());
        __go_array.push("start trace".to_string());
        __go_array.push("stop trace".to_string());
        __go_array.push("CountPagesInUse (test)".to_string());
        __go_array.push("ReadMetricsSlow (test)".to_string());
        __go_array.push("ReadMemStatsSlow (test)".to_string());
        __go_array.push("PageCachePagesLeaked (test)".to_string());
        __go_array.push("ResetDebugLog (test)".to_string());
        let __go_array: [String; 17] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *stwReasonStrings.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_order_58() {
    *worldsema.lock().unwrap() = Some(1 as u32);
}


pub(crate) fn __go_init_order_59() {
    *gcsema.lock().unwrap() = Some(1 as u32);
}


pub(crate) fn __go_init_order_60() {
    *forcegcperiod.lock().unwrap() = Some({ let __tmp_x = 120.0; let __tmp_y = 1e+09; __tmp_x * __tmp_y } as i64);
}


pub(crate) fn __go_init_order_61() {
    *needSysmonWorkaround.lock().unwrap() = Some(false);
}


impl crate::runtime2::m {
    pub fn become_spinning(&mut self) {
        { let new_val = true; *self.spinning.lock().unwrap() = Some(new_val); };
        (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
        (*(*sched.lock().unwrap().as_ref().unwrap()).needspinning.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
    }

    pub fn has_cgo_on_stack(&self) -> bool {
        return { let __tmp_x = (*self.ncgo.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } || (*self.isextra.lock().unwrap().as_ref().unwrap());
    }
}

impl stwReason {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = stwReasonStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })))
    }

    pub fn is_g_c(&self) -> bool {
        return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = stwReason(Arc::new(Mutex::new(Some(STW_G_C_MARK_TERM as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = stwReason(Arc::new(Mutex::new(Some(STW_G_C_SWEEP_TERM as u8)))); __tmp_x == __tmp_y };
    }
}

impl crate::runtime2::p {
    /// init initializes pp, which may be a freshly allocated p or a
    /// previously destroyed p, and transitions it to status _Pgcstop.
    pub fn init(&mut self, id: Arc<Mutex<Option<i32>>>) {
        { let new_val = id.lock().unwrap().as_ref().unwrap().clone(); *self.id.lock().unwrap() = Some(new_val); };
        { let new_val = __PGCSTOP as u32; *self.status.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.sudogbuf.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = (0) as usize;
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); self.sudogcache = new_val; };
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.deferpoolbuf.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = (0) as usize;
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); self.deferpool = new_val; };
        (*self.wb_buf.lock().unwrap().as_mut().unwrap()).reset();
        if { let __ptr_field = self.mcache.clone(); __ptr_field.is_nil() } {
        if { let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        if { let __slot_guard = mcache0.lock().unwrap(); let __not_nil = __slot_guard.as_ref().map(|__ptr| (*__ptr.lock().unwrap()).is_some()).unwrap_or(false); !__not_nil } {
        throw(Arc::new(Mutex::new(Some("missing mcache?".to_string()))));
    }
                // Use the bootstrap mcache0. Only one P will get
                // mcache0: the one with ID 0.
        { let new_val = GoPtr::local((*mcache0.lock().unwrap().as_ref().unwrap()).clone()); self.mcache = new_val; };
    } else {
        { let new_val = allocmcache(); self.mcache = new_val; };
    }
    }
                // Use the bootstrap mcache0. Only one P will get
                // mcache0: the one with ID 0.
        if RACEENABLED && { let __tmp_x = (*self.raceprocctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        { let new_val = raceprocctx0.lock().unwrap().as_ref().unwrap().clone(); *self.raceprocctx.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *raceprocctx0.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = raceproccreate(); *self.raceprocctx.lock().unwrap() = Some(new_val); };
    }
    }
                // bootstrap
        lock_init(GoPtr::local((*self.timers.lock().unwrap().as_ref().unwrap()).mu.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))))));
                // This P may get timers when it starts running. Set the mask here
                // since the P may not go through pidleget (notably P 0 on startup).
        (*timerpMask.lock().unwrap().as_ref().unwrap()).set(Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Similarly, we may not go through pidleget before this P starts
                // running if it is P 0 on startup.
        (*idlepMask.lock().unwrap().as_ref().unwrap()).clear(Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// destroy releases all of the resources associated with pp and
    /// transitions it to status _Pdead.
    ///
    /// sched.lock must be held and the world must be stopped.
    pub fn destroy(&mut self) {
        assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        assert_world_stopped();
                // Move all runnable goroutines to the global queue
        while { let __tmp_x = (*self.runqhead.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.runqtail.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
                // Pop from tail of local queue
        { let __target = self.runqtail.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&({ let __seq = { let __seq_holder = self.runq.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
            let __tmp_x = (*self.runqtail.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*self.runq.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
            __tmp_x % __tmp_y
        }) as usize].clone() }));

                // Push onto head of global queue
        globrunqputhead(gp.clone());
    }
                // Pop from tail of local queue
                // Push onto head of global queue
        if {
            let __tmp_x = { let __selector_holder = self.runnext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
        globrunqputhead(crate::runtime2::guintptr::ptr(&(*self.runnext.lock().unwrap().as_ref().unwrap())));
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); *self.runnext.lock().unwrap() = Some(new_val); };
    }
                // Move all timers to the local P.
        (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().timers.clone() }.lock().unwrap().as_mut().unwrap()).take(self.timers.clone());
                // Flush p's write barrier buffer.
        if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x != __tmp_y } {
        wb_buf_flush1(GoPtr::local(Arc::new(Mutex::new(Some(self.clone())))));
        (*self.gcw.lock().unwrap().as_mut().unwrap()).dispose();
    }
        for i in 0..(({ let __range_holder = self.sudogbuf.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*self.sudogbuf.lock().unwrap().as_mut().unwrap())[(i) as usize] = Default::default();
    }
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.sudogbuf.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = (0) as usize;
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); self.sudogcache = new_val; };
        *self.pinner_cache.lock().unwrap() = None;
        for j in 0..(({ let __range_holder = self.deferpoolbuf.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*self.deferpoolbuf.lock().unwrap().as_mut().unwrap())[(j) as usize] = Default::default();
    }
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.deferpoolbuf.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = (0) as usize;
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); self.deferpool = new_val; };
        let mut pp_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*pp_closure_clone.mspancache.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        (*(*mheap_.lock().unwrap().as_ref().unwrap()).spanalloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*pp_closure_clone.mspancache.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.addr()))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = 0; *(*pp_closure_clone.mspancache.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*pp_closure_clone.pcache.lock().unwrap().as_mut().unwrap()).flush((*mheap_.lock().unwrap().as_ref().unwrap()).pages.clone());
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
                // Safe to call since the world is stopped.
        freemcache(self.mcache.clone());
        { let new_val = GoPtr::nil(); self.mcache = new_val; };
        gfpurge(Arc::new(Mutex::new(Some(self.clone()))));
        if RACEENABLED {
        if { let __tmp_x = (*(*self.timers.lock().unwrap().as_ref().unwrap()).race_ctx.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // The race detector code uses a callback to fetch
                // the proc context, so arrange for that callback
                // to see the right thing.
                // This hack only works because we are the only
                // thread running.
        let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
        let mut phold: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        (*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(Arc::new(Mutex::new(Some(self.clone())))));
        racectxend(Arc::new(Mutex::new(Some({ let __selector_holder = (*self.timers.lock().unwrap().as_ref().unwrap()).race_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as usize; *(*self.timers.lock().unwrap().as_ref().unwrap()).race_ctx.lock().unwrap() = Some(new_val); };
        (*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_mut().unwrap()).set(phold.clone());
    }
                // The race detector code uses a callback to fetch
                // the proc context, so arrange for that callback
                // to see the right thing.
                // This hack only works because we are the only
                // thread running.
        raceprocdestroy(Arc::new(Mutex::new(Some({ let __selector_holder = self.raceprocctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as usize; *self.raceprocctx.lock().unwrap() = Some(new_val); };
    }
                // The race detector code uses a callback to fetch
                // the proc context, so arrange for that callback
                // to see the right thing.
                // This hack only works because we are the only
                // thread running.
        { let new_val = 0 as i64; *self.gc_assist_time.lock().unwrap() = Some(new_val); };
        { let new_val = __PDEAD as u32; *self.status.lock().unwrap() = Some(new_val); };
    }
}

impl pMask {
    /// read returns true if P id's bit is set.
    pub fn read(&self, id: Arc<Mutex<Option<u32>>>) -> bool {
        let mut word = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u32; __tmp_x / __tmp_y })));
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = (1 as u32); let __tmp_y = ({ let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u32; __tmp_x % __tmp_y }); __tmp_x << __tmp_y })));
        return { let __tmp_x = ({ let __tmp_x = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(self.0.clone(), ({ let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))); let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }); let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }

    /// set sets P id's bit.
    pub fn set(&self, id: Arc<Mutex<Option<i32>>>) {
        let mut word = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as i32; __tmp_x / __tmp_y })));
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = (1 as u32); let __tmp_y = ({ let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as i32; __tmp_x % __tmp_y }); __tmp_x << __tmp_y })));
        { let __elem_ptr_0 = Some(GoSliceElemPtr::new(self.0.clone(), ({ let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)); let __arg0 = Arc::new(Mutex::new(__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); let __result = internal_runtime_atomic::or(__arg0.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); if let Some(__ptr) = __elem_ptr_0.as_ref() { let mut __elem_guard_0 = __ptr.borrow_mut(); *__elem_guard_0 = (*__arg0.lock().unwrap()).clone(); }; __result };
    }

    /// clear clears P id's bit.
    pub fn clear(&self, id: Arc<Mutex<Option<i32>>>) {
        let mut word = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as i32; __tmp_x / __tmp_y })));
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = (1 as u32); let __tmp_y = ({ let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as i32; __tmp_x % __tmp_y }); __tmp_x << __tmp_y })));
        { let __elem_ptr_0 = Some(GoSliceElemPtr::new(self.0.clone(), ({ let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)); let __arg0 = Arc::new(Mutex::new(__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); let __result = internal_runtime_atomic::and(__arg0.clone(), Arc::new(Mutex::new(Some(!(*mask.lock().unwrap().as_ref().unwrap()))))); if let Some(__ptr) = __elem_ptr_0.as_ref() { let mut __elem_guard_0 = __ptr.borrow_mut(); *__elem_guard_0 = (*__arg0.lock().unwrap()).clone(); }; __result };
    }
}

impl gQueue {
    /// empty reports whether q is empty.
    pub fn empty(&self) -> bool {
        return {
            let __tmp_x = { let __selector_holder = self.head.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x == __tmp_y
        };
    }

    /// push adds gp to the head of q.
    pub fn push(&self, gp: GoPtr<crate::runtime2::g>) {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*self.head.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        (*self.head.lock().unwrap().as_mut().unwrap()).set(gp.clone());
        if {
            let __tmp_x = { let __selector_holder = self.tail.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x == __tmp_y
        } {
        (*self.tail.lock().unwrap().as_mut().unwrap()).set(gp.clone());
    }
    }

    /// pushBack adds gp to the tail of q.
    pub fn push_back(&self, gp: GoPtr<crate::runtime2::g>) {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        if {
            let __tmp_x = { let __selector_holder = self.tail.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
        (*{ let __ptr = crate::runtime2::guintptr::ptr(&(*self.tail.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().schedlink.clone() }.lock().unwrap().as_mut().unwrap()).set(gp.clone());
    } else {
        (*self.head.lock().unwrap().as_mut().unwrap()).set(gp.clone());
    }
        (*self.tail.lock().unwrap().as_mut().unwrap()).set(gp.clone());
    }

    /// pushBackAll adds all Gs in q2 to the tail of q. After this q2 must
    /// not be used.
    pub fn push_back_all(&mut self, q2: Arc<Mutex<Option<gQueue>>>) {
        if {
            let __tmp_x = { let __selector_holder = (*q2.lock().unwrap().as_ref().unwrap()).tail.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x == __tmp_y
        } {
        return;
    }
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr = crate::runtime2::guintptr::ptr(&(*(*q2.lock().unwrap().as_ref().unwrap()).tail.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().schedlink.clone() }.lock().unwrap() = Some(new_val); };
        if {
            let __tmp_x = { let __selector_holder = self.tail.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*(*q2.lock().unwrap().as_ref().unwrap()).head.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr = crate::runtime2::guintptr::ptr(&(*self.tail.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().schedlink.clone() }.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*(*q2.lock().unwrap().as_ref().unwrap()).head.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.head.lock().unwrap() = Some(new_val); };
    }
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*(*q2.lock().unwrap().as_ref().unwrap()).tail.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.tail.lock().unwrap() = Some(new_val); };
    }

    /// pop removes and returns the head of queue q. It returns nil if
    /// q is empty.
    pub fn pop(&mut self) -> GoPtr<crate::runtime2::g> {
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*self.head.lock().unwrap().as_ref().unwrap()));
        if !gp.is_nil() {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.head.lock().unwrap() = Some(new_val); };
        if {
            let __tmp_x = { let __selector_holder = self.head.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x == __tmp_y
        } {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); *self.tail.lock().unwrap() = Some(new_val); };
    }
    }
        gp.clone()
    }

    /// popList takes all Gs in q and returns them as a gList.
    pub fn pop_list(&mut self) -> Arc<Mutex<Option<gList>>> {
        let mut stack = Arc::new(Mutex::new(Some(gList { head: Arc::new(Mutex::new(Some({ let __selector_holder = self.head.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
        { let new_val = gQueue { head: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))), tail: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))) }; *self = new_val; };
        return { let __owned = stack.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
}

impl gList {
    /// empty reports whether l is empty.
    pub fn empty(&self) -> bool {
        return {
            let __tmp_x = { let __selector_holder = self.head.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x == __tmp_y
        };
    }

    /// push adds gp to the head of l.
    pub fn push(&self, gp: GoPtr<crate::runtime2::g>) {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*self.head.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        (*self.head.lock().unwrap().as_mut().unwrap()).set(gp.clone());
    }

    /// pushAll prepends all Gs in q to l.
    pub fn push_all(&mut self, q: Arc<Mutex<Option<gQueue>>>) {
        if !(*q.lock().unwrap().as_ref().unwrap()).empty() {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*self.head.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr = crate::runtime2::guintptr::ptr(&(*(*q.lock().unwrap().as_ref().unwrap()).tail.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().schedlink.clone() }.lock().unwrap() = Some(new_val); };
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*(*q.lock().unwrap().as_ref().unwrap()).head.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.head.lock().unwrap() = Some(new_val); };
    }
    }

    /// pop removes and returns the head of l. If l is empty, it returns nil.
    pub fn pop(&mut self) -> GoPtr<crate::runtime2::g> {
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*self.head.lock().unwrap().as_ref().unwrap()));
        if !gp.is_nil() {
        { let new_val = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.head.lock().unwrap() = Some(new_val); };
    }
        gp.clone()
    }
}

impl randomOrder {
    pub fn reset(&mut self, count: Arc<Mutex<Option<u32>>>) {
        { let new_val = count.lock().unwrap().as_ref().unwrap().clone(); *self.count.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.coprimes.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = (0) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); self.coprimes = new_val; };
        let mut i = Arc::new(Mutex::new(Some(1 as u32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        if { let __tmp_x = gcd(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = count.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 1 as u32; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = self.coprimes.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*i.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; self.coprimes = new_val; };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    pub fn start(&self, i: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<randomEnum>>> {
        Arc::new(Mutex::new(Some(randomEnum { count: Arc::new(Mutex::new(Some({ let __selector_holder = self.count.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), pos: Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.count.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }))), inc: Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.coprimes.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({
            let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.count.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y };
            let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.coprimes.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))).lock().unwrap().as_ref().unwrap());
            __tmp_x % __tmp_y
        }) as usize].clone() }))), ..Default::default() })))
    }
}

impl randomEnum {
    pub fn done(&self) -> bool {
        return { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.count.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y };
    }

    pub fn next(&mut self) {
        { let __target = self.i.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = {
            let __tmp_x = ({ let __tmp_x = (*self.pos.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.inc.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y });
            let __tmp_y = (*self.count.lock().unwrap().as_ref().unwrap());
            __tmp_x % __tmp_y
        }; *self.pos.lock().unwrap() = Some(new_val); };
    }

    pub fn position(&self) -> u32 {
        return (*self.pos.lock().unwrap().as_ref().unwrap());
    }
}

fn __go_init_0() {
    { let new_val = Box::new(move || { gosched() }) as Box<dyn FnMut() -> () + Send + Sync>; *internal_runtime_exithook::Gosched.lock().unwrap() = Some(new_val); };
    { let new_val = Box::new(move || -> u64 {
        return (*(*getg().lock().unwrap().as_ref().unwrap()).goid.lock().unwrap().as_ref().unwrap());
    }) as Box<dyn FnMut() -> u64 + Send + Sync>; *internal_runtime_exithook::Goid.lock().unwrap() = Some(new_val); };
    { let new_val = Box::new(move |__arg0: Arc<Mutex<Option<String>>>| { throw(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync>; *internal_runtime_exithook::Throw.lock().unwrap() = Some(new_val); };
}

/// start forcegc helper goroutine
fn __go_init_1() {
    std::thread::spawn(move || {
        forcegchelper();
    });
}

pub fn forcegchelper() {
    { let new_val = getg().clone(); (*forcegc.lock().unwrap().as_mut().unwrap()).g = new_val; };
    lock_init(GoPtr::local((*forcegc.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32))))))));
    loop {
        lock(GoPtr::local((*forcegc.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if (*(*forcegc.lock().unwrap().as_ref().unwrap()).idle.lock().unwrap().as_ref().unwrap()).load() {
        throw(Arc::new(Mutex::new(Some("forcegc: phase error".to_string()))));
    }
        (*(*forcegc.lock().unwrap().as_ref().unwrap()).idle.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
        goparkunlock((*forcegc.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_FORCE_G_C_IDLE as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SYSTEM_GOROUTINE as u8))))))), Arc::new(Mutex::new(Some(1))));

                // this goroutine is explicitly resumed by sysmon
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gctrace.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "GC forced".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
    }

                // Time-triggered, fully concurrent.
        gc_start(Arc::new(Mutex::new(Some(gcTrigger { kind: Arc::new(Mutex::new(Some(crate::mgc::gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_TIME as i32))))))), now: Arc::new(Mutex::new(Some(nanotime()))), ..Default::default() }))));
    }
}

/// Gosched yields the processor, allowing other goroutines to run. It does not
/// suspend the current goroutine, so execution resumes automatically.
///
///go:nosplit
pub fn gosched() {
    check_timeouts();
    mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { gosched_m(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
}

/// goschedguarded yields the processor like gosched, but also checks
/// for forbidden states and opts out of the yield in those cases.
///
///go:nosplit
pub fn goschedguarded() {
    mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { goschedguarded_m(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
}

/// Puts the current goroutine into a waiting state and calls unlockf on the
/// system stack.
///
/// If unlockf returns false, the goroutine is resumed.
///
/// unlockf must not access this G's stack, as it may be moved between
/// the call to gopark and the call to unlockf.
///
/// Note that because unlockf is called after putting the G into a waiting
/// state, the G may have already been readied by the time unlockf is called
/// unless there is external synchronization preventing the G from being
/// readied. If unlockf returns false, it must guarantee that the G cannot be
/// externally readied.
///
/// Reason explains why the goroutine has been parked. It is displayed in stack
/// traces and heap dumps. Reasons should be unique and descriptive. Do not
/// re-use reasons, add new ones.
///
/// gopark should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gvisor.dev/gvisor
///   - github.com/sagernet/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname gopark
pub fn gopark(unlockf: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>, lock: Arc<Mutex<Option<usize>>>, reason: Arc<Mutex<Option<waitReason>>>, traceReason: Arc<Mutex<Option<traceBlockReason>>>, traceskip: Arc<Mutex<Option<i32>>>) {
    if { let __tmp_x = (*reason.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SLEEP as u8)))); __tmp_x != __tmp_y } {
        check_timeouts();
    }
        // timeouts may expire while two goroutines keep the scheduler busy
    let mut mp = acquirem();
    let mut gp: GoPtr<crate::runtime2::g> = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone();
    let mut status = readgstatus(gp.clone());
    if { let __tmp_x = status; let __tmp_y = __GRUNNING as u32; __tmp_x != __tmp_y } && { let __tmp_x = status; let __tmp_y = __GSCANRUNNING as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("gopark: bad g status".to_string()))));
    }
    { let new_val = lock.lock().unwrap().as_ref().unwrap().clone(); *(*mp.lock().unwrap().as_ref().unwrap()).waitlock.lock().unwrap() = Some(new_val); };
    { let new_val = unlockf.clone(); (*mp.lock().unwrap().as_mut().unwrap()).waitunlockf = new_val; };
    { let new_val = reason.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = traceReason.lock().unwrap().as_ref().unwrap().clone(); *(*mp.lock().unwrap().as_ref().unwrap()).wait_trace_block_reason.lock().unwrap() = Some(new_val); };
    { let new_val = traceskip.lock().unwrap().as_ref().unwrap().clone(); *(*mp.lock().unwrap().as_ref().unwrap()).wait_trace_skip.lock().unwrap() = Some(new_val); };
    releasem(GoPtr::local(mp.clone()));

        // can't do anything that might move the G between Ms here.
    mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { park_m(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
}

/// Puts the current goroutine into a waiting state and unlocks the lock.
/// The goroutine can be made runnable again by calling goready(gp).
pub fn goparkunlock(lock: Arc<Mutex<Option<mutex>>>, reason: Arc<Mutex<Option<waitReason>>>, traceReason: Arc<Mutex<Option<traceBlockReason>>>, traceskip: Arc<Mutex<Option<i32>>>) {
    gopark(
        Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>, __arg1: Arc<Mutex<Option<usize>>>| -> bool { parkunlock_c(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>))),
        Arc::new(Mutex::new(Some(Arc::as_ptr(&lock) as usize))),
        Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = traceReason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = traceskip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
    );
}

/// goready should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gvisor.dev/gvisor
///   - github.com/sagernet/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname goready
pub fn goready(gp: GoPtr<crate::runtime2::g>, traceskip: Arc<Mutex<Option<i32>>>) {
    let gp_closure_clone = gp.clone(); let traceskip_closure_clone = traceskip.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        ready(gp_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = traceskip_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

///go:nosplit
pub fn acquire_sudog() -> Arc<Mutex<Option<crate::runtime2::sudog>>> {
        // Delicate dance: the semaphore implementation calls
        // acquireSudog, acquireSudog calls new(sudog),
        // new calls malloc, malloc can call the garbage collector,
        // and the garbage collector calls the semaphore implementation
        // in stopTheWorld.
        // Break the cycle by doing acquirem/releasem around new(sudog).
        // The acquirem/releasem increments m.locks during new(sudog),
        // which keeps the garbage collector from being invoked.
    let mut mp = acquirem();
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).sudoglock.clone()));
                // First, try to grab a batch from central cache.
        while {
            let __go_cond_0 = {
                let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
                let __tmp_y = ({ let __tmp_x = (({ let __cap_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); let __tmp_y = 2; __tmp_x / __tmp_y } as i32);
                __tmp_x < __tmp_y
            };
            if __go_cond_0 {
                let __go_cond_1 = { let __nil_target = (*sched.lock().unwrap().as_ref().unwrap()).sudogcache.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
                __go_cond_1
            } else {
                false
            }
        } {
        let mut s = (*sched.lock().unwrap().as_ref().unwrap()).sudogcache.clone();
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); (*sched.lock().unwrap().as_mut().unwrap()).sudogcache = new_val; };
        *(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        { let new_val = { let __append_target = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(s.clone()); __append_target.clone() }; pp.with_mut(|__ptr_value| { __ptr_value.sudogcache = new_val; }); };
    }
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).sudoglock.clone()));
                // If the central cache is empty, allocate a new one.
        if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(sudog::default())))); __append_target.clone() }; pp.with_mut(|__ptr_value| { __ptr_value.sudogcache = new_val; }); };
    }
    }
        // First, try to grab a batch from central cache.
        // If the central cache is empty, allocate a new one.
    let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
    let mut s = { let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone();
    (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = Default::default();
    { let new_val = Arc::new(Mutex::new(Some({
        let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        drop(__seq_guard);
        let __low = 0;
        let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
        let __max = __source_cap;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }))); pp.with_mut(|__ptr_value| { __ptr_value.sudogcache = new_val; }); };
    if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).elem.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("acquireSudog: found s.elem != nil in cache".to_string()))));
    }
    releasem(GoPtr::local(mp.clone()));
    return s.clone();
}

///go:nosplit
pub fn release_sudog(s: Arc<Mutex<Option<sudog>>>) {
    if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).elem.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("runtime: sudog with non-nil elem".to_string()))));
    }
    if (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).is_select.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("runtime: sudog with non-false isSelect".to_string()))));
    }
    if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("runtime: sudog with non-nil next".to_string()))));
    }
    if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("runtime: sudog with non-nil prev".to_string()))));
    }
    if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).waitlink.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("runtime: sudog with non-nil waitlink".to_string()))));
    }
    if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).c.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("runtime: sudog with non-nil c".to_string()))));
    }
    let mut gp = getg();
    if { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).param.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("runtime: releaseSudog with non-nil gp.param".to_string()))));
    }
    let mut mp = acquirem();
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    if {
        let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
        let __tmp_y = (({ let __cap_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32);
        __tmp_x == __tmp_y
    } {
                // Transfer half of local cache to the central cache.
        let mut first: Arc<Mutex<Option<sudog>>> = Arc::new(Mutex::new(None));let mut last: Arc<Mutex<Option<sudog>>> = Arc::new(Mutex::new(None));
        while {
            let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
            let __tmp_y = ({ let __tmp_x = (({ let __cap_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); let __tmp_y = 2; __tmp_x / __tmp_y } as i32);
            __tmp_x > __tmp_y
        } {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        let mut p = { let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone();
        (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = Default::default();
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); pp.with_mut(|__ptr_value| { __ptr_value.sudogcache = new_val; }); };
        if { let __nil_result = (*first.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = p.clone(); first = new_val; };
    } else {
        { let new_val = p.clone(); (*last.lock().unwrap().as_mut().unwrap()).next = new_val; };
    }
        { let new_val = p.clone(); last = new_val; };
    }
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).sudoglock.clone()));
        { let new_val = (*sched.lock().unwrap().as_ref().unwrap()).sudogcache.clone(); (*last.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = first.clone(); (*sched.lock().unwrap().as_mut().unwrap()).sudogcache = new_val; };
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).sudoglock.clone()));
    }
        // Transfer half of local cache to the central cache.
    { let new_val = { let __append_target = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.sudogcache.clone()); __ptr_value }.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(s.clone()); __append_target.clone() }; pp.with_mut(|__ptr_value| { __ptr_value.sudogcache = new_val; }); };
    releasem(GoPtr::local(mp.clone()));
}

///go:nosplit
pub fn badctxt() {
    throw(Arc::new(Mutex::new(Some("ctxt != 0".to_string()))));
}

/// allGsSnapshot returns a snapshot of the slice of all Gs.
///
/// The world must be stopped or allglock must be held.
pub fn all_gs_snapshot() -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::runtime2::g>>>>>>> {
    assert_world_stopped_or_lock_held(allglock.clone());

        // Because the world is stopped or allglock is held, allgadd
        // cannot happen concurrently with this. allgs grows
        // monotonically and existing entries never change, so we can
        // simply return a copy of the slice header. For added safety,
        // we trim everything past len because that can still change.
    Arc::new(Mutex::new(Some({
        let mut __seq = { let __seq_holder = allgs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned };
        let __low = 0;
        let __high = ((*allgs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize;
        let __max = ((*allgs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    })))
}

/// atomicAllG returns &allgs[0] and len(allgs) for use with atomicAllGIndex.
pub fn atomic_all_g() -> (GoPtr<GoPtr<crate::runtime2::g>>, usize) {
    let mut length = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local(allglen.clone()));
    let mut ptr: GoPtr<GoPtr<crate::runtime2::g>> = GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(allgptr.clone())))) as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    (ptr.clone(), length)
}

/// atomicAllGIndex returns ptr[i] with the allgptr returned from atomicAllG.
pub fn atomic_all_g_index(ptr: GoPtr<GoPtr<crate::runtime2::g>>, i: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::runtime2::g>>> {
    { let __v = (*Arc::new(Mutex::new({ let __ptr = add(Arc::new(Mutex::new(Some(ptr.addr()))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Arc<Mutex<Option<g>>>>(unimplemented!("unsafe.Pointer conversion to Arc<Mutex<Option<g>>>")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }
}

/// forEachG calls fn on every G from allgs.
///
/// forEachG takes a lock to exclude concurrent addition of new Gs.
pub fn for_each_g(r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>>>>) {
    lock(GoPtr::local(allglock.clone()));
    { let __range_holder = allgs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for gp in __range_values.iter() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*gp).clone()) };
    } }
    unlock(GoPtr::local(allglock.clone()));
}

/// forEachGRace calls fn on every G from allgs.
///
/// forEachGRace avoids locking, but does not exclude addition of new Gs during
/// execution, which may be missed.
pub fn for_each_g_race(r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>>>>) {
    let (mut ptr, mut length) = atomic_all_g();
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = length; __tmp_x < __tmp_y } {
        let mut gp = atomic_all_g_index(ptr.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(gp.clone()) };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    ()
}

pub fn dumpgstatus(gp: GoPtr<crate::runtime2::g>) {
    let mut thisg = getg();
    {
            let __go_print_arg_0 = format!("{}", "runtime:   gp: gp=".to_string());
            let __go_print_arg_1 = format!("{}", format!("0x{:x}", gp.addr()));
            let __go_print_arg_2 = format!("{}", ", goid=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", ", gp->atomicstatus=".to_string());
            let __go_print_arg_5 = format!("{}", readgstatus(gp.clone()));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
    {
            let __go_print_arg_0 = format!("{}", "runtime: getg:  g=".to_string());
            let __go_print_arg_1 = format!("{}", format!("&{}", (*thisg.lock().unwrap().as_ref().unwrap())));
            let __go_print_arg_2 = format!("{}", ", goid=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*thisg.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", ",  g->atomicstatus=".to_string());
            let __go_print_arg_5 = format!("{}", readgstatus(GoPtr::local(thisg.clone())));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
}

/// sched.lock must be held.
pub fn checkmcount() {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Exclude extra M's, which are used for cgocallback from threads
        // created in C.
        //
        // The purpose of the SetMaxThreads limit is to avoid accidental fork
        // bomb from something like millions of goroutines blocking on system
        // calls, causing the runtime to create millions of threads. By
        // definition, this isn't a problem for threads created in C, so we
        // exclude them from the limit. See https://go.dev/issue/60004.
    let mut count = Arc::new(Mutex::new(Some({
        let __tmp_x = { let __tmp_x = mcount(); let __tmp_y = (*Arc::new(Mutex::new(Some((*extraMInUse.lock().unwrap().as_mut().unwrap()).load() as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y };
        let __tmp_y = (*Arc::new(Mutex::new(Some((*extraMLength.lock().unwrap().as_mut().unwrap()).load() as i32))).lock().unwrap().as_ref().unwrap());
        __tmp_x - __tmp_y
    })));
    if { let __tmp_x = { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).maxmcount.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: program exceeds ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).maxmcount.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "-thread limit\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        throw(Arc::new(Mutex::new(Some("thread exhaustion".to_string()))));
    }
}

/// mReserveID returns the next ID to use for a new m. This new m is immediately
/// considered 'running' by checkdead.
///
/// sched.lock must be held.
pub fn m_reserve_i_d() -> i64 {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if {
        let __tmp_x = { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).mnext.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i64; __tmp_x + __tmp_y };
        let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).mnext.clone(); __field }.lock().unwrap().as_ref().unwrap());
        __tmp_x < __tmp_y
    } {
        throw(Arc::new(Mutex::new(Some("runtime: thread ID overflow".to_string()))));
    }
    let mut id = Arc::new(Mutex::new(Some({ let __selector_holder = (*sched.lock().unwrap().as_ref().unwrap()).mnext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).mnext.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    checkmcount();
    return { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// Pre-allocated ID may be passed as 'id', or omitted by passing -1.
pub fn mcommoninit(mp: Arc<Mutex<Option<m>>>, id: Arc<Mutex<Option<i64>>>) {
    let mut gp = getg();

        // g0 stack won't make sense for user (and is not necessary unwindable).
    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        callers_1(Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some({
            let __seq_holder = (*mp.lock().unwrap().as_ref().unwrap()).createstack.clone();
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
        }))));
    }

    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if { let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        { let new_val = id.lock().unwrap().as_ref().unwrap().clone(); *(*mp.lock().unwrap().as_ref().unwrap()).id.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = m_reserve_i_d(); *(*mp.lock().unwrap().as_ref().unwrap()).id.lock().unwrap() = Some(new_val); };
    }

    mrandinit(mp.clone());

    mpreinit(mp.clone());
    if { let __nil_target = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = {
            let __tmp_x = (*(*(*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
            let __tmp_y = STACK_GUARD as usize;
            __tmp_x + __tmp_y
        }; *(*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stackguard1.lock().unwrap() = Some(new_val); };
    }

        // Add to allm so garbage collector doesn't free g->m
        // when it is just in a register or thread-local storage.
    { let new_val = (*allm.lock().unwrap().as_ref().unwrap()).clone(); (*mp.lock().unwrap().as_mut().unwrap()).alllink = new_val; };

        // NumCgoCall() and others iterate over allm w/o schedlock,
        // so we need to publish it safely.
    atomicstorep(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(allm.clone())))) as usize))), Arc::new(Mutex::new(Some(Arc::as_ptr(&mp) as usize))));
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Allocate memory to hold a cgo traceback if the cgo call crashes.
    if (*iscgo.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = "darwin".to_string(); let __tmp_y = "solaris".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "illumos".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(cgoCallers::default()))).clone(); (*mp.lock().unwrap().as_mut().unwrap()).cgo_callers = new_val; };
    }
    m_prof_stack_init(mp.clone());
}

/// mProfStackInit is used to eagerly initialize stack trace buffers for
/// profiling. Lazy allocation would have to deal with reentrancy issues in
/// malloc and runtime locks for mLockProfile.
/// TODO(mknyszek): Implement lazy allocation if this becomes a problem.
pub fn m_prof_stack_init(mp: Arc<Mutex<Option<m>>>) {
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
                // debug.profstack is set to 0 by the user, or we're being called from
                // schedinit before parsedebugvars.
        return;
    }
        // debug.profstack is set to 0 by the user, or we're being called from
        // schedinit before parsedebugvars.
    { let new_val = make_prof_stack_f_p(); (*mp.lock().unwrap().as_mut().unwrap()).prof_stack = new_val; };
    { let new_val = make_prof_stack_f_p(); (*(*mp.lock().unwrap().as_ref().unwrap()).m_lock_profile.lock().unwrap().as_mut().unwrap()).stack = new_val; };
}

/// makeProfStackFP creates a buffer large enough to hold a maximum-sized stack
/// trace as well as any additional frames needed for frame pointer unwinding
/// with delayed inline expansion.
pub fn make_prof_stack_f_p() -> Arc<Mutex<Option<Vec<usize>>>> {
        // The "1" term is to account for the first stack entry being
        // taken up by a "skip" sentinel value for profilers which
        // defer inline frame expansion until the profile is reported.
        // The "maxSkip" term is for frame pointer unwinding, where we
        // want to end up with debug.profstackdebth frames but will discard
        // some "physical" frames to account for skipping.
    Arc::new(Mutex::new(Some(vec![0; ({ let __tmp_x = { let __tmp_x = 1; let __tmp_y = MAX_SKIP; __tmp_x + __tmp_y } as i32; let __tmp_y = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize])))
}

/// makeProfStack returns a buffer large enough to hold a maximum-sized stack
/// trace.
pub fn make_prof_stack() -> Arc<Mutex<Option<Vec<usize>>>> {
    Arc::new(Mutex::new(Some(vec![0; ((*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize])))
}

/// Mark gp ready to run.
pub fn ready(gp: GoPtr<crate::runtime2::g>, traceskip: Arc<Mutex<Option<i32>>>, next: Arc<Mutex<Option<bool>>>) {
    let mut status = readgstatus(gp.clone());

        // Mark runnable.
    let mut mp = acquirem();
    if { let __tmp_x = { let __tmp_x = status; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GWAITING as u32; __tmp_x != __tmp_y } {
        dumpgstatus(gp.clone());
        throw(Arc::new(Mutex::new(Some("bad g->status in ready".to_string()))));
    }

        // status is Gwaiting or Gscanwaiting, make Grunnable and put on runq
    let mut trace_local = trace_acquire();
    casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(gp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = traceskip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    runqput(
        crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())),
        gp.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
    );
    wakep();
    releasem(GoPtr::local(mp.clone()));
}

/// Similar to stopTheWorld but best-effort and can be called several times.
/// There is no reverse operation, used during crashing.
/// This function must not lock any mutexes.
pub fn freezetheworld() {
    (*freezing.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).dontfreezetheworld.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
                // Don't prempt Ps to stop goroutines. That will perturb
                // scheduler state, making debugging more difficult. Instead,
                // allow goroutines to continue execution.
                //
                // fatalpanic will tracebackothers to trace all goroutines. It
                // is unsafe to trace a running goroutine, so tracebackothers
                // will skip running goroutines. That is OK and expected, we
                // expect users of dontfreezetheworld to use core files anyway.
                //
                // However, allowing the scheduler to continue running free
                // introduces a race: a goroutine may be stopped when
                // tracebackothers checks its status, and then start running
                // later when we are in the middle of traceback, potentially
                // causing a crash.
                //
                // To mitigate this, when an M naturally enters the scheduler,
                // schedule checks if freezing is set and if so stops
                // execution. This guarantees that while Gs can transition from
                // running to stopped, they can never transition from stopped
                // to running.
                //
                // The sleep here allows racing Ms that missed freezing and are
                // about to run a G to complete the transition to running
                // before we start traceback.
        usleep(Arc::new(Mutex::new(Some(1000 as u32))));
        return;
    }

        // Don't prempt Ps to stop goroutines. That will perturb
        // scheduler state, making debugging more difficult. Instead,
        // allow goroutines to continue execution.
        //
        // fatalpanic will tracebackothers to trace all goroutines. It
        // is unsafe to trace a running goroutine, so tracebackothers
        // will skip running goroutines. That is OK and expected, we
        // expect users of dontfreezetheworld to use core files anyway.
        //
        // However, allowing the scheduler to continue running free
        // introduces a race: a goroutine may be stopped when
        // tracebackothers checks its status, and then start running
        // later when we are in the middle of traceback, potentially
        // causing a crash.
        //
        // To mitigate this, when an M naturally enters the scheduler,
        // schedule checks if freezing is set and if so stops
        // execution. This guarantees that while Gs can transition from
        // running to stopped, they can never transition from stopped
        // to running.
        //
        // The sleep here allows racing Ms that missed freezing and are
        // about to run a G to complete the transition to running
        // before we start traceback.
        // stopwait and preemption requests can be lost
        // due to races with concurrently executing threads,
        // so try several times
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
                // this should tell the scheduler to not start any new goroutines
        { let new_val = FREEZE_STOP_WAIT as i32; *(*sched.lock().unwrap().as_ref().unwrap()).stopwait.lock().unwrap() = Some(new_val); };
        (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));

                // this should stop running goroutines
        if !preemptall() {
        break
    }
                // no running goroutines
        usleep(Arc::new(Mutex::new(Some(1000 as u32))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // this should tell the scheduler to not start any new goroutines
        // this should stop running goroutines
        // no running goroutines
        // to be sure
    usleep(Arc::new(Mutex::new(Some(1000 as u32))));
    preemptall();
    usleep(Arc::new(Mutex::new(Some(1000 as u32))));
}

/// All reads and writes of g's status go through readgstatus, casgstatus
/// castogscanstatus, casfrom_Gscanstatus.
///
///go:nosplit
pub fn readgstatus(gp: GoPtr<crate::runtime2::g>) -> u32 {
    (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.atomicstatus.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load()
}

/// The Gscanstatuses are acting like locks and this releases them.
/// If it proves to be a performance hit we should be able to make these
/// simple atomic stores but for now we are going to throw if
/// we see an inconsistent state.
pub fn casfrom__gscanstatus(gp: GoPtr<crate::runtime2::g>, oldval: Arc<Mutex<Option<u32>>>, newval: Arc<Mutex<Option<u32>>>) {
    let mut success = Arc::new(Mutex::new(Some(false)));

        // Check that transition is valid.
    { let _switch_val = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (__GSCANRUNNABLE as u32) || _switch_val == (__GSCANWAITING as u32) || _switch_val == (__GSCANRUNNING as u32) || _switch_val == (__GSCANSYSCALL as u32) || _switch_val == (__GSCANPREEMPTED as u32) {
            if { let __tmp_x = { let __v = (*newval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; __tmp_x == __tmp_y } {
        { let new_val = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.atomicstatus.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some({ let __arg_holder = oldval.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = newval.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *success.lock().unwrap() = Some(new_val); };
    }
        } else {
            {
            let __go_print_arg_0 = format!("{}", "runtime: casfrom_Gscanstatus bad oldval gp=".to_string());
            let __go_print_arg_1 = format!("{}", format!("0x{:x}", gp.addr()));
            let __go_print_arg_2 = format!("{}", ", oldval=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*oldval.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", ", newval=".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*newval.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
            dumpgstatus(gp.clone());
            throw(Arc::new(Mutex::new(Some("casfrom_Gscanstatus:top gp->status is not in scan state".to_string()))));
        }
    }
    if !{ let __v = (*success.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: casfrom_Gscanstatus failed gp=".to_string());
            let __go_print_arg_1 = format!("{}", format!("0x{:x}", gp.addr()));
            let __go_print_arg_2 = format!("{}", ", oldval=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*oldval.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", ", newval=".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*newval.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        dumpgstatus(gp.clone());
        throw(Arc::new(Mutex::new(Some("casfrom_Gscanstatus: gp->status is not in scan state".to_string()))));
    }
    release_lock_rank_and_m(Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))))));
}

/// This will return false if the gp is not in the expected status and the cas fails.
/// This acts like a lock acquire while the casfromgstatus acts like a lock release.
pub fn castogscanstatus(gp: Arc<Mutex<Option<g>>>, oldval: Arc<Mutex<Option<u32>>>, newval: Arc<Mutex<Option<u32>>>) -> bool {
    { let _switch_val = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (__GRUNNABLE as u32) || _switch_val == (__GRUNNING as u32) || _switch_val == (__GWAITING as u32) || _switch_val == (__GSYSCALL as u32) {
            if { let __tmp_x = { let __v = (*newval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GSCAN as u32; __tmp_x | __tmp_y }; __tmp_x == __tmp_y } {
        let mut r = (*(*gp.lock().unwrap().as_ref().unwrap()).atomicstatus.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some({ let __arg_holder = oldval.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = newval.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if r {
        acquire_lock_rank_and_m(Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))))));
    }
        return r;
    }
        }
    }
    {
            let __go_print_arg_0 = format!("{}", "runtime: castogscanstatus oldval=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*oldval.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", " newval=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*newval.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
    throw(Arc::new(Mutex::new(Some("castogscanstatus".to_string()))));
    std::panic::panic_any(Box::new("not reached".to_string()) as Box<dyn Any + Send + Sync>);
}

/// If asked to move to or from a Gscanstatus this will throw. Use the castogscanstatus
/// and casfrom_Gscanstatus instead.
/// casgstatus will loop if the g->atomicstatus is in a Gscan status until the routine that
/// put it in the Gscan state is finished.
///
///go:nosplit
pub fn casgstatus(gp: GoPtr<crate::runtime2::g>, oldval: Arc<Mutex<Option<u32>>>, newval: Arc<Mutex<Option<u32>>>) {
    if ({ let __tmp_x = { let __tmp_x = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GSCAN as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y }) || ({ let __tmp_x = { let __tmp_x = { let __v = (*newval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GSCAN as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y }) || { let __tmp_x = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*newval.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        let newval_closure_clone = newval.clone(); let oldval_closure_clone = oldval.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "runtime: casgstatus: oldval=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*oldval_closure_clone.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", " newval=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*newval_closure_clone.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        throw(Arc::new(Mutex::new(Some("casgstatus: bad incoming values".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }

        // Call on the systemstack to prevent print and throw from counting
        // against the nosplit stack reservation.
    lock_with_rank_may_acquire(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))))));

        // See https://golang.org/cl/21503 for justification of the yield delay.
    const yieldDelay: i32 = 5 * 1000;

    let mut nextYield: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        // loop if gp->atomicstatus is in a scan state giving
        // GC time to finish and change the state to oldval.
    let mut i = Arc::new(Mutex::new(Some(0)));
    while !(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.atomicstatus.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some({ let __arg_holder = oldval.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = newval.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if { let __tmp_x = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GWAITING as u32; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.atomicstatus.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = __GRUNNABLE as u32; __tmp_x == __tmp_y } {
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        throw(Arc::new(Mutex::new(Some("casgstatus: waiting for Gwaiting but is Grunnable".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
                // Call on the systemstack to prevent throw from counting
                // against the nosplit stack reservation.
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = nanotime(); let __tmp_y = yieldDelay as i64; __tmp_x + __tmp_y }; *nextYield.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = nanotime(); let __tmp_y = { let __v = (*nextYield.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut x = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } && { let __tmp_x = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.atomicstatus.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        procyield(Arc::new(Mutex::new(Some(1 as u32))));
        { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } else {
        osyield();
        { let new_val = { let __tmp_x = nanotime(); let __tmp_y = ((yieldDelay as i64) / (2 as i64)) as i64; __tmp_x + __tmp_y }; *nextYield.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Call on the systemstack to prevent throw from counting
        // against the nosplit stack reservation.
    if { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sync_group.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let gp_closure_clone = gp.clone(); let newval_closure_clone = newval.clone(); let oldval_closure_clone = oldval.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        (*{ let __ptr_value = gp_closure_clone.with_mut(|__ptr_value| __ptr_value.sync_group.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).changegstatus(gp_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = oldval_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = newval_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }

    if { let __tmp_x = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y } {
                // Track every gTrackingPeriod time a goroutine transitions out of running.
        if (*casgstatusAlwaysTrack.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().tracking_seq.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = G_TRACKING_PERIOD as u8; __tmp_x % __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let new_val = true; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.tracking.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
        { let __target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.tracking_seq.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // Track every gTrackingPeriod time a goroutine transitions out of running.
    if !(*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().tracking.clone() }.lock().unwrap().as_ref().unwrap()) {
        return;
    }

        // Handle various kinds of tracking.
        //
        // Currently:
        // - Time spent in runnable.
        // - Time spent blocked on a sync.Mutex or sync.RWMutex.
    '__go_switch_1: loop {
        { let _switch_val = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (__GRUNNABLE as u32) {
                        // We transitioned out of runnable, so measure how much
                        // time we spent in this state and add it to
                        // runnableTime.
            let mut now = nanotime();
            { let __target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.runnable_time.clone()); __ptr_value }.clone(); let __rhs = { let __tmp_x = now; let __tmp_y = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().tracking_stamp.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
            { let new_val = 0 as i64; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.tracking_stamp.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (__GWAITING as u32) {
            if !crate::runtime2::waitReason::is_mutex_wait(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
                // Not blocking on a lock.
        break '__go_switch_1
    }
                        // Not blocking on a lock.
                        // Blocking on a lock, measure it. Note that because we're
                        // sampling, we have to multiply by our sampling period to get
                        // a more representative estimate of the absolute value.
                        // gTrackingPeriod also represents an accurate sampling period
                        // because we can only enter this state from _Grunning.
            let mut now = nanotime();
            (*(*sched.lock().unwrap().as_ref().unwrap()).total_mutex_wait_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = now; let __tmp_y = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().tracking_stamp.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = G_TRACKING_PERIOD as i64; __tmp_x * __tmp_y }))));
            { let new_val = 0 as i64; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.tracking_stamp.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        }
    };
        break;
    }
        // We transitioned out of runnable, so measure how much
        // time we spent in this state and add it to
        // runnableTime.
        // Not blocking on a lock.
        // Blocking on a lock, measure it. Note that because we're
        // sampling, we have to multiply by our sampling period to get
        // a more representative estimate of the absolute value.
        // gTrackingPeriod also represents an accurate sampling period
        // because we can only enter this state from _Grunning.
    '__go_switch_2: loop {
        { let _switch_val = { let __v = (*newval.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (__GWAITING as u32) {
            if !crate::runtime2::waitReason::is_mutex_wait(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
                // Not blocking on a lock.
        break '__go_switch_2
    }
                        // Not blocking on a lock.
                        // Blocking on a lock. Write down the timestamp.
            let mut now = nanotime();
            { let new_val = now; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.tracking_stamp.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (__GRUNNABLE as u32) {
                        // We just transitioned into runnable, so record what
                        // time that happened.
            let mut now = nanotime();
            { let new_val = now; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.tracking_stamp.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (__GRUNNING as u32) {
                        // We're transitioning into running, so turn off
                        // tracking and record how much time we spent in
                        // runnable.
            { let new_val = false; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.tracking.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
            (*(*sched.lock().unwrap().as_ref().unwrap()).time_to_run.lock().unwrap().as_ref().unwrap()).record(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.runnable_time.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            { let new_val = 0 as i64; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.runnable_time.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        }
    };
        break;
    }
}

/// casGToWaiting transitions gp from old to _Gwaiting, and sets the wait reason.
///
/// Use this over casgstatus when possible to ensure that a waitreason is set.
pub fn cas_g_to_waiting(gp: GoPtr<crate::runtime2::g>, old: Arc<Mutex<Option<u32>>>, reason: Arc<Mutex<Option<waitReason>>>) {
        // Set the wait reason before calling casgstatus, because casgstatus will use it.
    { let new_val = reason.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    casgstatus(gp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__GWAITING as u32))));
}

/// casGToWaitingForSuspendG transitions gp from old to _Gwaiting, and sets the wait reason.
/// The wait reason must be a valid isWaitingForSuspendG wait reason.
///
/// Use this over casgstatus when possible to ensure that a waitreason is set.
pub fn cas_g_to_waiting_for_suspend_g(gp: GoPtr<crate::runtime2::g>, old: Arc<Mutex<Option<u32>>>, reason: Arc<Mutex<Option<waitReason>>>) {
    if !crate::runtime2::waitReason::is_waiting_for_suspend_g(&(*reason.lock().unwrap().as_ref().unwrap())) {
        throw(Arc::new(Mutex::new(Some("casGToWaitingForSuspendG with non-isWaitingForSuspendG wait reason".to_string()))));
    }
    cas_g_to_waiting(gp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// casGToPreemptScan transitions gp from _Grunning to _Gscan|_Gpreempted.
///
/// TODO(austin): This is the only status operation that both changes
/// the status and locks the _Gscan bit. Rethink this.
pub fn cas_g_to_preempt_scan(gp: GoPtr<crate::runtime2::g>, old: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) {
    if { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GRUNNING as u32; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((__GSCAN as u32) | (__GPREEMPTED as u32)) as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad g transition".to_string()))));
    }
    acquire_lock_rank_and_m(Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))))));
    while !(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.atomicstatus.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(((__GSCAN as u32) | (__GPREEMPTED as u32)) as u32)))) {
    }
}

/// casGFromPreempted attempts to transition gp from _Gpreempted to
/// _Gwaiting. If successful, the caller is responsible for
/// re-scheduling gp.
pub fn cas_g_from_preempted(gp: Arc<Mutex<Option<g>>>, old: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> bool {
    if { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GPREEMPTED as u32; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GWAITING as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad g transition".to_string()))));
    }
    { let new_val = crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_PREEMPTED as u8)))); *(*gp.lock().unwrap().as_ref().unwrap()).waitreason.lock().unwrap() = Some(new_val); };
    if !(*(*gp.lock().unwrap().as_ref().unwrap()).atomicstatus.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(__GPREEMPTED as u32))), Arc::new(Mutex::new(Some(__GWAITING as u32)))) {
        return false;
    }
    {
        let mut sg = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone();;
        if { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
            { let __recv = sg.clone(); let __recv_ptr: *mut crate::synctest::synctestGroup = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::synctest::synctestGroup }; let __result = unsafe { &mut *__recv_ptr }.changegstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GPREEMPTED as u32))), Arc::new(Mutex::new(Some(__GWAITING as u32)))); __result };;
        }
    }
    true
}

/// stopTheWorld stops all P's from executing goroutines, interrupting
/// all goroutines at GC safe points and records reason as the reason
/// for the stop. On return, only the current goroutine's P is running.
/// stopTheWorld must not be called from a system stack and the caller
/// must not hold worldsema. The caller must call startTheWorld when
/// other P's should resume execution.
///
/// stopTheWorld is safe for multiple goroutines to call at the
/// same time. Each will execute its own stop, and the stops will
/// be serialized.
///
/// This is also used by routines that do stack dumps. If the system is
/// in panic or being exited, this may not reliably stop all
/// goroutines.
///
/// Returns the STW context. When starting the world, this context must be
/// passed to startTheWorld.
pub fn stop_the_world(reason: Arc<Mutex<Option<stwReason>>>) -> Arc<Mutex<Option<worldStop>>> {
    semacquire(GoPtr::local(worldsema.clone()));
    let mut gp = getg();
    { let new_val = stwReason::string(&(*reason.lock().unwrap().as_ref().unwrap())); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).preemptoff.lock().unwrap() = __moved_val; };
    let reason_closure_clone = reason.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = stop_the_world_with_sema(Arc::new(Mutex::new(Some({ let __arg_holder = reason_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *stopTheWorldContext.lock().unwrap() = __moved_val; };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        // avoid write to stack
    { let __owned = stopTheWorldContext.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }
}

/// startTheWorld undoes the effects of stopTheWorld.
///
/// w must be the worldStop returned by stopTheWorld.
pub fn start_the_world(w: Arc<Mutex<Option<worldStop>>>) {
    let w_closure_clone = w.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        start_the_world_with_sema(Arc::new(Mutex::new(Some(0 as i64))), Arc::new(Mutex::new(Some({ let __arg_holder = w_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // worldsema must be held over startTheWorldWithSema to ensure
        // gomaxprocs cannot change while worldsema is held.
        //
        // Release worldsema with direct handoff to the next waiter, but
        // acquirem so that semrelease1 doesn't try to yield our time.
        //
        // Otherwise if e.g. ReadMemStats is being called in a loop,
        // it might stomp on other attempts to stop the world, such as
        // for starting or ending GC. The operation this blocks is
        // so heavy-weight that we should just try to be as fair as
        // possible here.
        //
        // We don't want to just allow us to get preempted between now
        // and releasing the semaphore because then we keep everyone
        // (including, for example, GCs) waiting longer.
    let mut mp = acquirem();
    { let new_val = "".to_string(); *(*mp.lock().unwrap().as_ref().unwrap()).preemptoff.lock().unwrap() = Some(new_val); };
    semrelease1(GoPtr::local(worldsema.clone()), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(0))));
    releasem(GoPtr::local(mp.clone()));
}

/// stopTheWorldGC has the same effect as stopTheWorld, but blocks
/// until the GC is not running. It also blocks a GC from starting
/// until startTheWorldGC is called.
pub fn stop_the_world_g_c(reason: Arc<Mutex<Option<stwReason>>>) -> Arc<Mutex<Option<worldStop>>> {
    semacquire(GoPtr::local(gcsema.clone()));
    stop_the_world(Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// startTheWorldGC undoes the effects of stopTheWorldGC.
///
/// w must be the worldStop returned by stopTheWorld.
pub fn start_the_world_g_c(w: Arc<Mutex<Option<worldStop>>>) {
    start_the_world(Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    semrelease(GoPtr::local(gcsema.clone()));
}

/// stopTheWorldWithSema is the core implementation of stopTheWorld.
/// The caller is responsible for acquiring worldsema and disabling
/// preemption first and then should stopTheWorldWithSema on the system
/// stack:
///
///	semacquire(&worldsema, 0)
///	m.preemptoff = "reason"
///	var stw worldStop
///	systemstack(func() {
///		stw = stopTheWorldWithSema(reason)
///	})
///
/// When finished, the caller must either call startTheWorld or undo
/// these three operations separately:
///
///	m.preemptoff = ""
///	systemstack(func() {
///		now = startTheWorldWithSema(stw)
///	})
///	semrelease(&worldsema)
///
/// It is allowed to acquire worldsema once and then execute multiple
/// startTheWorldWithSema/stopTheWorldWithSema pairs.
/// Other P's are able to execute between successive calls to
/// startTheWorldWithSema and stopTheWorldWithSema.
/// Holding worldsema causes any other goroutines invoking
/// stopTheWorld to block.
///
/// Returns the STW context. When starting the world, this context must be
/// passed to startTheWorldWithSema.
///
///go:systemstack
pub fn stop_the_world_with_sema(reason: Arc<Mutex<Option<stwReason>>>) -> Arc<Mutex<Option<worldStop>>> {
        // Mark the goroutine which called stopTheWorld preemptible so its
        // stack may be scanned by the GC or observed by the execution tracer.
        //
        // This lets a mark worker scan us or the execution tracer take our
        // stack while we try to stop the world since otherwise we could get
        // in a mutual preemption deadlock.
        //
        // We must not modify anything on the G stack because a stack shrink
        // may occur, now that we switched to _Gwaiting, specifically if we're
        // doing this during the mark phase (mark termination excepted, since
        // we know that stack scanning is done by that point). A stack shrink
        // is otherwise OK though because in order to return from this function
        // (and to leave the system stack) we must have preempted all
        // goroutines, including any attempting to scan our stack, in which
        // case, any stack shrinking will have already completed by the time we
        // exit.
        //
        // N.B. The execution tracer is not aware of this status transition and
        // andles it specially based on the wait reason.
    cas_g_to_waiting_for_suspend_g(
        (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone(),
        Arc::new(Mutex::new(Some(__GRUNNING as u32))),
        Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_STOPPING_THE_WORLD as u8)))))))
    );

    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).s_t_w_start(Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    let mut gp = getg();

        // If we hold a lock, then we won't be able to stop another M
        // that is blocked trying to acquire the lock.
    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("stopTheWorld: holding locks".to_string()))));
    }

    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut start = nanotime();
    { let new_val = gomaxprocs.lock().unwrap().as_ref().unwrap().clone(); *(*sched.lock().unwrap().as_ref().unwrap()).stopwait.lock().unwrap() = Some(new_val); };
    (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
    preemptall();

        // stop current P
    { let new_val = __PGCSTOP as u32; *{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap() = Some(new_val); };
    { let new_val = start; *{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().gc_stop_time.clone() }.lock().unwrap() = Some(new_val); };
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }

        // try to retake all P's in Psyscall status
    { let new_val = trace_acquire(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *trace_local.lock().unwrap() = __moved_val; };
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pp in __range_values.iter() {
        let mut s = Arc::new(Mutex::new(Some({ let __selector_holder = (*pp.lock().unwrap().as_ref().unwrap()).status.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __PSYSCALL as u32; __tmp_x == __tmp_y } && internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local((*pp.lock().unwrap().as_ref().unwrap()).status.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__PGCSTOP as u32)))) {
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).proc_steal(GoPtr::local(pp.clone()), Arc::new(Mutex::new(Some(false))));
    }
        { let __target = (*pp.lock().unwrap().as_ref().unwrap()).syscalltick.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = nanotime(); *(*pp.lock().unwrap().as_ref().unwrap()).gc_stop_time.lock().unwrap() = Some(new_val); };
        { let __target = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    } }
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // stop idle P's
    let mut now = nanotime();
    loop {
        let (mut pp, _) = pidleget(Arc::new(Mutex::new(Some(now))));
        if pp.is_nil() {
        break
    }
        { let new_val = __PGCSTOP as u32; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = nanotime(); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_stop_time.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let __target = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut wait = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y })));
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // wait for remaining P's to stop voluntarily
    if { let __v = (*wait.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        loop {
                // wait for 100us, then try to re-preempt in case of any races
        if notetsleep((*sched.lock().unwrap().as_ref().unwrap()).stopnote.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = 100; let __tmp_y = 1000; __tmp_x * __tmp_y } as i64)))) {
        noteclear((*sched.lock().unwrap().as_ref().unwrap()).stopnote.clone());
        break
    }
        preemptall();
    }
    }

        // wait for 100us, then try to re-preempt in case of any races
    let mut finish = nanotime();
    let mut startTime = Arc::new(Mutex::new(Some({ let __tmp_x = finish; let __tmp_y = start; __tmp_x - __tmp_y })));
    if stwReason::is_g_c(&(*reason.lock().unwrap().as_ref().unwrap())) {
        (*(*sched.lock().unwrap().as_ref().unwrap()).stw_stopping_time_g_c.lock().unwrap().as_ref().unwrap()).record(Arc::new(Mutex::new(Some({ let __arg_holder = startTime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        (*(*sched.lock().unwrap().as_ref().unwrap()).stw_stopping_time_other.lock().unwrap().as_ref().unwrap()).record(Arc::new(Mutex::new(Some({ let __arg_holder = startTime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Double-check we actually stopped everything, and all the invariants hold.
        // Also accumulate all the time spent by each P in _Pgcstop up to the point
        // where everything was stopped. This will be accumulated into the total pause
        // CPU time by the caller.
    let mut stoppingCPUTime = Arc::new(Mutex::new(Some(0 as i64)));
    let mut bad = Arc::new(Mutex::new(Some("".to_string())));
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        { let new_val = "stopTheWorld: not stopped (stopwait != 0)".to_string(); *bad.lock().unwrap() = Some(new_val); };
    } else {
        { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pp in __range_values.iter() {
        if { let __tmp_x = (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).status.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PGCSTOP as u32; __tmp_x != __tmp_y } {
        { let new_val = "stopTheWorld: not stopped (status != _Pgcstop)".to_string(); *bad.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).gc_stop_time.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } && { let __tmp_x = (*bad.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "stopTheWorld: broken CPU time accounting".to_string(); *bad.lock().unwrap() = Some(new_val); };
    }
        { let __rhs = { let __tmp_x = finish; let __tmp_y = (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).gc_stop_time.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let mut guard = stoppingCPUTime.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = 0 as i64; *(*pp.lock().unwrap().as_ref().unwrap()).gc_stop_time.lock().unwrap() = Some(new_val); };
    } }
    }
    if (*freezing.lock().unwrap().as_ref().unwrap()).load() {
                // Some other thread is panicking. This can cause the
                // sanity checks above to fail if the panic happens in
                // the signal handler on a stopped thread. Either way,
                // we should halt this thread.
        lock(GoPtr::local(deadlock.clone()));
        lock(GoPtr::local(deadlock.clone()));
    }
        // Some other thread is panicking. This can cause the
        // sanity checks above to fail if the panic happens in
        // the signal handler on a stopped thread. Either way,
        // we should halt this thread.
    if { let __tmp_x = (*bad.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some({ let __arg_holder = bad.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    world_stopped();

        // Switch back to _Grunning, now that the world is stopped.
    casgstatus(
        (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone(),
        Arc::new(Mutex::new(Some(__GWAITING as u32))),
        Arc::new(Mutex::new(Some(__GRUNNING as u32)))
    );

    return Arc::new(Mutex::new(Some(worldStop { reason: Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), started_stopping: Arc::new(Mutex::new(Some(start))), finished_stopping: Arc::new(Mutex::new(Some(finish))), stopping_c_p_u_time: Arc::new(Mutex::new(Some({ let __arg_holder = stoppingCPUTime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
}

/// reason is the same STW reason passed to stopTheWorld. start is the start
/// time returned by stopTheWorld.
///
/// now is the current time; prefer to pass 0 to capture a fresh timestamp.
///
/// stattTheWorldWithSema returns now.
pub fn start_the_world_with_sema(mut now: Arc<Mutex<Option<i64>>>, w: Arc<Mutex<Option<worldStop>>>) -> i64 {
    assert_world_stopped();

    let mut mp = acquirem();
    if netpollinited() {
        let (mut list, mut delta) = netpoll(Arc::new(Mutex::new(Some(0 as i64))));
        injectglist(list.clone());
        netpoll_adjust_waiters(Arc::new(Mutex::new(Some(delta))));
    }
        // non-blocking
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    let mut procs = { let __owned = gomaxprocs.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    if { let __tmp_x = (*newprocs.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        { let new_val = newprocs.lock().unwrap().as_ref().unwrap().clone(); *procs.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as i32; *newprocs.lock().unwrap() = Some(new_val); };
    }
    let mut p1: GoPtr<crate::runtime2::p> = GoPtr::local(procresize(Arc::new(Mutex::new(Some({ let __arg_holder = procs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
    (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
    if (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).load() {
        (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
        notewakeup((*sched.lock().unwrap().as_ref().unwrap()).sysmonnote.clone());
    }
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    world_started();

    while !p1.is_nil() {
        let mut p: GoPtr<crate::runtime2::p> = p1.clone();
        p1 = crate::runtime2::puintptr::ptr(&(*{ let __ptr_value = p1.with_mut(|__ptr_value| __ptr_value.link.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
        if {
            let __tmp_x = { let __selector_holder = { let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
        let mut mp: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*{ let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
        { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        if {
            let __tmp_x = { let __selector_holder = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.nextp.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
        throw(Arc::new(Mutex::new(Some("startTheWorld: inconsistent mp->nextp".to_string()))));
    }
        (*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.nextp.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).set(p.clone());
        notewakeup({ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.park.clone()); __ptr_value }.clone());
    } else {
                // Start M to run P.  Do not start another M below.
        newm(Arc::new(Mutex::new(None)), p.clone(), Arc::new(Mutex::new(Some(-1 as i64))));
    }
    }

        // Start M to run P.  Do not start another M below.
        // Capture start-the-world time before doing clean-up tasks.
    if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = nanotime(); *now.lock().unwrap() = Some(new_val); };
    }
    let mut totalTime = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*w.lock().unwrap().as_ref().unwrap()).started_stopping.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
    if stwReason::is_g_c(&(*(*w.lock().unwrap().as_ref().unwrap()).reason.lock().unwrap().as_ref().unwrap())) {
        (*(*sched.lock().unwrap().as_ref().unwrap()).stw_total_time_g_c.lock().unwrap().as_ref().unwrap()).record(Arc::new(Mutex::new(Some({ let __arg_holder = totalTime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        (*(*sched.lock().unwrap().as_ref().unwrap()).stw_total_time_other.lock().unwrap().as_ref().unwrap()).record(Arc::new(Mutex::new(Some({ let __arg_holder = totalTime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).s_t_w_done();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Wakeup an additional proc in case we have excessive runnable goroutines
        // in local queues or in the global queue. If we don't, the proc will park itself.
        // If we have lots of excessive work, resetspinning will unpark additional procs as necessary.
    wakep();

    releasem(GoPtr::local(mp.clone()));

    return { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// usesLibcall indicates whether this runtime performs system calls
/// via libcall.
pub fn uses_libcall() -> bool {
    { let _switch_val = G_O_O_S;
    if _switch_val == ("aix".to_string()) || _switch_val == ("darwin".to_string()) || _switch_val == ("illumos".to_string()) || _switch_val == ("ios".to_string()) || _switch_val == ("solaris".to_string()) || _switch_val == ("windows".to_string()) {
            return true;
        } else if _switch_val == ("openbsd".to_string()) {
            return { let __tmp_x = "arm64".to_string(); let __tmp_y = "mips64".to_string(); __tmp_x != __tmp_y };
        }
    }
    false
}

/// mStackIsSystemAllocated indicates whether this runtime starts on a
/// system-allocated stack.
pub fn m_stack_is_system_allocated() -> bool {
    { let _switch_val = G_O_O_S;
    if _switch_val == ("aix".to_string()) || _switch_val == ("darwin".to_string()) || _switch_val == ("plan9".to_string()) || _switch_val == ("illumos".to_string()) || _switch_val == ("ios".to_string()) || _switch_val == ("solaris".to_string()) || _switch_val == ("windows".to_string()) {
            return true;
        } else if _switch_val == ("openbsd".to_string()) {
            return { let __tmp_x = "arm64".to_string(); let __tmp_y = "mips64".to_string(); __tmp_x != __tmp_y };
        }
    }
    false
}

/// mstart is the entry-point for new Ms.
/// It is written in assembly, uses ABI0, is marked TOPFRAME, and calls mstart0.
pub fn mstart() {
    unimplemented!("Go function declaration has no body");
}


/// mPark causes a thread to park itself, returning once woken.
///
///go:nosplit
pub fn m_park() {
    let mut gp = getg();
    notesleep((*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).park.clone());
    noteclear((*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).park.clone());
}

/// forEachP calls fn(p) for every P p when p reaches a GC safe point.
/// If a P is currently executing code, this will bring the P to a GC
/// safe point and execute fn on that P. If the P is not executing code
/// (it is idle or in a syscall), this will call fn(p) directly while
/// preventing the P from exiting its state. This does not ensure that
/// fn will run on every CPU executing Go code, but it acts as a global
/// memory barrier. GC uses this as a "ragged barrier."
///
/// The caller must hold worldsema. fn must not refer to any
/// part of the current goroutine's stack, since the GC may move it.
pub fn for_each_p(reason: Arc<Mutex<Option<waitReason>>>, r#fn: Arc<Mutex<Option<Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync>>>>) {
    let fn_closure_clone = r#fn.clone(); let reason_closure_clone = reason.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut gp: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
        cas_g_to_waiting_for_suspend_g(gp.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some({ let __arg_holder = reason_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        for_each_p_internal(fn_closure_clone.clone());
        casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

/// forEachPInternal calls fn(p) for every P p when p reaches a GC safe point.
/// It is the internal implementation of forEachP.
///
/// The caller must hold worldsema and either must ensure that a GC is not
/// running (otherwise this may deadlock with the GC trying to preempt this P)
/// or it must leave its goroutine in a preemptible state before it switches
/// to the systemstack. Due to these restrictions, prefer forEachP when possible.
///
///go:systemstack
pub fn for_each_p_internal(r#fn: Arc<Mutex<Option<Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync>>>>) {
    let mut mp = acquirem();
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));

    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("forEachP: sched.safePointWait != 0".to_string()))));
    }
    { let new_val = { let __tmp_x = (*gomaxprocs.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x - __tmp_y }; *(*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.lock().unwrap() = Some(new_val); };
    { let new_val = r#fn.clone(); (*sched.lock().unwrap().as_mut().unwrap()).safe_point_fn = new_val; };

        // Ask all Ps to run the safe point function.
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p2 in __range_values.iter() {
        if { let __left_addr = { let __ptr = GoPtr::local(p2.clone()); __ptr.addr() }; let __right_addr = pp.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        internal_runtime_atomic::store((*p2.lock().unwrap().as_ref().unwrap()).run_safe_point_fn.clone(), Arc::new(Mutex::new(Some(1 as u32))));
    }
    } }
    preemptall();

        // Any P entering _Pidle or _Psyscall from now on will observe
        // p.runSafePointFn == 1 and will call runSafePointFn when
        // changing its status to _Pidle/_Psyscall.
        // Run safe point function for all idle Ps. sched.pidle will
        // not change because we hold sched.lock.
    let mut p: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*sched.lock().unwrap().as_ref().unwrap()).pidle.lock().unwrap().as_ref().unwrap()));
    while !p.is_nil() {
        if internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local({ let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.run_safe_point_fn.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(1 as u32))), Arc::new(Mutex::new(Some(0 as u32)))) {
        { let __f_ptr: *mut Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(p.clone()) };
        { let __target = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        p = crate::runtime2::puintptr::ptr(&(*{ let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.link.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
    }

    let mut wait = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y })));
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Run fn for the current P.
    { let __f_ptr: *mut Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(pp.clone()) };

        // Force Ps currently in _Psyscall into _Pidle and hand them
        // off to induce safe point function execution.
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p2 in __range_values.iter() {
        let mut s = Arc::new(Mutex::new(Some({ let __selector_holder = (*p2.lock().unwrap().as_ref().unwrap()).status.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
                // We need to be fine-grained about tracing here, since handoffp
                // might call into the tracer, and the tracer is non-reentrant.
        let mut trace_local = trace_acquire();
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __PSYSCALL as u32; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).run_safe_point_fn.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x == __tmp_y } && internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local((*p2.lock().unwrap().as_ref().unwrap()).status.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__PIDLE as u32)))) {
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // It's important that we traceRelease before we call handoffp, which may also traceAcquire.
        (*trace_local.lock().unwrap().as_ref().unwrap()).proc_steal(GoPtr::local(p2.clone()), Arc::new(Mutex::new(Some(false))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // It's important that we traceRelease before we call handoffp, which may also traceAcquire.
        { let __target = (*p2.lock().unwrap().as_ref().unwrap()).syscalltick.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        handoffp(GoPtr::local(p2.clone()));
    } else if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    } }

        // We need to be fine-grained about tracing here, since handoffp
        // might call into the tracer, and the tracer is non-reentrant.
        // It's important that we traceRelease before we call handoffp, which may also traceAcquire.
        // Wait for remaining Ps to run fn.
    if { let __v = (*wait.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        loop {
                // Wait for 100us, then try to re-preempt in
                // case of any races.
                //
                // Requires system stack.
        if notetsleep((*sched.lock().unwrap().as_ref().unwrap()).safe_point_note.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = 100; let __tmp_y = 1000; __tmp_x * __tmp_y } as i64)))) {
        noteclear((*sched.lock().unwrap().as_ref().unwrap()).safe_point_note.clone());
        break
    }
        preemptall();
    }
    }
        // Wait for 100us, then try to re-preempt in
        // case of any races.
        //
        // Requires system stack.
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("forEachP: not done".to_string()))));
    }
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p2 in __range_values.iter() {
        if { let __tmp_x = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).run_safe_point_fn.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("forEachP: P did not run fn".to_string()))));
    }
    } }

    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    *(*sched.lock().unwrap().as_ref().unwrap()).safe_point_fn.lock().unwrap() = None;
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    releasem(GoPtr::local(mp.clone()));
}

/// runSafePointFn runs the safe point function, if any, for this P.
/// This should be called like
///
///	if getg().m.p.runSafePointFn != 0 {
///	    runSafePointFn()
///	}
///
/// runSafePointFn must be checked on any transition in to _Pidle or
/// _Psyscall to avoid a race where forEachP sees that the P is running
/// just before the P goes into _Pidle/_Psyscall and neither forEachP
/// nor the P run the safe-point function.
pub fn run_safe_point_fn() {
    let mut p: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));

        // Resolve the race between forEachP running the safe-point
        // function on this P's behalf and this P running the
        // safe-point function directly.
    if !internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local({ let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.run_safe_point_fn.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(1 as u32))), Arc::new(Mutex::new(Some(0 as u32)))) {
        return;
    }
    { let __f_holder = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_fn.clone(); let __f_ptr: *mut Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(p.clone()) };
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        notewakeup((*sched.lock().unwrap().as_ref().unwrap()).safe_point_note.clone());
    }
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// Allocate a new m unassociated with any thread.
/// Can use p for allocation context if needed.
/// fn is recorded as the new m's m.mstartfn.
/// id is optional pre-allocated m ID. Omit by passing -1.
///
/// This function is allowed to have write barriers even if the caller
/// isn't because it borrows pp.
///
///go:yeswritebarrierrec
pub fn allocm(pp: GoPtr<crate::runtime2::p>, r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>, id: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<crate::runtime2::m>>> {
    (*allocmLock.lock().unwrap().as_mut().unwrap()).rlock();

        // The caller owns pp, but we may borrow (i.e., acquirep) it. We must
        // disable preemption to ensure it is not stolen, which would make the
        // caller lose ownership.
    acquirem();

    let mut gp = getg();
    if {
        let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x == __tmp_y
    } {
        acquirep(pp.clone());
    }

        // temporarily borrow p for mallocs in this function
        // Release the free M list. We need to do this somewhere and
        // this may free up a stack we can use.
    if { let __nil_target = (*sched.lock().unwrap().as_ref().unwrap()).freem.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let mut newList: Arc<Mutex<Option<m>>> = Arc::new(Mutex::new(None));
        let mut freem = (*sched.lock().unwrap().as_ref().unwrap()).freem.clone();
    while { let __nil_result = (*freem.lock().unwrap()).is_some(); __nil_result } {
                // Wait for freeWait to indicate that freem's stack is unused.
        let mut wait = (*(*freem.lock().unwrap().as_ref().unwrap()).free_wait.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = wait; let __tmp_y = FREE_M_WAIT as u32; __tmp_x == __tmp_y } {
        let mut next = (*freem.lock().unwrap().as_ref().unwrap()).freelink.clone();
        { let new_val = newList.clone(); (*freem.lock().unwrap().as_mut().unwrap()).freelink = new_val; };
        { let new_val = freem.clone(); newList = new_val; };
        { let new_val = next.clone(); freem = new_val; };
        continue
    }

                // Drop any remaining trace resources.
                // Ms can continue to emit events all the way until wait != freeMWait,
                // so it's only safe to call traceThreadDestroy at this point.
        if trace_enabled() || trace_shutting_down() {
        trace_thread_destroy(freem.clone());
    }

                // Free the stack if needed. For freeMRef, there is
                // nothing to do except drop freem from the sched.freem
                // list.
        if { let __tmp_x = wait; let __tmp_y = FREE_M_STACK as u32; __tmp_x == __tmp_y } {
                // stackfree must be on the system stack, but allocm is
                // reachable off the system stack transitively from
                // startm.
        let freem_closure_clone = freem.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        stackfree(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*freem_closure_clone.lock().unwrap().as_ref().unwrap()).g0.lock().unwrap().as_ref().unwrap()).stack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
                // stackfree must be on the system stack, but allocm is
                // reachable off the system stack transitively from
                // startm.
        { let new_val = (*freem.lock().unwrap().as_ref().unwrap()).freelink.clone(); freem = new_val; };
    }
                // Wait for freeWait to indicate that freem's stack is unused.
                // Drop any remaining trace resources.
                // Ms can continue to emit events all the way until wait != freeMWait,
                // so it's only safe to call traceThreadDestroy at this point.
                // Free the stack if needed. For freeMRef, there is
                // nothing to do except drop freem from the sched.freem
                // list.
                // stackfree must be on the system stack, but allocm is
                // reachable off the system stack transitively from
                // startm.
        { let new_val = newList.clone(); (*sched.lock().unwrap().as_mut().unwrap()).freem = new_val; };
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }

        // Wait for freeWait to indicate that freem's stack is unused.
        // Drop any remaining trace resources.
        // Ms can continue to emit events all the way until wait != freeMWait,
        // so it's only safe to call traceThreadDestroy at this point.
        // Free the stack if needed. For freeMRef, there is
        // nothing to do except drop freem from the sched.freem
        // list.
        // stackfree must be on the system stack, but allocm is
        // reachable off the system stack transitively from
        // startm.
    let mut mp = Arc::new(Mutex::new(Some(m::default())));
    { let new_val = r#fn.clone(); (*mp.lock().unwrap().as_mut().unwrap()).mstartfn = new_val; };
    mcommoninit(mp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // In case of cgo or Solaris or illumos or Darwin, pthread_create will make us a stack.
        // Windows and Plan 9 will layout sched stack on OS stack.
    if (*iscgo.lock().unwrap().as_ref().unwrap()) || m_stack_is_system_allocated() {
        { let new_val = malg(Arc::new(Mutex::new(Some(-1 as i32)))).clone(); (*mp.lock().unwrap().as_mut().unwrap()).g0 = new_val; };
    } else {
        { let new_val = malg(Arc::new(Mutex::new(Some({ let __tmp_x = 16384; let __tmp_y = internal_runtime_sys::STACK_GUARD_MULTIPLIER; __tmp_x * __tmp_y } as i32)))).clone(); (*mp.lock().unwrap().as_mut().unwrap()).g0 = new_val; };
    }
    { let new_val = mp.clone(); (*(*mp.lock().unwrap().as_ref().unwrap()).g0.lock().unwrap().as_mut().unwrap()).m = new_val; };

    if { let __left_addr = pp.addr(); let __right_addr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())).addr(); let __eq = __left_addr == __right_addr; __eq } {
        releasep();
    }

    releasem(GoPtr::local((*gp.lock().unwrap().as_ref().unwrap()).m.clone()));
    (*allocmLock.lock().unwrap().as_mut().unwrap()).runlock();
    return mp.clone();
}

/// needm is called when a cgo callback happens on a
/// thread without an m (a thread not created by Go).
/// In this case, needm is expected to find an m to use
/// and return with m, g initialized correctly.
/// Since m and g are not set now (likely nil, but see below)
/// needm is limited in what routines it can call. In particular
/// it can only call nosplit functions (textflag 7) and cannot
/// do any scheduling that requires an m.
///
/// In order to avoid needing heavy lifting here, we adopt
/// the following strategy: there is a stack of available m's
/// that can be stolen. Using compare-and-swap
/// to pop from the stack has ABA races, so we simulate
/// a lock by doing an exchange (via Casuintptr) to steal the stack
/// head and replace the top pointer with MLOCKED (1).
/// This serves as a simple spin lock that we can use even
/// without an m. The thread that locks the stack in this way
/// unlocks the stack by storing a valid stack head pointer.
///
/// In order to make sure that there is always an m structure
/// available to be stolen, we maintain the invariant that there
/// is always one more than needed. At the beginning of the
/// program (if cgo is in use) the list is seeded with a single m.
/// If needm finds that it has taken the last m off the list, its job
/// is - once it has installed its own m so that it can do things like
/// allocate memory - to create a spare m and put it on the list.
///
/// Each of these extra m's also has a g0 and a curg that are
/// pressed into service as the scheduling stack and current
/// goroutine for the duration of the cgo callback.
///
/// It calls dropm to put the m back on the list,
/// 1. when the callback is done with the m in non-pthread platforms,
/// 2. or when the C thread exiting on pthread platforms.
///
/// The signal argument indicates whether we're called from a signal
/// handler.
///
///go:nosplit
pub fn needm(signal: Arc<Mutex<Option<bool>>>) {
    if ((*iscgo.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x == __tmp_y }) && !(*cgoHasExtraM.lock().unwrap().as_ref().unwrap()) {
                // Can happen if C/C++ code calls Go from a global ctor.
                // Can also happen on Windows if a global ctor uses a
                // callback created by syscall.NewCallback. See issue #6751
                // for details.
                //
                // Can not throw, because scheduler is not initialized yet.
        write_err_str(Arc::new(Mutex::new(Some("fatal error: cgo callback before cgo call\n".to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }

        // Can happen if C/C++ code calls Go from a global ctor.
        // Can also happen on Windows if a global ctor uses a
        // callback created by syscall.NewCallback. See issue #6751
        // for details.
        //
        // Can not throw, because scheduler is not initialized yet.
        // Save and block signals before getting an M.
        // The signal handler may call needm itself,
        // and we must avoid a deadlock. Also, once g is installed,
        // any incoming signals will try to execute,
        // but we won't have the sigaltstack settings and other data
        // set up appropriately until the end of minit, which will
        // unblock the signals. This is the same dance as when
        // starting a new m to run Go code via newosproc.
    let mut sigmask: Arc<Mutex<Option<sigset>>> = Arc::new(Mutex::new(Some(crate::os_darwin::sigset(Arc::new(Mutex::new(Some(0)))))));
    sigsave(sigmask.clone());
    sigblock(Arc::new(Mutex::new(Some(false))));

        // getExtraM is safe here because of the invariant above,
        // that the extra list always contains or will soon contain
        // at least one m.
    let (mut mp, mut last) = get_extra_m();

        // Set needextram when we've just emptied the list,
        // so that the eventual call into cgocallbackg will
        // allocate a new m for the extra list. We delay the
        // allocation until then so that it can be done
        // after exitsyscall makes sure it is okay to be
        // running at all (that is, there's no garbage collection
        // running right now).
    { let new_val = last; *{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.needextram.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Store the original signal mask for use by minit.
    { let new_val = sigmask.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.sigmask.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Install TLS on some platforms (previously setg
        // would do this if necessary).
    os_setup_t_l_s(mp.clone());

        // Install g (= m->g0) and set the stack bounds
        // to match the current stack.
    setg({ let __field = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0.clone()); __ptr_value }.clone(); __field });
    let mut sp = internal_runtime_sys::get_caller_s_p();
    callback_update_system_stack(mp.clone(), Arc::new(Mutex::new(Some(sp))), Arc::new(Mutex::new(Some({ let __arg_holder = signal.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Should mark we are already in Go now.
        // Otherwise, we may call needm again when we get a signal, before cgocallbackg1,
        // which means the extram list may be empty, that will cause a deadlock.
    { let new_val = false; *{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.is_extra_in_c.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Initialize this thread to use the m.
    asminit();
    minit();

        // Emit a trace event for this dead -> syscall transition,
        // but only if we're not in a signal handler.
        //
        // N.B. the tracer can run on a bare M just fine, we just have
        // to make sure to do this before setg(nil) and unminit.
    let mut trace_local: Arc<Mutex<Option<traceLocker>>> = Arc::new(Mutex::new(Some(Default::default())));
    if !{ let __v = (*signal.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = trace_acquire(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *trace_local.lock().unwrap() = __moved_val; };
    }

        // mp.curg is now a real goroutine.
    casgstatus({ let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().curg.clone(); __field_value }, Arc::new(Mutex::new(Some(__GDEAD as u32))), Arc::new(Mutex::new(Some(__GSYSCALL as u32))));
    (*(*sched.lock().unwrap().as_ref().unwrap()).ngsys.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));

    if !{ let __v = (*signal.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_create_syscall({ let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().curg.clone(); __field_value });
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
    { let new_val = signal.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.is_extra_in_sig.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
}

/// dropm puts the current m back onto the extra list.
///
/// 1. On systems without pthreads, like Windows
/// dropm is called when a cgo callback has called needm but is now
/// done with the callback and returning back into the non-Go thread.
///
/// The main expense here is the call to signalstack to release the
/// m's signal stack, and then the call to needm on the next callback
/// from this thread. It is tempting to try to save the m for next time,
/// which would eliminate both these costs, but there might not be
/// a next time: the current thread (which Go does not control) might exit.
/// If we saved the m for that thread, there would be an m leak each time
/// such a thread exited. Instead, we acquire and release an m on each
/// call. These should typically not be scheduling operations, just a few
/// atomics, so the cost should be small.
///
/// 2. On systems with pthreads
/// dropm is called while a non-Go thread is exiting.
/// We allocate a pthread per-thread variable using pthread_key_create,
/// to register a thread-exit-time destructor.
/// And store the g into a thread-specific value associated with the pthread key,
/// when first return back to C.
/// So that the destructor would invoke dropm while the non-Go thread is exiting.
/// This is much faster since it avoids expensive signal-related syscalls.
///
/// This always runs without a P, so //go:nowritebarrierrec is required.
///
/// This may run with a different stack than was recorded in g0 (there is no
/// call to callbackUpdateSystemStack prior to dropm), so this must be
/// //go:nosplit to avoid the stack bounds check.
///
///go:nowritebarrierrec
///go:nosplit
pub fn dropm() {
        // Clear m and g, and return m to the extra list.
        // After the call to setg we can only call nosplit functions
        // with no pointer manipulation.
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();

        // Emit a trace event for this syscall -> dead transition.
        //
        // N.B. the tracer can run on a bare M just fine, we just have
        // to make sure to do this before setg(nil) and unminit.
    let mut trace_local: Arc<Mutex<Option<traceLocker>>> = Arc::new(Mutex::new(Some(Default::default())));
    if !(*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).is_extra_in_sig.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = trace_acquire(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *trace_local.lock().unwrap() = __moved_val; };
    }

        // Return mp.curg to dead state.
    casgstatus((*mp.lock().unwrap().as_ref().unwrap()).curg.clone(), Arc::new(Mutex::new(Some(__GSYSCALL as u32))), Arc::new(Mutex::new(Some(__GDEAD as u32))));
    { let new_val = false; *{ let __ptr_value = (*mp.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.preempt_stop.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    (*(*sched.lock().unwrap().as_ref().unwrap()).ngsys.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));

    if !(*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).is_extra_in_sig.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_destroy_syscall();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }

        // Trash syscalltick so that it doesn't line up with mp.old.syscalltick anymore.
        //
        // In the new tracer, we model needm and dropm and a goroutine being created and
        // destroyed respectively. The m then might get reused with a different procid but
        // still with a reference to oldp, and still with the same syscalltick. The next
        // time a G is "created" in needm, it'll return and quietly reacquire its P from a
        // different m with a different procid, which will confuse the trace parser. By
        // trashing syscalltick, we ensure that it'll appear as if we lost the P to the
        // tracer parser and that we just reacquired it.
        //
        // Trash the value by decrementing because that gets us as far away from the value
        // the syscall exit code expects as possible. Setting to zero is risky because
        // syscalltick could already be zero (and in fact, is initialized to zero).
    { let __target = (*mp.lock().unwrap().as_ref().unwrap()).syscalltick.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }

        // Reset trace state unconditionally. This goroutine is being 'destroyed'
        // from the perspective of the tracer.
    (*{ let __ptr_value = (*mp.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).reset();

        // Flush all the M's buffers. This is necessary because the M might
        // be used on a different thread with a different procid, so we have
        // to make sure we don't write into the same buffer.
    if trace_enabled() || trace_shutting_down() {
                // Acquire sched.lock across thread destruction. One of the invariants of the tracer
                // is that a thread cannot disappear from the tracer's view (allm or freem) without
                // it noticing, so it requires that sched.lock be held over traceThreadDestroy.
                //
                // This isn't strictly necessary in this case, because this thread never leaves allm,
                // but the critical section is short and dropm is rare on pthread platforms, so just
                // take the lock and play it safe. traceThreadDestroy also asserts that the lock is held.
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        trace_thread_destroy(mp.clone());
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
        // Acquire sched.lock across thread destruction. One of the invariants of the tracer
        // is that a thread cannot disappear from the tracer's view (allm or freem) without
        // it noticing, so it requires that sched.lock be held over traceThreadDestroy.
        //
        // This isn't strictly necessary in this case, because this thread never leaves allm,
        // but the critical section is short and dropm is rare on pthread platforms, so just
        // take the lock and play it safe. traceThreadDestroy also asserts that the lock is held.
    { let new_val = false; *(*mp.lock().unwrap().as_ref().unwrap()).is_extra_in_sig.lock().unwrap() = Some(new_val); };

        // Block signals before unminit.
        // Unminit unregisters the signal handling stack (but needs g on some systems).
        // Setg(nil) clears g, which is the signal handler's cue not to run Go handlers.
        // It's important not to try to handle a signal between those two steps.
    let mut sigmask = Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).sigmask.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    sigblock(Arc::new(Mutex::new(Some(false))));
    unminit();

    setg(Arc::new(Mutex::new(None)));

        // Clear g0 stack bounds to ensure that needm always refreshes the
        // bounds when reusing this M.
    let mut g0_local = (*mp.lock().unwrap().as_ref().unwrap()).g0.clone();
    { let new_val = 0 as usize; *(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as usize; *(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as usize; *(*g0_local.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as usize; *(*g0_local.lock().unwrap().as_ref().unwrap()).stackguard1.lock().unwrap() = Some(new_val); };
    { let new_val = false; *(*mp.lock().unwrap().as_ref().unwrap()).g0_stack_accurate.lock().unwrap() = Some(new_val); };

    put_extra_m(mp.clone());

    msigrestore(Arc::new(Mutex::new(Some({ let __arg_holder = sigmask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// lockextra locks the extra list and returns the list head.
/// The caller must unlock the list by storing a new list head
/// to extram. If nilokay is true, then lockextra will
/// return a nil list head if that's what it finds. If nilokay is false,
/// lockextra will keep waiting until the list head is no longer nil.
///
///go:nosplit
pub fn lockextra(nilokay: Arc<Mutex<Option<bool>>>) -> GoPtr<crate::runtime2::m> {
    const locked: i32 = 1;


    let mut incr = Arc::new(Mutex::new(Some(false)));
    loop {
        let mut old = (*extraM.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = old; let __tmp_y = locked as usize; __tmp_x == __tmp_y } {
        osyield_no_g();
        continue
    }
        if { let __tmp_x = old; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } && !{ let __v = (*nilokay.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if !{ let __v = (*incr.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Add 1 to the number of threads
                // waiting for an M.
                // This is cleared by newextram.
        (*extraMWaiters.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
        { let new_val = true; *incr.lock().unwrap() = Some(new_val); };
    }
                // Add 1 to the number of threads
                // waiting for an M.
                // This is cleared by newextram.
        usleep_no_g(Arc::new(Mutex::new(Some(1 as u32))));
        continue
    }
                // Add 1 to the number of threads
                // waiting for an M.
                // This is cleared by newextram.
        if (*extraM.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some(locked as usize)))) {
        return GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(old))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
        osyield_no_g();
        continue
    }
}

///go:nosplit
pub fn unlockextra(mp: GoPtr<crate::runtime2::m>, delta: Arc<Mutex<Option<i32>>>) {
    (*extraMLength.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    (*extraM.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(mp.addr()))).lock().unwrap().as_ref().unwrap()) as usize))));
}

/// Return an M from the extra M list. Returns last == true if the list becomes
/// empty because of this call.
///
/// Spins waiting for an extra M, so caller must ensure that the list always
/// contains or will soon contain at least one M.
///
///go:nosplit
pub fn get_extra_m() -> (GoPtr<crate::runtime2::m>, bool) {
    let mut mp: GoPtr<crate::runtime2::m> = GoPtr::nil();
    let mut last: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    mp = lockextra(Arc::new(Mutex::new(Some(false))));
    (*extraMInUse.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
    unlockextra(crate::runtime2::muintptr::ptr(&(*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(Some(-1 as i32))));
    (
        mp.clone(),
        crate::runtime2::muintptr::ptr(&(*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())).is_nil()
    )
}

/// Returns an extra M back to the list. mp must be from getExtraM. Newly
/// allocated M's should use addExtraM.
///
///go:nosplit
pub fn put_extra_m(mp: Arc<Mutex<Option<m>>>) {
    (*extraMInUse.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
    add_extra_m(mp.clone());
}

/// Adds a newly allocated M to the extra M list.
///
///go:nosplit
pub fn add_extra_m(mp: Arc<Mutex<Option<m>>>) {
    let mut mnext: GoPtr<crate::runtime2::m> = lockextra(Arc::new(Mutex::new(Some(true))));
    (*(*mp.lock().unwrap().as_ref().unwrap()).schedlink.lock().unwrap().as_mut().unwrap()).set(mnext.clone());
    unlockextra(GoPtr::local(mp.clone()), Arc::new(Mutex::new(Some(1 as i32))));
}

/// Create a new m. It will start off with a call to fn, or else the scheduler.
/// fn needs to be static and not a heap allocated closure.
/// May run with m.p==nil, so write barriers are not allowed.
///
/// id is optional pre-allocated m ID. Omit by passing -1.
///
///go:nowritebarrierrec
pub fn newm(r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>, pp: GoPtr<crate::runtime2::p>, id: Arc<Mutex<Option<i64>>>) {
        // allocm adds a new M to allm, but they do not start until created by
        // the OS in newm1 or the template thread.
        //
        // doAllThreadsSyscall requires that every M in allm will eventually
        // start and be signal-able, even with a STW.
        //
        // Disable preemption here until we start the thread to ensure that
        // newm is not preempted between allocm and starting the new thread,
        // ensuring that anything added to allm is guaranteed to eventually
        // start.
    acquirem();

    let mut mp = allocm(pp.clone(), r#fn.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    (*(*mp.lock().unwrap().as_ref().unwrap()).nextp.lock().unwrap().as_mut().unwrap()).set(pp.clone());
    { let new_val = initSigmask.lock().unwrap().as_ref().unwrap().clone(); *(*mp.lock().unwrap().as_ref().unwrap()).sigmask.lock().unwrap() = Some(new_val); };
    {
        let mut gp = getg();;
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result };
                    if __go_cond_2 {
                        let __go_cond_3 = { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
                        __go_cond_3
                    } else {
                        false
                    }
                };
                if __go_cond_1 {
                    let __go_cond_4 = {
                        let __go_cond_5 = { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locked_ext.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
                        if __go_cond_5 {
                            true
                        } else {
                            let __go_cond_6 = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).incgo.lock().unwrap().as_ref().unwrap());
                            __go_cond_6
                        }
                    };
                    __go_cond_4
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_7 = { let __tmp_x = "darwin".to_string(); let __tmp_y = "plan9".to_string(); __tmp_x != __tmp_y };
                __go_cond_7
            } else {
                false
            }
        } {
            lock(GoPtr::local((*newmHandoff.lock().unwrap().as_ref().unwrap()).lock.clone()));;
            if { let __tmp_x = (*{ let __field = (*newmHandoff.lock().unwrap().as_ref().unwrap()).have_template_thread.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("on a locked thread with no template thread".to_string()))));
    };
            { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some((*(*(*newmHandoff.lock().unwrap().as_ref().unwrap()).newm.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*mp.lock().unwrap().as_ref().unwrap()).schedlink.lock().unwrap() = Some(new_val); };;
            (*(*newmHandoff.lock().unwrap().as_ref().unwrap()).newm.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(mp.clone()));;
            if (*{ let __field = (*newmHandoff.lock().unwrap().as_ref().unwrap()).waiting.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = false; *(*newmHandoff.lock().unwrap().as_ref().unwrap()).waiting.lock().unwrap() = Some(new_val); };
        notewakeup((*newmHandoff.lock().unwrap().as_ref().unwrap()).wake.clone());
    };
            unlock(GoPtr::local((*newmHandoff.lock().unwrap().as_ref().unwrap()).lock.clone()));;
            releasem(GoPtr::local((*getg().lock().unwrap().as_ref().unwrap()).m.clone()));;
            return;;
        }
    }
        // We're on a locked M or a thread that may have been
        // started by C. The kernel state of this thread may
        // be strange (the user may have locked it for that
        // purpose). We don't want to clone that into another
        // thread. Instead, ask a known-good thread to create
        // the thread for us.
        //
        // This is disabled on Plan 9. See golang.org/issue/22227.
        //
        // TODO: This may be unnecessary on Windows, which
        // doesn't model thread creation off fork.
        // The M has not started yet, but the template thread does not
        // participate in STW, so it will always process queued Ms and
        // it is safe to releasem.
    newm1(GoPtr::local(mp.clone()));
    releasem(GoPtr::local((*getg().lock().unwrap().as_ref().unwrap()).m.clone()));
}

pub fn newm1(mp: GoPtr<crate::runtime2::m>) {
    if (*iscgo.lock().unwrap().as_ref().unwrap()) {
        let mut ts: Arc<Mutex<Option<cgothreadstart>>> = Arc::new(Mutex::new(Some(Default::default())));
        if { let __nil_result = (*_cgo_thread_start.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("_cgo_thread_start missing".to_string()))));
    }
        (*(*ts.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_mut().unwrap()).set(GoPtr::local({ let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().g0.clone(); __field_value }));
        { let new_val = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.tls.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u64>(unimplemented!("unsafe.Pointer conversion to u64")) } })).clone(); (*ts.lock().unwrap().as_mut().unwrap()).tls = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(mstart.clone()) as Box<dyn Any + Send + Sync>))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*ts.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap() = __moved_val; };
        if MSANENABLED {
        msanwrite(Arc::new(Mutex::new(Some(Arc::as_ptr(&ts.clone()) as usize))), Arc::new(Mutex::new(Some(std::mem::size_of::<cgothreadstart>()))));
    }
        if ASANENABLED {
        asanwrite(Arc::new(Mutex::new(Some(Arc::as_ptr(&ts.clone()) as usize))), Arc::new(Mutex::new(Some(std::mem::size_of::<cgothreadstart>()))));
    }
        (*execLock.lock().unwrap().as_mut().unwrap()).rlock();
        asmcgocall(Arc::new(Mutex::new(Some({ let __arg_holder = _cgo_thread_start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&ts.clone()) as usize))));
        (*execLock.lock().unwrap().as_mut().unwrap()).runlock();
        return;
    }
        // Prevent process clone.
    (*execLock.lock().unwrap().as_mut().unwrap()).rlock();
    newosproc(mp.clone());
    (*execLock.lock().unwrap().as_mut().unwrap()).runlock();
}

/// Stops execution of the current m until new work is available.
/// Returns with acquired P.
pub fn stopm() {
    let mut gp = getg();

    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("stopm holding locks".to_string()))));
    }
    if {
        let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x != __tmp_y
    } {
        throw(Arc::new(Mutex::new(Some("stopm holding p".to_string()))));
    }
    if (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).spinning.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("stopm spinning".to_string()))));
    }

    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    mput({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field });
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    m_park();
    acquirep(crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).nextp.lock().unwrap().as_ref().unwrap())));
    { let new_val = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).nextp.lock().unwrap() = Some(new_val); };
}

pub fn mspinning() {
        // startm's caller incremented nmspinning. Set the new M's spinning.
    { let new_val = true; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).spinning.lock().unwrap() = Some(new_val); };
}

/// Schedules some M to run the p (creates an M if necessary).
/// If p==nil, tries to get an idle P, if no idle P's does nothing.
/// May run with m.p==nil, so write barriers are not allowed.
/// If spinning is set, the caller has incremented nmspinning and must provide a
/// P. startm will set m.spinning in the newly started M.
///
/// Callers passing a non-nil P must call from a non-preemptible context. See
/// comment on acquirem below.
///
/// Argument lockheld indicates whether the caller already acquired the
/// scheduler lock. Callers holding the lock when making the call must pass
/// true. The lock might be temporarily dropped, but will be reacquired before
/// returning.
///
/// Must not have write barriers because this may be called without a P.
///
///go:nowritebarrierrec
pub fn startm(mut pp: GoPtr<crate::runtime2::p>, spinning: Arc<Mutex<Option<bool>>>, lockheld: Arc<Mutex<Option<bool>>>) {
        // Disable preemption.
        //
        // Every owned P must have an owner that will eventually stop it in the
        // event of a GC stop request. startm takes transient ownership of a P
        // (either from argument or pidleget below) and transfers ownership to
        // a started M, which will be responsible for performing the stop.
        //
        // Preemption must be disabled during this transient ownership,
        // otherwise the P this is running on may enter GC stop while still
        // holding the transient P, leaving that P in limbo and deadlocking the
        // STW.
        //
        // Callers passing a non-nil P must already be in non-preemptible
        // context, otherwise such preemption could occur on function entry to
        // startm. Callers passing a nil P may be preemptible, so we must
        // disable preemption before acquiring a P from pidleget below.
    let mut mp = acquirem();
    if !{ let __v = (*lockheld.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
    if pp.is_nil() {
        if { let __v = (*spinning.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // TODO(prattmic): All remaining calls to this function
                // with _p_ == nil could be cleaned up to find a P
                // before calling startm.
        throw(Arc::new(Mutex::new(Some("startm: P required for spinning=true".to_string()))));
    }
                // TODO(prattmic): All remaining calls to this function
                // with _p_ == nil could be cleaned up to find a P
                // before calling startm.
        { let (__tmp_0, __tmp_1) = pidleget(Arc::new(Mutex::new(Some(0 as i64)))); pp = __tmp_0.clone(); };
        if pp.is_nil() {
        if !{ let __v = (*lockheld.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
        releasem(GoPtr::local(mp.clone()));
        return;
    }
    }
        // TODO(prattmic): All remaining calls to this function
        // with _p_ == nil could be cleaned up to find a P
        // before calling startm.
    let mut nmp: GoPtr<crate::runtime2::m> = mget();
    if nmp.is_nil() {
                // No M is available, we must drop sched.lock and call newm.
                // However, we already own a P to assign to the M.
                //
                // Once sched.lock is released, another G (e.g., in a syscall),
                // could find no idle P while checkdead finds a runnable G but
                // no running M's because this new M hasn't started yet, thus
                // throwing in an apparent deadlock.
                // This apparent deadlock is possible when startm is called
                // from sysmon, which doesn't count as a running M.
                //
                // Avoid this situation by pre-allocating the ID for the new M,
                // thus marking it as 'running' before we drop sched.lock. This
                // new M will eventually run the scheduler to execute any
                // queued G's.
        let mut id = m_reserve_i_d();
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let mut r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __v = (*spinning.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // The caller incremented nmspinning, so set m.spinning in the new M.
        { let new_val = Box::new(move || { mspinning() }) as Box<dyn FnMut() -> () + Send + Sync>; *r#fn.lock().unwrap() = Some(new_val); };
    }
                // The caller incremented nmspinning, so set m.spinning in the new M.
        newm(r#fn.clone(), pp.clone(), Arc::new(Mutex::new(Some(id))));
        if { let __v = (*lockheld.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
                // Ownership transfer of pp committed by start in newm.
                // Preemption is now safe.
        releasem(GoPtr::local(mp.clone()));
        return;
    }
        // No M is available, we must drop sched.lock and call newm.
        // However, we already own a P to assign to the M.
        //
        // Once sched.lock is released, another G (e.g., in a syscall),
        // could find no idle P while checkdead finds a runnable G but
        // no running M's because this new M hasn't started yet, thus
        // throwing in an apparent deadlock.
        // This apparent deadlock is possible when startm is called
        // from sysmon, which doesn't count as a running M.
        //
        // Avoid this situation by pre-allocating the ID for the new M,
        // thus marking it as 'running' before we drop sched.lock. This
        // new M will eventually run the scheduler to execute any
        // queued G's.
        // The caller incremented nmspinning, so set m.spinning in the new M.
        // Ownership transfer of pp committed by start in newm.
        // Preemption is now safe.
    if !{ let __v = (*lockheld.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
    if (*{ let __ptr_value = nmp.borrow(); __ptr_value.as_ref().unwrap().spinning.clone() }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("startm: m is spinning".to_string()))));
    }
    if {
        let __tmp_x = { let __selector_holder = { let __ptr_value = nmp.with_mut(|__ptr_value| __ptr_value.nextp.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x != __tmp_y
    } {
        throw(Arc::new(Mutex::new(Some("startm: m has p".to_string()))));
    }
    if { let __v = (*spinning.lock().unwrap().as_ref().unwrap()).clone(); __v } && !runqempty(pp.clone()) {
        throw(Arc::new(Mutex::new(Some("startm: p has runnable gs".to_string()))));
    }

        // The caller incremented nmspinning, so set m.spinning in the new M.
    { let new_val = spinning.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = nmp.with_mut(|__ptr_value| __ptr_value.spinning.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    (*{ let __ptr_value = nmp.with_mut(|__ptr_value| __ptr_value.nextp.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).set(pp.clone());
    notewakeup({ let __ptr_value = nmp.with_mut(|__ptr_value| __ptr_value.park.clone()); __ptr_value }.clone());

        // Ownership transfer of pp committed by wakeup. Preemption is now
        // safe.
    releasem(GoPtr::local(mp.clone()));
}

/// Hands off P from syscall or locked M.
/// Always runs without a P, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn handoffp(pp: GoPtr<crate::runtime2::p>) {
        // handoffp must start an M in any situation where
        // findrunnable would return a G to run on pp.
        // if it has local work, start it straight away
    if !runqempty(pp.clone()) || { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        startm(pp.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false))));
        return;
    }

        // if there's trace work to do, start it straight away
    if (trace_enabled() || trace_shutting_down()) && { let __nil_result = (*trace_reader_available().lock().unwrap()).is_some(); __nil_result } {
        startm(pp.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false))));
        return;
    }

        // if it has GC work, start it straight away
    if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } && gc_mark_work_available(pp.clone()) {
        startm(pp.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false))));
        return;
    }

        // no local work, check that there are no spinning/idle M's,
        // otherwise our help is not required
    if {
        let __go_cond_0 = {
            let __tmp_x = {
                let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).load();
                let __tmp_y = (*(*sched.lock().unwrap().as_ref().unwrap()).npidle.lock().unwrap().as_mut().unwrap()).load();
                __tmp_x + __tmp_y
            };
            let __tmp_y = 0 as i32;
            __tmp_x == __tmp_y
        };
        if __go_cond_0 {
            let __go_cond_1 = (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as i32))), Arc::new(Mutex::new(Some(1 as i32))));
            __go_cond_1
        } else {
            false
        }
    } {
        (*(*sched.lock().unwrap().as_ref().unwrap()).needspinning.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
        startm(pp.clone(), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(false))));
        return;
    }
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load() {
        { let new_val = __PGCSTOP as u32; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = nanotime(); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_stop_time.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let __target = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        notewakeup((*sched.lock().unwrap().as_ref().unwrap()).stopnote.clone());
    }
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return;
    }
    if { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().run_safe_point_fn.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } && internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.run_safe_point_fn.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(1 as u32))), Arc::new(Mutex::new(Some(0 as u32)))) {
        { let __f_holder = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_fn.clone(); let __f_ptr: *mut Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(pp.clone()) };
        { let __target = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).safe_point_wait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        notewakeup((*sched.lock().unwrap().as_ref().unwrap()).safe_point_note.clone());
    }
    }
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        startm(pp.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false))));
        return;
    }

        // If this is the last running P and nobody is polling network,
        // need to wakeup another M to poll network.
    if { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).npidle.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = { let __tmp_x = (*gomaxprocs.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x - __tmp_y }; __tmp_x == __tmp_y } && { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).lastpoll.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        startm(pp.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false))));
        return;
    }

        // The scheduler lock cannot be held when calling wakeNetPoller below
        // because wakeNetPoller may call wakep which may call startm.
    let mut when = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.timers.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).wake_time();
    pidleput(pp.clone(), Arc::new(Mutex::new(Some(0 as i64))));
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if { let __tmp_x = when; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        wake_net_poller(Arc::new(Mutex::new(Some(when))));
    }
}

/// Tries to add one more P to execute G's.
/// Called when a G is made runnable (newproc, ready).
/// Must be called with a P.
///
/// wakep should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gvisor.dev/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname wakep
pub fn wakep() {
        // Be conservative about spinning threads, only start one if none exist
        // already.
    if { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } || !(*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as i32))), Arc::new(Mutex::new(Some(1 as i32)))) {
        return;
    }

        // Disable preemption until ownership of pp transfers to the next M in
        // startm. Otherwise preemption here would leave pp stuck waiting to
        // enter _Pgcstop.
        //
        // See preemption comment on acquirem in startm for more details.
    let mut mp = acquirem();

    let mut pp: GoPtr<crate::runtime2::p> = GoPtr::nil();
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    { let (__tmp_0, __tmp_1) = pidleget_spinning(Arc::new(Mutex::new(Some(0 as i64)))); pp = __tmp_0.clone(); };
    if pp.is_nil() {
        if { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("wakep: negative nmspinning".to_string()))));
    }
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        releasem(GoPtr::local(mp.clone()));
        return;
    }

        // Since we always have a P, the race in the "No M is available"
        // comment in startm doesn't apply during the small window between the
        // unlock here and lock in startm. A checkdead in between will always
        // see at least one running M (ours).
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    startm(pp.clone(), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(false))));

    releasem(GoPtr::local(mp.clone()));
}

/// Stops execution of the current m that is locked to a g until the g is runnable again.
/// Returns with acquired P.
pub fn stoplockedm() {
    let mut gp = getg();

    if {
        let __go_cond_0 = {
            let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).lockedg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x == __tmp_y
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = { let __left_addr = crate::runtime2::muintptr::ptr(&(*{ let __ptr = crate::runtime2::guintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).lockedg.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().lockedm.clone() }.lock().unwrap().as_ref().unwrap())).addr(); let __right_addr = { let __ptr = GoPtr::local((*gp.lock().unwrap().as_ref().unwrap()).m.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq };
            __go_cond_1
        }
    } {
        throw(Arc::new(Mutex::new(Some("stoplockedm: inconsistent locking".to_string()))));
    }
    if {
        let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x != __tmp_y
    } {
                // Schedule another M to run this p.
        let mut pp: GoPtr<crate::runtime2::p> = releasep();
        handoffp(pp.clone());
    }
        // Schedule another M to run this p.
    incidlelocked(Arc::new(Mutex::new(Some(1 as i32))));

        // Wait until another thread schedules lockedg again.
    m_park();
    let mut status = readgstatus(crate::runtime2::guintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).lockedg.lock().unwrap().as_ref().unwrap())));
    if { let __tmp_x = { let __tmp_x = status; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GRUNNABLE as u32; __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime:stoplockedm: lockedg (atomicstatus=".to_string());
            let __go_print_arg_1 = format!("{}", status);
            let __go_print_arg_2 = format!("{}", ") is not Grunnable or Gscanrunnable\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        dumpgstatus(crate::runtime2::guintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).lockedg.lock().unwrap().as_ref().unwrap())));
        throw(Arc::new(Mutex::new(Some("stoplockedm: not runnable".to_string()))));
    }
    acquirep(crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).nextp.lock().unwrap().as_ref().unwrap())));
    { let new_val = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).nextp.lock().unwrap() = Some(new_val); };
}

/// Schedules the locked m to run the locked gp.
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn startlockedm(gp: GoPtr<crate::runtime2::g>) {
    let mut mp: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.lockedm.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
    if { let __left_addr = mp.addr(); let __right_addr = { let __ptr = GoPtr::local((*getg().lock().unwrap().as_ref().unwrap()).m.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq } {
        throw(Arc::new(Mutex::new(Some("startlockedm: locked to me".to_string()))));
    }
    if {
        let __tmp_x = { let __selector_holder = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.nextp.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x != __tmp_y
    } {
        throw(Arc::new(Mutex::new(Some("startlockedm: m has p".to_string()))));
    }

        // directly handoff current P to the locked m
    incidlelocked(Arc::new(Mutex::new(Some(-1 as i32))));
    let mut pp: GoPtr<crate::runtime2::p> = releasep();
    (*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.nextp.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).set(pp.clone());
    notewakeup({ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.park.clone()); __ptr_value }.clone());
    stopm();
}

/// Stops the current m for stopTheWorld.
/// Returns when the world is restarted.
pub fn gcstopm() {
    let mut gp = getg();

    if !(*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load() {
        throw(Arc::new(Mutex::new(Some("gcstopm: not waiting for gc".to_string()))));
    }
    if (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).spinning.lock().unwrap().as_ref().unwrap()) {
        { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).spinning.lock().unwrap() = Some(new_val); };
                // OK to just drop nmspinning here,
                // startTheWorld will unpark threads as necessary.
        if { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("gcstopm: negative nmspinning".to_string()))));
    }
    }
        // OK to just drop nmspinning here,
        // startTheWorld will unpark threads as necessary.
    let mut pp: GoPtr<crate::runtime2::p> = releasep();
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    { let new_val = __PGCSTOP as u32; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = nanotime(); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_stop_time.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        notewakeup((*sched.lock().unwrap().as_ref().unwrap()).stopnote.clone());
    }
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    stopm();
}

/// Schedules gp to run on the current M.
/// If inheritTime is true, gp inherits the remaining time in the
/// current time slice. Otherwise, it starts a new time slice.
/// Never returns.
///
/// Write barriers are allowed because this is called immediately after
/// acquiring a P in several places.
///
///go:yeswritebarrierrec
pub fn execute(gp: GoPtr<crate::runtime2::g>, inheritTime: Arc<Mutex<Option<bool>>>) {
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();

    if (*{ let __field = (*goroutineProfile.lock().unwrap().as_ref().unwrap()).active.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // Make sure that gp has had its stack written out to the goroutine
                // profile, exactly as it was when the goroutine profiler first stopped
                // the world.
        try_record_goroutine_profile(gp.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(move || { osyield() }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }

        // Make sure that gp has had its stack written out to the goroutine
        // profile, exactly as it was when the goroutine profiler first stopped
        // the world.
        // Assign gp.m before entering _Grunning so running Gs have an
        // M.
    { let new_val = gp.clone(); (*mp.lock().unwrap().as_mut().unwrap()).curg = new_val; };
    { let new_val = mp.clone(); gp.with_mut(|__ptr_value| { __ptr_value.m = new_val; }); };
    casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
    { let new_val = 0 as i64; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitsince.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = false; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.preempt.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stackguard0.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    if !{ let __v = (*inheritTime.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __target = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().schedtick.clone() }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Check whether the profiler needs to be turned on or off.
    let mut hz = Arc::new(Mutex::new(Some({ let __selector_holder = (*sched.lock().unwrap().as_ref().unwrap()).profilehz.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).profilehz.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*hz.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        set_thread_c_p_u_profiler(Arc::new(Mutex::new(Some({ let __arg_holder = hz.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_start();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    gogo({ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.clone());
}

/// Finds a runnable goroutine to execute.
/// Tries to steal from other P's, get g from local or global queue, poll network.
/// tryWakeP indicates that the returned goroutine is not normal (GC worker, trace
/// reader) so the caller should try to wake a P.
pub fn find_runnable() -> (GoPtr<crate::runtime2::g>, bool, bool) {
    let mut gp: Arc<Mutex<Option<g>>> = Arc::new(Mutex::new(None));
    let mut inheritTime: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut tryWakeP: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();

    'top: loop {
                // The conditions here and in handoffp must agree: if
                // findrunnable would return a G to run, handoffp must start
                // an M.
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        if (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load() {
        gcstopm();
        continue 'top;
    }
        if { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().run_safe_point_fn.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        run_safe_point_fn();
    }

                // now and pollUntil are saved for work stealing later,
                // which may steal timers. It's important that between now
                // and then, nothing blocks, so these numbers remain mostly
                // relevant.
        let (mut now, mut pollUntil, _) = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.timers.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).check(Arc::new(Mutex::new(Some(0 as i64))));

                // Try to schedule the trace reader.
        if trace_enabled() || trace_shutting_down() {
        let mut gp = trace_reader();
        if { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result } {
        let mut trace_local = trace_acquire();
        casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(0))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return (GoPtr::local(gp.clone()), false, true);
    }
    }

                // Try to schedule a GC worker.
        if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        let (mut gp, mut tnow) = (*gcController.lock().unwrap().as_mut().unwrap()).find_runnable_g_c_worker(pp.clone(), Arc::new(Mutex::new(Some(now))));
        if !gp.is_nil() {
        return (gp.clone(), false, true);
    }
        { let new_val = tnow; now = new_val; };
    }

                // Check the global runnable queue once in a while to ensure fairness.
                // Otherwise two goroutines can completely occupy the local runqueue
                // by constantly respawning each other.
        if { let __tmp_x = { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().schedtick.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 61 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let mut gp: GoPtr<crate::runtime2::g> = globrunqget(pp.clone(), Arc::new(Mutex::new(Some(1 as i32))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if !gp.is_nil() {
        return (gp.clone(), false, false);
    }
    }

                // Wake up the finalizer G.
        if { let __tmp_x = { let __tmp_x = (*fingStatus.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = ({ let __tmp_x = FING_WAIT as u32; let __tmp_y = FING_WAKE as u32; __tmp_x | __tmp_y }) as u32; __tmp_x & __tmp_y }; let __tmp_y = { let __tmp_x = FING_WAIT as u32; let __tmp_y = FING_WAKE as u32; __tmp_x | __tmp_y } as u32; __tmp_x == __tmp_y } {
        {
        let mut gp = wakefing();;
        if { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result } {
            ready(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(true))));;
        }
    }
    }
        if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr != 0 } {
        asmcgocall(Arc::new(Mutex::new(Some((*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(None)));
    }

                // local runq
        {
        let (mut gp, mut inheritTime) = runqget(pp.clone());;
        if !gp.is_nil() {
            return (gp.clone(), inheritTime, false);;
        }
    }

                // global runq
        if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let mut gp: GoPtr<crate::runtime2::g> = globrunqget(pp.clone(), Arc::new(Mutex::new(Some(0 as i32))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if !gp.is_nil() {
        return (gp.clone(), false, false);
    }
    }

                // Poll network.
                // This netpoll is only an optimization before we resort to stealing.
                // We can safely skip it if there are no waiters or a thread is blocked
                // in netpoll already. If there is any kind of logical race with that
                // blocked thread (e.g. it has already returned from netpoll, but does
                // not set lastpoll yet), this thread will do blocking netpoll below
                // anyway.
        if netpollinited() && netpoll_any_waiters() && { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).lastpoll.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        {
        let (mut list, mut delta) = netpoll(Arc::new(Mutex::new(Some(0 as i64))));;
        if !(*list.lock().unwrap().as_ref().unwrap()).empty() {
            let mut gp: GoPtr<crate::runtime2::g> = (*list.lock().unwrap().as_mut().unwrap()).pop();;
            injectglist(list.clone());;
            netpoll_adjust_waiters(Arc::new(Mutex::new(Some(delta))));;
            let mut trace_local = trace_acquire();;
            casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));;
            if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(gp.clone(), Arc::new(Mutex::new(Some(0))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    };
            return (gp.clone(), false, false);;
        }
    }
    }

                // non-blocking
                // Spinning Ms: steal work from other Ps.
                //
                // Limit the number of spinning Ms to half the number of busy Ps.
                // This is necessary to prevent excessive CPU consumption when
                // GOMAXPROCS>>1 but the program parallelism is low.
        if {
            let __go_cond_0 = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); __field }.lock().unwrap().as_ref().unwrap());
            if __go_cond_0 {
                true
            } else {
                let __go_cond_1 = {
                    let __tmp_x = { let __tmp_x = 2 as i32; let __tmp_y = (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).load(); __tmp_x * __tmp_y };
                    let __tmp_y = { let __tmp_x = (*gomaxprocs.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*sched.lock().unwrap().as_ref().unwrap()).npidle.lock().unwrap().as_mut().unwrap()).load(); __tmp_x - __tmp_y };
                    __tmp_x < __tmp_y
                };
                __go_cond_1
            }
        } {
        if !(*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __recv = mp.clone(); let __recv_ptr: *mut crate::runtime2::m = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::runtime2::m }; let __result = unsafe { &mut *__recv_ptr }.become_spinning(); __result };
    }
        let (mut gp, mut inheritTime, mut tnow, mut w, mut newWork) = steal_work(Arc::new(Mutex::new(Some(now))));
        if !gp.is_nil() {
                // Successfully stole.
        return (gp.clone(), inheritTime, false);
    }
                // Successfully stole.
        if newWork {
                // There may be new timer or GC work; restart to
                // discover.
        continue 'top;
    }
                // There may be new timer or GC work; restart to
                // discover.
        { let new_val = tnow; now = new_val; };
        if { let __tmp_x = w; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && ({ let __tmp_x = pollUntil; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = w; let __tmp_y = pollUntil; __tmp_x < __tmp_y }) {
                // Earlier timer to wait for.
        { let new_val = w; pollUntil = new_val; };
    }
    }

                // Successfully stole.
                // There may be new timer or GC work; restart to
                // discover.
                // Earlier timer to wait for.
                // We have nothing to do.
                //
                // If we're in the GC mark phase, can safely scan and blacken objects,
                // and have work to do, run idle-time marking rather than give up the P.
        if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } && gc_mark_work_available(pp.clone()) && (*gcController.lock().unwrap().as_ref().unwrap()).add_idle_mark_worker() {
        let mut node: GoPtr<crate::mgc::gcBgMarkWorkerNode> = GoPtr::raw({ let __ptr = (*gcBgMarkWorkerPool.lock().unwrap().as_ref().unwrap()).pop().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if !node.is_nil() {
        { let new_val = crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_IDLE_MODE as i32)))); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*{ let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.gp.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
        let mut trace_local = trace_acquire();
        casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(gp.clone(), Arc::new(Mutex::new(Some(0))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return (gp.clone(), false, false);
    }
        (*gcController.lock().unwrap().as_ref().unwrap()).remove_idle_mark_worker();
    }

                // wasm only:
                // If a callback returned and no other goroutine is awake,
                // then wake event handler goroutine which pauses execution
                // until a callback was triggered.
        let (__tmp_0, mut otherReady) = before_idle(Arc::new(Mutex::new(Some(now))), Arc::new(Mutex::new(Some(pollUntil)))); gp = __tmp_0.clone();;
        if { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result } {
        let mut trace_local = trace_acquire();
        casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(0))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return (GoPtr::local(gp.clone()), false, false);
    }
        if otherReady {
        continue 'top;
    }

                // Before we drop our P, make a snapshot of the allp slice,
                // which can change underfoot once we no longer block
                // safe-points. We don't need to snapshot the contents because
                // everything up to cap(allp) is immutable.
        let mut allpSnapshot = Arc::new(Mutex::new(Some((*allp.lock().unwrap().as_ref().unwrap()).clone())));

                // Also snapshot masks. Value changes are OK, but we can't allow
                // len to change out from under us.
        let mut idlepMaskSnapshot = Arc::new(Mutex::new(Some((*idlepMask.lock().unwrap().as_ref().unwrap()).clone())));
        let mut timerpMaskSnapshot = Arc::new(Mutex::new(Some((*timerpMask.lock().unwrap().as_ref().unwrap()).clone())));

                // return P and block
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load() || { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().run_safe_point_fn.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        continue 'top;
    }
        if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        let mut gp: GoPtr<crate::runtime2::g> = globrunqget(pp.clone(), Arc::new(Mutex::new(Some(0 as i32))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return (gp.clone(), false, false);
    }
        if !(*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).needspinning.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 1 as u32; __tmp_x == __tmp_y } {
                // See "Delicate dance" comment below.
        { let __recv = mp.clone(); let __recv_ptr: *mut crate::runtime2::m = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::runtime2::m }; let __result = unsafe { &mut *__recv_ptr }.become_spinning(); __result };
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        continue 'top;
    }
                // See "Delicate dance" comment below.
        if { let __left_addr = releasep().addr(); let __right_addr = pp.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        throw(Arc::new(Mutex::new(Some("findrunnable: wrong p".to_string()))));
    }
        { let new_val = pidleput(pp.clone(), Arc::new(Mutex::new(Some(now)))); now = new_val; };
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

                // Delicate dance: thread transitions from spinning to non-spinning
                // state, potentially concurrently with submission of new work. We must
                // drop nmspinning first and then check all sources again (with
                // #StoreLoad memory barrier in between). If we do it the other way
                // around, another thread can submit work after we've checked all
                // sources but before we drop nmspinning; as a result nobody will
                // unpark a thread to run the work.
                //
                // This applies to the following sources of work:
                //
                // * Goroutines added to the global or a per-P run queue.
                // * New/modified-earlier timers on a per-P timer heap.
                // * Idle-priority GC work (barring golang.org/issue/19112).
                //
                // If we discover new work below, we need to restore m.spinning as a
                // signal for resetspinning to unpark a new worker thread (because
                // there can be more than one starving goroutine).
                //
                // However, if after discovering new work we also observe no idle Ps
                // (either here or in resetspinning), we have a problem. We may be
                // racing with a non-spinning M in the block above, having found no
                // work and preparing to release its P and park. Allowing that P to go
                // idle will result in loss of work conservation (idle P while there is
                // runnable work). This could result in complete deadlock in the
                // unlikely event that we discover new work (from netpoll) right as we
                // are racing with _all_ other Ps going idle.
                //
                // We use sched.needspinning to synchronize with non-spinning Ms going
                // idle. If needspinning is set when they are about to drop their P,
                // they abort the drop and instead become a new spinning M on our
                // behalf. If we are not racing and the system is truly fully loaded
                // then no spinning threads are required, and the next thread to
                // naturally become spinning will clear the flag.
                //
                // Also see "Worker thread parking/unparking" comment at the top of the
                // file.
        let mut wasSpinning = Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = false; *(*mp.lock().unwrap().as_ref().unwrap()).spinning.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("findrunnable: negative nmspinning".to_string()))));
    }
                // Note the for correctness, only the last M transitioning from
                // spinning to non-spinning must perform these rechecks to
                // ensure no missed work. However, the runtime has some cases
                // of transient increments of nmspinning that are decremented
                // without going through this path, so we must be conservative
                // and perform the check on all spinning Ms.
                //
                // See https://go.dev/issue/43997.
                // Check global and P runqueues again.
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        let (mut pp, _) = pidleget_spinning(Arc::new(Mutex::new(Some(0 as i64))));
        if !pp.is_nil() {
        let mut gp: GoPtr<crate::runtime2::g> = globrunqget(pp.clone(), Arc::new(Mutex::new(Some(0 as i32))));
        if gp.is_nil() {
        throw(Arc::new(Mutex::new(Some("global runq empty with non-zero runqsize".to_string()))));
    }
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        acquirep(pp.clone());
        { let __recv = mp.clone(); let __recv_ptr: *mut crate::runtime2::m = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::runtime2::m }; let __result = unsafe { &mut *__recv_ptr }.become_spinning(); __result };
        return (gp.clone(), false, false);
    }
    }
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let mut pp: GoPtr<crate::runtime2::p> = check_runqs_no_p(allpSnapshot.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = idlepMaskSnapshot.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if !pp.is_nil() {
        acquirep(pp.clone());
        { let __recv = mp.clone(); let __recv_ptr: *mut crate::runtime2::m = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::runtime2::m }; let __result = unsafe { &mut *__recv_ptr }.become_spinning(); __result };
        continue 'top;
    }
                // Check for idle-priority GC work again.
        let (__tmp_0, mut gp) = check_idle_g_c_no_p(); pp = __tmp_0;;
        if !pp.is_nil() {
        acquirep(pp.clone());
        { let __recv = mp.clone(); let __recv_ptr: *mut crate::runtime2::m = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::runtime2::m }; let __result = unsafe { &mut *__recv_ptr }.become_spinning(); __result };
                // Run the idle worker.
        { let new_val = crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_IDLE_MODE as i32)))); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        let mut trace_local = trace_acquire();
        casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(gp.clone(), Arc::new(Mutex::new(Some(0))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return (gp.clone(), false, false);
    }
                // Run the idle worker.
                // Finally, check for timer creation or expiry concurrently with
                // transitioning from spinning to non-spinning.
                //
                // Note that we cannot use checkTimers here because it calls
                // adjusttimers which may need to allocate memory, and that isn't
                // allowed when we don't have an active P.
        { let new_val = check_timers_no_p(allpSnapshot.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = timerpMaskSnapshot.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(pollUntil)))); pollUntil = new_val; };
    }

                // Note the for correctness, only the last M transitioning from
                // spinning to non-spinning must perform these rechecks to
                // ensure no missed work. However, the runtime has some cases
                // of transient increments of nmspinning that are decremented
                // without going through this path, so we must be conservative
                // and perform the check on all spinning Ms.
                //
                // See https://go.dev/issue/43997.
                // Check global and P runqueues again.
                // Check for idle-priority GC work again.
                // Run the idle worker.
                // Finally, check for timer creation or expiry concurrently with
                // transitioning from spinning to non-spinning.
                //
                // Note that we cannot use checkTimers here because it calls
                // adjusttimers which may need to allocate memory, and that isn't
                // allowed when we don't have an active P.
                // Poll network until next timer.
        if netpollinited() && (netpoll_any_waiters() || { let __tmp_x = pollUntil; let __tmp_y = 0 as i64; __tmp_x != __tmp_y }) && { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).lastpoll.lock().unwrap().as_mut().unwrap()).swap(Arc::new(Mutex::new(Some(0 as i64)))); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        (*(*sched.lock().unwrap().as_ref().unwrap()).poll_until.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(pollUntil))));
        if {
            let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
        throw(Arc::new(Mutex::new(Some("findrunnable: netpoll with p".to_string()))));
    }
        if (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("findrunnable: netpoll with spinning".to_string()))));
    }
        let mut delay = Arc::new(Mutex::new(Some(-(1) as i64)));
        if { let __tmp_x = pollUntil; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        if { let __tmp_x = now; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = nanotime(); now = new_val; };
    }
        { let new_val = { let __tmp_x = pollUntil; let __tmp_y = now; __tmp_x - __tmp_y }; *delay.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*delay.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = 0 as i64; *delay.lock().unwrap() = Some(new_val); };
    }
    }
        if { let __tmp_x = (*faketime.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
                // When using fake time, just poll.
        { let new_val = 0 as i64; *delay.lock().unwrap() = Some(new_val); };
    }
                // When using fake time, just poll.
        let (mut list, mut delta) = netpoll(Arc::new(Mutex::new(Some({ let __arg_holder = delay.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Refresh now again, after potentially blocking.
        { let new_val = nanotime(); now = new_val; };
        (*(*sched.lock().unwrap().as_ref().unwrap()).poll_until.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*(*sched.lock().unwrap().as_ref().unwrap()).lastpoll.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(now))));
        if { let __tmp_x = (*faketime.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && (*list.lock().unwrap().as_ref().unwrap()).empty() {
                // Using fake time and nothing is ready; stop M.
                // When all M's stop, checkdead will call timejump.
        stopm();
        continue 'top;
    }
                // Using fake time and nothing is ready; stop M.
                // When all M's stop, checkdead will call timejump.
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let (mut pp, _) = pidleget(Arc::new(Mutex::new(Some(now))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if pp.is_nil() {
        injectglist(list.clone());
        netpoll_adjust_waiters(Arc::new(Mutex::new(Some(delta))));
    } else {
        acquirep(pp.clone());
        if !(*list.lock().unwrap().as_ref().unwrap()).empty() {
        let mut gp: GoPtr<crate::runtime2::g> = (*list.lock().unwrap().as_mut().unwrap()).pop();
        injectglist(list.clone());
        netpoll_adjust_waiters(Arc::new(Mutex::new(Some(delta))));
        let mut trace_local = trace_acquire();
        casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(gp.clone(), Arc::new(Mutex::new(Some(0))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return (gp.clone(), false, false);
    }
        if { let __v = (*wasSpinning.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __recv = mp.clone(); let __recv_ptr: *mut crate::runtime2::m = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::runtime2::m }; let __result = unsafe { &mut *__recv_ptr }.become_spinning(); __result };
    }
        continue 'top;
    }
    } else if { let __tmp_x = pollUntil; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && netpollinited() {
        let mut pollerPollUntil = (*(*sched.lock().unwrap().as_ref().unwrap()).poll_until.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = pollerPollUntil; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = pollerPollUntil; let __tmp_y = pollUntil; __tmp_x > __tmp_y } {
        netpoll_break();
    }
    }
                // When using fake time, just poll.
                // block until new work is available
                // Refresh now again, after potentially blocking.
                // Using fake time and nothing is ready; stop M.
                // When all M's stop, checkdead will call timejump.
        stopm();
        continue 'top;
    };
}

/// pollWork reports whether there is non-background work this P could
/// be doing. This is a fairly lightweight check to be used for
/// background work loops, like idle GC. It checks a subset of the
/// conditions checked by the actual scheduler.
pub fn poll_work() -> bool {
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        return true;
    }
    let mut p: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    if !runqempty(p.clone()) {
        return true;
    }
    if netpollinited() && netpoll_any_waiters() && { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).lastpoll.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        {
        let (mut list, mut delta) = netpoll(Arc::new(Mutex::new(Some(0 as i64))));;
        if !(*list.lock().unwrap().as_ref().unwrap()).empty() {
            injectglist(list.clone());;
            netpoll_adjust_waiters(Arc::new(Mutex::new(Some(delta))));;
            return true;;
        }
    }
    }
    false
}

/// stealWork attempts to steal a runnable goroutine or timer from any P.
///
/// If newWork is true, new work may have been readied.
///
/// If now is not 0 it is the current time. stealWork returns the passed time or
/// the current time if now was passed as 0.
pub fn steal_work(mut now: Arc<Mutex<Option<i64>>>) -> (GoPtr<crate::runtime2::g>, bool, i64, i64, bool) {
    let mut gp: Arc<Mutex<Option<g>>> = Arc::new(Mutex::new(None));
    let mut inheritTime: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut rnow: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut pollUntil: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut newWork: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));

    let mut ranTimer = Arc::new(Mutex::new(Some(false)));

    const stealTries: i32 = 4;

    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        let mut stealTimersOrRunNextG = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x == __tmp_y })));

        let mut r#enum = (*stealOrder.lock().unwrap().as_ref().unwrap()).start(Arc::new(Mutex::new(Some(cheaprand()))));
    while !(*r#enum.lock().unwrap().as_ref().unwrap()).done() {
        if (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load() {
                // GC work may be available.
        return (GoPtr::nil(), false, { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
                // GC work may be available.
        let mut p2 = { let __seq = { let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[((*r#enum.lock().unwrap().as_ref().unwrap()).position()) as usize].clone() }.clone();
        if { let __left_addr = pp.addr(); let __right_addr = { let __ptr = GoPtr::local(p2.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq } {
        (*r#enum.lock().unwrap().as_mut().unwrap()).next();; continue
    }

                // Steal timers from p2. This call to checkTimers is the only place
                // where we might hold a lock on a different P's timers. We do this
                // once on the last pass before checking runnext because stealing
                // from the other P's runnext should be the last resort, so if there
                // are timers to steal do that first.
                //
                // We only check timers on one of the stealing iterations because
                // the time stored in now doesn't change in this loop and checking
                // the timers for each P more than once with the same value of now
                // is probably a waste of time.
                //
                // timerpMask tells us whether the P may have timers at all. If it
                // can't, no need to check at all.
        if { let __v = (*stealTimersOrRunNextG.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*timerpMask.lock().unwrap().as_ref().unwrap()).read(Arc::new(Mutex::new(Some((*r#enum.lock().unwrap().as_ref().unwrap()).position())))) {
        let (mut tnow, mut w, mut ran) = (*(*p2.lock().unwrap().as_ref().unwrap()).timers.lock().unwrap().as_mut().unwrap()).check(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = tnow; *now.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = w; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && ({ let __tmp_x = { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = w; let __tmp_y = { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) {
        { let new_val = w; *pollUntil.lock().unwrap() = Some(new_val); };
    }
        if ran {
                // Running the timers may have
                // made an arbitrary number of G's
                // ready and added them to this P's
                // local run queue. That invalidates
                // the assumption of runqsteal
                // that it always has room to add
                // stolen G's. So check now if there
                // is a local G to run.
        {
        let (mut gp, mut inheritTime) = runqget(pp.clone());;
        if !gp.is_nil() {
            return (gp.clone(), inheritTime, { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*ranTimer.lock().unwrap().as_ref().unwrap()).clone(); __v });;
        }
    }
        { let new_val = true; *ranTimer.lock().unwrap() = Some(new_val); };
    }
    }

                // Running the timers may have
                // made an arbitrary number of G's
                // ready and added them to this P's
                // local run queue. That invalidates
                // the assumption of runqsteal
                // that it always has room to add
                // stolen G's. So check now if there
                // is a local G to run.
                // Don't bother to attempt to steal if p2 is idle.
        if !(*idlepMask.lock().unwrap().as_ref().unwrap()).read(Arc::new(Mutex::new(Some((*r#enum.lock().unwrap().as_ref().unwrap()).position())))) {
        {
        let mut gp: GoPtr<crate::runtime2::g> = runqsteal(pp.clone(), p2.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = stealTimersOrRunNextG.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if !gp.is_nil() {
            return (gp.clone(), false, { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*ranTimer.lock().unwrap().as_ref().unwrap()).clone(); __v });;
        }
    }
    }
        (*r#enum.lock().unwrap().as_mut().unwrap()).next();
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // GC work may be available.
        // Steal timers from p2. This call to checkTimers is the only place
        // where we might hold a lock on a different P's timers. We do this
        // once on the last pass before checking runnext because stealing
        // from the other P's runnext should be the last resort, so if there
        // are timers to steal do that first.
        //
        // We only check timers on one of the stealing iterations because
        // the time stored in now doesn't change in this loop and checking
        // the timers for each P more than once with the same value of now
        // is probably a waste of time.
        //
        // timerpMask tells us whether the P may have timers at all. If it
        // can't, no need to check at all.
        // Running the timers may have
        // made an arbitrary number of G's
        // ready and added them to this P's
        // local run queue. That invalidates
        // the assumption of runqsteal
        // that it always has room to add
        // stolen G's. So check now if there
        // is a local G to run.
        // Don't bother to attempt to steal if p2 is idle.
        // No goroutines found to steal. Regardless, running a timer may have
        // made some goroutine ready that we missed. Indicate the next timer to
        // wait for.
    return (GoPtr::nil(), false, { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*ranTimer.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

/// Check all Ps for a runnable G to steal.
///
/// On entry we have no P. If a G is available to steal and a P is available,
/// the P is returned which the caller should acquire and attempt to steal the
/// work to.
pub fn check_runqs_no_p(allpSnapshot: Arc<Mutex<Option<Vec<Arc<Mutex<Option<p>>>>>>>, idlepMaskSnapshot: Arc<Mutex<Option<pMask>>>) -> GoPtr<crate::runtime2::p> {
    { let __range_holder = allpSnapshot.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (id, p2) in __range_values.iter().enumerate() {
        if !(*idlepMaskSnapshot.lock().unwrap().as_ref().unwrap()).read(Arc::new(Mutex::new(Some(id as u32)))) && !runqempty(GoPtr::local(p2.clone())) {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let (mut pp, _) = pidleget_spinning(Arc::new(Mutex::new(Some(0 as i64))));
        if pp.is_nil() {
                // Can't get a P, don't bother checking remaining Ps.
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return GoPtr::nil();
    }
                // Can't get a P, don't bother checking remaining Ps.
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return pp.clone();
    }
    } }

        // Can't get a P, don't bother checking remaining Ps.
        // No work available.
    return GoPtr::nil();
}

/// Check all Ps for a timer expiring sooner than pollUntil.
///
/// Returns updated pollUntil value.
pub fn check_timers_no_p(allpSnapshot: Arc<Mutex<Option<Vec<Arc<Mutex<Option<p>>>>>>>, timerpMaskSnapshot: Arc<Mutex<Option<pMask>>>, mut pollUntil: Arc<Mutex<Option<i64>>>) -> i64 {
    { let __range_holder = allpSnapshot.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (id, p2) in __range_values.iter().enumerate() {
        if (*timerpMaskSnapshot.lock().unwrap().as_ref().unwrap()).read(Arc::new(Mutex::new(Some(id as u32)))) {
        let mut w = (*(*p2.lock().unwrap().as_ref().unwrap()).timers.lock().unwrap().as_ref().unwrap()).wake_time();
        if { let __tmp_x = w; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && ({ let __tmp_x = { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = w; let __tmp_y = { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) {
        { let new_val = w; *pollUntil.lock().unwrap() = Some(new_val); };
    }
    }
    } }

    return { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// Check for idle-priority GC, without a P on entry.
///
/// If some GC work, a P, and a worker G are all available, the P and G will be
/// returned. The returned P has not been wired yet.
pub fn check_idle_g_c_no_p() -> (GoPtr<crate::runtime2::p>, GoPtr<crate::runtime2::g>) {
        // N.B. Since we have no P, gcBlackenEnabled may change at any time; we
        // must check again after acquiring a P. As an optimization, we also check
        // if an idle mark worker is needed at all. This is OK here, because if we
        // observe that one isn't needed, at least one is currently running. Even if
        // it stops running, its own journey into the scheduler should schedule it
        // again, if need be (at which point, this check will pass, if relevant).
    if { let __tmp_x = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local(gcBlackenEnabled.clone())); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || !(*gcController.lock().unwrap().as_ref().unwrap()).need_idle_mark_worker() {
        return (GoPtr::nil(), GoPtr::nil());
    }
    if !gc_mark_work_available(GoPtr::nil()) {
        return (GoPtr::nil(), GoPtr::nil());
    }

        // Work is available; we can start an idle GC worker only if there is
        // an available P and available worker G.
        //
        // We can attempt to acquire these in either order, though both have
        // synchronization concerns (see below). Workers are almost always
        // available (see comment in findRunnableGCWorker for the one case
        // there may be none). Since we're slightly less likely to find a P,
        // check for that first.
        //
        // Synchronization: note that we must hold sched.lock until we are
        // committed to keeping it. Otherwise we cannot put the unnecessary P
        // back in sched.pidle without performing the full set of idle
        // transition checks.
        //
        // If we were to check gcBgMarkWorkerPool first, we must somehow handle
        // the assumption in gcControllerState.findRunnableGCWorker that an
        // empty gcBgMarkWorkerPool is only possible if gcMarkDone is running.
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let (mut pp, mut now) = pidleget_spinning(Arc::new(Mutex::new(Some(0 as i64))));
    if pp.is_nil() {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return (GoPtr::nil(), GoPtr::nil());
    }

        // Now that we own a P, gcBlackenEnabled can't change (as it requires STW).
    if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } || !(*gcController.lock().unwrap().as_ref().unwrap()).add_idle_mark_worker() {
        pidleput(pp.clone(), Arc::new(Mutex::new(Some(now))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return (GoPtr::nil(), GoPtr::nil());
    }

    let mut node: GoPtr<crate::mgc::gcBgMarkWorkerNode> = GoPtr::raw({ let __ptr = (*gcBgMarkWorkerPool.lock().unwrap().as_ref().unwrap()).pop().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    if node.is_nil() {
        pidleput(pp.clone(), Arc::new(Mutex::new(Some(now))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*gcController.lock().unwrap().as_ref().unwrap()).remove_idle_mark_worker();
        return (GoPtr::nil(), GoPtr::nil());
    }

    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    (
        pp.clone(),
        crate::runtime2::guintptr::ptr(&(*{ let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.gp.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()))
    )
}

/// wakeNetPoller wakes up the thread sleeping in the network poller if it isn't
/// going to wake up before the when argument; or it wakes an idle P to service
/// timers and the network poller if there isn't one already.
pub fn wake_net_poller(when: Arc<Mutex<Option<i64>>>) {
    if { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).lastpoll.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // In findrunnable we ensure that when polling the pollUntil
                // field is either zero or the time to which the current
                // poll is expected to run. This can have a spurious wakeup
                // but should never miss a wakeup.
        let mut pollerPollUntil = (*(*sched.lock().unwrap().as_ref().unwrap()).poll_until.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = pollerPollUntil; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = pollerPollUntil; let __tmp_y = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        netpoll_break();
    }
    } else {
                // There are no threads in the network poller, try to get
                // one there so it can handle new timers.
        if { let __tmp_x = "darwin".to_string(); let __tmp_y = "plan9".to_string(); __tmp_x != __tmp_y } {
        wakep();
    }
    }
}

pub fn resetspinning() {
    let mut gp = getg();
    if !(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).spinning.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("resetspinning: not a spinning m".to_string()))));
    }
    { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).spinning.lock().unwrap() = Some(new_val); };
    let mut nmspinning = (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
    if { let __tmp_x = nmspinning; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("findrunnable: negative nmspinning".to_string()))));
    }

        // M wakeup policy is deliberately somewhat conservative, so check if we
        // need to wakeup another P here. See "Worker thread parking/unparking"
        // comment at the top of the file for details.
    wakep();
}

/// injectglist adds each runnable G on the list to some run queue,
/// and clears glist. If there is no current P, they are added to the
/// global queue, and up to npidle M's are started to run them.
/// Otherwise, for each idle P, this adds a G to the global queue
/// and starts an M. Any remaining G's are added to the current P's
/// local run queue.
/// This may temporarily acquire sched.lock.
/// Can run concurrently with GC.
pub fn injectglist(glist: Arc<Mutex<Option<gList>>>) {
    if { let __recv = glist.clone(); let __recv_ptr: *const gList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const gList }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        return;
    }

        // Mark all the goroutines as runnable before we put them
        // on the run queues.
    let mut head: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*(*glist.lock().unwrap().as_ref().unwrap()).head.lock().unwrap().as_ref().unwrap()));
    let mut tail: GoPtr<crate::runtime2::g> = GoPtr::nil();
    let mut qsize = Arc::new(Mutex::new(Some(0)));
    let mut trace_local = trace_acquire();
    let mut gp: GoPtr<crate::runtime2::g> = head.clone();
    while !gp.is_nil() {
        tail = gp.clone();
        { let mut guard = qsize.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(gp.clone(), Arc::new(Mutex::new(Some(0))));
    }
        gp = crate::runtime2::guintptr::ptr(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
    }
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Turn the gList into a gQueue.
    let mut q: Arc<Mutex<Option<gQueue>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*(*q.lock().unwrap().as_ref().unwrap()).head.lock().unwrap().as_mut().unwrap()).set(head.clone());
    (*(*q.lock().unwrap().as_ref().unwrap()).tail.lock().unwrap().as_mut().unwrap()).set(tail.clone());
    { let new_val = gList { head: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))) }; *glist.lock().unwrap() = Some(new_val); };

    let mut startIdle = Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| {
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut mp = acquirem();
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let (mut pp, _) = pidleget_spinning(Arc::new(Mutex::new(Some(0 as i64))));
        if pp.is_nil() {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        releasem(GoPtr::local(mp.clone()));
        break
    }
        startm(pp.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(true))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        releasem(GoPtr::local(mp.clone()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>)));

        // See comment in startm.
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    if pp.is_nil() {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        globrunqputbatch(q.clone(), Arc::new(Mutex::new(Some((*qsize.lock().unwrap().as_ref().unwrap()) as i32))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = startIdle.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(qsize.clone()) };
        return;
    }

    let mut npidle = Arc::new(Mutex::new(Some((*(*sched.lock().unwrap().as_ref().unwrap()).npidle.lock().unwrap().as_mut().unwrap()).load() as i32)));
    let mut globq: Arc<Mutex<Option<gQueue>>> = Arc::new(Mutex::new(Some(Default::default())));let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let new_val = 0; *n.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*npidle.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && !(*q.lock().unwrap().as_ref().unwrap()).empty() {
        let mut g: GoPtr<crate::runtime2::g> = (*q.lock().unwrap().as_mut().unwrap()).pop();
        (*globq.lock().unwrap().as_ref().unwrap()).push_back(g.clone());
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        globrunqputbatch(globq.clone(), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = startIdle.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(n.clone()) };
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = qsize.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }

    if !(*q.lock().unwrap().as_ref().unwrap()).empty() {
        runqputbatch(pp.clone(), q.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = qsize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Some P's might have become idle after we loaded `sched.npidle`
        // but before any goroutines were added to the queue, which could
        // lead to idle P's when there is work available in the global queue.
        // That could potentially last until other goroutines become ready
        // to run. That said, we need to find a way to hedge
        //
        // Calling wakep() here is the best bet, it will do nothing in the
        // common case (no racing on `sched.npidle`), while it could wake one
        // more P to execute G's, which might end up with >1 P's: the first one
        // wakes another P and so forth until there is no more work, but this
        // ought to be an extremely rare case.
        //
        // Also see "Worker thread parking/unparking" comment at the top of the file for details.
    wakep();
}

/// One round of scheduler: find a runnable goroutine and execute it.
/// Never returns.
pub fn schedule() {
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();

    if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("schedule: holding locks".to_string()))));
    }

    if {
        let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).lockedg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x != __tmp_y
    } {
        stoplockedm();
        execute(crate::runtime2::guintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).lockedg.lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(Some(false))));
    }

        // Never returns.
        // We should not schedule away from a g that is executing a cgo call,
        // since the cgo call is using the m's g0 stack.
    if (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).incgo.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("schedule: in cgo".to_string()))));
    }

    'top: loop {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        { let new_val = false; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.preempt.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

                // Safety check: if we are spinning, the run queue should be empty.
                // Check this before calling checkTimers, as that might call
                // goready to put a ready goroutine on the local run queue.
        if {
            let __go_cond_0 = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); __field }.lock().unwrap().as_ref().unwrap());
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __go_cond_2 = {
                        let __tmp_x = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runnext.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                        let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
                        __tmp_x != __tmp_y
                    };
                    if __go_cond_2 {
                        true
                    } else {
                        let __go_cond_3 = { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().runqhead.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().runqtail.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y };
                        __go_cond_3
                    }
                };
                __go_cond_1
            } else {
                false
            }
        } {
        throw(Arc::new(Mutex::new(Some("schedule: spinning with local work".to_string()))));
    }

        let (mut gp, mut inheritTime, mut tryWakeP) = find_runnable();

        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).dontfreezetheworld.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } && (*freezing.lock().unwrap().as_ref().unwrap()).load() {
                // See comment in freezetheworld. We don't want to perturb
                // scheduler state, so we didn't gcstopm in findRunnable, but
                // also don't want to allow new goroutines to run.
                //
                // Deadlock here rather than in the findRunnable loop so if
                // findRunnable is stuck in a loop we don't perturb that
                // either.
        lock(GoPtr::local(deadlock.clone()));
        lock(GoPtr::local(deadlock.clone()));
    }

                // See comment in freezetheworld. We don't want to perturb
                // scheduler state, so we didn't gcstopm in findRunnable, but
                // also don't want to allow new goroutines to run.
                //
                // Deadlock here rather than in the findRunnable loop so if
                // findRunnable is stuck in a loop we don't perturb that
                // either.
                // This thread is going to run a goroutine and is not spinning anymore,
                // so if it was marked as spinning we need to reset it now and potentially
                // start a new spinning M.
        if (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        resetspinning();
    }

        if (*(*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).user.lock().unwrap().as_ref().unwrap()) && !sched_enabled(gp.clone()) {
                // Scheduling of this goroutine is disabled. Put it on
                // the list of pending runnable goroutines for when we
                // re-enable user scheduling and look again.
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if sched_enabled(gp.clone()) {
                // Something re-enabled scheduling while we
                // were acquiring the lock.
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    } else {
        (*(*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).runnable.lock().unwrap().as_ref().unwrap()).push_back(gp.clone());
        { let __target = (*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        continue 'top;
    }
    }

                // Scheduling of this goroutine is disabled. Put it on
                // the list of pending runnable goroutines for when we
                // re-enable user scheduling and look again.
                // Something re-enabled scheduling while we
                // were acquiring the lock.
                // If about to schedule a not-normal goroutine (a GCworker or tracereader),
                // wake a P if there is one.
        if tryWakeP {
        wakep();
    }
        if {
            let __tmp_x = { let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.lockedm.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
                // Hands off own p to the locked m,
                // then blocks waiting for a new p.
        startlockedm(gp.clone());
        continue 'top;
    }

                // Hands off own p to the locked m,
                // then blocks waiting for a new p.
        execute(gp.clone(), Arc::new(Mutex::new(Some(inheritTime))));
        break 'top;
    };
}

/// dropg removes the association between m and the current goroutine m->curg (gp for short).
/// Typically a caller sets gp's status away from Grunning and then
/// immediately calls dropg to finish the job. The caller is also responsible
/// for arranging that gp will be restarted using ready at an
/// appropriate time. After calling dropg and arranging for gp to be
/// readied later, the caller can do other work but eventually should
/// call schedule to restart the scheduling of goroutines on this m.
pub fn dropg() {
    let mut gp = getg();

    set_m_no_w_b(Arc::new(Mutex::new(Some({ let __ptr_value = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone()))), Arc::new(Mutex::new(None)));
    set_g_no_w_b(GoPtr::local(Arc::new(Mutex::new(Some((*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone())))), GoPtr::nil());
}

pub fn parkunlock_c(gp: Arc<Mutex<Option<g>>>, lock: Arc<Mutex<Option<usize>>>) -> bool {
    unlock(GoPtr::raw({ let __ptr = lock.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
    true
}

/// park continuation on g0.
pub fn park_m(gp: Arc<Mutex<Option<g>>>) {
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();

    let mut trace_local = trace_acquire();

        // If g is in a synctest group, we don't want to let the group
        // become idle until after the waitunlockf (if any) has confirmed
        // that the park is happening.
        // We need to record gp.syncGroup here, since waitunlockf can change it.
    let mut sg = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone();
    if { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = sg.clone(); let __recv_ptr: *mut crate::synctest::synctestGroup = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::synctest::synctestGroup }; let __result = unsafe { &mut *__recv_ptr }.inc_active(); __result };
    }

    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // Trace the event before the transition. It may take a
                // stack trace, but we won't own the stack after the
                // transition anymore.
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_park(Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).wait_trace_block_reason.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).wait_trace_skip.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }

        // Trace the event before the transition. It may take a
        // stack trace, but we won't own the stack after the
        // transition anymore.
        // N.B. Not using casGToWaiting here because the waitreason is
        // set by park_m's caller.
    casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(__GWAITING as u32))));
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    dropg();

    {
        let mut r#fn = (*mp.lock().unwrap().as_ref().unwrap()).waitunlockf.clone();;
        if { let __nil_result = (*r#fn.lock().unwrap()).is_some(); __nil_result } {
            let mut ok = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(gp.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).waitlock.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) };;
            *(*mp.lock().unwrap().as_ref().unwrap()).waitunlockf.lock().unwrap() = None;;
            *(*mp.lock().unwrap().as_ref().unwrap()).waitlock.lock().unwrap() = None;;
            if !ok {
        let mut trace_local = trace_acquire();
        casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
        if { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = sg.clone(); let __recv_ptr: *mut crate::synctest::synctestGroup = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::synctest::synctestGroup }; let __result = unsafe { &mut *__recv_ptr }.dec_active(); __result };
    }
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(2))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        execute(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(true))));
    };
        }
    }

        // Schedule it back, never returns.
    if { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = sg.clone(); let __recv_ptr: *mut crate::synctest::synctestGroup = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::synctest::synctestGroup }; let __result = unsafe { &mut *__recv_ptr }.dec_active(); __result };
    }

    schedule();
}

pub fn gosched_impl(gp: GoPtr<crate::runtime2::g>, preempted: Arc<Mutex<Option<bool>>>) {
    let mut trace_local = trace_acquire();
    let mut status = readgstatus(gp.clone());
    if { let __tmp_x = { let __tmp_x = status; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GRUNNING as u32; __tmp_x != __tmp_y } {
        dumpgstatus(gp.clone());
        throw(Arc::new(Mutex::new(Some("bad g status".to_string()))));
    }
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // Trace the event before the transition. It may take a
                // stack trace, but we won't own the stack after the
                // transition anymore.
        if { let __v = (*preempted.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_preempt();
    } else {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_sched();
    }
    }
        // Trace the event before the transition. It may take a
        // stack trace, but we won't own the stack after the
        // transition anymore.
    casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    dropg();
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    globrunqput(gp.clone());
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if (*mainStarted.lock().unwrap().as_ref().unwrap()) {
        wakep();
    }

    schedule();
}

/// Gosched continuation on g0.
pub fn gosched_m(gp: Arc<Mutex<Option<g>>>) {
    gosched_impl(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(false))));
}

/// goschedguarded is a forbidden-states-avoided version of gosched_m.
pub fn goschedguarded_m(gp: Arc<Mutex<Option<g>>>) {
    if !can_preempt_m({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field }) {
        gogo((*gp.lock().unwrap().as_ref().unwrap()).sched.clone());
    }
        // never return
    gosched_impl(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(false))));
}

pub fn gopreempt_m(gp: GoPtr<crate::runtime2::g>) {
    gosched_impl(gp.clone(), Arc::new(Mutex::new(Some(true))));
}

/// preemptPark parks gp and puts it in _Gpreempted.
///
///go:systemstack
pub fn preempt_park(gp: GoPtr<crate::runtime2::g>) {
    let mut status = readgstatus(gp.clone());
    if { let __tmp_x = { let __tmp_x = status; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GRUNNING as u32; __tmp_x != __tmp_y } {
        dumpgstatus(gp.clone());
        throw(Arc::new(Mutex::new(Some("bad g status".to_string()))));
    }

    if (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().async_safe_point.clone() }.lock().unwrap().as_ref().unwrap()) {
                // Double-check that async preemption does not
                // happen in SPWRITE assembly functions.
                // isAsyncSafePoint must exclude this case.
        let mut f = findfunc(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        throw(Arc::new(Mutex::new(Some("preempt at unknown pc".to_string()))));
    }
        if {
            let __tmp_x = { let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).flag.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_S_P_WRITE as u8)))); __tmp_x & __tmp_y };
            let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x != __tmp_y
        } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: unexpected SPWRITE function".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "in async preempt".to_string());
            eprintln!("{} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        throw(Arc::new(Mutex::new(Some("preempt SPWRITE".to_string()))));
    }
    }

        // Double-check that async preemption does not
        // happen in SPWRITE assembly functions.
        // isAsyncSafePoint must exclude this case.
        // Transition from _Grunning to _Gscan|_Gpreempted. We can't
        // be in _Grunning when we dropg because then we'd be running
        // without an M, but the moment we're in _Gpreempted,
        // something could claim this G before we've fully cleaned it
        // up. Hence, we set the scan bit to lock down further
        // transitions until we can dropg.
    cas_g_to_preempt_scan(
        gp.clone(),
        Arc::new(Mutex::new(Some(__GRUNNING as u32))),
        Arc::new(Mutex::new(Some(((__GSCAN as u32) | (__GPREEMPTED as u32)) as u32)))
    );
    dropg();

        // Be careful about how we trace this next event. The ordering
        // is subtle.
        //
        // The moment we CAS into _Gpreempted, suspendG could CAS to
        // _Gwaiting, do its work, and ready the goroutine. All of
        // this could happen before we even get the chance to emit
        // an event. The end result is that the events could appear
        // out of order, and the tracer generally assumes the scheduler
        // takes care of the ordering between GoPark and GoUnpark.
        //
        // The answer here is simple: emit the event while we still hold
        // the _Gscan bit on the goroutine. We still need to traceAcquire
        // and traceRelease across the CAS because the tracer could be
        // what's calling suspendG in the first place, and we want the
        // CAS and event emission to appear atomic to the tracer.
    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_park(Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_PREEMPTED as u8))))))), Arc::new(Mutex::new(Some(0))));
    }
    casfrom__gscanstatus(
        gp.clone(),
        Arc::new(Mutex::new(Some(((__GSCAN as u32) | (__GPREEMPTED as u32)) as u32))),
        Arc::new(Mutex::new(Some(__GPREEMPTED as u32)))
    );
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    schedule();
}

/// goyield is like Gosched, but it:
/// - emits a GoPreempt trace event instead of a GoSched trace event
/// - puts the current G on the runq of the current P instead of the globrunq
///
/// goyield should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gvisor.dev/gvisor
///   - github.com/sagernet/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname goyield
pub fn goyield() {
    check_timeouts();
    mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { goyield_m(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
}

pub fn goyield_m(gp: Arc<Mutex<Option<g>>>) {
    let mut trace_local = trace_acquire();
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // Trace the event before the transition. It may take a
                // stack trace, but we won't own the stack after the
                // transition anymore.
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_preempt();
    }
        // Trace the event before the transition. It may take a
        // stack trace, but we won't own the stack after the
        // transition anymore.
    casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    dropg();
    runqput(pp.clone(), GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(false))));
    schedule();
}

/// save updates getg().sched to refer to pc and sp so that a following
/// gogo will restore pc and sp.
///
/// save must not have write barriers because invoking a write barrier
/// can clobber getg().sched.
///
///go:nosplit
///go:nowritebarrierrec
pub fn save(pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>, bp: Arc<Mutex<Option<usize>>>) {
    let mut gp = getg();

    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } || { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // m.g0.sched is special and must describe the context
                // for exiting the thread. mstart1 writes to it directly.
                // m.gsignal.sched should not be used at all.
                // This check makes sure save calls do not accidentally
                // run in contexts where they'd write to system g's.
        throw(Arc::new(Mutex::new(Some("save on system g not allowed".to_string()))));
    }

        // m.g0.sched is special and must describe the context
        // for exiting the thread. mstart1 writes to it directly.
        // m.gsignal.sched should not be used at all.
        // This check makes sure save calls do not accidentally
        // run in contexts where they'd write to system g's.
    { let new_val = pc.lock().unwrap().as_ref().unwrap().clone(); *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
    { let new_val = sp.lock().unwrap().as_ref().unwrap().clone(); *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as usize; *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as usize; *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).ret.lock().unwrap() = Some(new_val); };
    { let new_val = bp.lock().unwrap().as_ref().unwrap().clone(); *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).bp.lock().unwrap() = Some(new_val); };

        // We need to ensure ctxt is zero, but can't have a write
        // barrier here. However, it should always already be zero.
        // Assert that.
    if { let __nil_target = (*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        badctxt();
    }
}

/// The goroutine g is about to enter a system call.
/// Record that it's not using the cpu anymore.
/// This is called only from the go syscall library and cgocall,
/// not from the low-level system calls used by the runtime.
///
/// Entersyscall cannot split the stack: the save must
/// make g->sched refer to the caller's stack segment, because
/// entersyscall is going to return immediately after.
///
/// Nothing entersyscall calls can split the stack either.
/// We cannot safely move the stack during an active call to syscall,
/// because we do not know which of the uintptr arguments are
/// really pointers (back into the stack).
/// In practice, this means that we make the fast path run through
/// entersyscall doing no-split things, and the slow path has to use systemstack
/// to run bigger things on the system stack.
///
/// reentersyscall is the entry point used by cgo callbacks, where explicitly
/// saved SP and PC are restored. This is needed when exitsyscall will be called
/// from a function further up in the call stack than the parent, as g->syscallsp
/// must always point to a valid stack frame. entersyscall below is the normal
/// entry point for syscalls, which obtains the SP and PC from the caller.
///
///go:nosplit
pub fn reentersyscall(pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>, bp: Arc<Mutex<Option<usize>>>) {
    let mut trace_local = trace_acquire();
    let mut gp = getg();

        // Disable preemption because during this function g is in Gsyscall status,
        // but can have inconsistent g->sched, do not let GC observe it.
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

        // Entersyscall must not call any function that might split/grow the stack.
        // (See details in comment above.)
        // Catch calls that might, by replacing the stack guard with something that
        // will trip any stack check and leaving a flag to tell newstack to die.
    { let new_val = STACK_PREEMPT as usize; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).throwsplit.lock().unwrap() = Some(new_val); };

        // Leave SP around for GC and traceback.
    save(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = sp.lock().unwrap().as_ref().unwrap().clone(); *(*gp.lock().unwrap().as_ref().unwrap()).syscallsp.lock().unwrap() = Some(new_val); };
    { let new_val = pc.lock().unwrap().as_ref().unwrap().clone(); *(*gp.lock().unwrap().as_ref().unwrap()).syscallpc.lock().unwrap() = Some(new_val); };
    { let new_val = bp.lock().unwrap().as_ref().unwrap().clone(); *(*gp.lock().unwrap().as_ref().unwrap()).syscallbp.lock().unwrap() = Some(new_val); };
    casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(__GSYSCALL as u32))));
    if STATIC_LOCK_RANKING {
                // When doing static lock ranking casgstatus can call
                // systemstack which clobbers g.sched.
        save(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // When doing static lock ranking casgstatus can call
        // systemstack which clobbers g.sched.
    if {
        let __go_cond_0 = {
            let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = {
                let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap());
                __tmp_x < __tmp_y
            };
            __go_cond_1
        }
    } {
        let gp_closure_clone = gp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "entersyscall inconsistent sp ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp_closure_clone.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", " [".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", ",".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", "]\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("entersyscall".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
    if {
        let __go_cond_0 = {
            let __go_cond_1 = { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallbp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
            if __go_cond_1 {
                let __go_cond_2 = {
                    let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallbp.clone(); __field }.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
                    __tmp_x < __tmp_y
                };
                __go_cond_2
            } else {
                false
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_3 = {
                let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallbp.clone(); __field }.lock().unwrap().as_ref().unwrap());
                __tmp_x < __tmp_y
            };
            __go_cond_3
        }
    } {
        let gp_closure_clone = gp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "entersyscall inconsistent bp ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp_closure_clone.lock().unwrap().as_ref().unwrap()).syscallbp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", " [".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", ",".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", "]\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("entersyscall".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }

    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        let trace_closure_clone = trace_local.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        (*trace_closure_clone.lock().unwrap().as_ref().unwrap()).go_sys_call();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
                // systemstack itself clobbers g.sched.{pc,sp} and we might
                // need them later when the G is genuinely blocked in a
                // syscall
        save(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // systemstack itself clobbers g.sched.{pc,sp} and we might
        // need them later when the G is genuinely blocked in a
        // syscall
    if (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).load() {
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || { entersyscall_sysmon() }) as Box<dyn FnMut() -> () + Send + Sync>))));
        save(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    if {
        let __tmp_x = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().run_safe_point_fn.clone() }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = 0 as u32;
        __tmp_x != __tmp_y
    } {
                // runSafePointFn may stack split if run on this stack
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || { run_safe_point_fn() }) as Box<dyn FnMut() -> () + Send + Sync>))));
        save(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // runSafePointFn may stack split if run on this stack
    { let new_val = { let __selector_holder = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().syscalltick.clone() }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).syscalltick.lock().unwrap() = Some(new_val); };
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).oldp.lock().unwrap().as_mut().unwrap()).set(pp.clone());
    { let new_val = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap() = Some(new_val); };
    internal_runtime_atomic::store({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some(__PSYSCALL as u32))));
    if (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load() {
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || { entersyscall_gcwait() }) as Box<dyn FnMut() -> () + Send + Sync>))));
        save(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
}

/// Standard syscall entry used by the go syscall library and normal cgo calls.
///
/// This is exported via linkname to assembly in the syscall package and x/sys.
///
/// Other packages should not be accessing entersyscall directly,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gvisor.dev/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:nosplit
///go:linkname entersyscall
pub fn entersyscall() {
        // N.B. getcallerfp cannot be written directly as argument in the call
        // to reentersyscall because it forces spilling the other arguments to
        // the stack. This results in exceeding the nosplit stack requirements
        // on some platforms.
    let mut fp = getcallerfp();
    reentersyscall(
        Arc::new(Mutex::new(Some(internal_runtime_sys::get_caller_p_c()))),
        Arc::new(Mutex::new(Some(internal_runtime_sys::get_caller_s_p()))),
        Arc::new(Mutex::new(Some(fp)))
    );
}

pub fn entersyscall_sysmon() {
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).load() {
        (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
        notewakeup((*sched.lock().unwrap().as_ref().unwrap()).sysmonnote.clone());
    }
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

pub fn entersyscall_gcwait() {
    let mut gp = getg();
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).oldp.lock().unwrap().as_ref().unwrap()));

    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut trace_local = trace_acquire();
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } && internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(__PSYSCALL as u32))), Arc::new(Mutex::new(Some(__PGCSTOP as u32)))) {
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // This is a steal in the new tracer. While it's very likely
                // that we were the ones to put this P into _Psyscall, between
                // then and now it's totally possible it had been stolen and
                // then put back into _Psyscall for us to acquire here. In such
                // case ProcStop would be incorrect.
                //
                // TODO(mknyszek): Consider emitting a ProcStop instead when
                // gp.m.syscalltick == pp.syscalltick, since then we know we never
                // lost the P.
        (*trace_local.lock().unwrap().as_ref().unwrap()).proc_steal(pp.clone(), Arc::new(Mutex::new(Some(true))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // This is a steal in the new tracer. While it's very likely
                // that we were the ones to put this P into _Psyscall, between
                // then and now it's totally possible it had been stolen and
                // then put back into _Psyscall for us to acquire here. In such
                // case ProcStop would be incorrect.
                //
                // TODO(mknyszek): Consider emitting a ProcStop instead when
                // gp.m.syscalltick == pp.syscalltick, since then we know we never
                // lost the P.
        { let new_val = nanotime(); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_stop_time.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let __target = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.syscalltick.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        {
        { let __target = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); };
        if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
            notewakeup((*sched.lock().unwrap().as_ref().unwrap()).stopnote.clone());;
        }
    }
    } else if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // This is a steal in the new tracer. While it's very likely
        // that we were the ones to put this P into _Psyscall, between
        // then and now it's totally possible it had been stolen and
        // then put back into _Psyscall for us to acquire here. In such
        // case ProcStop would be incorrect.
        //
        // TODO(mknyszek): Consider emitting a ProcStop instead when
        // gp.m.syscalltick == pp.syscalltick, since then we know we never
        // lost the P.
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// entersyscallblock should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gvisor.dev/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname entersyscallblock
///go:nosplit
pub fn entersyscallblock() {
    let mut gp = getg();

    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).throwsplit.lock().unwrap() = Some(new_val); };
    { let new_val = STACK_PREEMPT as usize; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().syscalltick.clone() }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).syscalltick.lock().unwrap() = Some(new_val); };
    { let __target = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().syscalltick.clone() }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

        // Leave SP around for GC and traceback.
    let mut pc = internal_runtime_sys::get_caller_p_c();
    let mut sp = internal_runtime_sys::get_caller_s_p();
    let mut bp = getcallerfp();
    save(Arc::new(Mutex::new(Some(pc))), Arc::new(Mutex::new(Some(sp))), Arc::new(Mutex::new(Some(bp))));
    { let new_val = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).syscallsp.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).syscallpc.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).bp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).syscallbp.lock().unwrap() = Some(new_val); };
    if {
        let __go_cond_0 = {
            let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = {
                let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap());
                __tmp_x < __tmp_y
            };
            __go_cond_1
        }
    } {
        let mut sp1 = Arc::new(Mutex::new(Some(sp)));
        let mut sp2 = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut sp3 = Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let gp_closure_clone = gp.clone(); let sp1_closure_clone = sp1.clone(); let sp2_closure_clone = sp2.clone(); let sp3_closure_clone = sp3.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "entersyscallblock inconsistent sp ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*sp1_closure_clone.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", " ".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*sp2_closure_clone.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", " ".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*sp3_closure_clone.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_6 = format!("{}", " [".to_string());
            let __go_print_arg_7 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_8 = format!("{}", ",".to_string());
            let __go_print_arg_9 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_10 = format!("{}", "]\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8, __go_print_arg_9, __go_print_arg_10)
        };
        throw(Arc::new(Mutex::new(Some("entersyscallblock".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
    casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(__GSYSCALL as u32))));
    if {
        let __go_cond_0 = {
            let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = {
                let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap());
                __tmp_x < __tmp_y
            };
            __go_cond_1
        }
    } {
        let gp_closure_clone = gp.clone(); let sp_closure_clone = sp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "entersyscallblock inconsistent sp ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(sp_closure_clone as u64)))));
            let __go_print_arg_2 = format!("{}", " ".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", " ".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp_closure_clone.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", " [".to_string());
            let __go_print_arg_7 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_8 = format!("{}", ",".to_string());
            let __go_print_arg_9 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_10 = format!("{}", "]\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8, __go_print_arg_9, __go_print_arg_10)
        };
        throw(Arc::new(Mutex::new(Some("entersyscallblock".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
    if {
        let __go_cond_0 = {
            let __go_cond_1 = { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallbp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
            if __go_cond_1 {
                let __go_cond_2 = {
                    let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallbp.clone(); __field }.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
                    __tmp_x < __tmp_y
                };
                __go_cond_2
            } else {
                false
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_3 = {
                let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallbp.clone(); __field }.lock().unwrap().as_ref().unwrap());
                __tmp_x < __tmp_y
            };
            __go_cond_3
        }
    } {
        let bp_closure_clone = bp.clone(); let gp_closure_clone = gp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "entersyscallblock inconsistent bp ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(bp_closure_clone as u64)))));
            let __go_print_arg_2 = format!("{}", " ".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).bp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", " ".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp_closure_clone.lock().unwrap().as_ref().unwrap()).syscallbp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", " [".to_string());
            let __go_print_arg_7 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_8 = format!("{}", ",".to_string());
            let __go_print_arg_9 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_10 = format!("{}", "]\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8, __go_print_arg_9, __go_print_arg_10)
        };
        throw(Arc::new(Mutex::new(Some("entersyscallblock".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }

    systemstack(Arc::new(Mutex::new(Some(Box::new(move || { entersyscallblock_handoff() }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Resave for traceback during blocked call.
    save(
        Arc::new(Mutex::new(Some(internal_runtime_sys::get_caller_p_c()))),
        Arc::new(Mutex::new(Some(internal_runtime_sys::get_caller_s_p()))),
        Arc::new(Mutex::new(Some(getcallerfp())))
    );

    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
}

pub fn entersyscallblock_handoff() {
    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_sys_call();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    handoffp(releasep());
}

/// The goroutine g exited its system call.
/// Arrange for it to run on a cpu again.
/// This is called only from the go syscall library, not
/// from the low-level system calls used by the runtime.
///
/// Write barriers are not allowed because our P may have been stolen.
///
/// This is exported via linkname to assembly in the syscall package.
///
/// exitsyscall should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gvisor.dev/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:nosplit
///go:nowritebarrierrec
///go:linkname exitsyscall
pub fn exitsyscall() {
    let mut gp = getg();

    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    if { let __tmp_x = internal_runtime_sys::get_caller_s_p(); let __tmp_y = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("exitsyscall: syscall frame is no longer valid".to_string()))));
    }

    { let new_val = 0 as i64; *(*gp.lock().unwrap().as_ref().unwrap()).waitsince.lock().unwrap() = Some(new_val); };
    let mut oldp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).oldp.lock().unwrap().as_ref().unwrap()));
    { let new_val = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).oldp.lock().unwrap() = Some(new_val); };
    if exitsyscallfast(oldp.clone()) {
                // When exitsyscallfast returns success, we have a P so can now use
                // write barriers
        if (*{ let __field = (*goroutineProfile.lock().unwrap().as_ref().unwrap()).active.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // Make sure that gp has had its stack written out to the goroutine
                // profile, exactly as it was when the goroutine profiler first
                // stopped the world.
        let gp_closure_clone = gp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        try_record_goroutine_profile_w_b(gp_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
                // Make sure that gp has had its stack written out to the goroutine
                // profile, exactly as it was when the goroutine profiler first
                // stopped the world.
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        let mut lostP = Arc::new(Mutex::new(Some({
            let __go_cond_0 = { let __left_addr = oldp.addr(); let __right_addr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())).addr(); let __eq = __left_addr == __right_addr; !__eq };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_1 = {
                    let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).syscalltick.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().syscalltick.clone() }.lock().unwrap().as_ref().unwrap());
                    __tmp_x != __tmp_y
                };
                __go_cond_1
            }
        })));
        let lostP_closure_clone = lostP.clone(); let trace_closure_clone = trace_local.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        (*trace_closure_clone.lock().unwrap().as_ref().unwrap()).go_sys_exit(Arc::new(Mutex::new(Some({ let __arg_holder = lostP_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __v = (*lostP_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*trace_closure_clone.lock().unwrap().as_ref().unwrap()).go_start();
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
                // Write out syscall exit eagerly.
                //
                // It's important that we write this *after* we know whether we
                // lost our P or not (determined by exitsyscallfast).
                // We lost the P at some point, even though we got it back here.
                // Trace that we're starting again, because there was a traceGoSysBlock
                // call somewhere in exitsyscallfast (indicating that this goroutine
                // had blocked) and we're about to start running again.
                // There's a cpu for us, so we can run.
        { let __target = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().syscalltick.clone() }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
                // We need to cas the status and scan before resuming...
        casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GSYSCALL as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // Garbage collector isn't running (since we are),
                // so okay to clear syscallsp.
        { let new_val = 0 as usize; *(*gp.lock().unwrap().as_ref().unwrap()).syscallsp.lock().unwrap() = Some(new_val); };
        { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // restore the preemption request in case we've cleared it in newstack
        { let new_val = STACK_PREEMPT as usize; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    } else {
                // otherwise restore the real stackGuard, we've spoiled it in entersyscall/entersyscallblock
        { let new_val = { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    }
                // restore the preemption request in case we've cleared it in newstack
                // otherwise restore the real stackGuard, we've spoiled it in entersyscall/entersyscallblock
        { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).throwsplit.lock().unwrap() = Some(new_val); };
        if (*(*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).user.lock().unwrap().as_ref().unwrap()) && !sched_enabled(GoPtr::local(gp.clone())) {
                // Scheduling of this goroutine is disabled.
        gosched();
    }
                // Scheduling of this goroutine is disabled.
        return;
    }

        // When exitsyscallfast returns success, we have a P so can now use
        // write barriers
        // Make sure that gp has had its stack written out to the goroutine
        // profile, exactly as it was when the goroutine profiler first
        // stopped the world.
        // Write out syscall exit eagerly.
        //
        // It's important that we write this *after* we know whether we
        // lost our P or not (determined by exitsyscallfast).
        // We lost the P at some point, even though we got it back here.
        // Trace that we're starting again, because there was a traceGoSysBlock
        // call somewhere in exitsyscallfast (indicating that this goroutine
        // had blocked) and we're about to start running again.
        // There's a cpu for us, so we can run.
        // We need to cas the status and scan before resuming...
        // Garbage collector isn't running (since we are),
        // so okay to clear syscallsp.
        // restore the preemption request in case we've cleared it in newstack
        // otherwise restore the real stackGuard, we've spoiled it in entersyscall/entersyscallblock
        // Scheduling of this goroutine is disabled.
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }

        // Call the scheduler.
    mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { exitsyscall0(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));

        // Scheduler returned, so we're allowed to run now.
        // Delete the syscallsp information that we left for
        // the garbage collector during the system call.
        // Must wait until now because until gosched returns
        // we don't know for sure that the garbage collector
        // is not running.
    { let new_val = 0 as usize; *(*gp.lock().unwrap().as_ref().unwrap()).syscallsp.lock().unwrap() = Some(new_val); };
    { let __target = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().syscalltick.clone() }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).throwsplit.lock().unwrap() = Some(new_val); };
}

///go:nosplit
pub fn exitsyscallfast(oldp: GoPtr<crate::runtime2::p>) -> bool {
        // Freezetheworld sets stopwait but does not retake P's.
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = FREEZE_STOP_WAIT as i32; __tmp_x == __tmp_y } {
        return false;
    }

        // Try to re-acquire the last P.
    let mut trace_local = trace_acquire();
    if !oldp.is_nil() && { let __tmp_x = (*{ let __ptr_value = oldp.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PSYSCALL as u32; __tmp_x == __tmp_y } && internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local({ let __ptr_value = oldp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(__PSYSCALL as u32))), Arc::new(Mutex::new(Some(__PIDLE as u32)))) {
                // There's a cpu for us, so we can run.
        wirep(oldp.clone());
        exitsyscallfast_reacquired(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return true;
    }
        // There's a cpu for us, so we can run.
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Try to get any other idle P.
    if {
        let __tmp_x = { let __selector_holder = (*sched.lock().unwrap().as_ref().unwrap()).pidle.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x != __tmp_y
    } {
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        let mut ok_closure_clone = ok.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = exitsyscallfast_pidle(); *ok_closure_clone.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return true;
    }
    }
    false
}

/// exitsyscallfast_reacquired is the exitsyscall path on which this G
/// has successfully reacquired the P it was running on before the
/// syscall.
///
///go:nosplit
pub fn exitsyscallfast_reacquired(trace_local: Arc<Mutex<Option<traceLocker>>>) {
    let mut gp = getg();
    if {
        let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).syscalltick.lock().unwrap().as_ref().unwrap());
        let __tmp_y = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().syscalltick.clone() }.lock().unwrap().as_ref().unwrap());
        __tmp_x != __tmp_y
    } {
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // The p was retaken and then enter into syscall again (since gp.m.syscalltick has changed).
                // traceGoSysBlock for this syscall was already emitted,
                // but here we effectively retake the p from the new syscall running on the same p.
        let gp_closure_clone = gp.clone(); let trace_closure_clone = trace_local.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        (*trace_closure_clone.lock().unwrap().as_ref().unwrap()).proc_steal(crate::runtime2::puintptr::ptr(&(*(*(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(Some(true))));
        (*trace_closure_clone.lock().unwrap().as_ref().unwrap()).proc_start();
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
                // The p was retaken and then enter into syscall again (since gp.m.syscalltick has changed).
                // traceGoSysBlock for this syscall was already emitted,
                // but here we effectively retake the p from the new syscall running on the same p.
                // We're stealing the P. It's treated
                // as if it temporarily stopped running. Then, start running.
        { let __target = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().syscalltick.clone() }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn exitsyscallfast_pidle() -> bool {
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let (mut pp, _) = pidleget(Arc::new(Mutex::new(Some(0 as i64))));
    if !pp.is_nil() && (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).load() {
        (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
        notewakeup((*sched.lock().unwrap().as_ref().unwrap()).sysmonnote.clone());
    }
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if !pp.is_nil() {
        acquirep(pp.clone());
        return true;
    }
    false
}

/// exitsyscall slow path on g0.
/// Failed to acquire P, enqueue gp as runnable.
///
/// Called via mcall, so gp is the calling g from this M.
///
///go:nowritebarrierrec
pub fn exitsyscall0(gp: Arc<Mutex<Option<g>>>) {
    let mut trace_local: Arc<Mutex<Option<traceLocker>>> = Arc::new(Mutex::new(Some(Default::default())));
    trace_exiting_syscall();
    { let new_val = trace_acquire(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *trace_local.lock().unwrap() = __moved_val; };
    casgstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GSYSCALL as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
    trace_exited_syscall();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // Write out syscall exit eagerly.
                //
                // It's important that we write this *after* we know whether we
                // lost our P or not (determined by exitsyscallfast).
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_sys_exit(Arc::new(Mutex::new(Some(true))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // Write out syscall exit eagerly.
        //
        // It's important that we write this *after* we know whether we
        // lost our P or not (determined by exitsyscallfast).
    dropg();
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut pp: GoPtr<crate::runtime2::p> = GoPtr::nil();
    if sched_enabled(GoPtr::local(gp.clone())) {
        { let (__tmp_0, __tmp_1) = pidleget(Arc::new(Mutex::new(Some(0 as i64)))); pp = __tmp_0.clone(); };
    }
    let mut locked: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    if pp.is_nil() {
        globrunqput(GoPtr::local(gp.clone()));
                // Below, we stoplockedm if gp is locked. globrunqput releases
                // ownership of gp, so we must check if gp is locked prior to
                // committing the release by unlocking sched.lock, otherwise we
                // could race with another M transitioning gp from unlocked to
                // locked.
        { let new_val = {
            let __tmp_x = { let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).lockedm.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        }; *locked.lock().unwrap() = Some(new_val); };
    } else if (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).load() {
        (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
        notewakeup((*sched.lock().unwrap().as_ref().unwrap()).sysmonnote.clone());
    }
        // Below, we stoplockedm if gp is locked. globrunqput releases
        // ownership of gp, so we must check if gp is locked prior to
        // committing the release by unlocking sched.lock, otherwise we
        // could race with another M transitioning gp from unlocked to
        // locked.
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if !pp.is_nil() {
        acquirep(pp.clone());
        execute(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(false))));
    }
        // Never returns.
    if { let __v = (*locked.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Wait until another thread schedules gp and so m again.
                //
                // N.B. lockedm must be this M, as this g was running on this M
                // before entersyscall.
        stoplockedm();
        execute(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(false))));
    }
        // Wait until another thread schedules gp and so m again.
        //
        // N.B. lockedm must be this M, as this g was running on this M
        // before entersyscall.
        // Never returns.
    stopm();
    schedule();
}

/// Allocate a new g, with a stack big enough for stacksize bytes.
pub fn malg(mut stacksize: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::runtime2::g>>> {
    let mut newg = Arc::new(Mutex::new(Some(g::default())));
    if { let __tmp_x = { let __v = (*stacksize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        { let new_val = round2(Arc::new(Mutex::new(Some({ let __tmp_x = STACK_SYSTEM as i32; let __tmp_y = { let __v = (*stacksize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); *stacksize.lock().unwrap() = Some(new_val); };
        let newg_closure_clone = newg.clone(); let stacksize_closure_clone = stacksize.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = stackalloc(Arc::new(Mutex::new(Some((*stacksize_closure_clone.lock().unwrap().as_ref().unwrap()) as u32)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*newg_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap() = __moved_val; };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        { let new_val = { let __tmp_x = (*(*(*newg.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *(*newg.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
        { let new_val = !(0 as usize) as usize; *(*newg.lock().unwrap().as_ref().unwrap()).stackguard1.lock().unwrap() = Some(new_val); };
                // Clear the bottom word of the stack. We record g
                // there on gsignal stack during VDSO on ARM and ARM64.
        { unimplemented!("unsafe.Pointer dereference assignment"); };
    }
        // Clear the bottom word of the stack. We record g
        // there on gsignal stack during VDSO on ARM and ARM64.
    return newg.clone();
}

/// Purge all cached G's from gfree list to the global list.
pub fn gfpurge(pp: Arc<Mutex<Option<p>>>) {
    let mut inc: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut stackQ: Arc<Mutex<Option<gQueue>>> = Arc::new(Mutex::new(Some(Default::default())));let mut noStackQ: Arc<Mutex<Option<gQueue>>> = Arc::new(Mutex::new(Some(Default::default())));
    while !(*(*pp.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).empty() {
        let mut gp: GoPtr<crate::runtime2::g> = (*(*pp.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_mut().unwrap()).pop();
        { let __target = (*(*pp.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        (*noStackQ.lock().unwrap().as_ref().unwrap()).push(gp.clone());
    } else {
        (*stackQ.lock().unwrap().as_ref().unwrap()).push(gp.clone());
    }
        { let mut guard = inc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    lock(GoPtr::local((*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).lock.clone()));
    (*(*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).no_stack.lock().unwrap().as_mut().unwrap()).push_all(Arc::new(Mutex::new(Some({ let __arg_holder = noStackQ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    (*(*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_mut().unwrap()).push_all(Arc::new(Mutex::new(Some({ let __arg_holder = stackQ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let __target = (*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).n.clone(); let __rhs = (*inc.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    unlock(GoPtr::local((*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

pub fn mcount() -> i32 {
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).mnext.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmfreed.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
}

pub fn __system() {
    __system();
}

pub fn __external_code() {
    __external_code();
}

pub fn __lost_external_code() {
    __lost_external_code();
}

pub fn __g_c() {
    __g_c();
}

pub fn __lost_s_i_g_p_r_o_f_during_atomic64() {
    __lost_s_i_g_p_r_o_f_during_atomic64();
}

pub fn __lost_contended_runtime_lock() {
    __lost_contended_runtime_lock();
}

pub fn __v_d_s_o() {
    __v_d_s_o();
}

/// Called if we receive a SIGPROF signal.
/// Called by the signal handler, may run during STW.
///
///go:nowritebarrierrec
pub fn sigprof(mut pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>, lr: Arc<Mutex<Option<usize>>>, gp: GoPtr<crate::runtime2::g>, mut mp: Arc<Mutex<Option<m>>>) {
    if { let __tmp_x = (*(*prof.lock().unwrap().as_ref().unwrap()).hz.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        return;
    }

        // If mp.profilehz is 0, then profiling is not enabled for this thread.
        // We must check this to avoid a deadlock between setcpuprofilerate
        // and the call to cpuprof.add, below.
    if { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).profilehz.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        return;
    }

        // On mips{,le}/arm, 64bit atomics are emulated with spinlocks, in
        // internal/runtime/atomic. If SIGPROF arrives while the program is inside
        // the critical section, it creates a deadlock (when writing the sample).
        // As a workaround, create a counter of SIGPROFs while in critical section
        // to store the count, and pass it to sigprof.add() later when SIGPROF is
        // received from somewhere else (with _LostSIGPROFDuringAtomic64 as pc).
    if { let __tmp_x = "arm64".to_string(); let __tmp_y = "mips".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "arm64".to_string(); let __tmp_y = "mipsle".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm".to_string(); __tmp_x == __tmp_y } {
        {
        let mut f = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if (*f.lock().unwrap().as_ref().unwrap()).valid() {
            if internal_stringslite::has_prefix(funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(Some("internal/runtime/atomic".to_string())))) {
        { let __target = (*cpuprof.lock().unwrap().as_ref().unwrap()).lost_atomic.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return;
    };
        }
    }
        if { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = (*goarm.lock().unwrap().as_ref().unwrap()); let __tmp_y = 7 as u8; __tmp_x < __tmp_y } && { let __tmp_x = "darwin".to_string(); let __tmp_y = "linux".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xffff0000 as usize; __tmp_x & __tmp_y }; let __tmp_y = 0xffff0000 as usize; __tmp_x == __tmp_y } {
                // internal/runtime/atomic functions call into kernel
                // helpers on arm < 7. See
                // internal/runtime/atomic/sys_linux_arm.s.
        { let __target = (*cpuprof.lock().unwrap().as_ref().unwrap()).lost_atomic.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return;
    }
    }

        // internal/runtime/atomic functions call into kernel
        // helpers on arm < 7. See
        // internal/runtime/atomic/sys_linux_arm.s.
        // Profiling runs concurrently with GC, so it must not allocate.
        // Set a trap in case the code does allocate.
        // Note that on windows, one thread takes profiles of all the
        // other threads, so mp is usually not getg().m.
        // In fact mp may not even be stopped.
        // See golang.org/issue/17165.
    { let __target = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

    let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut stk: Arc<Mutex<Option<[usize; 64]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut n = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).ncgo.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } && { let __ptr_field = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } && { let __tmp_x = (*{ let __ptr_value = (*mp.lock().unwrap().as_ref().unwrap()).curg.borrow(); __ptr_value.as_ref().unwrap().syscallpc.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = (*{ let __ptr_value = (*mp.lock().unwrap().as_ref().unwrap()).curg.borrow(); __ptr_value.as_ref().unwrap().syscallsp.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        let mut cgoOff = Arc::new(Mutex::new(Some(0)));
                // Check cgoCallersUse to make sure that we are not
                // interrupting other code that is fiddling with
                // cgoCallers.  We are running in a signal handler
                // with all signals blocked, so we don't have to worry
                // about any other code interrupting us.
        if { let __tmp_x = (*(*mp.lock().unwrap().as_ref().unwrap()).cgo_callers_use.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } && { let __nil_target = (*mp.lock().unwrap().as_ref().unwrap()).cgo_callers.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __seq_holder = { let __named_array = (*(*mp.lock().unwrap().as_ref().unwrap()).cgo_callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        while { let __tmp_x = ({ let __v = (*cgoOff.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 32; __tmp_x < __tmp_y } && { let __tmp_x = { let __seq_holder = { let __named_array = (*(*mp.lock().unwrap().as_ref().unwrap()).cgo_callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*cgoOff.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let mut guard = cgoOff.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let __rhs = (*{
            let _dst_start = 0;
            let _dst_len = (*stk.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __array_holder = { let __named_array = (*(*mp.lock().unwrap().as_ref().unwrap()).cgo_callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __array_guard = __array_holder.lock().unwrap(); let __seq = __array_guard.as_ref().unwrap(); __seq[..({ let __v = (*cgoOff.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*stk.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        }.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        (*{ let __named_array = (*(*mp.lock().unwrap().as_ref().unwrap()).cgo_callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }.lock().unwrap().as_mut().unwrap())[(0) as usize] = 0 as usize;
    }
                // Collect Go stack that leads to the cgo call.
        (*u.lock().unwrap().as_mut().unwrap()).init_at(
            Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = (*mp.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.syscallpc.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = (*mp.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.syscallsp.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some(0 as usize))),
            (*mp.lock().unwrap().as_ref().unwrap()).curg.clone(),
            Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some(UNWIND_SILENT_ERRORS as u8))))))),
        );
    } else if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = uses_libcall();
                if __go_cond_2 {
                    let __go_cond_3 = {
                        let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).libcallg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                        let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize))));
                        __tmp_x != __tmp_y
                    };
                    __go_cond_3
                } else {
                    false
                }
            };
            if __go_cond_1 {
                let __go_cond_4 = { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).libcallpc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
                __go_cond_4
            } else {
                false
            }
        };
        if __go_cond_0 {
            let __go_cond_5 = { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).libcallsp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
            __go_cond_5
        } else {
            false
        }
    } {
        (*u.lock().unwrap().as_mut().unwrap()).init_at(
            Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).libcallpc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).libcallsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some(0 as usize))),
            crate::runtime2::guintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).libcallg.lock().unwrap().as_ref().unwrap())),
            Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some(UNWIND_SILENT_ERRORS as u8))))))),
        );
    } else if { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).vdso_s_p.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        (*u.lock().unwrap().as_mut().unwrap()).init_at(
            Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).vdso_p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).vdso_s_p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some(0 as usize))),
            gp.clone(),
            Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some((UNWIND_SILENT_ERRORS as u8 | UNWIND_JUMP_STACK as u8) as u8))))))),
        );
    } else {
        (*u.lock().unwrap().as_mut().unwrap()).init_at(
            Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = lr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            gp.clone(),
            Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some((UNWIND_SILENT_ERRORS as u8 | UNWIND_TRAP as u8 as u8 | UNWIND_JUMP_STACK as u8) as u8))))))),
        );
    }
        // Check cgoCallersUse to make sure that we are not
        // interrupting other code that is fiddling with
        // cgoCallers.  We are running in a signal handler
        // with all signals blocked, so we don't have to worry
        // about any other code interrupting us.
        // Collect Go stack that leads to the cgo call.
        // Libcall, i.e. runtime syscall on windows.
        // Collect Go stack that leads to the call.
        // VDSO call, e.g. nanotime1 on Linux.
        // Collect Go stack that leads to the call.
    { let __rhs = traceback_p_cs(
        u.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({
            let __seq_holder = stk.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        })))
    ); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } {
                // Normal traceback is impossible or has failed.
                // Account it against abstract "System" or "GC".
        { let new_val = 2; *n.lock().unwrap() = Some(new_val); };
        if in_v_d_s_o_page(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = { let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__v_d_s_o.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y }; *pc.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*firstmoduledata.lock().unwrap().as_ref().unwrap()).etext.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__external_code.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y }; *pc.lock().unwrap() = Some(new_val); };
    }
                // "ExternalCode" is better than "etext".
        (*stk.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v };
        if { let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        (*stk.lock().unwrap().as_mut().unwrap())[(1) as usize] = { let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__g_c.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y };
    } else {
        (*stk.lock().unwrap().as_mut().unwrap())[(1) as usize] = { let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__system.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y };
    }
    }

        // Normal traceback is impossible or has failed.
        // Account it against abstract "System" or "GC".
        // "ExternalCode" is better than "etext".
    if { let __tmp_x = (*(*prof.lock().unwrap().as_ref().unwrap()).hz.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
                // Note: it can happen on Windows that we interrupted a system thread
                // with no g, so gp could nil. The other nil checks are done out of
                // caution, but not expected to be nil in practice.
        let mut tagPtr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
        if !gp.is_nil() && { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __ptr_field = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } {
        { let new_val = { let __ptr_value = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.labels.clone()); __ptr_value }.clone().clone(); tagPtr = new_val; };
    }
        (*cpuprof.lock().unwrap().as_mut().unwrap()).add(tagPtr.clone(), Arc::new(Mutex::new(Some({
            let __seq_holder = stk.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))));
        let mut gprof: GoPtr<crate::runtime2::g> = gp.clone();
        let mut mp: Arc<Mutex<Option<m>>> = Arc::new(Mutex::new(None));
        let mut pp: GoPtr<crate::runtime2::p> = GoPtr::nil();
        if !gp.is_nil() && { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if { let __ptr_field = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } {
        gprof = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.clone();
    }
        { let new_val = { let __ptr_value = gp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().m.clone(); __field_value }; mp = new_val; };
        pp = crate::runtime2::puintptr::ptr(&(*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    }
        trace_c_p_u_sample(
            gprof.clone(),
            mp.clone(),
            pp.clone(),
            Arc::new(Mutex::new(Some({
                let __seq_holder = stk.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            })))
        );
    }
        // Note: it can happen on Windows that we interrupted a system thread
        // with no g, so gp could nil. The other nil checks are done out of
        // caution, but not expected to be nil in practice.
    { let __target = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
}

/// Change number of processors.
///
/// sched.lock must be held, and the world must be stopped.
///
/// gcworkbufs must not be being modified by either the GC or the write barrier
/// code, so the GC must not be running if the number of Ps actually changes.
///
/// Returns list of Ps with local work, they need to be scheduled by the caller.
pub fn procresize(nprocs: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::runtime2::p>>> {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    assert_world_stopped();

    let mut old = { let __owned = gomaxprocs.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    if { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
        throw(Arc::new(Mutex::new(Some("procresize: invalid arg".to_string()))));
    }
    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).gomaxprocs(Arc::new(Mutex::new(Some({ let __arg_holder = nprocs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // update statistics
    let mut now = nanotime();
    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).procresizetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let __target = (*sched.lock().unwrap().as_ref().unwrap()).totaltime.clone(); let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some((*old.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = now; let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).procresizetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x * __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    { let new_val = now; *(*sched.lock().unwrap().as_ref().unwrap()).procresizetime.lock().unwrap() = Some(new_val); };

    let mut maskWords = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 31 as i32; __tmp_x + __tmp_y }); let __tmp_y = 32 as i32; __tmp_x / __tmp_y })));

        // Grow allp if necessary.
    if { let __tmp_x = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*allp.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
                // Synchronize with retake, which could be running
                // concurrently since it doesn't run on a P.
        lock(GoPtr::local(allpLock.clone()));
        if { let __tmp_x = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*allp.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        { let new_val = { let __collection_holder = Arc::new(Mutex::new(Some({
            let __seq_holder = allp.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))).clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *allp.lock().unwrap() = new_val; };
    } else {
        let mut nallp: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::runtime2::p>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ({ let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
                // Copy everything up to allp's cap so we
                // never lose old allocated Ps.
        {
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = allp.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
                let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
                drop(__seq_guard);
                let __low = 0;
                let __high = ((*allp.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)) as usize;
                let __max = __source_cap;
                if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))).lock().unwrap().as_ref().unwrap()).clone();
            let _n = std::cmp::min((*nallp.lock().unwrap().as_ref().unwrap()).len(), _src.len());
            for _i in 0.._n {
                (*nallp.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        };
        { let new_val = { let __collection_holder = nallp.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *allp.lock().unwrap() = new_val; };
    }
                // Copy everything up to allp's cap so we
                // never lose old allocated Ps.
        if { let __tmp_x = { let __v = (*maskWords.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*idlepMask.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        { let new_val = Some(pMask(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*idlepMask.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*maskWords.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))); *idlepMask.lock().unwrap() = new_val; };
        { let new_val = Some(pMask(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*timerpMask.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*maskWords.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))); *timerpMask.lock().unwrap() = new_val; };
    } else {
        let mut nidlepMask = Arc::new(Mutex::new(Some(vec![0; ({ let __v = (*maskWords.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
                // No need to copy beyond len, old Ps are irrelevant.
        {
            let _src = { let __slice_holder = { let __named_slice = (*idlepMask.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() };
            let _n = std::cmp::min((*nidlepMask.lock().unwrap().as_ref().unwrap()).len(), _src.len());
            for _i in 0.._n {
                (*nidlepMask.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        };
        { let new_val = Some(pMask(nidlepMask.clone())); *idlepMask.lock().unwrap() = new_val; };
        let mut ntimerpMask = Arc::new(Mutex::new(Some(vec![0; ({ let __v = (*maskWords.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
        {
            let _src = { let __slice_holder = { let __named_slice = (*timerpMask.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() };
            let _n = std::cmp::min((*ntimerpMask.lock().unwrap().as_ref().unwrap()).len(), _src.len());
            for _i in 0.._n {
                (*ntimerpMask.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        };
        { let new_val = Some(pMask(ntimerpMask.clone())); *timerpMask.lock().unwrap() = new_val; };
    }
                // No need to copy beyond len, old Ps are irrelevant.
        unlock(GoPtr::local(allpLock.clone()));
    }

        // Synchronize with retake, which could be running
        // concurrently since it doesn't run on a P.
        // Copy everything up to allp's cap so we
        // never lose old allocated Ps.
        // No need to copy beyond len, old Ps are irrelevant.
        // initialize new P's
    let mut i = { let __owned = old.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut pp = { let __seq = { let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        if { let __nil_result = (*pp.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(p::default()))).clone(); pp = new_val; };
    }
        { let __recv = pp.clone(); let __recv_ptr: *mut crate::runtime2::p = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::runtime2::p }; let __result = unsafe { &mut *__recv_ptr }.init(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        atomicstorep(Arc::new(Mutex::new(Some({ let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&pp) as usize))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut gp = getg();
    if {
        let __go_cond_0 = {
            let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().id.clone() }.lock().unwrap().as_ref().unwrap());
                let __tmp_y = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v };
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
                // continue to use the current P
        { let new_val = __PRUNNING as u32; *{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap() = Some(new_val); };
        { let __recv_field = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().mcache.clone() }.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.prepare_for_sweep()); __result };
    } else {
                // release the current P and acquire allp[0].
                //
                // We must do this before destroying our current P
                // because p.destroy itself has write barriers, so we
                // need to do that from a valid P.
        if {
            let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // Pretend that we were descheduled
                // and then scheduled again to keep
                // the trace consistent.
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_sched();
        (*trace_local.lock().unwrap().as_ref().unwrap()).proc_stop(crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // Pretend that we were descheduled
                // and then scheduled again to keep
                // the trace consistent.
        { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().m.clone() }.lock().unwrap() = Some(new_val); };
    }
                // Pretend that we were descheduled
                // and then scheduled again to keep
                // the trace consistent.
        { let new_val = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap() = Some(new_val); };
        let mut pp = { let __seq = { let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
        { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize)))); *(*pp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap() = Some(new_val); };
        { let new_val = __PIDLE as u32; *(*pp.lock().unwrap().as_ref().unwrap()).status.lock().unwrap() = Some(new_val); };
        acquirep(GoPtr::local(pp.clone()));
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_start();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }

        // continue to use the current P
        // release the current P and acquire allp[0].
        //
        // We must do this before destroying our current P
        // because p.destroy itself has write barriers, so we
        // need to do that from a valid P.
        // Pretend that we were descheduled
        // and then scheduled again to keep
        // the trace consistent.
        // g.m.p is now set, so we no longer need mcache0 for bootstrapping.
    *mcache0.lock().unwrap() = Some(Arc::new(Mutex::new(None)));

        // release resources from unused P's
    let mut i = { let __owned = nprocs.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut pp = { let __seq = { let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        { let __recv = pp.clone(); let __recv_ptr: *mut crate::runtime2::p = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::runtime2::p }; let __result = unsafe { &mut *__recv_ptr }.destroy(); __result };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // can't free P itself because it can be referenced by an M in syscall
        // Trim allp.
    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*allp.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        lock(GoPtr::local(allpLock.clone()));
        { let new_val = { let __collection_holder = Arc::new(Mutex::new(Some({
            let __seq_holder = allp.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))).clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *allp.lock().unwrap() = new_val; };
        { let new_val = Some(pMask(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*idlepMask.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*maskWords.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))); *idlepMask.lock().unwrap() = new_val; };
        { let new_val = Some(pMask(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*timerpMask.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*maskWords.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))))); *timerpMask.lock().unwrap() = new_val; };
        unlock(GoPtr::local(allpLock.clone()));
    }

    let mut runnablePs: Arc<Mutex<Option<p>>> = Arc::new(Mutex::new(None));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i32; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        let mut pp = { let __seq = { let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        if { let __left_addr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())).addr(); let __right_addr = { let __ptr = GoPtr::local(pp.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }; continue
    }
        { let new_val = __PIDLE as u32; *(*pp.lock().unwrap().as_ref().unwrap()).status.lock().unwrap() = Some(new_val); };
        if runqempty(GoPtr::local(pp.clone())) {
        pidleput(GoPtr::local(pp.clone()), Arc::new(Mutex::new(Some(now))));
    } else {
        (*(*pp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_mut().unwrap()).set(mget());
        (*(*pp.lock().unwrap().as_ref().unwrap()).link.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(runnablePs.clone()));
        { let new_val = pp.clone(); runnablePs = new_val; };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    (*stealOrder.lock().unwrap().as_mut().unwrap()).reset(Arc::new(Mutex::new(Some((*nprocs.lock().unwrap().as_ref().unwrap()) as u32))));
    let mut int32p: Arc<Mutex<Option<i32>>> = gomaxprocs.clone();
    internal_runtime_atomic::store(Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&int32p) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u32>(unimplemented!("unsafe.Pointer conversion to u32")) } })), Arc::new(Mutex::new(Some((*nprocs.lock().unwrap().as_ref().unwrap()) as u32))));
    if { let __tmp_x = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nprocs.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // Notify the limiter that the amount of procs has changed.
        (*gcCPULimiter.lock().unwrap().as_mut().unwrap()).reset_capacity(Arc::new(Mutex::new(Some(now))), Arc::new(Mutex::new(Some({ let __arg_holder = nprocs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // Notify the limiter that the amount of procs has changed.
    return runnablePs.clone();
}

/// Associate p and the current m.
///
/// This function is allowed to have write barriers even if the caller
/// isn't because it immediately acquires pp.
///
///go:yeswritebarrierrec
pub fn acquirep(pp: GoPtr<crate::runtime2::p>) {
        // Do the part that isn't allowed to have write barriers.
    wirep(pp.clone());

        // Have p; write barriers now allowed.
        // Perform deferred mcache flush before this P can allocate
        // from a potentially stale mcache.
    { let __recv_field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mcache.clone()); __ptr_value }.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.prepare_for_sweep()); __result };

    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).proc_start();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

/// wirep is the first step of acquirep, which actually associates the
/// current M to pp. This is broken out so we can disallow write
/// barriers for this part, since we don't yet have a P.
///
///go:nowritebarrierrec
///go:nosplit
pub fn wirep(pp: GoPtr<crate::runtime2::p>) {
    let mut gp = getg();

    if {
        let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x != __tmp_y
    } {
                // Call on the systemstack to avoid a nosplit overflow build failure
                // on some platforms when built with -N -l. See #64113.
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        throw(Arc::new(Mutex::new(Some("wirep: already in go".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
        // Call on the systemstack to avoid a nosplit overflow build failure
        // on some platforms when built with -N -l. See #64113.
    if {
        let __go_cond_0 = {
            let __tmp_x = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PIDLE as u32; __tmp_x != __tmp_y };
            __go_cond_1
        }
    } {
                // Call on the systemstack to avoid a nosplit overflow build failure
                // on some platforms when built with -N -l. See #64113.
        let pp_closure_clone = pp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut id = Arc::new(Mutex::new(Some(0 as i64)));
        if {
            let __tmp_x = { let __selector_holder = { let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x != __tmp_y
        } {
        { let new_val = { let __selector_holder = { let __ptr = crate::runtime2::muintptr::ptr(&(*{ let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().id.clone() }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *id.lock().unwrap() = Some(new_val); };
    }
        {
            let __go_print_arg_0 = format!("{}", "wirep: p->m=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = pp_closure_clone.borrow(); __ptr_value.as_ref().unwrap().m.clone() }.lock().unwrap().as_ref().unwrap()).clone());
            let __go_print_arg_2 = format!("{}", "(".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", ") p->status=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __ptr_value = pp_closure_clone.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("wirep: invalid p state".to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
        // Call on the systemstack to avoid a nosplit overflow build failure
        // on some platforms when built with -N -l. See #64113.
    (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_mut().unwrap()).set(pp.clone());
    (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).set(GoPtr::local((*gp.lock().unwrap().as_ref().unwrap()).m.clone()));
    { let new_val = __PRUNNING as u32; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
}

/// Disassociate p and the current m.
pub fn releasep() -> GoPtr<crate::runtime2::p> {
    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).proc_stop(crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    releasep_no_trace()
}

/// Disassociate p and the current m without tracing an event.
pub fn releasep_no_trace() -> GoPtr<crate::runtime2::p> {
    let mut gp = getg();

    if {
        let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x == __tmp_y
    } {
        throw(Arc::new(Mutex::new(Some("releasep: invalid arg".to_string()))));
    }
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    if {
        let __go_cond_0 = { let __left_addr = crate::runtime2::muintptr::ptr(&(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())).addr(); let __right_addr = { let __ptr = GoPtr::local((*gp.lock().unwrap().as_ref().unwrap()).m.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PRUNNING as u32; __tmp_x != __tmp_y };
            __go_cond_1
        }
    } {
        {
            let __go_print_arg_0 = format!("{}", "releasep: m=".to_string());
            let __go_print_arg_1 = format!("{}", format!("&{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field }.lock().unwrap().as_ref().unwrap())));
            let __go_print_arg_2 = format!("{}", " m->p=".to_string());
            let __go_print_arg_3 = format!("{}", { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); format!("0x{:x}", __ptr.addr()) });
            let __go_print_arg_4 = format!("{}", " p->m=".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_6 = format!("{}", " p->status=".to_string());
            let __go_print_arg_7 = format!("{}", (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_8 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
        throw(Arc::new(Mutex::new(Some("releasep: invalid p state".to_string()))));
    }
    { let new_val = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap() = Some(new_val); };
    { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = __PIDLE as u32; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    pp.clone()
}

pub fn incidlelocked(v: Arc<Mutex<Option<i32>>>) {
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).nmidlelocked.clone(); let __rhs = (*v.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        checkdead();
    }
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// Check for deadlock situation.
/// The check is based on number of running M's, if 0 -> deadlock.
/// sched.lock must be held.
pub fn checkdead() {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // For -buildmode=c-shared or -buildmode=c-archive it's OK if
        // there are no running goroutines. The calling program is
        // assumed to be running.
        // One exception is Wasm, which is single-threaded. If we are
        // in Go and all goroutines are blocked, it deadlocks.
    if ((*islibrary.lock().unwrap().as_ref().unwrap()) || (*isarchive.lock().unwrap().as_ref().unwrap())) && { let __tmp_x = "arm64".to_string(); let __tmp_y = "wasm".to_string(); __tmp_x != __tmp_y } {
        return;
    }

        // If we are dying because of a signal caught on an already idle thread,
        // freezetheworld will cause all running threads to block.
        // And runtime will essentially enter into deadlock state,
        // except that there is a thread that will call exit soon.
    if { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        return;
    }

        // If we are not running under cgo, but we have an extra M then account
        // for it. (It is possible to have an extra M on Windows without cgo to
        // accommodate callbacks created by syscall.NewCallback. See issue #6751
        // for details.)
    let mut run0: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    if !(*iscgo.lock().unwrap().as_ref().unwrap()) && (*cgoHasExtraM.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (*extraMLength.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        { let new_val = 1 as i32; *run0.lock().unwrap() = Some(new_val); };
    }

    let mut run = Arc::new(Mutex::new(Some({
        let __tmp_x = {
            let __tmp_x = { let __tmp_x = mcount(); let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmidle.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y };
            let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmidlelocked.clone(); __field }.lock().unwrap().as_ref().unwrap());
            __tmp_x - __tmp_y
        };
        let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmsys.clone(); __field }.lock().unwrap().as_ref().unwrap());
        __tmp_x - __tmp_y
    })));
    if { let __tmp_x = { let __v = (*run.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*run0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return;
    }
    if { let __tmp_x = { let __v = (*run.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: checkdead: nmidle=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmidle.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " nmidlelocked=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmidlelocked.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " mcount=".to_string());
            let __go_print_arg_5 = format!("{}", mcount());
            let __go_print_arg_6 = format!("{}", " nmsys=".to_string());
            let __go_print_arg_7 = format!("{}", (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmsys.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_8 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        throw(Arc::new(Mutex::new(Some("checkdead: inconsistent counts".to_string()))));
    }

    let mut grunning = Arc::new(Mutex::new(Some(0)));
    let mut grunning_closure_clone = grunning.clone(); for_each_g(Arc::new(Mutex::new(Some(Box::new(move |gp: Arc<Mutex<Option<g>>>| {
        if is_system_goroutine(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(false)))) {
        return;
    }
        let mut s = readgstatus(GoPtr::local(gp.clone()));
        { let _switch_val = { let __tmp_x = s; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y };
    if _switch_val == (__GWAITING as u32) || _switch_val == (__GPREEMPTED as u32) {
            { let mut guard = grunning_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        } else if _switch_val == (__GRUNNABLE as u32) || _switch_val == (__GRUNNING as u32) || _switch_val == (__GSYSCALL as u32) {
            {
            let __go_print_arg_0 = format!("{}", "runtime: checkdead: find g ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " in status ".to_string());
            let __go_print_arg_3 = format!("{}", s);
            let __go_print_arg_4 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
            unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
            throw(Arc::new(Mutex::new(Some("checkdead: runnable g".to_string()))));
        }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>))));
    if { let __tmp_x = { let __v = (*grunning.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        fatal(Arc::new(Mutex::new(Some("no goroutines (main called runtime.Goexit) - deadlock!".to_string()))));
    }

        // unlock so that GODEBUG=scheddetail=1 doesn't hang
        // Maybe jump time forward for playground.
    if { let __tmp_x = (*faketime.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        {
        let mut when = time_sleep_until();;
        if { let __tmp_x = when; let __tmp_y = MAX_WHEN as i64; __tmp_x < __tmp_y } {
            { let new_val = when; *faketime.lock().unwrap() = Some(new_val); };;
            let (mut pp, _) = pidleget(Arc::new(Mutex::new(Some({ let __arg_holder = faketime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            if pp.is_nil() {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        throw(Arc::new(Mutex::new(Some("checkdead: no p for timer".to_string()))));
    };
            let mut mp: GoPtr<crate::runtime2::m> = mget();;
            if mp.is_nil() {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        throw(Arc::new(Mutex::new(Some("checkdead: no m for timer".to_string()))));
    };
            (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));;
            { let new_val = true; *{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.spinning.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };;
            (*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.nextp.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).set(pp.clone());;
            notewakeup({ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.park.clone()); __ptr_value }.clone());;
            return;;
        }
    }
    }

        // Start an M to steal the timer.
        // There should always be a free P since
        // nothing is running.
        // There should always be a free M since
        // nothing is running.
        // M must be spinning to steal. We set this to be
        // explicit, but since this is the only M it would
        // become spinning on its own anyways.
        // There are no goroutines running, so we can look at the P's.
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pp in __range_values.iter() {
        if {
            let __tmp_x = (({ let __len_target = { let __field = (*(*pp.lock().unwrap().as_ref().unwrap()).timers.lock().unwrap().as_ref().unwrap()).heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
            let __tmp_y = 0;
            __tmp_x > __tmp_y
        } {
        return;
    }
    } }

    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    fatal(Arc::new(Mutex::new(Some("all goroutines are asleep - deadlock!".to_string()))));
}

/// Tell all goroutines that they have been preempted and they should stop.
/// This function is purely best-effort. It can fail to inform a goroutine if a
/// processor just started running it.
/// No locks need to be held.
/// Returns true if preemption request was issued to at least one goroutine.
pub fn preemptall() -> bool {
    let mut res = Arc::new(Mutex::new(Some(false)));
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pp in __range_values.iter() {
        if { let __tmp_x = (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).status.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PRUNNING as u32; __tmp_x != __tmp_y } {
        continue
    }
        if preemptone((*pp).clone()) {
        { let new_val = true; *res.lock().unwrap() = Some(new_val); };
    }
    } }
    return { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// Tell the goroutine running on processor P to stop.
/// This function is purely best-effort. It can incorrectly fail to inform the
/// goroutine. It can inform the wrong goroutine. Even if it informs the
/// correct goroutine, that goroutine might ignore the request if it is
/// simultaneously executing newstack.
/// No lock needs to be held.
/// Returns true if preemption request was issued.
/// The actual preemption will happen at some point in the future
/// and will be indicated by the gp->status no longer being
/// Grunning
pub fn preemptone(pp: Arc<Mutex<Option<p>>>) -> bool {
    let mut mp: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*(*pp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()));
    if mp.is_nil() || { let __left_addr = mp.addr(); let __right_addr = { let __ptr = GoPtr::local((*getg().lock().unwrap().as_ref().unwrap()).m.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq } {
        return false;
    }
    let mut gp: GoPtr<crate::runtime2::g> = { let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().curg.clone(); __field_value };
    if gp.is_nil() || { let __left_addr = gp.addr(); let __right_addr = { let __ptr = GoPtr::local({ let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().g0.clone(); __field_value }); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq } {
        return false;
    }

    { let new_val = true; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.preempt.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Every call in a goroutine checks for stack overflow by
        // comparing the current stack pointer to gp->stackguard0.
        // Setting gp->stackguard0 to StackPreempt folds
        // preemption into the normal stack overflow check.
    { let new_val = STACK_PREEMPT as usize; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stackguard0.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Request an async preemption of this P.
    if PREEMPT_M_SUPPORTED && { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        { let new_val = true; *(*pp.lock().unwrap().as_ref().unwrap()).preempt.lock().unwrap() = Some(new_val); };
        preempt_m(mp.clone());
    }

    true
}

pub fn schedtrace(detailed: Arc<Mutex<Option<bool>>>) {
    let mut now = nanotime();
    if { let __tmp_x = (*starttime.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = now; *starttime.lock().unwrap() = Some(new_val); };
    }

    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    {
            let __go_print_arg_0 = format!("{}", "SCHED ".to_string());
            let __go_print_arg_1 = format!("{}", { let __tmp_x = ({ let __tmp_x = now; let __tmp_y = (*starttime.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = 1e6 as i64; __tmp_x / __tmp_y });
            let __go_print_arg_2 = format!("{}", "ms: gomaxprocs=".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*gomaxprocs.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", " idleprocs=".to_string());
            let __go_print_arg_5 = format!("{}", (*(*sched.lock().unwrap().as_ref().unwrap()).npidle.lock().unwrap().as_mut().unwrap()).load());
            let __go_print_arg_6 = format!("{}", " threads=".to_string());
            let __go_print_arg_7 = format!("{}", mcount());
            let __go_print_arg_8 = format!("{}", " spinningthreads=".to_string());
            let __go_print_arg_9 = format!("{}", (*(*sched.lock().unwrap().as_ref().unwrap()).nmspinning.lock().unwrap().as_mut().unwrap()).load());
            let __go_print_arg_10 = format!("{}", " needspinning=".to_string());
            let __go_print_arg_11 = format!("{}", (*(*sched.lock().unwrap().as_ref().unwrap()).needspinning.lock().unwrap().as_mut().unwrap()).load());
            let __go_print_arg_12 = format!("{}", " idlethreads=".to_string());
            let __go_print_arg_13 = format!("{}", (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmidle.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_14 = format!("{}", " runqueue=".to_string());
            let __go_print_arg_15 = format!("{}", (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8, __go_print_arg_9, __go_print_arg_10, __go_print_arg_11, __go_print_arg_12, __go_print_arg_13, __go_print_arg_14, __go_print_arg_15)
        };
    if { let __v = (*detailed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", " gcwaiting=".to_string());
            let __go_print_arg_1 = format!("{}", (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load());
            let __go_print_arg_2 = format!("{}", " nmidlelocked=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).nmidlelocked.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " stopwait=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).stopwait.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", " sysmonwait=".to_string());
            let __go_print_arg_7 = format!("{}", (*(*sched.lock().unwrap().as_ref().unwrap()).sysmonwait.lock().unwrap().as_ref().unwrap()).load());
            let __go_print_arg_8 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
    }

        // We must be careful while reading data from P's, M's and G's.
        // Even if we hold schedlock, most data can be changed concurrently.
        // E.g. (p->m ? p->m->id : -1) can crash if p->m changes from non-nil to nil.
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, pp) in __range_values.iter().enumerate() {
        let mut mp: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*(*pp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()));
        let mut h = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local((*pp.lock().unwrap().as_ref().unwrap()).runqhead.clone()));
        let mut t = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local((*pp.lock().unwrap().as_ref().unwrap()).runqtail.clone()));
        if { let __v = (*detailed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", "  P".to_string());
            let __go_print_arg_1 = format!("{}", i);
            let __go_print_arg_2 = format!("{}", ": status=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).status.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " schedtick=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).schedtick.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", " syscalltick=".to_string());
            let __go_print_arg_7 = format!("{}", (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).syscalltick.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_8 = format!("{}", " m=".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
        if !mp.is_nil() {
        {
            let __go_print_arg_0 = format!("{}", (*{ let __ptr_value = mp.borrow(); __ptr_value.as_ref().unwrap().id.clone() }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", "nil".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", " runqsize=".to_string());
            let __go_print_arg_1 = format!("{}", { let __tmp_x = t; let __tmp_y = h; __tmp_x - __tmp_y });
            let __go_print_arg_2 = format!("{}", " gfreecnt=".to_string());
            let __go_print_arg_3 = format!("{}", (*(*(*pp.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).n.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " timerslen=".to_string());
            let __go_print_arg_5 = format!("{}", ({ let __len_target = { let __field = (*(*pp.lock().unwrap().as_ref().unwrap()).timers.lock().unwrap().as_ref().unwrap()).heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
    } else {
                // In non-detailed mode format lengths of per-P run queues as:
                // [len1 len2 len3 len4]
        {
            let __go_print_arg_0 = format!("{}", " ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "[".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", { let __tmp_x = t; let __tmp_y = h; __tmp_x - __tmp_y });
            eprint!("{}", __go_print_arg_0)
        };
        if { let __tmp_x = (i as i32); let __tmp_y = ({ let __tmp_x = ((*allp.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "]\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
    }
    } }

        // In non-detailed mode format lengths of per-P run queues as:
        // [len1 len2 len3 len4]
    if !{ let __v = (*detailed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return;
    }

    let mut mp = (*allm.lock().unwrap().as_ref().unwrap()).clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        {
            let __go_print_arg_0 = format!("{}", "  M".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).id.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", ": p=".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        if !pp.is_nil() {
        {
            let __go_print_arg_0 = format!("{}", (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().id.clone() }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", "nil".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", " curg=".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        if { let __ptr_field = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } {
        {
            let __go_print_arg_0 = format!("{}", (*{ let __ptr_value = (*mp.lock().unwrap().as_ref().unwrap()).curg.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", "nil".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", " mallocing=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " throwing=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).throwing.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
            let __go_print_arg_4 = format!("{}", " preemptoff=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
            let __go_print_arg_6 = format!("{}", " locks=".to_string());
            let __go_print_arg_7 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_8 = format!("{}", " dying=".to_string());
            let __go_print_arg_9 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).dying.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_10 = format!("{}", " spinning=".to_string());
            let __go_print_arg_11 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).spinning.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_12 = format!("{}", " blocked=".to_string());
            let __go_print_arg_13 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).blocked.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_14 = format!("{}", " lockedg=".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8, __go_print_arg_9, __go_print_arg_10, __go_print_arg_11, __go_print_arg_12, __go_print_arg_13, __go_print_arg_14)
        };
        {
        let mut lockedg: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).lockedg.lock().unwrap().as_ref().unwrap()));;
        if !lockedg.is_nil() {
            {
            let __go_print_arg_0 = format!("{}", (*{ let __ptr_value = lockedg.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };;
        } else {
            {
            let __go_print_arg_0 = format!("{}", "nil".to_string());
            eprint!("{}", __go_print_arg_0)
        };;
        }
    }
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        { let new_val = (*mp.lock().unwrap().as_ref().unwrap()).alllink.clone(); mp = new_val; };
    }

    for_each_g(Arc::new(Mutex::new(Some(Box::new(move |gp: Arc<Mutex<Option<g>>>| {
        {
            let __go_print_arg_0 = format!("{}", "  G".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", ": status=".to_string());
            let __go_print_arg_3 = format!("{}", readgstatus(GoPtr::local(gp.clone())));
            let __go_print_arg_4 = format!("{}", "(".to_string());
            let __go_print_arg_5 = format!("{}", (*crate::runtime2::waitReason::string(&(*(*gp.lock().unwrap().as_ref().unwrap()).waitreason.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", ") m=".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        if { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", "nil".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", " lockedm=".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        {
        let mut lockedm: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*(*gp.lock().unwrap().as_ref().unwrap()).lockedm.lock().unwrap().as_ref().unwrap()));;
        if !lockedm.is_nil() {
            {
            let __go_print_arg_0 = format!("{}", (*{ let __ptr_value = lockedm.borrow(); __ptr_value.as_ref().unwrap().id.clone() }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };;
        } else {
            {
            let __go_print_arg_0 = format!("{}", "nil".to_string());
            eprint!("{}", __go_print_arg_0)
        };;
        }
    }
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>))));
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// schedEnableUser enables or disables the scheduling of user
/// goroutines.
///
/// This does not stop already running user goroutines, so the caller
/// should first stop the world when disabling user goroutines.
pub fn sched_enable_user(enable: Arc<Mutex<Option<bool>>>) {
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if {
        let __tmp_x = (*(*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).user.lock().unwrap().as_ref().unwrap());
        let __tmp_y = !{ let __v = (*enable.lock().unwrap().as_ref().unwrap()).clone(); __v };
        __tmp_x == __tmp_y
    } {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return;
    }
    { let new_val = !{ let __v = (*enable.lock().unwrap().as_ref().unwrap()).clone(); __v }; *(*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).user.lock().unwrap() = Some(new_val); };
    if { let __v = (*enable.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut n = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).n.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = 0 as i32; *(*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
        globrunqputbatch((*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).runnable.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && { let __tmp_x = (*(*sched.lock().unwrap().as_ref().unwrap()).npidle.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        startm(GoPtr::nil(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false))));
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    } else {
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
}

/// schedEnabled reports whether gp should be scheduled. It returns
/// false is scheduling of gp is disabled.
///
/// sched.lock must be held.
pub fn sched_enabled(gp: GoPtr<crate::runtime2::g>) -> bool {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if (*(*(*sched.lock().unwrap().as_ref().unwrap()).disable.lock().unwrap().as_ref().unwrap()).user.lock().unwrap().as_ref().unwrap()) {
        return is_system_goroutine(gp.clone(), Arc::new(Mutex::new(Some(true))));
    }
    true
}

/// Put mp on midle list.
/// sched.lock must be held.
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn mput(mp: Arc<Mutex<Option<m>>>) {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some((*(*(*sched.lock().unwrap().as_ref().unwrap()).midle.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*mp.lock().unwrap().as_ref().unwrap()).schedlink.lock().unwrap() = Some(new_val); };
    (*(*sched.lock().unwrap().as_ref().unwrap()).midle.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(mp.clone()));
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).nmidle.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    checkdead();
}

/// Try to get an m from midle list.
/// sched.lock must be held.
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn mget() -> GoPtr<crate::runtime2::m> {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    let mut mp: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*(*sched.lock().unwrap().as_ref().unwrap()).midle.lock().unwrap().as_ref().unwrap()));
    if !mp.is_nil() {
        { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*sched.lock().unwrap().as_ref().unwrap()).midle.lock().unwrap() = Some(new_val); };
        { let __target = (*sched.lock().unwrap().as_ref().unwrap()).nmidle.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    mp.clone()
}

/// Put gp on the global runnable queue.
/// sched.lock must be held.
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn globrunqput(gp: GoPtr<crate::runtime2::g>) {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    (*(*sched.lock().unwrap().as_ref().unwrap()).runq.lock().unwrap().as_ref().unwrap()).push_back(gp.clone());
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
}

/// Put gp at the head of the global runnable queue.
/// sched.lock must be held.
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn globrunqputhead(gp: GoPtr<crate::runtime2::g>) {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    (*(*sched.lock().unwrap().as_ref().unwrap()).runq.lock().unwrap().as_ref().unwrap()).push(gp.clone());
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
}

/// Put a batch of runnable goroutines on the global runnable queue.
/// This clears *batch.
/// sched.lock must be held.
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn globrunqputbatch(batch: Arc<Mutex<Option<gQueue>>>, n: Arc<Mutex<Option<i32>>>) {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    (*(*sched.lock().unwrap().as_ref().unwrap()).runq.lock().unwrap().as_mut().unwrap()).push_back_all(Arc::new(Mutex::new(Some({ let __v = (*batch.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let new_val = gQueue { head: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))), tail: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))) }; *batch.lock().unwrap() = Some(new_val); };
}

/// Try get a batch of G's from the global runnable queue.
/// sched.lock must be held.
pub fn globrunqget(pp: GoPtr<crate::runtime2::p>, max: Arc<Mutex<Option<i32>>>) -> GoPtr<crate::runtime2::g> {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        return GoPtr::nil();
    }

    let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*gomaxprocs.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; let __tmp_y = 1 as i32; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = { let __selector_holder = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *n.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = max.lock().unwrap().as_ref().unwrap().clone(); *n.lock().unwrap() = Some(new_val); };
    }
    if {
        let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
        let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as i32))).lock().unwrap().as_ref().unwrap()) as i32; let __tmp_y = 2 as i32; __tmp_x / __tmp_y } as i32;
        __tmp_x > __tmp_y
    } {
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as i32))).lock().unwrap().as_ref().unwrap()) as i32; let __tmp_y = 2 as i32; __tmp_x / __tmp_y } as i32; *n.lock().unwrap() = Some(new_val); };
    }

    { let __target = (*sched.lock().unwrap().as_ref().unwrap()).runqsize.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

    let mut gp: GoPtr<crate::runtime2::g> = (*(*sched.lock().unwrap().as_ref().unwrap()).runq.lock().unwrap().as_mut().unwrap()).pop();
    { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        let mut gp1: GoPtr<crate::runtime2::g> = (*(*sched.lock().unwrap().as_ref().unwrap()).runq.lock().unwrap().as_mut().unwrap()).pop();
        runqput(pp.clone(), gp1.clone(), Arc::new(Mutex::new(Some(false))));
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    gp.clone()
}

/// pidleput puts p on the _Pidle list. now must be a relatively recent call
/// to nanotime or zero. Returns now or the current time if now was zero.
///
/// This releases ownership of p. Once sched.lock is released it is no longer
/// safe to use p.
///
/// sched.lock must be held.
///
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn pidleput(pp: GoPtr<crate::runtime2::p>, mut now: Arc<Mutex<Option<i64>>>) -> i64 {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if !runqempty(pp.clone()) {
        throw(Arc::new(Mutex::new(Some("pidleput: P has non-empty run queue".to_string()))));
    }
    if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = nanotime(); *now.lock().unwrap() = Some(new_val); };
    }
    if {
        let __tmp_x = (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.timers.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_mut().unwrap()).load();
        let __tmp_y = 0 as u32;
        __tmp_x == __tmp_y
    } {
        (*timerpMask.lock().unwrap().as_ref().unwrap()).clear(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
    (*idlepMask.lock().unwrap().as_ref().unwrap()).set(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    { let new_val = crate::runtime2::puintptr(Arc::new(Mutex::new(Some((*(*(*sched.lock().unwrap().as_ref().unwrap()).pidle.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.link.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    (*(*sched.lock().unwrap().as_ref().unwrap()).pidle.lock().unwrap().as_mut().unwrap()).set(pp.clone());
    (*(*sched.lock().unwrap().as_ref().unwrap()).npidle.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
    if !(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.limiter_event.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).start(Arc::new(Mutex::new(Some(crate::mgclimit::limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_IDLE as u8))))))), Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        throw(Arc::new(Mutex::new(Some("must be able to track idle limiter event".to_string()))));
    }
    return { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// pidleget tries to get a p from the _Pidle list, acquiring ownership.
///
/// sched.lock must be held.
///
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn pidleget(mut now: Arc<Mutex<Option<i64>>>) -> (GoPtr<crate::runtime2::p>, i64) {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*sched.lock().unwrap().as_ref().unwrap()).pidle.lock().unwrap().as_ref().unwrap()));
    if !pp.is_nil() {
                // Timer may get added at any time now.
        if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = nanotime(); *now.lock().unwrap() = Some(new_val); };
    }
        (*timerpMask.lock().unwrap().as_ref().unwrap()).set(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        (*idlepMask.lock().unwrap().as_ref().unwrap()).clear(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = crate::runtime2::puintptr(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.link.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*sched.lock().unwrap().as_ref().unwrap()).pidle.lock().unwrap() = Some(new_val); };
        (*(*sched.lock().unwrap().as_ref().unwrap()).npidle.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
        (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.limiter_event.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).stop(Arc::new(Mutex::new(Some(crate::mgclimit::limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_IDLE as u8))))))), Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // Timer may get added at any time now.
    (pp.clone(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v })
}

/// pidlegetSpinning tries to get a p from the _Pidle list, acquiring ownership.
/// This is called by spinning Ms (or callers than need a spinning M) that have
/// found work. If no P is available, this must synchronized with non-spinning
/// Ms that may be preparing to drop their P without discovering this work.
///
/// sched.lock must be held.
///
/// May run during STW, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn pidleget_spinning(mut now: Arc<Mutex<Option<i64>>>) -> (GoPtr<crate::runtime2::p>, i64) {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

    let (mut pp, __tmp_1) = pidleget(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *now.lock().unwrap() = Some(__tmp_1);;
    if pp.is_nil() {
                // See "Delicate dance" comment in findrunnable. We found work
                // that we cannot take, we must synchronize with non-spinning
                // Ms that may be preparing to drop their P.
        (*(*sched.lock().unwrap().as_ref().unwrap()).needspinning.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(1 as u32))));
        return (GoPtr::nil(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

        // See "Delicate dance" comment in findrunnable. We found work
        // that we cannot take, we must synchronize with non-spinning
        // Ms that may be preparing to drop their P.
    (pp.clone(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v })
}

/// runqempty reports whether pp has no Gs on its local run queue.
/// It never returns true spuriously.
pub fn runqempty(pp: GoPtr<crate::runtime2::p>) -> bool {
        // Defend against a race where 1) pp has G1 in runqnext but runqhead == runqtail,
        // 2) runqput on pp kicks G1 to the runq, 3) runqget on pp empties runqnext.
        // Simply observing that runqhead == runqtail and then observing that runqnext == nil
        // does not mean the queue is empty.
    loop {
        let mut head = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone()));
        let mut tail = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone()));
        let mut runnext = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runnext.clone()); __ptr_value }.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
        if { let __tmp_x = tail; let __tmp_y = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone())); __tmp_x == __tmp_y } {
        return { let __tmp_x = head; let __tmp_y = tail; __tmp_x == __tmp_y } && { let __tmp_x = runnext; let __tmp_y = 0 as usize; __tmp_x == __tmp_y };
    }
    }
}

/// runqput tries to put g on the local runnable queue.
/// If next is false, runqput adds g to the tail of the runnable queue.
/// If next is true, runqput puts g in the pp.runnext slot.
/// If the run queue is full, runnext puts g on the global queue.
/// Executed only by the owner P.
pub fn runqput(pp: GoPtr<crate::runtime2::p>, mut gp: GoPtr<crate::runtime2::g>, mut next: Arc<Mutex<Option<bool>>>) {
    if !HAVE_SYSMON && { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // A runnext goroutine shares the same time slice as the
                // current goroutine (inheritTime from runqget). To prevent a
                // ping-pong pair of goroutines from starving all others, we
                // depend on sysmon to preempt "long-running goroutines". That
                // is, any set of goroutines sharing the same time slice.
                //
                // If there is no sysmon, we must avoid runnext entirely or
                // risk starvation.
        { let new_val = false; *next.lock().unwrap() = Some(new_val); };
    }
        // A runnext goroutine shares the same time slice as the
        // current goroutine (inheritTime from runqget). To prevent a
        // ping-pong pair of goroutines from starving all others, we
        // depend on sysmon to preempt "long-running goroutines". That
        // is, any set of goroutines sharing the same time slice.
        //
        // If there is no sysmon, we must avoid runnext entirely or
        // risk starvation.
    if RANDOMIZE_SCHEDULER && { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = randn(Arc::new(Mutex::new(Some(2 as u32)))); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = false; *next.lock().unwrap() = Some(new_val); };
    }

    if { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut oldnext = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runnext.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if !(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runnext.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = oldnext.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(gp.addr()))).lock().unwrap().as_ref().unwrap()) as usize)))))))) {
        // TODO: unsupported goto retry_next
    }
        if { let __tmp_x = (*oldnext.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        return;
    }
                // Kick the old runnext out to the regular run queue.
        gp = crate::runtime2::guintptr::ptr(&(*oldnext.lock().unwrap().as_ref().unwrap()));
    }

    'retry: loop {
                // Kick the old runnext out to the regular run queue.
        let mut h = internal_runtime_atomic::load_acq({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone());
        let mut t = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if {
            let __tmp_x = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = h; __tmp_x - __tmp_y };
            let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
            __tmp_x < __tmp_y
        } {
        { let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y }) as usize].clone() }.set(gp.clone());
        internal_runtime_atomic::store_rel({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }))));
        return;
    }
                // store-release, makes the item available for consumption
        if runqputslow(pp.clone(), gp.clone(), Arc::new(Mutex::new(Some(h))), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }

                // the queue is not full, now the put above must succeed
        continue 'retry;
    };
}

/// Put g and a batch of work from local runnable queue on global queue.
/// Executed only by the owner P.
pub fn runqputslow(pp: GoPtr<crate::runtime2::p>, gp: GoPtr<crate::runtime2::g>, h: Arc<Mutex<Option<u32>>>, t: Arc<Mutex<Option<u32>>>) -> bool {
    let mut batch: Arc<Mutex<Option<[GoPtr<crate::runtime2::g>; 129]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| GoPtr::nil()))));

        // First, grab a batch from local queue.
    let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    { let new_val = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u32; __tmp_x / __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    if {
        let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
        let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = 256; let __tmp_y = 2; __tmp_x / __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()) as u32;
        __tmp_x != __tmp_y
    } {
        throw(Arc::new(Mutex::new(Some("runqputslow: queue is not full".to_string()))));
    }
    let mut i = Arc::new(Mutex::new(Some(0 as u32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*batch.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = crate::runtime2::guintptr::ptr(&({ let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
            let __tmp_x = ({ let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y });
            let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
            __tmp_x % __tmp_y
        }) as usize].clone() }));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if !internal_runtime_atomic::cas_rel({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))) {
        return false;
    }
    (*batch.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = gp.clone();

    if RANDOMIZE_SCHEDULER {
        let mut i = Arc::new(Mutex::new(Some(1 as u32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        let mut j = cheaprandn(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }))));
        {
            let __tmp_0 = { let __seq = { let __seq_holder = batch.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(j) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = batch.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*batch.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_0;
            (*batch.lock().unwrap().as_mut().unwrap())[(j) as usize] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

        // Link the goroutines.
    let mut i = Arc::new(Mutex::new(Some(0 as u32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*{ let __ptr = { let __seq = { let __seq_holder = batch.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().schedlink.clone() }.lock().unwrap().as_mut().unwrap()).set(batch.lock().unwrap().as_ref().unwrap()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }) as usize].clone());
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut q: Arc<Mutex<Option<gQueue>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*(*q.lock().unwrap().as_ref().unwrap()).head.lock().unwrap().as_mut().unwrap()).set(batch.lock().unwrap().as_ref().unwrap()[(0) as usize].clone());
    (*(*q.lock().unwrap().as_ref().unwrap()).tail.lock().unwrap().as_mut().unwrap()).set(batch.lock().unwrap().as_ref().unwrap()[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone());

        // Now put the batch on global queue.
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    globrunqputbatch(q.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }) as i32))));
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    true
}

/// runqputbatch tries to put all the G's on q on the local runnable queue.
/// If the queue is full, they are put on the global queue; in that case
/// this will temporarily acquire the scheduler lock.
/// Executed only by the owner P.
pub fn runqputbatch(pp: GoPtr<crate::runtime2::p>, q: Arc<Mutex<Option<gQueue>>>, mut qsize: Arc<Mutex<Option<i32>>>) {
    let mut h = internal_runtime_atomic::load_acq({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone());
    let mut t = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut n = Arc::new(Mutex::new(Some(0 as u32)));
    while {
        let __go_cond_0 = !{ let __recv = q.clone(); let __recv_ptr: *const gQueue = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const gQueue }; let __result = unsafe { &*__recv_ptr }.empty(); __result };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = h; __tmp_x - __tmp_y };
                let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
                __tmp_x < __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        let mut gp: GoPtr<crate::runtime2::g> = { let __recv = q.clone(); let __recv_ptr: *mut gQueue = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut gQueue }; let __result = unsafe { &mut *__recv_ptr }.pop(); __result };
        { let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y }) as usize].clone() }.set(gp.clone());
        { let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __rhs = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let mut guard = qsize.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

    if RANDOMIZE_SCHEDULER {
        let pp_closure_clone = pp.clone(); let mut off = Arc::new(Mutex::new(Some(Box::new(move |o: Arc<Mutex<Option<u32>>>| -> u32 {
        return {
            let __tmp_x = ({ let __tmp_x = (*{ let __ptr_value = pp_closure_clone.borrow(); __ptr_value.as_ref().unwrap().runqtail.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*o.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y });
            let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp_closure_clone.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
            __tmp_x % __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync>)));
        let mut i = Arc::new(Mutex::new(Some(1 as u32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = cheaprandn(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }))));
        {
            let __tmp_0 = { let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync> = { let mut __f_guard = off.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(j)))) }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync> = { let mut __f_guard = off.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(i.clone()) }) as usize].clone() };
            (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync> = { let mut __f_guard = off.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(i.clone()) }) as usize] = __tmp_0;
            (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync> = { let mut __f_guard = off.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> u32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(j)))) }) as usize] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    internal_runtime_atomic::store_rel({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if !{ let __recv = q.clone(); let __recv_ptr: *const gQueue = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const gQueue }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
        globrunqputbatch(q.clone(), Arc::new(Mutex::new(Some((*qsize.lock().unwrap().as_ref().unwrap()) as i32))));
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
}

/// Get g from local runnable queue.
/// If inheritTime is true, gp should inherit the remaining time in the
/// current time slice. Otherwise, it should start a new time slice.
/// Executed only by the owner P.
pub fn runqget(pp: GoPtr<crate::runtime2::p>) -> (GoPtr<crate::runtime2::g>, bool) {
    let mut gp: Arc<Mutex<Option<g>>> = Arc::new(Mutex::new(None));
    let mut inheritTime: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        // If there's a runnext, it's the next G to run.
    let mut next = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runnext.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

        // If the runnext is non-0 and the CAS fails, it could only have been stolen by another P,
        // because other Ps can race to set runnext to 0, but only the current P can set it to non-0.
        // Hence, there's no need to retry this CAS if it fails.
    if { let __tmp_x = (*next.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } && (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runnext.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))))))) {
        return (
            crate::runtime2::guintptr::ptr(&(*next.lock().unwrap().as_ref().unwrap())),
            true
        );
    }

    loop {
        let mut h = internal_runtime_atomic::load_acq({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone());
        let mut t = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = h; __tmp_x == __tmp_y } {
        return (GoPtr::nil(), false);
    }
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&({ let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = h; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y }) as usize].clone() }));
        if internal_runtime_atomic::cas_rel({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some(h))), Arc::new(Mutex::new(Some({ let __tmp_x = h; let __tmp_y = 1 as u32; __tmp_x + __tmp_y })))) {
        return (gp.clone(), false);
    }
    }
}

/// runqdrain drains the local runnable queue of pp and returns all goroutines in it.
/// Executed only by the owner P.
pub fn runqdrain(pp: GoPtr<crate::runtime2::p>) -> (Arc<Mutex<Option<gQueue>>>, u32) {
    let mut drainQ: Arc<Mutex<Option<gQueue>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut n: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

    let mut oldNext = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runnext.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = (*oldNext.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } && (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runnext.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = oldNext.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))))))) {
        (*drainQ.lock().unwrap().as_ref().unwrap()).push_back(crate::runtime2::guintptr::ptr(&(*oldNext.lock().unwrap().as_ref().unwrap())));
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    'retry: loop {
        let mut h = internal_runtime_atomic::load_acq({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone());
        let mut t = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut qn = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = h; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*qn.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return (drainQ.clone(), (*n.lock().unwrap().as_ref().unwrap()));
    }
        if { let __tmp_x = { let __v = (*qn.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x > __tmp_y } {
        continue 'retry;
    }

        if !internal_runtime_atomic::cas_rel({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some(h))), Arc::new(Mutex::new(Some({ let __tmp_x = h; let __tmp_y = { let __v = (*qn.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))) {
        continue 'retry;
    }

                // We've inverted the order in which it gets G's from the local P's runnable queue
                // and then advances the head pointer because we don't want to mess up the statuses of G's
                // while runqdrain() and runqsteal() are running in parallel.
                // Thus we should advance the head pointer before draining the local P into a gQueue,
                // so that we can update any gp.schedlink only after we take the full ownership of G,
                // meanwhile, other P's can't access to all G's in local P's runnable queue and steal them.
                // See https://groups.google.com/g/golang-dev/c/0pTKxEKhHSc/m/6Q85QjdVBQAJ for more details.
        let mut i = Arc::new(Mutex::new(Some(0 as u32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*qn.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&({ let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
            let __tmp_x = ({ let __tmp_x = h; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y });
            let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
            __tmp_x % __tmp_y
        }) as usize].clone() }));
        (*drainQ.lock().unwrap().as_ref().unwrap()).push_back(gp.clone());
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return (drainQ.clone(), (*n.lock().unwrap().as_ref().unwrap()));
    };
    unreachable!()
}

/// Grabs a batch of goroutines from pp's runnable queue into batch.
/// Batch is a ring buffer starting at batchHead.
/// Returns number of grabbed goroutines.
/// Can be executed by any P.
pub fn runqgrab(pp: Arc<Mutex<Option<p>>>, batch: Arc<Mutex<Option<[guintptr; 256]>>>, batchHead: Arc<Mutex<Option<u32>>>, stealRunNextG: Arc<Mutex<Option<bool>>>) -> u32 {
    loop {
        let mut h = internal_runtime_atomic::load_acq((*pp.lock().unwrap().as_ref().unwrap()).runqhead.clone());
        let mut t = internal_runtime_atomic::load_acq((*pp.lock().unwrap().as_ref().unwrap()).runqtail.clone());
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = t; let __tmp_y = h; __tmp_x - __tmp_y })));
        { let new_val = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u32; __tmp_x / __tmp_y }; __tmp_x - __tmp_y }; *n.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        if { let __v = (*stealRunNextG.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Try to steal from pp.runnext.
        {
        let mut next = Arc::new(Mutex::new(Some({ let __selector_holder = (*pp.lock().unwrap().as_ref().unwrap()).runnext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = (*next.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
            if { let __tmp_x = (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).status.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PRUNNING as u32; __tmp_x == __tmp_y } {
        if !OS_HAS_LOW_RES_TIMER {
        usleep(Arc::new(Mutex::new(Some(3 as u32))));
    } else {
        osyield();
    }
    };
            if !(*(*pp.lock().unwrap().as_ref().unwrap()).runnext.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))))))) {
        continue
    };
            (*batch.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*batchHead.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(256 as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y }) as usize] = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*{ let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));;
            return 1;;
        }
    }
    }
                // Try to steal from pp.runnext.
                // Sleep to ensure that pp isn't about to run the g
                // we are about to steal.
                // The important use case here is when the g running
                // on pp ready()s another g and then almost
                // immediately blocks. Instead of stealing runnext
                // in this window, back off to give pp a chance to
                // schedule runnext. This will avoid thrashing gs
                // between different Ps.
                // A sync chan send/recv takes ~50ns as of time of
                // writing, so 3us gives ~50x overshoot.
                // On some platforms system timer granularity is
                // 1-15ms, which is way too much for this
                // optimization. So just yield.
        return 0;
    }
                // Try to steal from pp.runnext.
                // Sleep to ensure that pp isn't about to run the g
                // we are about to steal.
                // The important use case here is when the g running
                // on pp ready()s another g and then almost
                // immediately blocks. Instead of stealing runnext
                // in this window, back off to give pp a chance to
                // schedule runnext. This will avoid thrashing gs
                // between different Ps.
                // A sync chan send/recv takes ~50ns as of time of
                // writing, so 3us gives ~50x overshoot.
                // On some platforms system timer granularity is
                // 1-15ms, which is way too much for this
                // optimization. So just yield.
        if {
            let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = 256; let __tmp_y = 2; __tmp_x / __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()) as u32;
            __tmp_x > __tmp_y
        } {
        continue
    }
        let mut i = Arc::new(Mutex::new(Some(0 as u32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut g = Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = (*pp.lock().unwrap().as_ref().unwrap()).runq.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
    let __tmp_x = ({ let __tmp_x = h; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y });
    let __tmp_y = (*Arc::new(Mutex::new(Some((*(*pp.lock().unwrap().as_ref().unwrap()).runq.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
    __tmp_x % __tmp_y
}) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        (*batch.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = ({ let __tmp_x = { let __v = (*batchHead.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some(256 as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y }) as usize] = crate::runtime2::guintptr(Arc::new(Mutex::new(Some((*{ let __v = (*g.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if internal_runtime_atomic::cas_rel((*pp.lock().unwrap().as_ref().unwrap()).runqhead.clone(), Arc::new(Mutex::new(Some(h))), Arc::new(Mutex::new(Some({ let __tmp_x = h; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))) {
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    }
}

/// Steal half of elements from local runnable queue of p2
/// and put onto local runnable queue of p.
/// Returns one of the stolen elements (or nil if failed).
pub fn runqsteal(pp: GoPtr<crate::runtime2::p>, p2: Arc<Mutex<Option<p>>>, stealRunNextG: Arc<Mutex<Option<bool>>>) -> GoPtr<crate::runtime2::g> {
    let mut t = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut n = runqgrab(p2.clone(), { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = stealRunNextG.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = n; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return GoPtr::nil();
    }
    { n -= 1; }
    let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&({ let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
        let __tmp_x = ({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = n; __tmp_x + __tmp_y });
        let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
        __tmp_x % __tmp_y
    }) as usize].clone() }));
    if { let __tmp_x = n; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return gp.clone();
    }
    let mut h = internal_runtime_atomic::load_acq({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqhead.clone()); __ptr_value }.clone());
    if {
        let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = h; __tmp_x - __tmp_y }; let __tmp_y = n; __tmp_x + __tmp_y };
        let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runq.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32;
        __tmp_x >= __tmp_y
    } {
        throw(Arc::new(Mutex::new(Some("runqsteal: runq overflow".to_string()))));
    }
    internal_runtime_atomic::store_rel({ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.runqtail.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = n; __tmp_x + __tmp_y }))));
    gp.clone()
}

pub fn gcd(mut a: Arc<Mutex<Option<u32>>>, mut b: Arc<Mutex<Option<u32>>>) -> u32 {
    while { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        {
            let __tmp_0 = (*b.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y };
            *a.lock().unwrap() = Some(__tmp_0);
            *b.lock().unwrap() = Some(__tmp_1);
        };
    }
    return { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct22 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub newm: Arc<Mutex<Option<muintptr>>>,
    pub waiting: Arc<Mutex<Option<bool>>>,
    pub wake: Arc<Mutex<Option<note>>>,
    pub have_template_thread: Arc<Mutex<Option<u32>>>,
}
impl AnonymousStruct22 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.newm.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.waiting.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.wake.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.have_template_thread.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            newm: __go_clone_1_0,
            waiting: __go_clone_2_0,
            wake: __go_clone_3_0,
            have_template_thread: __go_clone_4_0,
        }
    }
}


impl Default for AnonymousStruct22 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(note::default())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            lock: __go_default_0_0,
            newm: __go_default_1_0,
            waiting: __go_default_2_0,
            wake: __go_default_3_0,
            have_template_thread: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.newm.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.waiting.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.wake.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.have_template_thread.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for AnonymousStruct22 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct23 {
    pub signal_lock: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub hz: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct23 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.signal_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.hz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            signal_lock: __go_clone_0_0,
            hz: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct23 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            signal_lock: __go_default_0_0,
            hz: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct23 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.signal_lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.hz.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct23 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type newmHandoff = AnonymousStruct22;


pub(crate) type prof = AnonymousStruct23;


pub(crate) fn __go_init_functions() {
    self::__go_init_0();
    self::__go_init_1();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
    self::__go_init_1();
}


impl GoValueClone for worldStop {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for cgothreadstart {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for sysmontick {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gQueue {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gList {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for randomOrder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for randomEnum {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for initTask {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for tracestat {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
