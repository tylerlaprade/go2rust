use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{panic::{throw}, proc::{readgstatus}, runtime1::{debug}, runtime2::{__GSCAN, g, gobuf, m, muintptr, waitReason}, stubs::{getg, noescape}, stubs_arm64::{getfp}, symtab::{Frame, Frames, callers_frames, findfunc, funcInfo, funcdata, moduledata, srcFunc}, symtabinl::{inlineFrame, inlineUnwinder, new_inline_unwinder}, trace::{trace}, traceback::{callers_1, elide_wrapper_calling, gStatusStrings, gcallers}, tracebuf::{TRACE_BYTES_PER_NUMBER, traceWriter, unsafe_trace_writer}, traceevent::{TRACE_EV_STACK, TRACE_EV_STACKS}, tracemap::{traceMap, traceMapNode}, tracestatus::{TRACE_GO_RUNNING, TRACE_GO_SYSCALL, go_status_to_trace_go_status, traceGoStatus}, tracestring::{traceStringTable}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const TRACE_STACK_SIZE: i32 = 128;
pub(crate) const LOGICAL_STACK_SENTINEL: usize = !(0 as usize);


/// traceStackTable maps stack traces (arrays of PC's) to unique uint32 ids.
/// It is lock-free for reading.
#[derive(Clone)]
pub struct traceStackTable {
    pub tab: Arc<Mutex<Option<traceMap>>>,
}

impl traceStackTable {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            tab: __go_clone_0_0,
        }
    }
}


impl Default for traceStackTable {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(traceMap::default())));
        Self {
            tab: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for traceStackTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.tab.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for traceStackTable {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct traceFrame {
    pub p_c: Arc<Mutex<Option<usize>>>,
    pub func_i_d: Arc<Mutex<Option<u64>>>,
    pub file_i_d: Arc<Mutex<Option<u64>>>,
    pub line: Arc<Mutex<Option<u64>>>,
}

impl traceFrame {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.func_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.file_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            p_c: __go_clone_0_0,
            func_i_d: __go_clone_1_0,
            file_i_d: __go_clone_2_0,
            line: __go_clone_3_0,
        }
    }
}


