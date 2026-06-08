use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_spinbit::{lock, unlock}, mgc::{AnonymousStruct12}, os_darwin::{osyield}, panic::{throw}, profbuf::{PROF_BUF_NON_BLOCKING, profBuf, profBufReadMode}, runtime2::{g, m, mutex, p}, stubs::{systemstack}, trace::{trace, wakeableSleep}, tracebuf::{TRACE_BYTES_PER_NUMBER, traceBuf, traceWriter, trace_buf_flush, unsafe_trace_writer}, traceevent::{TRACE_EV_C_P_U_SAMPLE, TRACE_EV_C_P_U_SAMPLES}, traceruntime::{mTraceState, trace_enabled}, tracestack::{LOGICAL_STACK_SENTINEL, TRACE_STACK_SIZE, traceStackTable}, tracetime::{traceTime, trace_clock_now}};

use std::any::Any;
use std::sync::{Arc, Mutex};

/// traceStopReadCPU blocks until the trace CPU reading goroutine exits.
///
/// traceAdvanceSema must be held, and tracing must be disabled.
pub fn trace_stop_read_c_p_u() {
    if trace_enabled() {
        throw(Arc::new(Mutex::new(Some("traceStopReadCPU called with trace enabled".to_string()))));
    }

        // Once we close the profbuf, we'll be in one of two situations:
        // - The logger goroutine has already exited because it observed
        //   that the trace is disabled.
        // - The logger goroutine is asleep.
        //
        // Wake the goroutine so it can observe that their the buffer is
        // closed an exit.
    { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).cpu_log_write.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.store(Arc::new(Mutex::new(None)));
    { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).cpu_log_write.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.store(Arc::new(Mutex::new(None)));
    { let __recv = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).cpu_log_read.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).close(); __result };
    { let __recv = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).cpu_log_read.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).close(); __result };
    (*(*trace.lock().unwrap().as_ref().unwrap()).cpu_sleep.lock().unwrap().as_mut().unwrap()).wake();

        // Wait until the logger goroutine exits.
    (*trace.lock().unwrap().as_ref().unwrap()).cpu_log_done.recv().unwrap_or_default();

        // Clear state for the next trace.
    (*trace.lock().unwrap().as_mut().unwrap()).cpu_log_done = Default::default();
    (*(*trace.lock().unwrap().as_ref().unwrap()).cpu_log_read.lock().unwrap().as_mut().unwrap())[(0) as usize] = Default::default();
    (*(*trace.lock().unwrap().as_ref().unwrap()).cpu_log_read.lock().unwrap().as_mut().unwrap())[(1) as usize] = Default::default();
    (*(*trace.lock().unwrap().as_ref().unwrap()).cpu_sleep.lock().unwrap().as_mut().unwrap()).close();
}

