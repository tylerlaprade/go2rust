use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lockrank_off::{assert_world_stopped_or_lock_held}, mgc::{GC_MARK_WORKER_DEDICATED_MODE, GC_MARK_WORKER_FRACTIONAL_MODE, GC_MARK_WORKER_IDLE_MODE, GC_TRIGGER_TIME, SWEEP_MIN_HEAP_DISTANCE, __G_COFF, gcBgMarkWorkerNode, gcBlackenEnabled, gcMarkWorkerMode, gcTrigger, gcTriggerKind, gc_mark_work_available, gcphase, work}, mgclimit::{gcCPULimiter}, mgcscavenge::{gc_pace_scavenger}, mgcsweep::{gc_pace_sweeper, is_sweep_done}, mheap::{mheap_}, mstats::{sysMemStat}, panic::{throw}, print::{printlock, printunlock}, proc::{casgstatus, preemptone}, rand::{cheaprandn}, runtime1::{debug}, runtime2::{__GRUNNABLE, __GWAITING, __PRUNNING, allp, g, gcBgMarkWorkerPool, gomaxprocs, guintptr, lfnode, m, mutex, p, puintptr}, stubs::{getg}, time_nofake::{nanotime}, traceruntime::{traceLocker, trace_acquire, trace_release}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const GC_GOAL_UTILIZATION: f64 = GC_BACKGROUND_UTILIZATION;
pub(crate) const GC_BACKGROUND_UTILIZATION: f64 = 0.25;
pub(crate) const GC_CREDIT_SLACK: i32 = 2000;
pub(crate) const GC_ASSIST_TIME_SLACK: i32 = 5000;
pub(crate) const GC_OVER_ASSIST_WORK: i32 = 64 << 10;
pub(crate) const DEFAULT_HEAP_MINIMUM: i32 = ((((internal_goexperiment::HEAP_MINIMUM512_KI_B_INT) as i32) * ((512 as i32) << (10 as i32))) + (((1 as i32) - (internal_goexperiment::HEAP_MINIMUM512_KI_B_INT as i32)) * ((4 as i32) << (20 as i32))));
pub(crate) const MAX_STACK_SCAN_SLACK: i32 = 8 << 10;
pub(crate) const MEMORY_LIMIT_MIN_HEAP_GOAL_HEADROOM: i32 = 1 << 20;
pub(crate) const MEMORY_LIMIT_HEAP_GOAL_HEADROOM_PERCENT: i32 = 3;


pub(crate) const TRIGGER_RATIO_DEN: i32 = 64;
pub(crate) const MIN_TRIGGER_RATIO_NUM: i32 = 45;
pub(crate) const MAX_TRIGGER_RATIO_NUM: i32 = 61;


#[derive(Clone)]
pub struct gcControllerState {
    pub gc_percent: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub memory_limit: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub heap_minimum: Arc<Mutex<Option<u64>>>,
    pub runway: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub cons_mark: Arc<Mutex<Option<f64>>>,
    pub last_cons_mark: Arc<Mutex<Option<[f64; 4]>>>,
    pub gc_percent_heap_goal: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub sweep_dist_min_trigger: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub triggered: Arc<Mutex<Option<u64>>>,
    pub last_heap_goal: Arc<Mutex<Option<u64>>>,
    pub heap_live: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub heap_scan: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub last_heap_scan: Arc<Mutex<Option<u64>>>,
    pub last_stack_scan: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub max_stack_scan: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub globals_scan: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub heap_marked: Arc<Mutex<Option<u64>>>,
    pub heap_scan_work: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub stack_scan_work: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub globals_scan_work: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub bg_scan_credit: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub assist_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub dedicated_mark_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub fractional_mark_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub idle_mark_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub mark_start_time: Arc<Mutex<Option<i64>>>,
    pub dedicated_mark_workers_needed: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub idle_mark_workers: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub assist_work_per_byte: Arc<Mutex<Option<internal_runtime_atomic::types::Float64>>>,
    pub assist_bytes_per_work: Arc<Mutex<Option<internal_runtime_atomic::types::Float64>>>,
    pub fractional_utilization_goal: Arc<Mutex<Option<f64>>>,
    pub heap_in_use: Arc<Mutex<Option<sysMemStat>>>,
    pub heap_released: Arc<Mutex<Option<sysMemStat>>>,
    pub heap_free: Arc<Mutex<Option<sysMemStat>>>,
    pub total_alloc: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub total_free: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub mapped_ready: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub test: Arc<Mutex<Option<bool>>>,
    pub __blank_38_0: Arc<Mutex<Option<internal_cpu::r#mod::CacheLinePad>>>,
}