impl Default for traceFrame {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            p_c: __go_default_0_0,
            func_i_d: __go_default_1_0,
            file_i_d: __go_default_2_0,
            line: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for traceFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p_c.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.func_i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.file_i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.line.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for traceFrame {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("PC") {
            out.p_c = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl traceStackTable {
    /// put returns a unique id for the stack trace pcs and caches it in the table,
    /// if it sees the trace for the first time.
    pub fn put(&self, pcs: Arc<Mutex<Option<Vec<usize>>>>) -> u64 {
        if { let __tmp_x = ((*pcs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
        let (mut id, _) = (*self.tab.lock().unwrap().as_ref().unwrap()).put(noescape(Arc::new(Mutex::new(Some({ let __seq_holder = pcs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize })))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*pcs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<usize>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y }))));
        id
    }

    /// dump writes all previously cached stacks to trace buffers,
    /// releases all memory and resets state. It must only be called once the caller
    /// can guarantee that there are no more writers to the table.
    pub fn dump(&self, gen: Arc<Mutex<Option<usize>>>) {
        let mut stackBuf = Arc::new(Mutex::new(Some(vec![0; (TRACE_STACK_SIZE) as usize])));
        let mut w = unsafe_trace_writer(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)));
        {
        let mut root: GoPtr<crate::tracemap::traceMapNode> = GoPtr::raw({ let __ptr = (*(*self.tab.lock().unwrap().as_ref().unwrap()).root.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });;
        if !root.is_nil() {
            { let new_val = dump_stacks_rec(root.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), stackBuf.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_val; };;
        }
    }
        { let __recv = (*w.lock().unwrap().as_ref().unwrap()).flush(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
        (*self.tab.lock().unwrap().as_ref().unwrap()).reset();
    }
}

/// traceStack captures a stack trace from a goroutine and registers it in the trace
/// stack table. It then returns its unique ID. If gp == nil, then traceStack will
/// attempt to use the current execution context.
///
/// skip controls the number of leaf frames to omit in order to hide tracer internals
/// from stack traces, see CL 5523.
///
/// Avoid calling this function directly. gen needs to be the current generation
/// that this stack trace is being written out for, which needs to be synchronized with
/// generations moving forward. Prefer traceEventWriter.stack.
pub fn trace_stack(skip: Arc<Mutex<Option<i32>>>, mut gp: GoPtr<crate::runtime2::g>, gen: Arc<Mutex<Option<usize>>>) -> u64 {
    let mut pcBuf: Arc<Mutex<Option<[usize; 128]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));

        // Figure out gp and mp for the backtrace.
    let mut mp: GoPtr<crate::runtime2::m> = GoPtr::nil();
    if gp.is_nil() {
        mp = GoPtr::local((*getg().lock().unwrap().as_ref().unwrap()).m.clone());
        gp = { let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().curg.clone(); __field_value };
    }

        // Double-check that we own the stack we're about to trace.
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).trace_check_stack_ownership.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && !gp.is_nil() {
        let mut status = readgstatus(gp.clone());
                // If the scan bit is set, assume we're the ones that acquired it.
        if { let __tmp_x = { let __tmp_x = status; let __tmp_y = __GSCAN as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // Use the trace status to check this. There are a number of cases
                // where a running goroutine might be in _Gwaiting, and these cases
                // are totally fine for taking a stack trace. They're captured
                // correctly in goStatusToTraceGoStatus.
        '__go_switch_1: loop {
        {
        let _switch_val = { let __v = go_status_to_trace_go_status(Arc::new(Mutex::new(Some(status))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8)))) || _switch_val == crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __left_addr = { let __ptr = GoPtr::local(getg()); __ptr.addr() }; let __right_addr = gp.addr(); let __eq = __left_addr == __right_addr; __eq } || { let __left_addr = { let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().curg.clone(); __field_value }.addr(); let __right_addr = gp.addr(); let __eq = __left_addr == __right_addr; __eq } {
        break '__go_switch_1
    }
            _fallthrough = true;
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            eprint!("{}{}{}{}{}{}{}", format!("{}", "runtime: gp=".to_string()), format!("{}", (*Arc::new(Mutex::new(Some(gp.addr()))).lock().unwrap().as_ref().unwrap())), format!("{}", " gp.goid=".to_string()), format!("{}", (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", " status=".to_string()), format!("{}", { let __seq = { let __seq_holder = gStatusStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(status) as usize].clone() }), format!("{}", "\n".to_string()));
            throw(Arc::new(Mutex::new(Some("attempted to trace stack of a goroutine this thread does not own".to_string()))));
        }
    };
        break;
    }
    }
    }

        // If the scan bit is set, assume we're the ones that acquired it.
        // Use the trace status to check this. There are a number of cases
        // where a running goroutine might be in _Gwaiting, and these cases
        // are totally fine for taking a stack trace. They're captured
        // correctly in goStatusToTraceGoStatus.
    if !gp.is_nil() && mp.is_nil() {
                // We're getting the backtrace for a G that's not currently executing.
                // It may still have an M, if it's locked to some M.
        mp = crate::runtime2::muintptr::ptr(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.lockedm.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
    }
        // We're getting the backtrace for a G that's not currently executing.
        // It may still have an M, if it's locked to some M.
    let mut nstk = Arc::new(Mutex::new(Some(1)));
    if tracefpunwindoff() || (!mp.is_nil() && { let __recv_value = mp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).has_cgo_on_stack(); __result }) {
                // Slow path: Unwind using default unwinder. Used when frame pointer
                // unwinding is unavailable or disabled (tracefpunwindoff), or might
                // produce incomplete results or crashes (hasCgoOnStack). Note that no
                // cgo callback related crashes have been observed yet. The main
                // motivation is to take advantage of a potentially registered cgo
                // symbolizer.
        (*pcBuf.lock().unwrap().as_mut().unwrap())[(0) as usize] = LOGICAL_STACK_SENTINEL as usize;
        if { let __left_addr = { let __ptr = GoPtr::local(getg()); __ptr.addr() }; let __right_addr = gp.addr(); let __eq = __left_addr == __right_addr; __eq } {
        { let __rhs = callers_1(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); let mut guard = nstk.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else if !gp.is_nil() {
        { let __rhs = gcallers(gp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); let mut guard = nstk.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    } else {
                // Fast path: Unwind using frame pointers.
        (*pcBuf.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*Arc::new(Mutex::new(Some((*skip.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()).clone();
        if { let __left_addr = { let __ptr = GoPtr::local(getg()); __ptr.addr() }; let __right_addr = gp.addr(); let __eq = __left_addr == __right_addr; __eq } {
        { let __rhs = fp_traceback_p_cs(Arc::new(Mutex::new(Some(getfp()))), Arc::new(Mutex::new(Some({ let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); let mut guard = nstk.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else if !gp.is_nil() {
        if { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().syscallsp.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        (*pcBuf.lock().unwrap().as_mut().unwrap())[(1) as usize] = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().syscallpc.clone() }.lock().unwrap().as_ref().unwrap());
        { let __rhs = { let __tmp_x = 1; let __tmp_y = fp_traceback_p_cs(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.syscallbp.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (2) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __tmp_x + __tmp_y }; let mut guard = nstk.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
        (*pcBuf.lock().unwrap().as_mut().unwrap())[(1) as usize] = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap());
        { let __rhs = { let __tmp_x = 1; let __tmp_y = fp_traceback_p_cs(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).bp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (2) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __tmp_x + __tmp_y }; let mut guard = nstk.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
    }
        // Slow path: Unwind using default unwinder. Used when frame pointer
        // unwinding is unavailable or disabled (tracefpunwindoff), or might
        // produce incomplete results or crashes (hasCgoOnStack). Note that no
        // cgo callback related crashes have been observed yet. The main
        // motivation is to take advantage of a potentially registered cgo
        // symbolizer.
        // Fast path: Unwind using frame pointers.
        // Three cases:
        //
        // (1) We're called on the g0 stack through mcall(fn) or systemstack(fn). To
        // behave like gcallers above, we start unwinding from sched.bp, which
        // points to the caller frame of the leaf frame on g's stack. The return
        // address of the leaf frame is stored in sched.pc, which we manually
        // capture here.
        //
        // (2) We're called against a gp that we're not currently executing on, but that isn't
        // in a syscall, in which case it's currently not executing. gp.sched contains the most
        // up-to-date information about where it stopped, and like case (1), we match gcallers
        // here.
        //
        // (3) We're called against a gp that we're not currently executing on, but that is in
        // a syscall, in which case gp.syscallsp != 0. gp.syscall* contains the most up-to-date
        // information about where it stopped, and like case (1), we match gcallers here.
    if { let __tmp_x = { let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = nstk.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        // skip runtime.goexit
    if { let __tmp_x = { let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x == __tmp_y } {
        { let mut guard = nstk.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        // skip runtime.main
    let mut id = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).stack_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some({ let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
    id
}

pub fn dump_stacks_rec(node: GoPtr<crate::tracemap::traceMapNode>, mut w: Arc<Mutex<Option<traceWriter>>>, stackBuf: Arc<Mutex<Option<Vec<usize>>>>) -> Arc<Mutex<Option<crate::tracebuf::traceWriter>>> {
    let mut stack = { let __go_unsafe_result: Arc<Mutex<Option<Vec<usize>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result };

        // N.B. This might allocate, but that's OK because we're not writing to the M's buffer,
        // but one we're about to create (with ensure).
    let mut n = fpunwind_expand(stackBuf.clone(), stack.clone());
    let mut frames = make_trace_frames(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*w.lock().unwrap().as_mut().unwrap()).trace_locker.lock().unwrap().as_mut().unwrap()).gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __seq_holder = stackBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (n) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));

        // The maximum number of bytes required to hold the encoded stack, given that
        // it contains N frames.
    let mut maxBytes = Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = ({ let __tmp_x = (({ let __tmp_x = 2; let __tmp_y = ({ let __tmp_x = 4; let __tmp_y = ((*frames.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x * __tmp_y } as i32); __tmp_x + __tmp_y }) as i32); let __tmp_y = 10; __tmp_x * __tmp_y } as i32); __tmp_x + __tmp_y })));

        // Estimate the size of this record. This
        // bound is pretty loose, but avoids counting
        // lots of varint sizes.
        //
        // Add 1 because we might also write traceEvStacks.
    let mut flushed: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let (__tmp_0, __tmp_1) = (*w.lock().unwrap().as_ref().unwrap()).ensure(Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = { let __v = (*maxBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_tmp_0; *flushed.lock().unwrap() = Some(__tmp_1); };
    if { let __v = (*flushed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_EV_STACKS as u8 as u8))));
    }

        // Emit stack event.
    (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_EV_STACK as u8 as u8))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some((*frames.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))));
    { let __range_holder = frames.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for frame in __range_values.iter() {
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = frame.p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = frame.func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = frame.file_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = frame.line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    } }

        // Recursively walk all child nodes.
    for i in 0..(({ let __range_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut child = { let __seq = { let __seq_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.load();
        if { let __nil_result = (*child.lock().unwrap()).is_none(); __nil_result } {
        continue
    }
        { let new_val = dump_stacks_rec(GoPtr::raw({ let __ptr = child.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }), Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), stackBuf.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_val; };
    }
    return { let __owned = w.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// makeTraceFrames returns the frames corresponding to pcs. It may
/// allocate and may emit trace events.
pub fn make_trace_frames(gen: Arc<Mutex<Option<usize>>>, pcs: Arc<Mutex<Option<Vec<usize>>>>) -> Arc<Mutex<Option<Vec<traceFrame>>>> {
    let mut frames: Arc<Mutex<Option<Vec<traceFrame>>>> = Arc::new(Mutex::new(Some(Vec::<traceFrame>::with_capacity(((*pcs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    let mut ci = callers_frames(pcs.clone());
    loop {
        let (mut f, mut more) = { let __recv = ci.clone(); let __recv_ptr: *mut crate::symtab::Frames = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::symtab::Frames }; let __result = unsafe { &mut *__recv_ptr }.next(); __result };
        { let new_val = { let __append_target = frames.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*make_trace_frame(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; frames = new_val; };
        if !more {
        return frames.clone();
    }
    }
}

/// makeTraceFrame sets up a traceFrame for a frame.
pub fn make_trace_frame(gen: Arc<Mutex<Option<usize>>>, f: Arc<Mutex<Option<Frame>>>) -> Arc<Mutex<Option<traceFrame>>> {
    let mut frame: Arc<Mutex<Option<traceFrame>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = { let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).p_c.lock().unwrap() = Some(new_val); };

    let mut r#fn = Arc::new(Mutex::new(Some({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).function.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    const maxLen: i32 = 1 << 10;

    if { let __tmp_x = ((*r#fn.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1024; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*r#fn.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = ((*r#fn.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1024; __tmp_x - __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r#fn.lock().unwrap() = __moved_val; };
    }
    { let new_val = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *(*frame.lock().unwrap().as_ref().unwrap()).func_i_d.lock().unwrap() = Some(new_val); };
    { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*frame.lock().unwrap().as_ref().unwrap()).line.lock().unwrap() = __moved_val; };
    let mut file = Arc::new(Mutex::new(Some({ let __selector_holder = (*f.lock().unwrap().as_ref().unwrap()).file.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = ((*file.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1024; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*file.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = ((*file.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1024; __tmp_x - __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *file.lock().unwrap() = __moved_val; };
    }
    { let new_val = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *(*frame.lock().unwrap().as_ref().unwrap()).file_i_d.lock().unwrap() = Some(new_val); };
    return { let __owned = frame.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// tracefpunwindoff returns true if frame pointer unwinding for the tracer is
/// disabled via GODEBUG or not supported by the architecture.
pub fn tracefpunwindoff() -> bool {
    return { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).tracefpunwindoff.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } || ({ let __tmp_x = internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::ARCH_FAMILY as i32)))); let __tmp_y = internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::A_M_D64 as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::ARCH_FAMILY as i32)))); let __tmp_y = internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::A_R_M64 as i32)))); __tmp_x != __tmp_y });
}

/// fpTracebackPCs populates pcBuf with the return addresses for each frame and
/// returns the number of PCs written to pcBuf. The returned PCs correspond to
/// "physical frames" rather than "logical frames"; that is if A is inlined into
/// B, this will return a PC for only B.
pub fn fp_traceback_p_cs(mut fp: Arc<Mutex<Option<usize>>>, pcBuf: Arc<Mutex<Option<Vec<usize>>>>) -> i32 {
    let mut i: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*pcBuf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } && { let __nil_result = (*fp.lock().unwrap()).is_some(); __nil_result } {
                // return addr sits one word above the frame pointer
        (*pcBuf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*fp.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v };

                // follow the frame pointer to the next one
        { let new_val = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = fp.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *fp.lock().unwrap() = __moved_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // return addr sits one word above the frame pointer
        // follow the frame pointer to the next one
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// fpunwindExpand expands a call stack from pcBuf into dst,
/// returning the number of PCs written to dst.
/// pcBuf and dst should not overlap.
///
/// fpunwindExpand checks if pcBuf contains logical frames (which include inlined
/// frames) or physical frames (produced by frame pointer unwinding) using a
/// sentinel value in pcBuf[0]. Logical frames are simply returned without the
/// sentinel. Physical frames are turned into logical frames via inline unwinding
/// and by applying the skip value that's stored in pcBuf[0].
pub fn fpunwind_expand(dst: Arc<Mutex<Option<Vec<usize>>>>, pcBuf: Arc<Mutex<Option<Vec<usize>>>>) -> i32 {
    if { let __tmp_x = ((*pcBuf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    } else if { let __tmp_x = ((*pcBuf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = LOGICAL_STACK_SENTINEL as usize; __tmp_x == __tmp_y } {
        return (*{ let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min((*dst.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*dst.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) }.lock().unwrap().as_ref().unwrap());
    }

        // pcBuf contains logical rather than inlined frames, skip has already been
        // applied, just return it without the sentinel value in pcBuf[0].
    let dst_closure_clone = dst.clone(); let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut lastFuncID = Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_NORMAL as u8)))))));let mut skip = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));let mut skipOrAdd = Arc::new(Mutex::new(Some({ let mut n_closure_clone = n.clone(); let mut skip_closure_clone = skip.clone(); Box::new(move |retPC: Arc<Mutex<Option<usize>>>| -> bool {
        if { let __tmp_x = { let __v = (*skip_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
        { let mut guard = skip_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    } else if { let __tmp_x = ({ let __v = (*n_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*dst_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        (*dst_closure_clone.lock().unwrap().as_mut().unwrap())[({ let __v = (*n_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*retPC.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let mut guard = n_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __tmp_x = ({ let __v = (*n_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*dst_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> })));

        // skipOrAdd skips or appends retPC to newPCBuf and returns true if more
        // pcs can be added.
    'outer: for retPC in { let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.iter().copied() {
        let mut callPC = Arc::new(Mutex::new(Some({ let __tmp_x = retPC; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        let mut fi = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = callPC.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if !(*fi.lock().unwrap().as_ref().unwrap()).valid() {
                // There is no funcInfo if callPC belongs to a C function. In this case
                // we still keep the pc, but don't attempt to expand inlined frames.
        {
        let mut more = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = skipOrAdd.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(retPC.clone())))) };;
        if !more {
            break 'outer;
        }
    }
        continue
    }
                // There is no funcInfo if callPC belongs to a C function. In this case
                // we still keep the pc, but don't attempt to expand inlined frames.
        let (mut u, mut uf) = new_inline_unwinder(Arc::new(Mutex::new(Some({ let __arg_holder = fi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = callPC.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        while (*uf.lock().unwrap().as_ref().unwrap()).valid() {
        let mut sf = (*u.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = { let __selector_holder = (*sf.lock().unwrap().as_ref().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_WRAPPER as u8)))); __tmp_x == __tmp_y } && elide_wrapper_calling(Arc::new(Mutex::new(Some({ let __arg_holder = lastFuncID.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
    } else {
        let mut more = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = skipOrAdd.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*uf.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x + __tmp_y })))) };;
        if !more {
            break 'outer;
        }
    }
                // ignore wrappers
        { let new_val = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some((*(*(*sf.lock().unwrap().as_ref().unwrap()).func_i_d.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *lastFuncID.lock().unwrap() = Some(new_val); };
        { let new_val = (*u.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *uf.lock().unwrap() = __moved_val; };
    }
    }
        // There is no funcInfo if callPC belongs to a C function. In this case
        // we still keep the pc, but don't attempt to expand inlined frames.
        // ignore wrappers
    return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// startPCForTrace returns the start PC of a goroutine for tracing purposes.
/// If pc is a wrapper, it returns the PC of the wrapped function. Otherwise it
/// returns pc.
pub fn start_p_c_for_trace(pc: Arc<Mutex<Option<usize>>>) -> usize {
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        return { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        // may happen for locked g in extra M since its pc is 0.
    let mut w = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__WRAP_INFO as u8))));
    if { let __nil_result = (*w.lock().unwrap()).is_none(); __nil_result } {
        return { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        // not a wrapper
    return (*(*f.lock().unwrap().as_ref().unwrap()).datap.lock().unwrap().as_ref().unwrap()).text_addr(Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = w.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u32>(unimplemented!("unsafe.Pointer conversion to u32")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }))));
}

impl GoValueClone for traceStackTable {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceFrame {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