/// traceReadCPU attempts to read from the provided profBuf[gen%2] and write
/// into the trace. Returns true if there might be more to read or false
/// if the profBuf is closed or the caller should otherwise stop reading.
///
/// The caller is responsible for ensuring that gen does not change. Either
/// the caller must be in a traceAcquire/traceRelease block, or must be calling
/// with traceAdvanceSema held.
///
/// No more than one goroutine may be in traceReadCPU for the same
/// profBuf at a time.
///
/// Must not run on the system stack because profBuf.read performs race
/// operations.
pub fn trace_read_c_p_u(gen: Arc<Mutex<Option<usize>>>) -> bool {
    let mut pcBuf: Arc<Mutex<Option<[usize; 128]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));

    let (mut data, mut tags, mut eof) = { let __recv = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).cpu_log_read.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some(crate::profbuf::profBufReadMode(Arc::new(Mutex::new(Some(PROF_BUF_NON_BLOCKING as i32)))))))); __result };
    while { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 4; __tmp_x < __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = (*Arc::new(Mutex::new(Some((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        break
    }
                // truncated profile
        if { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = 4 as u64; __tmp_x < __tmp_y } || { let __nil_result = (*tags.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = ((*tags.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x < __tmp_y } {
        break
    }
                // malformed profile
        if { let __tmp_x = ((*tags.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x < __tmp_y } {
        break
    }

                // mismatched profile records and tags
                // Deserialize the data in the profile buffer.
        let mut recordLen = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        let mut timestamp = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })));
        let mut ppid = Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }; let __tmp_y = 1; __tmp_x >> __tmp_y })));
        {
        let mut hasP = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }; let __tmp_y = 0b1 as u64; __tmp_x & __tmp_y }); let __tmp_y = 0 as u64; __tmp_x != __tmp_y })));;
        if !{ let __v = (*hasP.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            { let new_val = !(0 as u64) as u64; *ppid.lock().unwrap() = Some(new_val); };;
        }
    }
        let mut goid = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() })));
        let mut mpid = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(4) as usize].clone() })));
        let mut stk = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (5) as usize; let __high = ({ let __v = (*recordLen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));

                // Overflow records always have their headers contain
                // all zeroes.
        let mut isOverflowRecord = Arc::new(Mutex::new(Some({ let __tmp_x = ((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(4) as usize].clone() }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y })));

                // Move the data iterator forward.
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*recordLen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };

                // No support here for reporting goroutine tags at the moment; if
                // that information is to be part of the execution trace, we'd
                // probably want to see when the tags are applied and when they
                // change, instead of only seeing them when we get a CPU sample.
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = tags.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); tags = new_val; };

        if { let __v = (*isOverflowRecord.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Looks like an overflow record from the profBuf. Not much to
                // do here, we only want to report full records.
        continue
    }

                // Looks like an overflow record from the profBuf. Not much to
                // do here, we only want to report full records.
                // Construct the stack for insertion to the stack table.
        let mut nstk = Arc::new(Mutex::new(Some(1)));
        (*pcBuf.lock().unwrap().as_mut().unwrap())[(0) as usize] = LOGICAL_STACK_SENTINEL as usize;
        while { let __tmp_x = ({ let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 128; __tmp_x < __tmp_y } && { let __tmp_x = ({ let __tmp_x = { let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y } as i32); let __tmp_y = ((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        (*pcBuf.lock().unwrap().as_mut().unwrap())[({ let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = stk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() } as usize))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = nstk.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

                // Write out a trace event.
        let mut w = unsafe_trace_writer(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).cpu_buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() });

                // Ensure we have a place to write to.
        let mut flushed: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = (*w.lock().unwrap().as_ref().unwrap()).ensure(Arc::new(Mutex::new(Some(52)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_tmp_0; *flushed.lock().unwrap() = Some(__tmp_1); };
        if { let __v = (*flushed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Annotate the batch as containing strings.
        (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_EV_C_P_U_SAMPLES as u8 as u8))));
    }

                // Annotate the batch as containing strings.
                // Add the stack to the table.
        let mut stackID = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).stack_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some({ let __seq_holder = pcBuf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));

                // Write out the CPU sample.
        (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_EV_C_P_U_SAMPLE as u8 as u8))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __arg_holder = timestamp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __arg_holder = mpid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __arg_holder = ppid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __arg_holder = goid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some(stackID))));

        (*(*trace.lock().unwrap().as_ref().unwrap()).cpu_buf.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize] = (*w.lock().unwrap().as_ref().unwrap()).trace_buf.clone();
    }
        // truncated profile
        // malformed profile
        // mismatched profile records and tags
        // Deserialize the data in the profile buffer.
        // Overflow records always have their headers contain
        // all zeroes.
        // Move the data iterator forward.
        // No support here for reporting goroutine tags at the moment; if
        // that information is to be part of the execution trace, we'd
        // probably want to see when the tags are applied and when they
        // change, instead of only seeing them when we get a CPU sample.
        // Looks like an overflow record from the profBuf. Not much to
        // do here, we only want to report full records.
        // Construct the stack for insertion to the stack table.
        // Write out a trace event.
        // Ensure we have a place to write to.
        /* traceEvCPUSamples + traceEvCPUSample + timestamp + g + m + p + stack ID */
        // Annotate the batch as containing strings.
        // Add the stack to the table.
        // Write out the CPU sample.
    !eof
}

/// traceCPUFlush flushes trace.cpuBuf[gen%2]. The caller must be certain that gen
/// has completed and that there are no more writers to it.
pub fn trace_c_p_u_flush(gen: Arc<Mutex<Option<usize>>>) {
        // Flush any remaining trace buffers containing CPU samples.
    {
        let mut buf_local = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).cpu_buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.clone();;
        if { let __nil_result = (*buf_local.lock().unwrap()).is_some(); __nil_result } {
            let buf_closure_clone = buf_local.clone(); let gen_closure_clone = gen.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        trace_buf_flush(buf_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = gen_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*trace.lock().unwrap().as_ref().unwrap()).cpu_buf.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*gen_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize] = Default::default();
    }) as Box<dyn FnMut() -> () + Send + Sync>))));;
        }
    }
}