impl gcControllerState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.gc_percent.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.memory_limit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.heap_minimum.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.runway.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.cons_mark.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.last_cons_mark.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.gc_percent_heap_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.sweep_dist_min_trigger.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.triggered.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.last_heap_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.heap_live.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.heap_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.last_heap_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.last_stack_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.max_stack_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.globals_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.heap_marked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.heap_scan_work.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.stack_scan_work.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.globals_scan_work.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.bg_scan_credit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = { let __guard = self.dedicated_mark_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_23_0 = { let __guard = self.fractional_mark_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_24_0 = { let __guard = self.idle_mark_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_25_0 = { let __guard = self.mark_start_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_0 = { let __guard = self.dedicated_mark_workers_needed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_27_0 = { let __guard = self.idle_mark_workers.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_0 = { let __guard = self.assist_work_per_byte.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_29_0 = { let __guard = self.assist_bytes_per_work.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_30_0 = { let __guard = self.fractional_utilization_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_31_0 = { let __guard = self.heap_in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_32_0 = { let __guard = self.heap_released.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_33_0 = { let __guard = self.heap_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_34_0 = { let __guard = self.total_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_35_0 = { let __guard = self.total_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_36_0 = { let __guard = self.mapped_ready.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_37_0 = { let __guard = self.test.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_38_0 = { let __guard = self.__blank_38_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            gc_percent: __go_clone_0_0,
            memory_limit: __go_clone_1_0,
            heap_minimum: __go_clone_2_0,
            runway: __go_clone_3_0,
            cons_mark: __go_clone_4_0,
            last_cons_mark: __go_clone_5_0,
            gc_percent_heap_goal: __go_clone_6_0,
            sweep_dist_min_trigger: __go_clone_7_0,
            triggered: __go_clone_8_0,
            last_heap_goal: __go_clone_9_0,
            heap_live: __go_clone_10_0,
            heap_scan: __go_clone_11_0,
            last_heap_scan: __go_clone_12_0,
            last_stack_scan: __go_clone_13_0,
            max_stack_scan: __go_clone_14_0,
            globals_scan: __go_clone_15_0,
            heap_marked: __go_clone_16_0,
            heap_scan_work: __go_clone_17_0,
            stack_scan_work: __go_clone_18_0,
            globals_scan_work: __go_clone_19_0,
            bg_scan_credit: __go_clone_20_0,
            assist_time: __go_clone_21_0,
            dedicated_mark_time: __go_clone_22_0,
            fractional_mark_time: __go_clone_23_0,
            idle_mark_time: __go_clone_24_0,
            mark_start_time: __go_clone_25_0,
            dedicated_mark_workers_needed: __go_clone_26_0,
            idle_mark_workers: __go_clone_27_0,
            assist_work_per_byte: __go_clone_28_0,
            assist_bytes_per_work: __go_clone_29_0,
            fractional_utilization_goal: __go_clone_30_0,
            heap_in_use: __go_clone_31_0,
            heap_released: __go_clone_32_0,
            heap_free: __go_clone_33_0,
            total_alloc: __go_clone_34_0,
            total_free: __go_clone_35_0,
            mapped_ready: __go_clone_36_0,
            test: __go_clone_37_0,
            __blank_38_0: __go_clone_38_0,
        }
    }
}


impl Default for gcControllerState {
    fn default() -> Self {
        Self { gc_percent: Arc::new(Mutex::new(Some(Default::default()))), memory_limit: Arc::new(Mutex::new(Some(Default::default()))), heap_minimum: Arc::new(Mutex::new(Some(0))), runway: Arc::new(Mutex::new(Some(Default::default()))), cons_mark: Arc::new(Mutex::new(Some(0.0))), last_cons_mark: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0.0)))), gc_percent_heap_goal: Arc::new(Mutex::new(Some(Default::default()))), sweep_dist_min_trigger: Arc::new(Mutex::new(Some(Default::default()))), triggered: Arc::new(Mutex::new(Some(0))), last_heap_goal: Arc::new(Mutex::new(Some(0))), heap_live: Arc::new(Mutex::new(Some(Default::default()))), heap_scan: Arc::new(Mutex::new(Some(Default::default()))), last_heap_scan: Arc::new(Mutex::new(Some(0))), last_stack_scan: Arc::new(Mutex::new(Some(Default::default()))), max_stack_scan: Arc::new(Mutex::new(Some(Default::default()))), globals_scan: Arc::new(Mutex::new(Some(Default::default()))), heap_marked: Arc::new(Mutex::new(Some(0))), heap_scan_work: Arc::new(Mutex::new(Some(Default::default()))), stack_scan_work: Arc::new(Mutex::new(Some(Default::default()))), globals_scan_work: Arc::new(Mutex::new(Some(Default::default()))), bg_scan_credit: Arc::new(Mutex::new(Some(Default::default()))), assist_time: Arc::new(Mutex::new(Some(Default::default()))), dedicated_mark_time: Arc::new(Mutex::new(Some(Default::default()))), fractional_mark_time: Arc::new(Mutex::new(Some(Default::default()))), idle_mark_time: Arc::new(Mutex::new(Some(Default::default()))), mark_start_time: Arc::new(Mutex::new(Some(0))), dedicated_mark_workers_needed: Arc::new(Mutex::new(Some(Default::default()))), idle_mark_workers: Arc::new(Mutex::new(Some(Default::default()))), assist_work_per_byte: Arc::new(Mutex::new(Some(Default::default()))), assist_bytes_per_work: Arc::new(Mutex::new(Some(Default::default()))), fractional_utilization_goal: Arc::new(Mutex::new(Some(0.0))), heap_in_use: Arc::new(Mutex::new(Some(crate::mstats::sysMemStat(Arc::new(Mutex::new(Some(0))))))), heap_released: Arc::new(Mutex::new(Some(crate::mstats::sysMemStat(Arc::new(Mutex::new(Some(0))))))), heap_free: Arc::new(Mutex::new(Some(crate::mstats::sysMemStat(Arc::new(Mutex::new(Some(0))))))), total_alloc: Arc::new(Mutex::new(Some(Default::default()))), total_free: Arc::new(Mutex::new(Some(Default::default()))), mapped_ready: Arc::new(Mutex::new(Some(Default::default()))), test: Arc::new(Mutex::new(Some(false))), __blank_38_0: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for gcControllerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.gc_percent.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.memory_limit.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.heap_minimum.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.runway.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.cons_mark.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", format_slice(&self.last_cons_mark));
        let __go_fmt_6 = format!("{}", (*self.gc_percent_heap_goal.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.sweep_dist_min_trigger.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.triggered.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.last_heap_goal.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.heap_live.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.heap_scan.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.last_heap_scan.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.last_stack_scan.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.max_stack_scan.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.globals_scan.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.heap_marked.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.heap_scan_work.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.stack_scan_work.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", (*self.globals_scan_work.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_20 = format!("{}", (*self.bg_scan_credit.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_21 = format!("{}", (*self.assist_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_22 = format!("{}", (*self.dedicated_mark_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_23 = format!("{}", (*self.fractional_mark_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_24 = format!("{}", (*self.idle_mark_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_25 = format!("{}", (*self.mark_start_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_26 = format!("{}", (*self.dedicated_mark_workers_needed.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_27 = format!("{}", (*self.idle_mark_workers.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_28 = format!("{}", (*self.assist_work_per_byte.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_29 = format!("{}", (*self.assist_bytes_per_work.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_30 = format!("{}", (*self.fractional_utilization_goal.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_31 = format!("{}", (*self.heap_in_use.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_32 = format!("{}", (*self.heap_released.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_33 = format!("{}", (*self.heap_free.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_34 = format!("{}", (*self.total_alloc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_35 = format!("{}", (*self.total_free.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_36 = format!("{}", (*self.mapped_ready.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_37 = format!("{}", (*self.test.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_38 = format!("{}", (*self.__blank_38_0.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8, __go_fmt_9, __go_fmt_10, __go_fmt_11, __go_fmt_12, __go_fmt_13, __go_fmt_14, __go_fmt_15, __go_fmt_16, __go_fmt_17, __go_fmt_18, __go_fmt_19, __go_fmt_20, __go_fmt_21, __go_fmt_22, __go_fmt_23, __go_fmt_24, __go_fmt_25, __go_fmt_26, __go_fmt_27, __go_fmt_28, __go_fmt_29, __go_fmt_30, __go_fmt_31, __go_fmt_32, __go_fmt_33, __go_fmt_34, __go_fmt_35, __go_fmt_36, __go_fmt_37, __go_fmt_38)
    }
}

impl GoJsonDecode for gcControllerState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static gcController: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<gcControllerState>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *gcController.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *gcController.lock().unwrap() = Some(Default::default());
}


impl gcControllerState {
    pub fn init(&mut self, gcPercent: Arc<Mutex<Option<i32>>>, memoryLimit: Arc<Mutex<Option<i64>>>) {
        { let new_val = DEFAULT_HEAP_MINIMUM as u64; *self.heap_minimum.lock().unwrap() = Some(new_val); };
        { let new_val = !(0 as u64) as u64; *self.triggered.lock().unwrap() = Some(new_val); };
        self.set_g_c_percent(Arc::new(Mutex::new(Some({ let __arg_holder = gcPercent.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.set_memory_limit(Arc::new(Mutex::new(Some({ let __arg_holder = memoryLimit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.commit(Arc::new(Mutex::new(Some(true))));
    }

    /// startCycle resets the GC controller's state and computes estimates
    /// for a new GC cycle. The caller must hold worldsema and the world
    /// must be stopped.
    pub fn start_cycle(&mut self, markStartTime: Arc<Mutex<Option<i64>>>, procs: Arc<Mutex<Option<i32>>>, trigger: Arc<Mutex<Option<gcTrigger>>>) {
        (*self.heap_scan_work.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*self.stack_scan_work.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*self.globals_scan_work.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*self.bg_scan_credit.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*self.assist_time.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*self.dedicated_mark_time.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*self.fractional_mark_time.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*self.idle_mark_time.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        { let new_val = markStartTime.lock().unwrap().as_ref().unwrap().clone(); *self.mark_start_time.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.heap_live.lock().unwrap().as_mut().unwrap()).load(); *self.triggered.lock().unwrap() = Some(new_val); };
                // Compute the background mark utilization goal. In general,
                // this may not come out exactly. We round the number of
                // dedicated workers so that the utilization is closest to
                // 25%. For small GOMAXPROCS, this would introduce too much
                // error, so we add fractional workers in that case.
        let mut totalUtilizationGoal = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*procs.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = GC_BACKGROUND_UTILIZATION as f64; __tmp_x * __tmp_y })));
        let mut dedicatedMarkWorkersNeeded = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*totalUtilizationGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.5; __tmp_x + __tmp_y }) as i64)));
        let mut utilError = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*dedicatedMarkWorkersNeeded.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*totalUtilizationGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; let __tmp_y = 1.0; __tmp_x - __tmp_y })));
        const maxUtilError: f64 = 0.3;

