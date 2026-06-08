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

/// A checkmarksMap stores the GC marks in "checkmarks" mode. It is a
/// per-arena bitmap with a bit for every word in the arena. The mark
/// is stored on the bit corresponding to the first word of the marked
/// allocation.
#[derive(Clone)]
pub struct checkmarksMap {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub b: Arc<Mutex<Option<[u8; 1048576]>>>,
}

impl checkmarksMap {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, b: { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for checkmarksMap {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), b: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for checkmarksMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), format_slice(&self.b))
    }
}

impl GoJsonDecode for checkmarksMap {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static useCheckmark: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *useCheckmark.lock().unwrap() = Some(false);
    *useCheckmark.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *useCheckmark.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_24() {
    *useCheckmark.lock().unwrap() = Some(false);
}


/// startCheckmarks prepares for the checkmarks phase.
///
/// The world must be stopped.
pub fn start_checkmarks() {
    assert_world_stopped();

        // Clear all checkmarks.
    { let __range_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).all_arenas.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ai in __range_values.iter().cloned() {
        let mut arena = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l1(&(ai))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l2(&(ai))) as usize].clone() }.clone();
        let mut bitmap: GoPtr<checkmarksMap> = (*arena.lock().unwrap().as_ref().unwrap()).checkmarks.clone();
        if bitmap.is_nil() {
                // Allocate bitmap on first use.
        bitmap = GoPtr::raw({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some(std::mem::size_of::<checkmarksMap>()))), Arc::new(Mutex::new(Some(0 as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if bitmap.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory allocating checkmarks bitmap".to_string()))));
    }
        { let new_val = bitmap.clone(); (*arena.lock().unwrap().as_mut().unwrap()).checkmarks = new_val; };
    } else {
                // Otherwise clear the existing bitmap.
        { let __clear_start = 0usize; let __clear_end = { let __clear_len_holder = { let __ptr_value = bitmap.with_mut(|__ptr_value| __ptr_value.b.clone()); __ptr_value }.clone(); let __clear_len_guard = __clear_len_holder.lock().unwrap(); __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let __clear_holder = { let __ptr_value = bitmap.with_mut(|__ptr_value| __ptr_value.b.clone()); __ptr_value }.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = 0; } } };
    }
    } }

        // Allocate bitmap on first use.
        // Otherwise clear the existing bitmap.
        // Enable checkmarking.
    { let new_val = true; *useCheckmark.lock().unwrap() = Some(new_val); };
}

/// endCheckmarks ends the checkmarks phase.
pub fn end_checkmarks() {
    if gc_mark_work_available(GoPtr::nil()) {
        throw(Arc::new(Mutex::new(Some("GC work not flushed".to_string()))));
    }
    { let new_val = false; *useCheckmark.lock().unwrap() = Some(new_val); };
}

/// setCheckmark throws if marking object is a checkmarks violation,
/// and otherwise sets obj's checkmark. It returns true if obj was
/// already checkmarked.
pub fn set_checkmark(obj: Arc<Mutex<Option<usize>>>, base: Arc<Mutex<Option<usize>>>, off: Arc<Mutex<Option<usize>>>, mbits: Arc<Mutex<Option<markBits>>>) -> bool {
    if !(*mbits.lock().unwrap().as_ref().unwrap()).is_marked() {
        printlock();
        eprint!("{}{}{}", format!("{}", "runtime: checkmarks found unexpected unmarked object obj=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*obj.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}", format!("{}", "runtime: found obj at *(".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "+".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", ")\n".to_string()));
                // Dump the source (base) object
        gc_dump_object(Arc::new(Mutex::new(Some("base".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Dump the object
        gc_dump_object(Arc::new(Mutex::new(Some("obj".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(!(0 as usize) as usize))));
        { let new_val = 2 as u8; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap() = Some(new_val); };
        throw(Arc::new(Mutex::new(Some("checkmark found unmarked object".to_string()))));
    }

        // Dump the source (base) object
        // Dump the object
    let mut ai = arena_index(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut arena = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
    let mut arenaWord = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x / __tmp_y }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = (*arena.lock().unwrap().as_ref().unwrap()).checkmarks.with_mut(|__ptr_value| __ptr_value.b.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x % __tmp_y })));
    let mut mask = Arc::new(Mutex::new(Some(({ let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x / __tmp_y }); let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }) as u8)));
    let mut bytep: Option<GoArrayElemPtr<u8, 1048576>> = Some(GoArrayElemPtr::new({ let __ptr_value = (*arena.lock().unwrap().as_ref().unwrap()).checkmarks.with_mut(|__ptr_value| __ptr_value.b.clone()); __ptr_value }.clone(), ({ let __v = (*arenaWord.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));

    if { let __tmp_x = { let __tmp_x = internal_runtime_atomic::load8(unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers")); let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
                // Already checkmarked.
        return true;
    }

        // Already checkmarked.
    internal_runtime_atomic::or8(unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    false
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for checkmarksMap {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