/// traceCPUSample writes a CPU profile sample stack to the execution tracer's
/// profiling buffer. It is called from a signal handler, so is limited in what
/// it can do. mp must be the thread that is currently stopped in a signal.
pub fn trace_c_p_u_sample(gp: GoPtr<crate::runtime2::g>, mp: Arc<Mutex<Option<m>>>, pp: GoPtr<crate::runtime2::p>, stk: Arc<Mutex<Option<Vec<usize>>>>) {
    if !trace_enabled() {
                // Tracing is usually turned off; don't spend time acquiring the signal
                // lock unless it's active.
        return;
    }
        // Tracing is usually turned off; don't spend time acquiring the signal
        // lock unless it's active.
    if { let __nil_result = (*mp.lock().unwrap()).is_none(); __nil_result } {
                // Drop samples that don't have an identifiable thread. We can't render
                // this in any useful way anyway.
        return;
    }

        // Drop samples that don't have an identifiable thread. We can't render
        // this in any useful way anyway.
        // We're going to conditionally write to one of two buffers based on the
        // generation. To make sure we write to the correct one, we need to make
        // sure this thread's trace seqlock is held. If it already is, then we're
        // in the tracer and we can just take advantage of that. If it isn't, then
        // we need to acquire it and read the generation.
    let mut locked = Arc::new(Mutex::new(Some(false)));
    if { let __tmp_x = { let __tmp_x = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
        { let new_val = true; *locked.lock().unwrap() = Some(new_val); };
    }
    let mut gen = (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).load();
    if { let __tmp_x = gen; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Tracing is disabled, as it turns out. Release the seqlock if necessary
                // and exit.
        if { let __v = (*locked.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
    }
        return;
    }

        // Tracing is disabled, as it turns out. Release the seqlock if necessary
        // and exit.
    let mut now = trace_clock_now();

        // The "header" here is the ID of the M that was running the profiled code,
        // followed by the IDs of the P and goroutine. (For normal CPU profiling, it's
        // usually the number of samples with the given stack.) Near syscalls, pp
        // may be nil. Reporting goid of 0 is fine for either g0 or a nil gp.
    let mut hdr: Arc<Mutex<Option<[u64; 3]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    if !pp.is_nil() {
                // Overflow records in profBuf have all header values set to zero. Make
                // sure that real headers have at least one bit set.
        (*hdr.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x << __tmp_y }; let __tmp_y = 0b1 as u64; __tmp_x | __tmp_y };
    } else {
        (*hdr.lock().unwrap().as_mut().unwrap())[(0) as usize] = 0b10 as u64;
    }
        // Overflow records in profBuf have all header values set to zero. Make
        // sure that real headers have at least one bit set.
    if !gp.is_nil() {
        (*hdr.lock().unwrap().as_mut().unwrap())[(1) as usize] = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap());
    }
    (*hdr.lock().unwrap().as_mut().unwrap())[(2) as usize] = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).procid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()).clone();

        // Allow only one writer at a time
    while !(*(*trace.lock().unwrap().as_ref().unwrap()).signal_lock.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as u32))), Arc::new(Mutex::new(Some(1 as u32)))) {
                // TODO: Is it safe to osyield here? https://go.dev/issue/52672
        osyield();
    }

        // TODO: Is it safe to osyield here? https://go.dev/issue/52672
    {
        let mut log = { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).cpu_log_write.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.load();;
        if { let __nil_result = (*log.lock().unwrap()).is_some(); __nil_result } {
            { let __recv = log.clone(); let __recv_ptr: *mut crate::profbuf::profBuf = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::profbuf::profBuf }; let __result = unsafe { &mut *__recv_ptr }.write(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some((*{ let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64))), Arc::new(Mutex::new(Some({ let __seq_holder = hdr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), stk.clone()); __result };;
        }
    }

        // Note: we don't pass a tag pointer here (how should profiling tags
        // interact with the execution tracer?), but if we did we'd need to be
        // careful about write barriers. See the long comment in profBuf.write.
    (*(*trace.lock().unwrap().as_ref().unwrap()).signal_lock.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));

        // Release the seqlock if we acquired it earlier.
    if { let __v = (*locked.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
    }
}