        if { let __tmp_x = { let __v = (*utilError.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -0.3; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*utilError.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = maxUtilError as f64; __tmp_x > __tmp_y } {
                // Rounding put us more than 30% off our goal. With
                // gcBackgroundUtilization of 25%, this happens for
                // GOMAXPROCS<=3 or GOMAXPROCS=6. Enable fractional
                // workers to compensate.
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*dedicatedMarkWorkersNeeded.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*totalUtilizationGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // Too many dedicated workers.
        { let mut guard = dedicatedMarkWorkersNeeded.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // Too many dedicated workers.
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*totalUtilizationGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*dedicatedMarkWorkersNeeded.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*procs.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; *self.fractional_utilization_goal.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = 0.0; *self.fractional_utilization_goal.lock().unwrap() = Some(new_val); };
    }
                // Rounding put us more than 30% off our goal. With
                // gcBackgroundUtilization of 25%, this happens for
                // GOMAXPROCS<=3 or GOMAXPROCS=6. Enable fractional
                // workers to compensate.
                // Too many dedicated workers.
                // In STW mode, we just want dedicated workers.
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gcstoptheworld.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some((*procs.lock().unwrap().as_ref().unwrap()) as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dedicatedMarkWorkersNeeded.lock().unwrap() = __moved_val; };
        { let new_val = 0.0; *self.fractional_utilization_goal.lock().unwrap() = Some(new_val); };
    }
                // Clear per-P state
        { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        { let new_val = 0 as i64; *(*p.lock().unwrap().as_ref().unwrap()).gc_assist_time.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as i64; *(*p.lock().unwrap().as_ref().unwrap()).gc_fractional_mark_time.lock().unwrap() = Some(new_val); };
    } }
        if { let __tmp_x = { let __selector_holder = (*trigger.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::mgc::gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_TIME as i32)))); __tmp_x == __tmp_y } {
                // During a periodic GC cycle, reduce the number of idle mark workers
                // required. However, we need at least one dedicated mark worker or
                // idle GC worker to ensure GC progress in some scenarios (see comment
                // on maxIdleMarkWorkers).
        if { let __tmp_x = { let __v = (*dedicatedMarkWorkersNeeded.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        self.set_max_idle_mark_workers(Arc::new(Mutex::new(Some(0 as i32))));
    } else {
                // TODO(mknyszek): The fundamental reason why we need this is because
                // we can't count on the fractional mark worker to get scheduled.
                // Fix that by ensuring it gets scheduled according to its quota even
                // if the rest of the application is idle.
        self.set_max_idle_mark_workers(Arc::new(Mutex::new(Some(1 as i32))));
    }
    } else {
                // N.B. gomaxprocs and dedicatedMarkWorkersNeeded are guaranteed not to
                // change during a GC cycle.
        self.set_max_idle_mark_workers(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*procs.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*dedicatedMarkWorkersNeeded.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))));
    }
                // During a periodic GC cycle, reduce the number of idle mark workers
                // required. However, we need at least one dedicated mark worker or
                // idle GC worker to ensure GC progress in some scenarios (see comment
                // on maxIdleMarkWorkers).
                // TODO(mknyszek): The fundamental reason why we need this is because
                // we can't count on the fractional mark worker to get scheduled.
                // Fix that by ensuring it gets scheduled according to its quota even
                // if the rest of the application is idle.
                // N.B. gomaxprocs and dedicatedMarkWorkersNeeded are guaranteed not to
                // change during a GC cycle.
                // Compute initial values for controls that are updated
                // throughout the cycle.
        (*self.dedicated_mark_workers_needed.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = dedicatedMarkWorkersNeeded.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.revise();
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gcpacertrace.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        let mut heapGoal = self.heap_goal();
        let mut assistRatio = (*self.assist_work_per_byte.lock().unwrap().as_ref().unwrap()).load();
        eprint!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "pacer: assist ratio=".to_string()), format!("{}", assistRatio), format!("{}", " (scan ".to_string()), format!("{}", { let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_scan.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 20; __tmp_x >> __tmp_y }), format!("{}", " MB in ".to_string()), format!("{}", { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).initial_heap_live.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 20; __tmp_x >> __tmp_y }), format!("{}", "->".to_string()), format!("{}", { let __tmp_x = heapGoal; let __tmp_y = 20; __tmp_x >> __tmp_y }), format!("{}", " MB)".to_string()), format!("{}", " workers=".to_string()), format!("{}", { let __v = (*dedicatedMarkWorkersNeeded.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "+".to_string()), format!("{}", (*self.fractional_utilization_goal.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
    }
    }

    /// revise updates the assist ratio during the GC cycle to account for
    /// improved estimates. This should be called whenever gcController.heapScan,
    /// gcController.heapLive, or if any inputs to gcController.heapGoal are
    /// updated. It is safe to call concurrently, but it may race with other
    /// calls to revise.
    ///
    /// The result of this race is that the two assist ratio values may not line
    /// up or may be stale. In practice this is OK because the assist ratio
    /// moves slowly throughout a GC cycle, and the assist ratio is a best-effort
    /// heuristic anyway. Furthermore, no part of the heuristic depends on
    /// the two assist ratio values being exact reciprocals of one another, since
    /// the two values are used to convert values from different sources.
    ///
    /// The worst case result of this raciness is that we may miss a larger shift
    /// in the ratio (say, if we decide to pace more aggressively against the
    /// hard heap goal) but even this "hard goal" is best-effort (see #40460).
    /// The dedicated GC should ensure we don't exceed the hard goal by too much
    /// in the rare case we do exceed it.
    ///
    /// It should only be called when gcBlackenEnabled != 0 (because this
    /// is when assists are enabled and the necessary statistics are
    /// available).
    pub fn revise(&self) {
        let mut gcPercent = (*self.gc_percent.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = gcPercent; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
                // If GC is disabled but we're running a forced GC,
                // act like GOGC is huge for the below calculations.
        { let new_val = 100000 as i32; gcPercent = new_val; };
    }
                // If GC is disabled but we're running a forced GC,
                // act like GOGC is huge for the below calculations.
        let mut live = (*self.heap_live.lock().unwrap().as_mut().unwrap()).load();
        let mut scan = (*self.heap_scan.lock().unwrap().as_mut().unwrap()).load();
        let mut work_local = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*self.heap_scan_work.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = (*self.stack_scan_work.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }; let __tmp_y = (*self.globals_scan_work.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y })));
                // Assume we're under the soft goal. Pace GC to complete at
                // heapGoal assuming the heap is in steady-state.
        let mut heapGoal = Arc::new(Mutex::new(Some(self.heap_goal() as i64)));
                // The expected scan work is computed as the amount of bytes scanned last
                // GC cycle (both heap and stack), plus our estimate of globals work for this cycle.
        let mut scanWorkExpected = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*self.last_heap_scan.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.last_stack_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }; let __tmp_y = (*self.globals_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }) as i64)));
                // maxScanWork is a worst-case estimate of the amount of scan work that
                // needs to be performed in this GC cycle. Specifically, it represents
                // the case where *all* scannable memory turns out to be live, and
                // *all* allocated stack space is scannable.
        let mut maxStackScan = (*self.max_stack_scan.lock().unwrap().as_mut().unwrap()).load();
        let mut maxScanWork = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = scan; let __tmp_y = maxStackScan; __tmp_x + __tmp_y }; let __tmp_y = (*self.globals_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }) as i64)));
        if { let __tmp_x = { let __v = (*work_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*scanWorkExpected.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // We've already done more scan work than expected. Because our expectation
                // is based on a steady-state scannable heap size, we assume this means our
                // heap is growing. Compute a new heap goal that takes our existing runway
                // computed for scanWorkExpected and extrapolates it to maxScanWork, the worst-case
                // scan work. This keeps our assist ratio stable if the heap continues to grow.
                //
                // The effect of this mechanism is that assists stay flat in the face of heap
                // growths. It's OK to use more memory this cycle to scan all the live heap,
                // because the next GC cycle is inevitably going to use *at least* that much
                // memory anyway.
        let mut extHeapGoal = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*heapGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.triggered.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*scanWorkExpected.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*maxScanWork.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.triggered.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        { let new_val = maxScanWork.lock().unwrap().as_ref().unwrap().clone(); *scanWorkExpected.lock().unwrap() = Some(new_val); };
                // hardGoal is a hard limit on the amount that we're willing to push back the
                // heap goal, and that's twice the heap goal (i.e. if GOGC=100 and the heap and/or
                // stacks and/or globals grow to twice their size, this limits the current GC cycle's
                // growth to 4x the original live heap's size).
                //
                // This maintains the invariant that we use no more memory than the next GC cycle
                // will anyway.
        let mut hardGoal = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = 1.0; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(gcPercent as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 100.0; __tmp_x / __tmp_y }; __tmp_x + __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*heapGoal.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as i64)));
        if { let __tmp_x = { let __v = (*extHeapGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hardGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = hardGoal.lock().unwrap().as_ref().unwrap().clone(); *extHeapGoal.lock().unwrap() = Some(new_val); };
    }
        { let new_val = extHeapGoal.lock().unwrap().as_ref().unwrap().clone(); *heapGoal.lock().unwrap() = Some(new_val); };
    }
                // We've already done more scan work than expected. Because our expectation
                // is based on a steady-state scannable heap size, we assume this means our
                // heap is growing. Compute a new heap goal that takes our existing runway
                // computed for scanWorkExpected and extrapolates it to maxScanWork, the worst-case
                // scan work. This keeps our assist ratio stable if the heap continues to grow.
                //
                // The effect of this mechanism is that assists stay flat in the face of heap
                // growths. It's OK to use more memory this cycle to scan all the live heap,
                // because the next GC cycle is inevitably going to use *at least* that much
                // memory anyway.
                // hardGoal is a hard limit on the amount that we're willing to push back the
                // heap goal, and that's twice the heap goal (i.e. if GOGC=100 and the heap and/or
                // stacks and/or globals grow to twice their size, this limits the current GC cycle's
                // growth to 4x the original live heap's size).
                //
                // This maintains the invariant that we use no more memory than the next GC cycle
                // will anyway.
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(live as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*heapGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // We're already past our heap goal, even the extrapolated one.
                // Leave ourselves some extra runway, so in the worst case we
                // finish by that point.
        const maxOvershoot: f64 = 1.1;

        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*heapGoal.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = maxOvershoot as f64; __tmp_x * __tmp_y }) as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *heapGoal.lock().unwrap() = __moved_val; };
                // Compute the upper bound on the scan work remaining.
        { let new_val = maxScanWork.lock().unwrap().as_ref().unwrap().clone(); *scanWorkExpected.lock().unwrap() = Some(new_val); };
    }
                // We're already past our heap goal, even the extrapolated one.
                // Leave ourselves some extra runway, so in the worst case we
                // finish by that point.
                // Compute the upper bound on the scan work remaining.
                // Compute the remaining scan work estimate.
                //
                // Note that we currently count allocations during GC as both
                // scannable heap (heapScan) and scan work completed
                // (scanWork), so allocation will change this difference
                // slowly in the soft regime and not at all in the hard
                // regime.
        let mut scanWorkRemaining = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*scanWorkExpected.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*work_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*scanWorkRemaining.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1000 as i64; __tmp_x < __tmp_y } {
                // We set a somewhat arbitrary lower bound on
                // remaining scan work since if we aim a little high,
                // we can miss by a little.
                //
                // We *do* need to enforce that this is at least 1,
                // since marking is racy and double-scanning objects
                // may legitimately make the remaining scan work
                // negative, even in the hard goal regime.
        { let new_val = 1000 as i64; *scanWorkRemaining.lock().unwrap() = Some(new_val); };
    }
                // We set a somewhat arbitrary lower bound on
                // remaining scan work since if we aim a little high,
                // we can miss by a little.
                //
                // We *do* need to enforce that this is at least 1,
                // since marking is racy and double-scanning objects
                // may legitimately make the remaining scan work
                // negative, even in the hard goal regime.
                // Compute the heap distance remaining.
        let mut heapRemaining = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*heapGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(live as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*heapRemaining.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
                // This shouldn't happen, but if it does, avoid
                // dividing by zero or setting the assist negative.
        { let new_val = 1 as i64; *heapRemaining.lock().unwrap() = Some(new_val); };
    }
                // This shouldn't happen, but if it does, avoid
                // dividing by zero or setting the assist negative.
                // Compute the mutator assist ratio so by the time the mutator
                // allocates the remaining heap bytes up to heapGoal, it will
                // have done (or stolen) the remaining amount of scan work.
                // Note that the assist ratio values are updated atomically
                // but not together. This means there may be some degree of
                // skew between the two values. This is generally OK as the
                // values shift relatively slowly over the course of a GC
                // cycle.
        let mut assistWorkPerByte = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*scanWorkRemaining.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*heapRemaining.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y })));
        let mut assistBytesPerWork = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*heapRemaining.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*scanWorkRemaining.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y })));
        (*self.assist_work_per_byte.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = assistWorkPerByte.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*self.assist_bytes_per_work.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = assistBytesPerWork.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// endCycle computes the consMark estimate for the next cycle.
    /// userForced indicates whether the current GC cycle was forced
    /// by the application.
    pub fn end_cycle(&mut self, now: Arc<Mutex<Option<i64>>>, procs: Arc<Mutex<Option<i32>>>, userForced: Arc<Mutex<Option<bool>>>) {
                // Record last heap goal for the scavenger.
                // We'll be updating the heap goal soon.
        { let new_val = self.heap_goal(); *(*gcController.lock().unwrap().as_ref().unwrap()).last_heap_goal.lock().unwrap() = Some(new_val); };
                // Compute the duration of time for which assists were turned on.
        let mut assistDuration = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.mark_start_time.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
                // Assume background mark hit its utilization goal.
        let mut utilization = Arc::new(Mutex::new(Some(GC_BACKGROUND_UTILIZATION)));
                // Add assist utilization; avoid divide by zero.
        if { let __tmp_x = { let __v = (*assistDuration.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.assist_time.lock().unwrap().as_mut().unwrap()).load() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*assistDuration.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*procs.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; let mut guard = utilization.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        if { let __tmp_x = (*self.heap_live.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = (*self.triggered.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
                // Shouldn't happen, but let's be very safe about this in case the
                // GC is somehow extremely short.
                //
                // In this case though, the only reasonable value for c.heapLive-c.triggered
                // would be 0, which isn't really all that useful, i.e. the GC was so short
                // that it didn't matter.
                //
                // Ignore this case and don't update anything.
        return;
    }
                // Shouldn't happen, but let's be very safe about this in case the
                // GC is somehow extremely short.
                //
                // In this case though, the only reasonable value for c.heapLive-c.triggered
                // would be 0, which isn't really all that useful, i.e. the GC was so short
                // that it didn't matter.
                //
                // Ignore this case and don't update anything.
        let mut idleUtilization = Arc::new(Mutex::new(Some(0.0)));
        if { let __tmp_x = { let __v = (*assistDuration.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.idle_mark_time.lock().unwrap().as_mut().unwrap()).load() as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*assistDuration.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*procs.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; *idleUtilization.lock().unwrap() = Some(new_val); };
    }
                // Determine the cons/mark ratio.
                //
                // The units we want for the numerator and denominator are both B / cpu-ns.
                // We get this by taking the bytes allocated or scanned, and divide by the amount of
                // CPU time it took for those operations. For allocations, that CPU time is
                //
                //    assistDuration * procs * (1 - utilization)
                //
                // Where utilization includes just background GC workers and assists. It does *not*
                // include idle GC work time, because in theory the mutator is free to take that at
                // any point.
                //
                // For scanning, that CPU time is
                //
                //    assistDuration * procs * (utilization + idleUtilization)
                //
                // In this case, we *include* idle utilization, because that is additional CPU time that
                // the GC had available to it.
                //
                // In effect, idle GC time is sort of double-counted here, but it's very weird compared
                // to other kinds of GC work, because of how fluid it is. Namely, because the mutator is
                // *always* free to take it.
                //
                // So this calculation is really:
                //     (heapLive-trigger) / (assistDuration * procs * (1-utilization)) /
                //         (scanWork) / (assistDuration * procs * (utilization+idleUtilization))
                //
                // Note that because we only care about the ratio, assistDuration and procs cancel out.
        let mut scanWork = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*self.heap_scan_work.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = (*self.stack_scan_work.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }; let __tmp_y = (*self.globals_scan_work.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y })));
        let mut currentConsMark = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.heap_live.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = (*self.triggered.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __v = (*utilization.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*idleUtilization.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*scanWork.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 1.0; let __tmp_y = { let __v = (*utilization.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x * __tmp_y }); __tmp_x / __tmp_y })));
                // Update our cons/mark estimate. This is the maximum of the value we just computed and the last
                // 4 cons/mark values we measured. The reason we take the maximum here is to bias a noisy
                // cons/mark measurement toward fewer assists at the expense of additional GC cycles (starting
                // earlier).
        let mut oldConsMark = Arc::new(Mutex::new(Some({ let __selector_holder = self.cons_mark.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = currentConsMark.lock().unwrap().as_ref().unwrap().clone(); *self.cons_mark.lock().unwrap() = Some(new_val); };
        for i in 0..(({ let __range_holder = self.last_cons_mark.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __tmp_x = { let __seq = { let __seq_holder = self.last_cons_mark.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __tmp_y = (*self.cons_mark.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = self.last_cons_mark.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; *self.cons_mark.lock().unwrap() = Some(new_val); };
    }
    }
        { let _dst_start = 0; let _dst_len = (*self.last_cons_mark.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = self.last_cons_mark.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*self.last_cons_mark.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        (*self.last_cons_mark.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = 4; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = { let __v = (*currentConsMark.lock().unwrap().as_ref().unwrap()).clone(); __v };
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gcpacertrace.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        printlock();
        let mut goal = Arc::new(Mutex::new(Some({ let __tmp_x = GC_GOAL_UTILIZATION as f64; let __tmp_y = 100.0; __tmp_x * __tmp_y })));
        eprint!("{}{}{}{}{}", format!("{}", "pacer: ".to_string()), format!("{}", (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*utilization.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100.0; __tmp_x * __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())), format!("{}", "% CPU (".to_string()), format!("{}", (*Arc::new(Mutex::new(Some((*goal.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap())), format!("{}", " exp.) for ".to_string()));
        eprint!("{}{}{}{}{}{}{}{}", format!("{}", (*self.heap_scan_work.lock().unwrap().as_mut().unwrap()).load()), format!("{}", "+".to_string()), format!("{}", (*self.stack_scan_work.lock().unwrap().as_mut().unwrap()).load()), format!("{}", "+".to_string()), format!("{}", (*self.globals_scan_work.lock().unwrap().as_mut().unwrap()).load()), format!("{}", " B work (".to_string()), format!("{}", { let __tmp_x = { let __tmp_x = (*self.last_heap_scan.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.last_stack_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }; let __tmp_y = (*self.globals_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }), format!("{}", " B exp.) ".to_string()));
        let mut live = (*self.heap_live.lock().unwrap().as_mut().unwrap()).load();
        eprint!("{}{}{}{}{}{}{}{}{}", format!("{}", "in ".to_string()), format!("{}", (*self.triggered.lock().unwrap().as_ref().unwrap())), format!("{}", " B -> ".to_string()), format!("{}", live), format!("{}", " B (\u{2206}goal ".to_string()), format!("{}", { let __tmp_x = (*Arc::new(Mutex::new(Some(live as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.last_heap_goal.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }), format!("{}", ", cons/mark ".to_string()), format!("{}", { let __v = (*oldConsMark.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ")".to_string()));
        eprintln!();
        printunlock();
    }
    }

    /// enlistWorker encourages another dedicated mark worker to start on
    /// another P if there are spare worker slots. It is used by putfull
    /// when more work is made available.
    ///
    ///go:nowritebarrier
    pub fn enlist_worker(&self) {
                // If there are idle Ps, wake one so it will run an idle worker.
                // NOTE: This is suspected of causing deadlocks. See golang.org/issue/19112.
                //
                //	if sched.npidle.Load() != 0 && sched.nmspinning.Load() == 0 {
                //		wakep()
                //		return
                //	}
                // There are no idle Ps. If we need more dedicated workers,
                // try to preempt a running P so it will switch to a worker.
        if { let __tmp_x = (*self.dedicated_mark_workers_needed.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return;
    }
                // Pick a random other P to preempt.
        if { let __tmp_x = (*gomaxprocs.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x <= __tmp_y } {
        return;
    }
        let mut gp = getg();
        if { let __nil_result = (*gp.lock().unwrap()).is_none(); __nil_result } || { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } || { let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        return;
    }
        let mut myID = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().id.clone() }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut tries = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*tries.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        let mut id = Arc::new(Mutex::new(Some(cheaprandn(Arc::new(Mutex::new(Some(({ let __tmp_x = (*gomaxprocs.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x - __tmp_y }) as u32)))) as i32)));
        if { let __tmp_x = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*myID.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let mut guard = id.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        let mut p = { let __seq = { let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        if { let __tmp_x = (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).status.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PRUNNING as u32; __tmp_x != __tmp_y } {
        { let mut guard = tries.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        if preemptone(p.clone()) {
        return;
    }
        { let mut guard = tries.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    /// findRunnableGCWorker returns a background mark worker for pp if it
    /// should be run. This must only be called when gcBlackenEnabled != 0.
    pub fn find_runnable_g_c_worker(&self, pp: GoPtr<crate::runtime2::p>, mut now: Arc<Mutex<Option<i64>>>) -> (GoPtr<crate::runtime2::g>, i64) {
        if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("gcControllerState.findRunnable: blackening not enabled".to_string()))));
    }
                // Since we have the current time, check if the GC CPU limiter
                // hasn't had an update in a while. This check is necessary in
                // case the limiter is on but hasn't been checked in a while and
                // so may have left sufficient headroom to turn off again.
        if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = nanotime(); *now.lock().unwrap() = Some(new_val); };
    }
        if (*gcCPULimiter.lock().unwrap().as_ref().unwrap()).need_update(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        (*gcCPULimiter.lock().unwrap().as_mut().unwrap()).update(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        if !gc_mark_work_available(pp.clone()) {
                // No work to be done right now. This can happen at
                // the end of the mark phase when there are still
                // assists tapering off. Don't bother running a worker
                // now because it'll just return immediately.
        return (GoPtr::nil(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
                // No work to be done right now. This can happen at
                // the end of the mark phase when there are still
                // assists tapering off. Don't bother running a worker
                // now because it'll just return immediately.
        if { let __tmp_x = (*self.dedicated_mark_workers_needed.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } && { let __tmp_x = (*self.fractional_utilization_goal.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0.0; __tmp_x == __tmp_y } {
                // No current need for dedicated workers, and no need at all for
                // fractional workers. Check before trying to acquire a worker; when
                // GOMAXPROCS is large, that can be expensive and is often unnecessary.
                //
                // When a dedicated worker stops running, the gcBgMarkWorker loop notes
                // the need for the worker before returning it to the pool. If we don't
                // see the need now, we wouldn't have found it in the pool anyway.
        return (GoPtr::nil(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
                // No current need for dedicated workers, and no need at all for
                // fractional workers. Check before trying to acquire a worker; when
                // GOMAXPROCS is large, that can be expensive and is often unnecessary.
                //
                // When a dedicated worker stops running, the gcBgMarkWorker loop notes
                // the need for the worker before returning it to the pool. If we don't
                // see the need now, we wouldn't have found it in the pool anyway.
                // Grab a worker before we commit to running below.
        let mut node: GoPtr<crate::mgc::gcBgMarkWorkerNode> = GoPtr::raw({ let __ptr = (*gcBgMarkWorkerPool.lock().unwrap().as_ref().unwrap()).pop().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if node.is_nil() {
                // There is at least one worker per P, so normally there are
                // enough workers to run on all Ps, if necessary. However, once
                // a worker enters gcMarkDone it may park without rejoining the
                // pool, thus freeing a P with no corresponding worker.
                // gcMarkDone never depends on another worker doing work, so it
                // is safe to simply do nothing here.
                //
                // If gcMarkDone bails out without completing the mark phase,
                // it will always do so with queued global work. Thus, that P
                // will be immediately eligible to re-run the worker G it was
                // just using, ensuring work can complete.
        return (GoPtr::nil(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
                // There is at least one worker per P, so normally there are
                // enough workers to run on all Ps, if necessary. However, once
                // a worker enters gcMarkDone it may park without rejoining the
                // pool, thus freeing a P with no corresponding worker.
                // gcMarkDone never depends on another worker doing work, so it
                // is safe to simply do nothing here.
                //
                // If gcMarkDone bails out without completing the mark phase,
                // it will always do so with queued global work. Thus, that P
                // will be immediately eligible to re-run the worker G it was
                // just using, ensuring work can complete.
        let mut decIfPositive = Arc::new(Mutex::new(Some(Box::new(move |val: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>| -> bool {
        loop {
        let mut v = { let __recv = val.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::Int64 = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::Int64 }; let __result = unsafe { &mut *__recv_ptr }.load(); __result };
        if { let __tmp_x = v; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return unimplemented!("GoPtr return requires compatible pointer value");
    }
        if { let __recv = val.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::Int64 = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::Int64 }; let __result = unsafe { &mut *__recv_ptr }.compare_and_swap(Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some({ let __tmp_x = v; let __tmp_y = 1 as i64; __tmp_x - __tmp_y })))); __result } {
        return unimplemented!("GoPtr return requires compatible pointer value");
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>) -> bool + Send + Sync>)));
        if { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>) -> bool + Send + Sync> = { let mut __f_guard = decIfPositive.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(self.dedicated_mark_workers_needed.clone()) } {
                // This P is now dedicated to marking until the end of
                // the concurrent mark phase.
        { let new_val = crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_DEDICATED_MODE as i32)))); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*self.fractional_utilization_goal.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0.0; __tmp_x == __tmp_y } {
        (*gcBgMarkWorkerPool.lock().unwrap().as_ref().unwrap()).push({ let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.node.clone()); __ptr_value }.clone());
        return (GoPtr::nil(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v });
    } else {
        let mut delta = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.mark_start_time.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } && { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_fractional_mark_time.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*delta.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; let __tmp_y = (*self.fractional_utilization_goal.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        (*gcBgMarkWorkerPool.lock().unwrap().as_ref().unwrap()).push({ let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.node.clone()); __ptr_value }.clone());
        return (GoPtr::nil(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
        { let new_val = crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_FRACTIONAL_MODE as i32)))); *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_mark_worker_mode.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
                // This P is now dedicated to marking until the end of
                // the concurrent mark phase.
                // No need for fractional workers.
                // Is this P behind on the fractional utilization
                // goal?
                //
                // This should be kept in sync with pollFractionalWorkerExit.
                // Nope. No need to run a fractional worker.
                // Run a fractional worker.
                // Run the background mark worker.
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*{ let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.gp.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
        let mut trace_local = trace_acquire();
        casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNABLE as u32))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).go_unpark(gp.clone(), Arc::new(Mutex::new(Some(0))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        (gp.clone(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v })
    }

    /// resetLive sets up the controller state for the next mark phase after the end
    /// of the previous one. Must be called after endCycle and before commit, before
    /// the world is started.
    ///
    /// The world must be stopped.
    pub fn reset_live(&mut self, bytesMarked: Arc<Mutex<Option<u64>>>) {
        { let new_val = bytesMarked.lock().unwrap().as_ref().unwrap().clone(); *self.heap_marked.lock().unwrap() = Some(new_val); };
        (*self.heap_live.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = bytesMarked.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*self.heap_scan.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some((*self.heap_scan_work.lock().unwrap().as_mut().unwrap()).load() as u64))));
        { let new_val = Arc::new(Mutex::new(Some((*self.heap_scan_work.lock().unwrap().as_mut().unwrap()).load() as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.last_heap_scan.lock().unwrap() = __moved_val; };
        (*self.last_stack_scan.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some((*self.stack_scan_work.lock().unwrap().as_mut().unwrap()).load() as u64))));
        { let new_val = !(0 as u64) as u64; *self.triggered.lock().unwrap() = Some(new_val); };
                // heapLive was updated, so emit a trace event.
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).heap_alloc(Arc::new(Mutex::new(Some({ let __arg_holder = bytesMarked.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }

    /// markWorkerStop must be called whenever a mark worker stops executing.
    ///
    /// It updates mark work accounting in the controller by a duration of
    /// work in nanoseconds and other bookkeeping.
    ///
    /// Safe to execute at any time.
    pub fn mark_worker_stop(&self, mode: Arc<Mutex<Option<gcMarkWorkerMode>>>, duration: Arc<Mutex<Option<i64>>>) {
        { let _switch_val = (*mode.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_DEDICATED_MODE as i32))))) {
            (*self.dedicated_mark_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = duration.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            (*self.dedicated_mark_workers_needed.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i64))));
        } else if _switch_val == (crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_FRACTIONAL_MODE as i32))))) {
            (*self.fractional_mark_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = duration.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        } else if _switch_val == (crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_IDLE_MODE as i32))))) {
            (*self.idle_mark_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = duration.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            self.remove_idle_mark_worker();
        } else {
            throw(Arc::new(Mutex::new(Some("markWorkerStop: unknown mark worker mode".to_string()))));
        }
    }
    }

    pub fn update(&self, dHeapLive: Arc<Mutex<Option<i64>>>, dHeapScan: Arc<Mutex<Option<i64>>>) {
        if { let __tmp_x = { let __v = (*dHeapLive.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        let mut trace_local = trace_acquire();
        let mut live = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_live.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = dHeapLive.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
                // gcController.heapLive changed.
        (*trace_local.lock().unwrap().as_ref().unwrap()).heap_alloc(Arc::new(Mutex::new(Some(live))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
                // gcController.heapLive changed.
        if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // Update heapScan when we're not in a current GC. It is fixed
                // at the beginning of a cycle.
        if { let __tmp_x = { let __v = (*dHeapScan.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_scan.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = dHeapScan.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    } else {
                // gcController.heapLive changed.
        self.revise();
    }
    }

    pub fn add_scannable_stack(&self, pp: GoPtr<crate::runtime2::p>, amount: Arc<Mutex<Option<i64>>>) {
        if pp.is_nil() {
        (*self.max_stack_scan.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = amount.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        { let __target = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.max_stack_scan_delta.clone()); __ptr_value }.clone(); let __rhs = (*amount.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().max_stack_scan_delta.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_STACK_SCAN_SLACK as i64; __tmp_x >= __tmp_y } || { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().max_stack_scan_delta.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = -((MAX_STACK_SCAN_SLACK as i64)) as i64; __tmp_x <= __tmp_y } {
        (*self.max_stack_scan.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.max_stack_scan_delta.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as i64; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.max_stack_scan_delta.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
    }

    pub fn add_globals(&self, amount: Arc<Mutex<Option<i64>>>) {
        (*self.globals_scan.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = amount.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// heapGoal returns the current heap goal.
    pub fn heap_goal(&self) -> u64 {
        let (mut goal, _) = self.heap_goal_internal();
        goal
    }

    /// heapGoalInternal is the implementation of heapGoal which returns additional
    /// information that is necessary for computing the trigger.
    ///
    /// The returned minTrigger is always <= goal.
    pub fn heap_goal_internal(&self) -> (u64, u64) {
    let mut goal: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut minTrigger: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

                // Start with the goal calculated for gcPercent.
        { let new_val = (*self.gc_percent_heap_goal.lock().unwrap().as_mut().unwrap()).load(); *goal.lock().unwrap() = Some(new_val); };
                // Check if the memory-limit-based goal is smaller, and if so, pick that.
        {
        let mut newGoal = self.memory_limit_heap_goal();;
        if { let __tmp_x = newGoal; let __tmp_y = { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            { let new_val = newGoal; *goal.lock().unwrap() = Some(new_val); };;
        } else {
            let mut sweepDistTrigger = (*self.sweep_dist_min_trigger.lock().unwrap().as_mut().unwrap()).load();;
            if { let __tmp_x = sweepDistTrigger; let __tmp_y = { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = sweepDistTrigger; *goal.lock().unwrap() = Some(new_val); };
    };
            { let new_val = sweepDistTrigger; *minTrigger.lock().unwrap() = Some(new_val); };;
            const minRunway: i32 = 64 << 10;
;
            if { let __tmp_x = (*self.triggered.lock().unwrap().as_ref().unwrap()); let __tmp_y = !(0 as u64) as u64; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*self.triggered.lock().unwrap().as_ref().unwrap()); let __tmp_y = minRunway as u64; __tmp_x + __tmp_y }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = (*self.triggered.lock().unwrap().as_ref().unwrap()); let __tmp_y = minRunway as u64; __tmp_x + __tmp_y }; *goal.lock().unwrap() = Some(new_val); };
    };
        }
    }
                // We're not limited by the memory limit goal, so perform a series of
                // adjustments that might move the goal forward in a variety of circumstances.
                // Set the goal to maintain a minimum sweep distance since
                // the last call to commit. Note that we never want to do this
                // if we're in the memory limit regime, because it could push
                // the goal up.
                // Since we ignore the sweep distance trigger in the memory
                // limit regime, we need to ensure we don't propagate it to
                // the trigger, because it could cause a violation of the
                // invariant that the trigger < goal.
                // Ensure that the heap goal is at least a little larger than
                // the point at which we triggered. This may not be the case if GC
                // start is delayed or if the allocation that pushed gcController.heapLive
                // over trigger is large or if the trigger is really close to
                // GOGC. Assist is proportional to this distance, so enforce a
                // minimum distance, even if it means going over the GOGC goal
                // by a tiny bit.
                //
                // Ignore this if we're in the memory limit regime: we'd prefer to
                // have the GC respond hard about how close we are to the goal than to
                // push the goal back in such a manner that it could cause us to exceed
                // the memory limit.
        return ((*goal.lock().unwrap().as_ref().unwrap()), (*minTrigger.lock().unwrap().as_ref().unwrap()));
    }

    /// memoryLimitHeapGoal returns a heap goal derived from memoryLimit.
    pub fn memory_limit_heap_goal(&self) -> u64 {
                // Start by pulling out some values we'll need. Be careful about overflow.
        let mut heapFree: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut heapAlloc: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut mappedReady: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        loop {
        { let new_val = (*self.heap_free.lock().unwrap().as_ref().unwrap()).load(); *heapFree.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = (*self.total_alloc.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = (*self.total_free.lock().unwrap().as_mut().unwrap()).load(); __tmp_x - __tmp_y }; *heapAlloc.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.mapped_ready.lock().unwrap().as_mut().unwrap()).load(); *mappedReady.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*heapFree.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*heapAlloc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*mappedReady.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        break
    }
    }
                // Free and unscavenged memory.
                // Heap object bytes in use.
                // Total unreleased mapped memory.
                // It is impossible for total unreleased mapped memory to exceed heap memory, but
                // because these stats are updated independently, we may observe a partial update
                // including only some values. Thus, we appear to break the invariant. However,
                // this condition is necessarily transient, so just try again. In the case of a
                // persistent accounting error, we'll deadlock here.
                // Below we compute a goal from memoryLimit. There are a few things to be aware of.
                // Firstly, the memoryLimit does not easily compare to the heap goal: the former
                // is total mapped memory by the runtime that hasn't been released, while the latter is
                // only heap object memory. Intuitively, the way we convert from one to the other is to
                // subtract everything from memoryLimit that both contributes to the memory limit (so,
                // ignore scavenged memory) and doesn't contain heap objects. This isn't quite what
                // lines up with reality, but it's a good starting point.
                //
                // In practice this computation looks like the following:
                //
                //    goal := memoryLimit - ((mappedReady - heapFree - heapAlloc) + max(mappedReady - memoryLimit, 0))
                //                    ^1                                    ^2
                //    goal -= goal / 100 * memoryLimitHeapGoalHeadroomPercent
                //    ^3
                //
                // Let's break this down.
                //
                // The first term (marker 1) is everything that contributes to the memory limit and isn't
                // or couldn't become heap objects. It represents, broadly speaking, non-heap overheads.
                // One oddity you may have noticed is that we also subtract out heapFree, i.e. unscavenged
                // memory that may contain heap objects in the future.
                //
                // Let's take a step back. In an ideal world, this term would look something like just
                // the heap goal. That is, we "reserve" enough space for the heap to grow to the heap
                // goal, and subtract out everything else. This is of course impossible; the definition
                // is circular! However, this impossible definition contains a key insight: the amount
                // we're *going* to use matters just as much as whatever we're currently using.
                //
                // Consider if the heap shrinks to 1/10th its size, leaving behind lots of free and
                // unscavenged memory. mappedReady - heapAlloc will be quite large, because of that free
                // and unscavenged memory, pushing the goal down significantly.
                //
                // heapFree is also safe to exclude from the memory limit because in the steady-state, it's
                // just a pool of memory for future heap allocations, and making new allocations from heapFree
                // memory doesn't increase overall memory use. In transient states, the scavenger and the
                // allocator actively manage the pool of heapFree memory to maintain the memory limit.
                //
                // The second term (marker 2) is the amount of memory we've exceeded the limit by, and is
                // intended to help recover from such a situation. By pushing the heap goal down, we also
                // push the trigger down, triggering and finishing a GC sooner in order to make room for
                // other memory sources. Note that since we're effectively reducing the heap goal by X bytes,
                // we're actually giving more than X bytes of headroom back, because the heap goal is in
                // terms of heap objects, but it takes more than X bytes (e.g. due to fragmentation) to store
                // X bytes worth of objects.
                //
                // The final adjustment (marker 3) reduces the maximum possible memory limit heap goal by
                // memoryLimitHeapGoalPercent. As the name implies, this is to provide additional headroom in
                // the face of pacing inaccuracies, and also to leave a buffer of unscavenged memory so the
                // allocator isn't constantly scavenging. The reduction amount also has a fixed minimum
                // (memoryLimitMinHeapGoalHeadroom, not pictured) because the aforementioned pacing inaccuracies
                // disproportionately affect small heaps: as heaps get smaller, the pacer's inputs get fuzzier.
                // Shorter GC cycles and less GC work means noisy external factors like the OS scheduler have a
                // greater impact.
        let mut memoryLimit = Arc::new(Mutex::new(Some((*self.memory_limit.lock().unwrap().as_mut().unwrap()).load() as u64)));
                // Compute term 1.
        let mut nonHeapMemory = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*mappedReady.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*heapFree.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*heapAlloc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
                // Compute term 2.
        let mut overage: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __v = (*mappedReady.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*memoryLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*mappedReady.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*memoryLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *overage.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*nonHeapMemory.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*overage.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*memoryLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // We're at a point where non-heap memory exceeds the memory limit on its own.
                // There's honestly not much we can do here but just trigger GCs continuously
                // and let the CPU limiter reign that in. Something has to give at this point.
                // Set it to heapMarked, the lowest possible goal.
        return (*self.heap_marked.lock().unwrap().as_ref().unwrap());
    }
                // We're at a point where non-heap memory exceeds the memory limit on its own.
                // There's honestly not much we can do here but just trigger GCs continuously
                // and let the CPU limiter reign that in. Something has to give at this point.
                // Set it to heapMarked, the lowest possible goal.
                // Compute the goal.
        let mut goal = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*memoryLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*nonHeapMemory.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*overage.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); __tmp_x - __tmp_y })));
                // Apply some headroom to the goal to account for pacing inaccuracies and to reduce
                // the impact of scavenging at allocation time in response to a high allocation rate
                // when GOGC=off. See issue #57069. Also, be careful about small limits.
        let mut headroom = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100 as u64; __tmp_x / __tmp_y }; let __tmp_y = MEMORY_LIMIT_HEAP_GOAL_HEADROOM_PERCENT as u64; __tmp_x * __tmp_y })));
        if { let __tmp_x = { let __v = (*headroom.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MEMORY_LIMIT_MIN_HEAP_GOAL_HEADROOM as u64; __tmp_x < __tmp_y } {
                // Set a fixed minimum to deal with the particularly large effect pacing inaccuracies
                // have for smaller heaps.
        { let new_val = MEMORY_LIMIT_MIN_HEAP_GOAL_HEADROOM as u64; *headroom.lock().unwrap() = Some(new_val); };
    }
                // Set a fixed minimum to deal with the particularly large effect pacing inaccuracies
                // have for smaller heaps.
        if { let __tmp_x = { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*headroom.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*headroom.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*headroom.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = headroom.lock().unwrap().as_ref().unwrap().clone(); *goal.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*headroom.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *goal.lock().unwrap() = Some(new_val); };
    }
                // Don't let us go below the live heap. A heap goal below the live heap doesn't make sense.
        if { let __tmp_x = { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = self.heap_marked.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *goal.lock().unwrap() = Some(new_val); };
    }
        return { let __v = (*goal.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// trigger returns the current point at which a GC should trigger along with
    /// the heap goal.
    ///
    /// The returned value may be compared against heapLive to determine whether
    /// the GC should trigger. Thus, the GC trigger condition should be (but may
    /// not be, in the case of small movements for efficiency) checked whenever
    /// the heap goal may change.
    pub fn trigger(&self) -> (u64, u64) {
        let (mut goal, mut minTrigger) = self.heap_goal_internal();
                // Invariant: the trigger must always be less than the heap goal.
                //
                // Note that the memory limit sets a hard maximum on our heap goal,
                // but the live heap may grow beyond it.
        if { let __tmp_x = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); let __tmp_y = goal; __tmp_x >= __tmp_y } {
                // The goal should never be smaller than heapMarked, but let's be
                // defensive about it. The only reasonable trigger here is one that
                // causes a continuous GC cycle at heapMarked, but respect the goal
                // if it came out as smaller than that.
        return (goal, goal);
    }
                // The goal should never be smaller than heapMarked, but let's be
                // defensive about it. The only reasonable trigger here is one that
                // causes a continuous GC cycle at heapMarked, but respect the goal
                // if it came out as smaller than that.
                // Below this point, c.heapMarked < goal.
                // heapMarked is our absolute minimum, and it's possible the trigger
                // bound we get from heapGoalinternal is less than that.
        if { let __tmp_x = minTrigger; let __tmp_y = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = { let __v = self.heap_marked.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }; minTrigger = new_val; };
    }
                // If we let the trigger go too low, then if the application
                // is allocating very rapidly we might end up in a situation
                // where we're allocating black during a nearly always-on GC.
                // The result of this is a growing heap and ultimately an
                // increase in RSS. By capping us at a point >0, we're essentially
                // saying that we're OK using more CPU during the GC to prevent
                // this growth in RSS.
        let mut triggerLowerBound = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = goal; let __tmp_y = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = TRIGGER_RATIO_DEN as u64; __tmp_x / __tmp_y }); let __tmp_y = MIN_TRIGGER_RATIO_NUM as u64; __tmp_x * __tmp_y }; let __tmp_y = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = minTrigger; let __tmp_y = { let __v = (*triggerLowerBound.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = (*triggerLowerBound.lock().unwrap().as_ref().unwrap()); minTrigger = new_val; };
    }
                // For small heaps, set the max trigger point at maxTriggerRatio of the way
                // from the live heap to the heap goal. This ensures we always have *some*
                // headroom when the GC actually starts. For larger heaps, set the max trigger
                // point at the goal, minus the minimum heap size.
                //
                // This choice follows from the fact that the minimum heap size is chosen
                // to reflect the costs of a GC with no work to do. With a large heap but
                // very little scan work to perform, this gives us exactly as much runway
                // as we would need, in the worst case.
        let mut maxTrigger = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = goal; let __tmp_y = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = TRIGGER_RATIO_DEN as u64; __tmp_x / __tmp_y }); let __tmp_y = MAX_TRIGGER_RATIO_NUM as u64; __tmp_x * __tmp_y }; let __tmp_y = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = goal; let __tmp_y = DEFAULT_HEAP_MINIMUM as u64; __tmp_x > __tmp_y } && { let __tmp_x = { let __tmp_x = goal; let __tmp_y = DEFAULT_HEAP_MINIMUM as u64; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*maxTrigger.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = goal; let __tmp_y = DEFAULT_HEAP_MINIMUM as u64; __tmp_x - __tmp_y }; *maxTrigger.lock().unwrap() = Some(new_val); };
    }
        { let new_val = std::cmp::max(({ let __v = (*maxTrigger.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64), (minTrigger as u64)); *maxTrigger.lock().unwrap() = Some(new_val); };
                // Compute the trigger from our bounds and the runway stored by commit.
        let mut trigger: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        let mut runway = (*self.runway.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = runway; let __tmp_y = goal; __tmp_x > __tmp_y } {
        { let new_val = minTrigger; *trigger.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = goal; let __tmp_y = runway; __tmp_x - __tmp_y }; *trigger.lock().unwrap() = Some(new_val); };
    }
        { let new_val = std::cmp::max(({ let __v = (*trigger.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64), (minTrigger as u64)); *trigger.lock().unwrap() = Some(new_val); };
        { let new_val = std::cmp::min(({ let __v = (*trigger.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64), ({ let __v = (*maxTrigger.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64)); *trigger.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*trigger.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = goal; __tmp_x > __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "trigger=".to_string()), format!("{}", { let __v = (*trigger.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " heapGoal=".to_string()), format!("{}", goal), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}", format!("{}", "minTrigger=".to_string()), format!("{}", minTrigger), format!("{}", " maxTrigger=".to_string()), format!("{}", { let __v = (*maxTrigger.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("produced a trigger greater than the heap goal".to_string()))));
    }
        return ({ let __v = (*trigger.lock().unwrap().as_ref().unwrap()).clone(); __v }, goal);
    }

    /// commit recomputes all pacing parameters needed to derive the
    /// trigger and the heap goal. Namely, the gcPercent-based heap goal,
    /// and the amount of runway we want to give the GC this cycle.
    ///
    /// This can be called any time. If GC is the in the middle of a
    /// concurrent phase, it will adjust the pacing of that phase.
    ///
    /// isSweepDone should be the result of calling isSweepDone(),
    /// unless we're testing or we know we're executing during a GC cycle.
    ///
    /// This depends on gcPercent, gcController.heapMarked, and
    /// gcController.heapLive. These must be up to date.
    ///
    /// Callers must call gcControllerState.revise after calling this
    /// function if the GC is enabled.
    ///
    /// mheap_.lock must be held or the world must be stopped.
    pub fn commit(&self, isSweepDone: Arc<Mutex<Option<bool>>>) {
        if !(*self.test.clone().lock().unwrap().as_ref().unwrap()) {
        assert_world_stopped_or_lock_held((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone());
    }
        if { let __v = (*isSweepDone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // The sweep is done, so there aren't any restrictions on the trigger
                // we need to think about.
        (*self.sweep_dist_min_trigger.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u64))));
    } else {
                // Concurrent sweep happens in the heap growth
                // from gcController.heapLive to trigger. Make sure we
                // give the sweeper some runway if it doesn't have enough.
        (*self.sweep_dist_min_trigger.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __tmp_x = (*self.heap_live.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = SWEEP_MIN_HEAP_DISTANCE as u64; __tmp_x + __tmp_y }))));
    }
                // The sweep is done, so there aren't any restrictions on the trigger
                // we need to think about.
                // Concurrent sweep happens in the heap growth
                // from gcController.heapLive to trigger. Make sure we
                // give the sweeper some runway if it doesn't have enough.
                // Compute the next GC goal, which is when the allocated heap
                // has grown by GOGC/100 over where it started the last cycle,
                // plus additional runway for non-heap sources of GC work.
        let mut gcPercentHeapGoal = Arc::new(Mutex::new(Some(!0 as u64)));
        {
        let mut gcPercent = (*self.gc_percent.lock().unwrap().as_mut().unwrap()).load();;
        if { let __tmp_x = gcPercent; let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
            { let new_val = { let __tmp_x = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = (*self.heap_marked.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.last_stack_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }; let __tmp_y = (*self.globals_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some(gcPercent as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let __tmp_y = 100 as u64; __tmp_x / __tmp_y }; __tmp_x + __tmp_y }; *gcPercentHeapGoal.lock().unwrap() = Some(new_val); };;
        }
    }
                // Apply the minimum heap size here. It's defined in terms of gcPercent
                // and is only updated by functions that call commit.
        if { let __tmp_x = { let __v = (*gcPercentHeapGoal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.heap_minimum.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = self.heap_minimum.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *gcPercentHeapGoal.lock().unwrap() = Some(new_val); };
    }
        (*self.gc_percent_heap_goal.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = gcPercentHeapGoal.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Compute the amount of runway we want the GC to have by using our
                // estimate of the cons/mark ratio.
                //
                // The idea is to take our expected scan work, and multiply it by
                // the cons/mark ratio to determine how long it'll take to complete
                // that scan work in terms of bytes allocated. This gives us our GC's
                // runway.
                //
                // However, the cons/mark ratio is a ratio of rates per CPU-second, but
                // here we care about the relative rates for some division of CPU
                // resources among the mutator and the GC.
                //
                // To summarize, we have B / cpu-ns, and we want B / ns. We get that
                // by multiplying by our desired division of CPU resources. We choose
                // to express CPU resources as GOMAPROCS*fraction. Note that because
                // we're working with a ratio here, we can omit the number of CPU cores,
                // because they'll appear in the numerator and denominator and cancel out.
                // As a result, this is basically just "weighing" the cons/mark ratio by
                // our desired division of resources.
                //
                // Furthermore, by setting the runway so that CPU resources are divided
                // this way, assuming that the cons/mark ratio is correct, we make that
                // division a reality.
        (*self.runway.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = (*self.cons_mark.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0.75; __tmp_x * __tmp_y }; let __tmp_y = 0.25; __tmp_x / __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*self.last_heap_scan.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.last_stack_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }; let __tmp_y = (*self.globals_scan.lock().unwrap().as_mut().unwrap()).load(); __tmp_x + __tmp_y }) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as u64))));
    }

    /// setGCPercent updates gcPercent. commit must be called after.
    /// Returns the old value of gcPercent.
    ///
    /// The world must be stopped, or mheap_.lock must be held.
    pub fn set_g_c_percent(&mut self, mut r#in: Arc<Mutex<Option<i32>>>) -> i32 {
        if !(*self.test.clone().lock().unwrap().as_ref().unwrap()) {
        assert_world_stopped_or_lock_held((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone());
    }
        let mut out = (*self.gc_percent.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = { let __v = (*r#in.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        { let new_val = -1 as i32; *r#in.lock().unwrap() = Some(new_val); };
    }
        { let new_val = { let __tmp_x = { let __tmp_x = DEFAULT_HEAP_MINIMUM as u64; let __tmp_y = (*Arc::new(Mutex::new(Some((*r#in.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let __tmp_y = 100 as u64; __tmp_x / __tmp_y }; *self.heap_minimum.lock().unwrap() = Some(new_val); };
        (*self.gc_percent.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = r#in.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        out
    }

    /// setMemoryLimit updates memoryLimit. commit must be called after
    /// Returns the old value of memoryLimit.
    ///
    /// The world must be stopped, or mheap_.lock must be held.
    pub fn set_memory_limit(&self, r#in: Arc<Mutex<Option<i64>>>) -> i64 {
        if !(*self.test.clone().lock().unwrap().as_ref().unwrap()) {
        assert_world_stopped_or_lock_held((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone());
    }
        let mut out = (*self.memory_limit.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = { let __v = (*r#in.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        (*self.memory_limit.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = r#in.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        out
    }

    /// addIdleMarkWorker attempts to add a new idle mark worker.
    ///
    /// If this returns true, the caller must become an idle mark worker unless
    /// there's no background mark worker goroutines in the pool. This case is
    /// harmless because there are already background mark workers running.
    /// If this returns false, the caller must NOT become an idle mark worker.
    ///
    /// nosplit because it may be called without a P.
    ///
    ///go:nosplit
    pub fn add_idle_mark_worker(&self) -> bool {
        loop {
        let mut old = (*self.idle_mark_workers.lock().unwrap().as_mut().unwrap()).load();
        let (mut n, mut max) = (Arc::new(Mutex::new(Some(({ let __tmp_x = old; let __tmp_y = (*Arc::new(Mutex::new(Some(!(((0 as u32) as u32)) as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x & __tmp_y }) as i32))), Arc::new(Mutex::new(Some(({ let __tmp_x = old; let __tmp_y = 32; __tmp_x >> __tmp_y }) as i32))));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // See the comment on idleMarkWorkers for why
                // n > max is tolerated.
        return false;
    }
                // See the comment on idleMarkWorkers for why
                // n > max is tolerated.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "n=".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " max=".to_string()), format!("{}", { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("negative idle mark workers".to_string()))));
    }
        let mut new = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i32; __tmp_x + __tmp_y }) as u32 as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*max.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }); __tmp_x | __tmp_y })));
        if (*self.idle_mark_workers.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    }
    }
    }

    /// needIdleMarkWorker is a hint as to whether another idle mark worker is needed.
    ///
    /// The caller must still call addIdleMarkWorker to become one. This is mainly
    /// useful for a quick check before an expensive operation.
    ///
    /// nosplit because it may be called without a P.
    ///
    ///go:nosplit
    pub fn need_idle_mark_worker(&self) -> bool {
        let mut p = (*self.idle_mark_workers.lock().unwrap().as_mut().unwrap()).load();
        let (mut n, mut max) = (Arc::new(Mutex::new(Some(({ let __tmp_x = p; let __tmp_y = (*Arc::new(Mutex::new(Some(!(((0 as u32) as u32)) as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x & __tmp_y }) as i32))), Arc::new(Mutex::new(Some(({ let __tmp_x = p; let __tmp_y = 32; __tmp_x >> __tmp_y }) as i32))));
        return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
    }

    /// removeIdleMarkWorker must be called when a new idle mark worker stops executing.
    pub fn remove_idle_mark_worker(&self) {
        loop {
        let mut old = (*self.idle_mark_workers.lock().unwrap().as_mut().unwrap()).load();
        let (mut n, mut max) = (Arc::new(Mutex::new(Some(({ let __tmp_x = old; let __tmp_y = (*Arc::new(Mutex::new(Some(!(((0 as u32) as u32)) as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x & __tmp_y }) as i32))), Arc::new(Mutex::new(Some(({ let __tmp_x = old; let __tmp_y = 32; __tmp_x >> __tmp_y }) as i32))));
        if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i32; __tmp_x - __tmp_y }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "n=".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " max=".to_string()), format!("{}", { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("negative idle mark workers".to_string()))));
    }
        let mut new = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i32; __tmp_x - __tmp_y }) as u32 as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*max.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }); __tmp_x | __tmp_y })));
        if (*self.idle_mark_workers.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
    }

    /// setMaxIdleMarkWorkers sets the maximum number of idle mark workers allowed.
    ///
    /// This method is optimistic in that it does not wait for the number of
    /// idle mark workers to reduce to max before returning; it assumes the workers
    /// will deschedule themselves.
    pub fn set_max_idle_mark_workers(&self, max: Arc<Mutex<Option<i32>>>) {
        loop {
        let mut old = (*self.idle_mark_workers.lock().unwrap().as_mut().unwrap()).load();
        let mut n = Arc::new(Mutex::new(Some(({ let __tmp_x = old; let __tmp_y = (*Arc::new(Mutex::new(Some(!(((0 as u32) as u32)) as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x & __tmp_y }) as i32)));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "n=".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " max=".to_string()), format!("{}", { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("negative idle mark workers".to_string()))));
    }
        let mut new = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u32 as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*max.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }); __tmp_x | __tmp_y })));
        if (*self.idle_mark_workers.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
    }
}

/// gcControllerCommit is gcController.commit, but passes arguments from live
/// (non-test) data. It also updates any consumers of the GC pacing, such as
/// sweep pacing and the background scavenger.
///
/// Calls gcController.commit.
///
/// The heap lock must be held, so this must be executed on the system stack.
///
///go:systemstack
pub fn gc_controller_commit() {
    assert_world_stopped_or_lock_held((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone());

    (*gcController.lock().unwrap().as_ref().unwrap()).commit(Arc::new(Mutex::new(Some(is_sweep_done()))));

        // Update mark pacing.
    if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x != __tmp_y } {
        (*gcController.lock().unwrap().as_ref().unwrap()).revise();
    }

        // TODO(mknyszek): This isn't really accurate any longer because the heap
        // goal is computed dynamically. Still useful to snapshot, but not as useful.
    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).heap_goal();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    let (mut trigger, mut heapGoal) = (*gcController.lock().unwrap().as_ref().unwrap()).trigger();
    gc_pace_sweeper(Arc::new(Mutex::new(Some(trigger))));
    gc_pace_scavenger(Arc::new(Mutex::new(Some((*(*gcController.lock().unwrap().as_ref().unwrap()).memory_limit.lock().unwrap().as_mut().unwrap()).load()))), Arc::new(Mutex::new(Some(heapGoal))), Arc::new(Mutex::new(Some({ let __selector_holder = (*gcController.lock().unwrap().as_ref().unwrap()).last_heap_goal.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for gcControllerState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
