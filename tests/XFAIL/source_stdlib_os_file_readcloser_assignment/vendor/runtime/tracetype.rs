use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alg::*;
use crate::arena::*;
use crate::asan0::*;
use crate::atomic_pointer::*;
use crate::badlinkname::*;
use crate::cgo::*;
use crate::cgocall::*;
use crate::cgocallback::*;
use crate::cgocheck::*;
use crate::chan::*;
use crate::checkptr::*;
use crate::compiler::*;
use crate::complex::*;
use crate::coro::*;
use crate::covercounter::*;
use crate::covermeta::*;
use crate::cpuflags::*;
use crate::cpuflags_arm64::*;
use crate::cpuprof::*;
use crate::create_file_unix::*;
use crate::debug::*;
use crate::debugcall::*;
use crate::debuglog::*;
use crate::debuglog_off::*;
use crate::defs_darwin_arm64::*;
use crate::env_posix::*;
use crate::error::*;
use crate::r#extern::*;
use crate::fastlog2::*;
use crate::fastlog2table::*;
use crate::fds_unix::*;
use crate::float::*;
use crate::hash64::*;
use crate::heapdump::*;
use crate::histogram::*;
use crate::iface::*;
use crate::lfstack::*;
use crate::linkname::*;
use crate::linkname_swiss::*;
use crate::linkname_unix::*;
use crate::lock_sema::*;
use crate::lock_spinbit::*;
use crate::lockrank::*;
use crate::lockrank_off::*;
use crate::malloc::*;
use crate::map_fast32_swiss::*;
use crate::map_fast64_swiss::*;
use crate::map_faststr_swiss::*;
use crate::map_swiss::*;
use crate::mbarrier::*;
use crate::mbitmap::*;
use crate::mcache::*;
use crate::mcentral::*;
use crate::mcheckmark::*;
use crate::mcleanup::*;
use crate::mem::*;
use crate::mem_darwin::*;
use crate::mem_nonsbrk::*;
use crate::metrics::*;
use crate::mfinal::*;
use crate::mfixalloc::*;
use crate::mgc::*;
use crate::mgclimit::*;
use crate::mgcmark::*;
use crate::mgcpacer::*;
use crate::mgcscavenge::*;
use crate::mgcstack::*;
use crate::mgcsweep::*;
use crate::mgcwork::*;
use crate::mheap::*;
use crate::minmax::*;
use crate::mpagealloc::*;
use crate::mpagealloc_64bit::*;
use crate::mpagecache::*;
use crate::mpallocbits::*;
use crate::mprof::*;
use crate::mranges::*;
use crate::msan0::*;
use crate::msize::*;
use crate::mspanset::*;
use crate::mstats::*;
use crate::mwbbuf::*;
use crate::nbpipe_pipe::*;
use crate::netpoll::*;
use crate::netpoll_kqueue::*;
use crate::netpoll_kqueue_event::*;
use crate::nonwindows_stub::*;
use crate::note_other::*;
use crate::os_darwin::*;
use crate::os_darwin_arm64::*;
use crate::os_nonopenbsd::*;
use crate::os_unix::*;
use crate::os_unix_nonlinux::*;
use crate::panic::*;
use crate::pinner::*;
use crate::plugin::*;
use crate::preempt::*;
use crate::preempt_nonwindows::*;
use crate::print::*;
use crate::proc::*;
use crate::profbuf::*;
use crate::proflabel::*;
use crate::race0::*;
use crate::rand::*;
use crate::rdebug::*;
use crate::retry::*;
use crate::r#mod::*;
use crate::runtime1::*;
use crate::runtime2::*;
use crate::runtime_boring::*;
use crate::rwmutex::*;
use crate::security_issetugid::*;
use crate::security_unix::*;
use crate::select::*;
use crate::sema::*;
use crate::signal_arm64::*;
use crate::signal_darwin::*;
use crate::signal_darwin_arm64::*;
use crate::signal_unix::*;
use crate::sigqueue::*;
use crate::sizeclasses::*;
use crate::slice::*;
use crate::softfloat64::*;
use crate::stack::*;
use crate::stkframe::*;
use crate::string::*;
use crate::stubs::*;
use crate::stubs_arm64::*;
use crate::stubs_nonlinux::*;
use crate::stubs_nonwasm::*;
use crate::symtab::*;
use crate::symtabinl::*;
use crate::synctest::*;
use crate::sys_arm64::*;
use crate::sys_darwin::*;
use crate::sys_darwin_arm64::*;
use crate::sys_libc::*;
use crate::sys_nonppc64x::*;
use crate::tagptr::*;
use crate::tagptr_64bit::*;
use crate::test_stubs::*;
use crate::time::*;
use crate::time_nofake::*;
use crate::timestub::*;
use crate::tls_stub::*;
use crate::trace::*;
use crate::traceallocfree::*;
use crate::traceback::*;
use crate::tracebuf::*;
use crate::tracecpu::*;
use crate::traceevent::*;
use crate::traceexp::*;
use crate::tracemap::*;
use crate::traceregion::*;
use crate::traceruntime::*;
use crate::tracestack::*;
use crate::tracestatus::*;
use crate::tracestring::*;
use crate::tracetime::*;
use crate::r#type::*;
use crate::typekind::*;
use crate::r#unsafe::*;
use crate::utf8::*;
use crate::vdso_in_none::*;
use crate::vgetrandom_unsupported::*;
use crate::write_err::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// traceTypeTable maps stack traces (arrays of PC's) to unique uint32 ids.
/// It is lock-free for reading.
#[derive(Clone)]
pub struct traceTypeTable {
    pub tab: Arc<Mutex<Option<traceMap>>>,
}

