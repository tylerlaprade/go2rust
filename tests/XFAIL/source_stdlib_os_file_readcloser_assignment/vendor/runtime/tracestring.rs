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
use crate::tracetime::*;
use crate::tracetype::*;
use crate::r#type::*;
use crate::typekind::*;
use crate::r#unsafe::*;
use crate::utf8::*;
use crate::vdso_in_none::*;
use crate::vgetrandom_unsupported::*;
use crate::write_err::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_TRACE_STRING_LEN: i32 = 1024;


/// traceStringTable is map of string -> unique ID that also manages
/// writing strings out into the trace.
#[derive(Clone)]
pub struct traceStringTable {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub buf: Arc<Mutex<Option<traceBuf>>>,
    pub tab: Arc<Mutex<Option<traceMap>>>,
}

impl traceStringTable {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, buf: self.buf.clone(), tab: { let __guard = self.tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for traceStringTable {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), buf: Arc::new(Mutex::new(None)), tab: Arc::new(Mutex::new(Some(traceMap::default()))) }
    }
}

impl std::fmt::Display for traceStringTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { let __guard = self.buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.tab.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for traceStringTable {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl traceStringTable {
    /// put adds a string to the table, emits it, and returns a unique ID for it.
    pub fn put(&mut self, gen: Arc<Mutex<Option<usize>>>, s: Arc<Mutex<Option<String>>>) -> u64 {
                // Put the string in the table.
        let mut ss: GoPtr<crate::string::stringStruct> = string_struct_of(s.clone());
        let (mut id, mut added) = (*self.tab.lock().unwrap().as_ref().unwrap()).put(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = ss.with_mut(|__ptr_value| __ptr_value.str.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = ss.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))));
        if added {
                // Write the string to the buffer.
        let gen_closure_clone = gen.clone(); let id_closure_clone = id.clone(); let s_closure_clone = s.clone(); let mut t_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        t_closure_clone.write_string(Arc::new(Mutex::new(Some({ let __arg_holder = gen_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(id_closure_clone))), Arc::new(Mutex::new(Some({ let __arg_holder = s_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
                // Write the string to the buffer.
        id
    }

    /// emit emits a string and creates an ID for it, but doesn't add it to the table. Returns the ID.
    pub fn emit(&mut self, gen: Arc<Mutex<Option<usize>>>, s: Arc<Mutex<Option<String>>>) -> u64 {
                // Grab an ID and write the string to the buffer.
        let mut id = (*self.tab.lock().unwrap().as_ref().unwrap()).steal_i_d();
        let gen_closure_clone = gen.clone(); let id_closure_clone = id.clone(); let s_closure_clone = s.clone(); let mut t_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        t_closure_clone.write_string(Arc::new(Mutex::new(Some({ let __arg_holder = gen_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(id_closure_clone))), Arc::new(Mutex::new(Some({ let __arg_holder = s_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        id
    }

    /// writeString writes the string to t.buf.
    ///
    /// Must run on the systemstack because it acquires t.lock.
    ///
    ///go:systemstack
    pub fn write_string(&mut self, gen: Arc<Mutex<Option<usize>>>, id: Arc<Mutex<Option<u64>>>, mut s: Arc<Mutex<Option<String>>>) {
                // Truncate the string if necessary.
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1024; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = (MAX_TRACE_STRING_LEN) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
        lock(GoPtr::local(self.lock.clone()));
        let mut w = unsafe_trace_writer(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = self.buf.clone(); __field });
                // Ensure we have a place to write to.
        let mut flushed: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = (*w.lock().unwrap().as_ref().unwrap()).ensure(Arc::new(Mutex::new(Some({ let __tmp_x = 22; let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_tmp_0; *flushed.lock().unwrap() = Some(__tmp_1); };
        if { let __v = (*flushed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Annotate the batch as containing strings.
        (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_EV_STRINGS as u8 as u8))));
    }
                // Annotate the batch as containing strings.
                // Write out the string.
        (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_EV_STRING as u8 as u8))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as u64))));
        (*w.lock().unwrap().as_mut().unwrap()).string_data(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Store back buf in case it was updated during ensure.
        { let new_val = (*w.lock().unwrap().as_ref().unwrap()).trace_buf.clone(); self.buf = new_val; };
        unlock(GoPtr::local(self.lock.clone()));
    }

    /// reset clears the string table and flushes any buffers it has.
    ///
    /// Must be called only once the caller is certain nothing else will be
    /// added to this table.
    pub fn reset(&mut self, gen: Arc<Mutex<Option<usize>>>) {
        if { let __nil_target = self.buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let gen_closure_clone = gen.clone(); let mut t_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        trace_buf_flush({ let __field = t_closure_clone.buf.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = gen_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        *self.buf.lock().unwrap() = None;
    }
                // Reset the table.
        (*self.tab.lock().unwrap().as_ref().unwrap()).reset();
    }
}

impl GoValueClone for traceStringTable {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
