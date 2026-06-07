use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

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
use crate::tracetype::*;
use crate::r#type::*;
use crate::typekind::*;
use crate::r#unsafe::*;
use crate::utf8::*;
use crate::vdso_in_none::*;
use crate::vgetrandom_unsupported::*;
use crate::write_err::*;

use std::sync::{Arc, Mutex};

pub(crate) const C0: usize = ((((((8 as usize) - (internal_goarch::PTR_SIZE as usize)) / (4 as usize)) * (2860486313 as usize)) + ((((internal_goarch::PTR_SIZE as usize) - (4 as usize)) / (4 as usize)) * (33054211828000289 as usize))) as usize);
pub(crate) const C1: usize = ((((((8 as usize) - (internal_goarch::PTR_SIZE as usize)) / (4 as usize)) * (3267000013 as usize)) + ((((internal_goarch::PTR_SIZE as usize) - (4 as usize)) / (4 as usize)) * (23344194077549503 as usize))) as usize);


pub(crate) const HASH_RANDOM_BYTES: i32 = internal_goarch::PTR_SIZE / 4 * 64;


pub(crate) static useAeshash: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static aeskeysched: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 128]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static hashkey: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[usize; 4]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *useAeshash.lock().unwrap() = Some(false);
    *aeskeysched.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *hashkey.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_zero_globals() {
    *useAeshash.lock().unwrap() = Some(false);
    *aeskeysched.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *hashkey.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


/// memhash should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/aacfactory/fns
///   - github.com/dgraph-io/ristretto
///   - github.com/minio/simdjson-go
///   - github.com/nbd-wtf/go-nostr
///   - github.com/outcaste-io/ristretto
///   - github.com/puzpuzpuz/xsync/v2
///   - github.com/puzpuzpuz/xsync/v3
///   - github.com/authzed/spicedb
///   - github.com/pingcap/badger
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname memhash
pub fn memhash(p: Arc<Mutex<Option<usize>>>, h: Arc<Mutex<Option<usize>>>, s: Arc<Mutex<Option<usize>>>) -> usize {
    unimplemented!("Go function declaration has no body");
}


pub fn read_unaligned64(p: Arc<Mutex<Option<usize>>>) -> u64 {
    let mut q: GoPtr<[u8; 8]> = GoPtr::raw({ let __ptr = p.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    if internal_goarch::BIG_ENDIAN {
        return { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(7) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(6) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(5) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 16; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(4) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 24; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(3) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(2) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 40; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(1) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 48; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 56; __tmp_x << __tmp_y }; __tmp_x | __tmp_y };
    }
    return { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(1) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(2) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 16; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(3) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 24; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(4) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(5) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 40; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(6) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 48; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = q.borrow(); __seq.as_ref().unwrap()[(7) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 56; __tmp_x << __tmp_y }; __tmp_x | __tmp_y };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