impl traceTypeTable {
    pub fn __go_value_clone(&self) -> Self {
        Self { tab: { let __guard = self.tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for traceTypeTable {
    fn default() -> Self {
        Self { tab: Arc::new(Mutex::new(Some(traceMap::default()))) }
    }
}

impl std::fmt::Display for traceTypeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.tab.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for traceTypeTable {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl traceTypeTable {
    /// put returns a unique id for the type typ and caches it in the table,
    /// if it's seeing it for the first time.
    ///
    /// N.B. typ must be kept alive forever for this to work correctly.
    pub fn put(&self, typ: GoPtr<internal_abi::r#type::Type>) -> u64 {
        if typ.is_nil() {
        return 0;
    }
                // Insert the pointer to the type itself.
        let (mut id, _) = (*self.tab.lock().unwrap().as_ref().unwrap()).put(noescape(Arc::new(Mutex::new(Some(&typ as *const _ as usize)))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))));
        id
    }

    /// dump writes all previously cached types to trace buffers and
    /// releases all memory and resets state. It must only be called once the caller
    /// can guarantee that there are no more writers to the table.
    pub fn dump(&self, gen: Arc<Mutex<Option<usize>>>) {
        let mut w = unsafe_trace_exp_writer(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::traceexp::traceExperiment(Arc::new(Mutex::new(Some(TRACE_EXPERIMENT_ALLOC_FREE as u8))))))));
        {
        let mut root: GoPtr<crate::tracemap::traceMapNode> = GoPtr::raw({ let __ptr = (*(*self.tab.lock().unwrap().as_ref().unwrap()).root.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });;
        if !root.is_nil() {
            { let new_val = dump_types_rec(root.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_val; };;
        }
    }
        { let __recv = (*w.lock().unwrap().as_ref().unwrap()).flush(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
        (*self.tab.lock().unwrap().as_ref().unwrap()).reset();
    }
}

pub fn dump_types_rec(node: GoPtr<crate::tracemap::traceMapNode>, mut w: Arc<Mutex<Option<traceWriter>>>) -> Arc<Mutex<Option<crate::tracebuf::traceWriter>>> {
    let mut typ: GoPtr<internal_abi::r#type::Type> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }))); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut typName = { let __recv = to_r_type(typ.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result };

        // The maximum number of bytes required to hold the encoded type.
    let mut maxBytes = Arc::new(Mutex::new(Some({ let __tmp_x = 51; let __tmp_y = ((*typName.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y })));

        // Estimate the size of this record. This
        // bound is pretty loose, but avoids counting
        // lots of varint sizes.
        //
        // Add 1 because we might also write a traceAllocFreeTypesBatch byte.
    let mut flushed: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let (__tmp_0, __tmp_1) = (*w.lock().unwrap().as_ref().unwrap()).ensure(Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = { let __v = (*maxBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_tmp_0; *flushed.lock().unwrap() = Some(__tmp_1); };
    if { let __v = (*flushed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Annotate the batch as containing types.
        (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_ALLOC_FREE_TYPES_BATCH as u8))));
    }

        // Annotate the batch as containing types.
        // Emit type.
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(typ.addr()))).lock().unwrap().as_ref().unwrap()) as usize as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).size(); __result } as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.ptr_bytes.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some((*typName.lock().unwrap().as_ref().unwrap()).len() as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).string_data(Arc::new(Mutex::new(Some({ let __arg_holder = typName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Recursively walk all child nodes.
    for i in 0..(({ let __range_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut child = { let __seq = { let __seq_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.load();
        if { let __nil_result = (*child.lock().unwrap()).is_none(); __nil_result } {
        continue
    }
        { let new_val = dump_types_rec(GoPtr::raw({ let __ptr = child.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }), Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_val; };
    }
    return { let __owned = w.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

impl GoValueClone for traceTypeTable {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
