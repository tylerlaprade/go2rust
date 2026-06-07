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

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DEBUG_PCLN: bool = false;


/// Frames may be used to get function/file/line information for a
/// slice of PC values returned by [Callers].
#[derive(Clone)]
pub struct Frames {
    pub callers: Arc<Mutex<Option<Vec<usize>>>>,
    pub next_p_c: Arc<Mutex<Option<usize>>>,
    pub frames: Arc<Mutex<Option<Vec<Frame>>>>,
    pub frame_store: Arc<Mutex<Option<[Frame; 2]>>>,
}

impl Frames {
    pub fn __go_value_clone(&self) -> Self {
        Self { callers: self.callers.clone(), next_p_c: { let __guard = self.next_p_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, frames: self.frames.clone(), frame_store: { let __guard = self.frame_store.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Frames {
    fn default() -> Self {
        Self { callers: Arc::new(Mutex::new(None)), next_p_c: Arc::new(Mutex::new(Some(0))), frames: Arc::new(Mutex::new(None)), frame_store: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for Frames {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", format_slice(&self.callers), (*self.next_p_c.lock().unwrap().as_ref().unwrap()), format_slice(&self.frames), format_slice(&self.frame_store))
    }
}

impl GoJsonDecode for Frames {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Frame is the information returned by [Frames] for each call frame.
#[derive(Clone)]
pub struct Frame {
    pub p_c: Arc<Mutex<Option<usize>>>,
    pub func: GoPtr<Func>,
    pub function: Arc<Mutex<Option<String>>>,
    pub file: Arc<Mutex<Option<String>>>,
    pub line: Arc<Mutex<Option<i32>>>,
    pub start_line: Arc<Mutex<Option<i32>>>,
    pub entry: Arc<Mutex<Option<usize>>>,
    pub func_info: Arc<Mutex<Option<funcInfo>>>,
}

impl Frame {
    pub fn __go_value_clone(&self) -> Self {
        Self { p_c: { let __guard = self.p_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, func: self.func.clone(), function: { let __guard = self.function.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, file: { let __guard = self.file.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, line: { let __guard = self.line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, start_line: { let __guard = self.start_line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, entry: { let __guard = self.entry.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, func_info: { let __guard = self.func_info.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Frame {
    fn default() -> Self {
        Self { p_c: Arc::new(Mutex::new(Some(0))), func: GoPtr::nil(), function: Arc::new(Mutex::new(Some(String::new()))), file: Arc::new(Mutex::new(Some(String::new()))), line: Arc::new(Mutex::new(Some(0))), start_line: Arc::new(Mutex::new(Some(0))), entry: Arc::new(Mutex::new(Some(0))), func_info: Arc::new(Mutex::new(Some(funcInfo::default()))) }
    }
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.p_c.lock().unwrap().as_ref().unwrap()), { if self.func.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.function.lock().unwrap().as_ref().unwrap()), (*self.file.lock().unwrap().as_ref().unwrap()), (*self.line.lock().unwrap().as_ref().unwrap()), (*self.start_line.lock().unwrap().as_ref().unwrap()), (*self.entry.lock().unwrap().as_ref().unwrap()), (*self.func_info.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Frame {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("PC") {
            out.p_c = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Function") {
            out.function = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("File") {
            out.file = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Line") {
            out.line = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Entry") {
            out.entry = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A Func represents a Go function in the running binary.
#[derive(Debug, Clone)]
pub struct Func {
    pub opaque: Arc<Mutex<Option<AnonymousStruct12>>>,
}

impl Func {
    pub fn __go_value_clone(&self) -> Self {
        Self { opaque: { let __guard = self.opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Func {
    fn default() -> Self {
        Self { opaque: Arc::new(Mutex::new(Some(AnonymousStruct12::default()))) }
    }
}

impl std::fmt::Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.opaque.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Func {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pcHeader holds data used by the pclntab lookups.
#[derive(Debug, Clone)]
pub struct pcHeader {
    pub magic: Arc<Mutex<Option<u32>>>,
    pub pad1: Arc<Mutex<Option<u8>>>,
    pub pad2: Arc<Mutex<Option<u8>>>,
    pub min_l_c: Arc<Mutex<Option<u8>>>,
    pub ptr_size: Arc<Mutex<Option<u8>>>,
    pub nfunc: Arc<Mutex<Option<i32>>>,
    pub nfiles: Arc<Mutex<Option<u64>>>,
    pub text_start: Arc<Mutex<Option<usize>>>,
    pub funcname_offset: Arc<Mutex<Option<usize>>>,
    pub cu_offset: Arc<Mutex<Option<usize>>>,
    pub filetab_offset: Arc<Mutex<Option<usize>>>,
    pub pctab_offset: Arc<Mutex<Option<usize>>>,
    pub pcln_offset: Arc<Mutex<Option<usize>>>,
}

impl pcHeader {
    pub fn __go_value_clone(&self) -> Self {
        Self { magic: { let __guard = self.magic.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad1: { let __guard = self.pad1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad2: { let __guard = self.pad2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, min_l_c: { let __guard = self.min_l_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ptr_size: { let __guard = self.ptr_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nfunc: { let __guard = self.nfunc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nfiles: { let __guard = self.nfiles.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, text_start: { let __guard = self.text_start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, funcname_offset: { let __guard = self.funcname_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cu_offset: { let __guard = self.cu_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, filetab_offset: { let __guard = self.filetab_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pctab_offset: { let __guard = self.pctab_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pcln_offset: { let __guard = self.pcln_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pcHeader {
    fn default() -> Self {
        Self { magic: Arc::new(Mutex::new(Some(0))), pad1: Arc::new(Mutex::new(Some(0))), pad2: Arc::new(Mutex::new(Some(0))), min_l_c: Arc::new(Mutex::new(Some(0))), ptr_size: Arc::new(Mutex::new(Some(0))), nfunc: Arc::new(Mutex::new(Some(0))), nfiles: Arc::new(Mutex::new(Some(0))), text_start: Arc::new(Mutex::new(Some(0))), funcname_offset: Arc::new(Mutex::new(Some(0))), cu_offset: Arc::new(Mutex::new(Some(0))), filetab_offset: Arc::new(Mutex::new(Some(0))), pctab_offset: Arc::new(Mutex::new(Some(0))), pcln_offset: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for pcHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.magic.lock().unwrap().as_ref().unwrap()), (*self.pad1.lock().unwrap().as_ref().unwrap()), (*self.pad2.lock().unwrap().as_ref().unwrap()), (*self.min_l_c.lock().unwrap().as_ref().unwrap()), (*self.ptr_size.lock().unwrap().as_ref().unwrap()), (*self.nfunc.lock().unwrap().as_ref().unwrap()), (*self.nfiles.lock().unwrap().as_ref().unwrap()), (*self.text_start.lock().unwrap().as_ref().unwrap()), (*self.funcname_offset.lock().unwrap().as_ref().unwrap()), (*self.cu_offset.lock().unwrap().as_ref().unwrap()), (*self.filetab_offset.lock().unwrap().as_ref().unwrap()), (*self.pctab_offset.lock().unwrap().as_ref().unwrap()), (*self.pcln_offset.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pcHeader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// moduledata records information about the layout of the executable
/// image. It is written by the linker. Any changes here must be
/// matched changes to the code in cmd/link/internal/ld/symtab.go:symtab.
/// moduledata is stored in statically allocated non-pointer memory;
/// none of the pointers here are visible to the garbage collector.
#[derive(Clone)]
pub struct moduledata {
    pub not_in_heap: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub pc_header: Arc<Mutex<Option<pcHeader>>>,
    pub funcnametab: Arc<Mutex<Option<Vec<u8>>>>,
    pub cutab: Arc<Mutex<Option<Vec<u32>>>>,
    pub filetab: Arc<Mutex<Option<Vec<u8>>>>,
    pub pctab: Arc<Mutex<Option<Vec<u8>>>>,
    pub pclntable: Arc<Mutex<Option<Vec<u8>>>>,
    pub ftab: Arc<Mutex<Option<Vec<functab>>>>,
    pub findfunctab: Arc<Mutex<Option<usize>>>,
    pub minpc: Arc<Mutex<Option<usize>>>,
    pub maxpc: Arc<Mutex<Option<usize>>>,
    pub text: Arc<Mutex<Option<usize>>>,
    pub etext: Arc<Mutex<Option<usize>>>,
    pub noptrdata: Arc<Mutex<Option<usize>>>,
    pub enoptrdata: Arc<Mutex<Option<usize>>>,
    pub data: Arc<Mutex<Option<usize>>>,
    pub edata: Arc<Mutex<Option<usize>>>,
    pub bss: Arc<Mutex<Option<usize>>>,
    pub ebss: Arc<Mutex<Option<usize>>>,
    pub noptrbss: Arc<Mutex<Option<usize>>>,
    pub enoptrbss: Arc<Mutex<Option<usize>>>,
    pub covctrs: Arc<Mutex<Option<usize>>>,
    pub ecovctrs: Arc<Mutex<Option<usize>>>,
    pub end: Arc<Mutex<Option<usize>>>,
    pub gcdata: Arc<Mutex<Option<usize>>>,
    pub gcbss: Arc<Mutex<Option<usize>>>,
    pub types: Arc<Mutex<Option<usize>>>,
    pub etypes: Arc<Mutex<Option<usize>>>,
    pub rodata: Arc<Mutex<Option<usize>>>,
    pub gofunc: Arc<Mutex<Option<usize>>>,
    pub textsectmap: Arc<Mutex<Option<Vec<textsect>>>>,
    pub typelinks: Arc<Mutex<Option<Vec<i32>>>>,
    pub itablinks: Arc<Mutex<Option<Vec<Arc<Mutex<Option<internal_abi::iface::ITab>>>>>>>,
    pub ptab: Arc<Mutex<Option<Vec<ptabEntry>>>>,
    pub pluginpath: Arc<Mutex<Option<String>>>,
    pub pkghashes: Arc<Mutex<Option<Vec<modulehash>>>>,
    pub inittasks: Arc<Mutex<Option<Vec<Arc<Mutex<Option<initTask>>>>>>>,
    pub modulename: Arc<Mutex<Option<String>>>,
    pub modulehashes: Arc<Mutex<Option<Vec<modulehash>>>>,
    pub hasmain: Arc<Mutex<Option<u8>>>,
    pub bad: Arc<Mutex<Option<bool>>>,
    pub gcdatamask: Arc<Mutex<Option<bitvector>>>,
    pub gcbssmask: Arc<Mutex<Option<bitvector>>>,
    pub typemap: Arc<Mutex<Option<BTreeMap<internal_abi::r#type::TypeOff, Arc<Mutex<Option<internal_abi::r#type::Type>>>>>>>,
    pub next: Arc<Mutex<Option<moduledata>>>,
}

impl moduledata {
    pub fn __go_value_clone(&self) -> Self {
        Self { not_in_heap: { let __guard = self.not_in_heap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pc_header: self.pc_header.clone(), funcnametab: self.funcnametab.clone(), cutab: self.cutab.clone(), filetab: self.filetab.clone(), pctab: self.pctab.clone(), pclntable: self.pclntable.clone(), ftab: self.ftab.clone(), findfunctab: { let __guard = self.findfunctab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, minpc: { let __guard = self.minpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, maxpc: { let __guard = self.maxpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, text: { let __guard = self.text.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, etext: { let __guard = self.etext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, noptrdata: { let __guard = self.noptrdata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, enoptrdata: { let __guard = self.enoptrdata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, edata: { let __guard = self.edata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bss: { let __guard = self.bss.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ebss: { let __guard = self.ebss.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, noptrbss: { let __guard = self.noptrbss.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, enoptrbss: { let __guard = self.enoptrbss.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, covctrs: { let __guard = self.covctrs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ecovctrs: { let __guard = self.ecovctrs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcdata: { let __guard = self.gcdata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcbss: { let __guard = self.gcbss.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, types: { let __guard = self.types.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, etypes: { let __guard = self.etypes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rodata: { let __guard = self.rodata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gofunc: { let __guard = self.gofunc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, textsectmap: self.textsectmap.clone(), typelinks: self.typelinks.clone(), itablinks: self.itablinks.clone(), ptab: self.ptab.clone(), pluginpath: { let __guard = self.pluginpath.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pkghashes: self.pkghashes.clone(), inittasks: self.inittasks.clone(), modulename: { let __guard = self.modulename.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, modulehashes: self.modulehashes.clone(), hasmain: { let __guard = self.hasmain.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bad: { let __guard = self.bad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcdatamask: { let __guard = self.gcdatamask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcbssmask: { let __guard = self.gcbssmask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typemap: self.typemap.clone(), next: self.next.clone() }
    }
}


impl Default for moduledata {
    fn default() -> Self {
        Self { not_in_heap: Arc::new(Mutex::new(Some(Default::default()))), pc_header: Arc::new(Mutex::new(None)), funcnametab: Arc::new(Mutex::new(None)), cutab: Arc::new(Mutex::new(None)), filetab: Arc::new(Mutex::new(None)), pctab: Arc::new(Mutex::new(None)), pclntable: Arc::new(Mutex::new(None)), ftab: Arc::new(Mutex::new(None)), findfunctab: Arc::new(Mutex::new(Some(0))), minpc: Arc::new(Mutex::new(Some(0))), maxpc: Arc::new(Mutex::new(Some(0))), text: Arc::new(Mutex::new(Some(0))), etext: Arc::new(Mutex::new(Some(0))), noptrdata: Arc::new(Mutex::new(Some(0))), enoptrdata: Arc::new(Mutex::new(Some(0))), data: Arc::new(Mutex::new(Some(0))), edata: Arc::new(Mutex::new(Some(0))), bss: Arc::new(Mutex::new(Some(0))), ebss: Arc::new(Mutex::new(Some(0))), noptrbss: Arc::new(Mutex::new(Some(0))), enoptrbss: Arc::new(Mutex::new(Some(0))), covctrs: Arc::new(Mutex::new(Some(0))), ecovctrs: Arc::new(Mutex::new(Some(0))), end: Arc::new(Mutex::new(Some(0))), gcdata: Arc::new(Mutex::new(Some(0))), gcbss: Arc::new(Mutex::new(Some(0))), types: Arc::new(Mutex::new(Some(0))), etypes: Arc::new(Mutex::new(Some(0))), rodata: Arc::new(Mutex::new(Some(0))), gofunc: Arc::new(Mutex::new(Some(0))), textsectmap: Arc::new(Mutex::new(None)), typelinks: Arc::new(Mutex::new(None)), itablinks: Arc::new(Mutex::new(None)), ptab: Arc::new(Mutex::new(None)), pluginpath: Arc::new(Mutex::new(Some(String::new()))), pkghashes: Arc::new(Mutex::new(None)), inittasks: Arc::new(Mutex::new(None)), modulename: Arc::new(Mutex::new(Some(String::new()))), modulehashes: Arc::new(Mutex::new(None)), hasmain: Arc::new(Mutex::new(Some(0))), bad: Arc::new(Mutex::new(Some(false))), gcdatamask: Arc::new(Mutex::new(Some(bitvector::default()))), gcbssmask: Arc::new(Mutex::new(Some(bitvector::default()))), typemap: Arc::new(Mutex::new(None)), next: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for moduledata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.not_in_heap.lock().unwrap().as_ref().unwrap()), { let __guard = self.pc_header.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice(&self.funcnametab), format_slice(&self.cutab), format_slice(&self.filetab), format_slice(&self.pctab), format_slice(&self.pclntable), format_slice(&self.ftab), (*self.findfunctab.lock().unwrap().as_ref().unwrap()), (*self.minpc.lock().unwrap().as_ref().unwrap()), (*self.maxpc.lock().unwrap().as_ref().unwrap()), (*self.text.lock().unwrap().as_ref().unwrap()), (*self.etext.lock().unwrap().as_ref().unwrap()), (*self.noptrdata.lock().unwrap().as_ref().unwrap()), (*self.enoptrdata.lock().unwrap().as_ref().unwrap()), (*self.data.lock().unwrap().as_ref().unwrap()), (*self.edata.lock().unwrap().as_ref().unwrap()), (*self.bss.lock().unwrap().as_ref().unwrap()), (*self.ebss.lock().unwrap().as_ref().unwrap()), (*self.noptrbss.lock().unwrap().as_ref().unwrap()), (*self.enoptrbss.lock().unwrap().as_ref().unwrap()), (*self.covctrs.lock().unwrap().as_ref().unwrap()), (*self.ecovctrs.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()), (*self.gcdata.lock().unwrap().as_ref().unwrap()), (*self.gcbss.lock().unwrap().as_ref().unwrap()), (*self.types.lock().unwrap().as_ref().unwrap()), (*self.etypes.lock().unwrap().as_ref().unwrap()), (*self.rodata.lock().unwrap().as_ref().unwrap()), (*self.gofunc.lock().unwrap().as_ref().unwrap()), format_slice(&self.textsectmap), format_slice(&self.typelinks), format_slice_wrapped(&self.itablinks), format_slice(&self.ptab), (*self.pluginpath.lock().unwrap().as_ref().unwrap()), format_slice(&self.pkghashes), format_slice_wrapped(&self.inittasks), (*self.modulename.lock().unwrap().as_ref().unwrap()), format_slice(&self.modulehashes), (*self.hasmain.lock().unwrap().as_ref().unwrap()), (*self.bad.lock().unwrap().as_ref().unwrap()), (*self.gcdatamask.lock().unwrap().as_ref().unwrap()), (*self.gcbssmask.lock().unwrap().as_ref().unwrap()), format_map(&self.typemap), { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for moduledata {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A modulehash is used to compare the ABI of a new module or a
/// package in a new module with the loaded program.
///
/// For each shared library a module links against, the linker creates an entry in the
/// moduledata.modulehashes slice containing the name of the module, the abi hash seen
/// at link time and a pointer to the runtime abi hash. These are checked in
/// moduledataverify1 below.
///
/// For each loaded plugin, the pkghashes slice has a modulehash of the
/// newly loaded package that can be used to check the plugin's version of
/// a package against any previously loaded version of the package.
/// This is done in plugin.lastmoduleinit.
#[derive(Debug, Clone)]
pub struct modulehash {
    pub modulename: Arc<Mutex<Option<String>>>,
    pub linktimehash: Arc<Mutex<Option<String>>>,
    pub runtimehash: Arc<Mutex<Option<String>>>,
}

impl modulehash {
    pub fn __go_value_clone(&self) -> Self {
        Self { modulename: { let __guard = self.modulename.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, linktimehash: { let __guard = self.linktimehash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runtimehash: self.runtimehash.clone() }
    }
}


impl Default for modulehash {
    fn default() -> Self {
        Self { modulename: Arc::new(Mutex::new(Some(String::new()))), linktimehash: Arc::new(Mutex::new(Some(String::new()))), runtimehash: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for modulehash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.modulename.lock().unwrap().as_ref().unwrap()), (*self.linktimehash.lock().unwrap().as_ref().unwrap()), { let __guard = self.runtimehash.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for modulehash {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct functab {
    pub entryoff: Arc<Mutex<Option<u32>>>,
    pub funcoff: Arc<Mutex<Option<u32>>>,
}

impl functab {
    pub fn __go_value_clone(&self) -> Self {
        Self { entryoff: { let __guard = self.entryoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, funcoff: { let __guard = self.funcoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for functab {
    fn default() -> Self {
        Self { entryoff: Arc::new(Mutex::new(Some(0))), funcoff: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for functab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.entryoff.lock().unwrap().as_ref().unwrap()), (*self.funcoff.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for functab {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct textsect {
    pub vaddr: Arc<Mutex<Option<usize>>>,
    pub end: Arc<Mutex<Option<usize>>>,
    pub baseaddr: Arc<Mutex<Option<usize>>>,
}

impl textsect {
    pub fn __go_value_clone(&self) -> Self {
        Self { vaddr: { let __guard = self.vaddr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, baseaddr: { let __guard = self.baseaddr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for textsect {
    fn default() -> Self {
        Self { vaddr: Arc::new(Mutex::new(Some(0))), end: Arc::new(Mutex::new(Some(0))), baseaddr: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for textsect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.vaddr.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()), (*self.baseaddr.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for textsect {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// findfuncbucket is an array of these structures.
/// Each bucket represents 4096 bytes of the text segment.
/// Each subbucket represents 256 bytes of the text segment.
/// To find a function given a pc, locate the bucket and subbucket for
/// that pc. Add together the idx and subbucket value to obtain a
/// function index. Then scan the functab array starting at that
/// index to find the target function.
/// This table uses 20 bytes for every 4096 bytes of code, or ~0.5% overhead.
#[derive(Debug, Clone)]
pub struct findfuncbucket {
    pub idx: Arc<Mutex<Option<u32>>>,
    pub subbuckets: Arc<Mutex<Option<[u8; 16]>>>,
}

impl findfuncbucket {
    pub fn __go_value_clone(&self) -> Self {
        Self { idx: { let __guard = self.idx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, subbuckets: { let __guard = self.subbuckets.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for findfuncbucket {
    fn default() -> Self {
        Self { idx: Arc::new(Mutex::new(Some(0))), subbuckets: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for findfuncbucket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.idx.lock().unwrap().as_ref().unwrap()), format_slice(&self.subbuckets))
    }
}

impl GoJsonDecode for findfuncbucket {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct funcInfo {
    pub _func: Arc<Mutex<Option<_func>>>,
    pub datap: Arc<Mutex<Option<moduledata>>>,
}

impl funcInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { _func: self._func.clone(), datap: self.datap.clone() }
    }
}

impl std::fmt::Display for funcInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self._func.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.datap.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for funcInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A srcFunc represents a logical function in the source code. This may
/// correspond to an actual symbol in the binary text, or it may correspond to a
/// source function that has been inlined.
#[derive(Clone)]
pub struct srcFunc {
    pub datap: Arc<Mutex<Option<moduledata>>>,
    pub name_off: Arc<Mutex<Option<i32>>>,
    pub start_line: Arc<Mutex<Option<i32>>>,
    pub func_i_d: Arc<Mutex<Option<internal_abi::symtab::FuncID>>>,
}

impl srcFunc {
    pub fn __go_value_clone(&self) -> Self {
        Self { datap: self.datap.clone(), name_off: { let __guard = self.name_off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, start_line: { let __guard = self.start_line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, func_i_d: { let __guard = self.func_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for srcFunc {
    fn default() -> Self {
        Self { datap: Arc::new(Mutex::new(None)), name_off: Arc::new(Mutex::new(Some(0))), start_line: Arc::new(Mutex::new(Some(0))), func_i_d: Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for srcFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", { let __guard = self.datap.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.name_off.lock().unwrap().as_ref().unwrap()), (*self.start_line.lock().unwrap().as_ref().unwrap()), (*self.func_i_d.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for srcFunc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct pcvalueCache {
    pub entries: Arc<Mutex<Option<[[pcvalueCacheEnt; 8]; 2]>>>,
    pub in_use: Arc<Mutex<Option<i32>>>,
}

impl pcvalueCache {
    pub fn __go_value_clone(&self) -> Self {
        Self { entries: { let __guard = self.entries.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_use: { let __guard = self.in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pcvalueCache {
    fn default() -> Self {
        Self { entries: Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| Default::default()))))), in_use: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for pcvalueCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_nested_slice(&self.entries), (*self.in_use.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pcvalueCache {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct pcvalueCacheEnt {
    pub targetpc: Arc<Mutex<Option<usize>>>,
    pub off: Arc<Mutex<Option<u32>>>,
    pub val: Arc<Mutex<Option<i32>>>,
    pub val_p_c: Arc<Mutex<Option<usize>>>,
}

impl pcvalueCacheEnt {
    pub fn __go_value_clone(&self) -> Self {
        Self { targetpc: { let __guard = self.targetpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, off: { let __guard = self.off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, val: { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, val_p_c: { let __guard = self.val_p_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pcvalueCacheEnt {
    fn default() -> Self {
        Self { targetpc: Arc::new(Mutex::new(Some(0))), off: Arc::new(Mutex::new(Some(0))), val: Arc::new(Mutex::new(Some(0))), val_p_c: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for pcvalueCacheEnt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.targetpc.lock().unwrap().as_ref().unwrap()), (*self.off.lock().unwrap().as_ref().unwrap()), (*self.val.lock().unwrap().as_ref().unwrap()), (*self.val_p_c.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pcvalueCacheEnt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct stackmap {
    pub n: Arc<Mutex<Option<i32>>>,
    pub nbit: Arc<Mutex<Option<i32>>>,
    pub bytedata: Arc<Mutex<Option<[u8; 1]>>>,
}

impl stackmap {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nbit: { let __guard = self.nbit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bytedata: { let __guard = self.bytedata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for stackmap {
    fn default() -> Self {
        Self { n: Arc::new(Mutex::new(Some(0))), nbit: Arc::new(Mutex::new(Some(0))), bytedata: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for stackmap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.n.lock().unwrap().as_ref().unwrap()), (*self.nbit.lock().unwrap().as_ref().unwrap()), format_slice(&self.bytedata))
    }
}

impl GoJsonDecode for stackmap {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub reuse: Arc<Mutex<Option<Vec<liveUserArenaChunk>>>>,
    pub fault: Arc<Mutex<Option<Vec<liveUserArenaChunk>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reuse: self.reuse.clone(), fault: self.fault.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), reuse: Arc::new(Mutex::new(None)), fault: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.reuse), format_slice(&self.fault))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct10 {
    pub enabled: Arc<Mutex<Option<bool>>>,
    pub pad: Arc<Mutex<Option<[u8; 3]>>>,
    pub alignme: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct10 {
    pub fn __go_value_clone(&self) -> Self {
        Self { enabled: { let __guard = self.enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alignme: { let __guard = self.alignme.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct10 {
    fn default() -> Self {
        Self { enabled: Arc::new(Mutex::new(Some(false))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), alignme: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.enabled.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad), (*self.alignme.lock().unwrap().as_ref().unwrap()))
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
        Self { spin_after_ragged_barrier: { let __guard = self.spin_after_ragged_barrier.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, restarted_due_to27993: { let __guard = self.restarted_due_to27993.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct11 {
    fn default() -> Self {
        Self { spin_after_ragged_barrier: Arc::new(Mutex::new(Some(Default::default()))), restarted_due_to27993: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.spin_after_ragged_barrier.lock().unwrap().as_ref().unwrap()), (*self.restarted_due_to27993.lock().unwrap().as_ref().unwrap()))
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
        Self {  }
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


#[derive(Debug, Clone)]
pub struct AnonymousStruct13 {
    pub fill: Arc<Mutex<Option<u64>>>,
    pub capacity: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct13 {
    pub fn __go_value_clone(&self) -> Self {
        Self { fill: { let __guard = self.fill.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, capacity: { let __guard = self.capacity.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct13 {
    fn default() -> Self {
        Self { fill: Arc::new(Mutex::new(Some(0))), capacity: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.fill.lock().unwrap().as_ref().unwrap()), (*self.capacity.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct13 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
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
        Self { gc_percent_goal: { let __guard = self.gc_percent_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, memory_limit_goal: { let __guard = self.memory_limit_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, assist_time: { let __guard = self.assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, background_time: { let __guard = self.background_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct14 {
    fn default() -> Self {
        Self { gc_percent_goal: Arc::new(Mutex::new(Some(Default::default()))), memory_limit_goal: Arc::new(Mutex::new(Some(Default::default()))), assist_time: Arc::new(Mutex::new(Some(Default::default()))), background_time: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.gc_percent_goal.lock().unwrap().as_ref().unwrap()), (*self.memory_limit_goal.lock().unwrap().as_ref().unwrap()), (*self.assist_time.lock().unwrap().as_ref().unwrap()), (*self.background_time.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct14 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct15 {
    pub index: Arc<Mutex<Option<scavengeIndex>>>,
    pub released_bg: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub released_eager: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
}
impl AnonymousStruct15 {
    pub fn __go_value_clone(&self) -> Self {
        Self { index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, released_bg: { let __guard = self.released_bg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, released_eager: { let __guard = self.released_eager.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct15 {
    fn default() -> Self {
        Self { index: Arc::new(Mutex::new(Some(scavengeIndex::default()))), released_bg: Arc::new(Mutex::new(Some(Default::default()))), released_eager: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct15 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.index.lock().unwrap().as_ref().unwrap()), (*self.released_bg.lock().unwrap().as_ref().unwrap()), (*self.released_eager.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct15 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct16 {
    pub base: Arc<Mutex<Option<usize>>>,
    pub end: Arc<Mutex<Option<usize>>>,
}
impl AnonymousStruct16 {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct16 {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(0))), end: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct17 {
    pub mcentral: Arc<Mutex<Option<mcentral>>>,
    pub pad: Arc<Mutex<Option<[u8; 88]>>>,
}
impl AnonymousStruct17 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mcentral: { let __guard = self.mcentral.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct17 {
    fn default() -> Self {
        Self { mcentral: Arc::new(Mutex::new(Some(mcentral::default()))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for AnonymousStruct17 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.mcentral.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad))
    }
}

impl GoJsonDecode for AnonymousStruct17 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct18 {
    pub arena_hints: GoPtr<crate::mheap::arenaHint>,
    pub quarantine_list: Arc<Mutex<Option<mSpanList>>>,
    pub ready_list: Arc<Mutex<Option<mSpanList>>>,
}
impl AnonymousStruct18 {
    pub fn __go_value_clone(&self) -> Self {
        Self { arena_hints: self.arena_hints.clone(), quarantine_list: { let __guard = self.quarantine_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ready_list: { let __guard = self.ready_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct18 {
    fn default() -> Self {
        Self { arena_hints: GoPtr::nil(), quarantine_list: Arc::new(Mutex::new(Some(mSpanList::default()))), ready_list: Arc::new(Mutex::new(Some(mSpanList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct18 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { if self.arena_hints.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.quarantine_list.lock().unwrap().as_ref().unwrap()), (*self.ready_list.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct18 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct19 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: GoPtr<crate::mheap::gcBitsArena>,
    pub next: Arc<Mutex<Option<gcBitsArena>>>,
    pub current: Arc<Mutex<Option<gcBitsArena>>>,
    pub previous: Arc<Mutex<Option<gcBitsArena>>>,
}
impl AnonymousStruct19 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: self.free.clone(), next: self.next.clone(), current: self.current.clone(), previous: self.previous.clone() }
    }
}


impl Default for AnonymousStruct19 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), free: GoPtr::nil(), next: Arc::new(Mutex::new(None)), current: Arc::new(Mutex::new(None)), previous: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct19 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { if self.free.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.current.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.previous.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct19 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct2 {
    pub debug_log_reader: Arc<Mutex<Option<debugLogReader>>>,
    pub first: Arc<Mutex<Option<bool>>>,
    pub lost: Arc<Mutex<Option<u64>>>,
    pub next_tick: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { debug_log_reader: { let __guard = self.debug_log_reader.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, first: { let __guard = self.first.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lost: { let __guard = self.lost.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next_tick: { let __guard = self.next_tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct2 {
    pub fn header(&mut self) -> (u64, u64, u64, i32) {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.header()
    }

    pub fn peek(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.peek()
    }

    pub fn print_val(&mut self) -> bool {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.print_val()
    }

    pub fn read_uint16_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u16 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_uint16_l_e_at(pos)
    }

    pub fn read_uint64_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_uint64_l_e_at(pos)
    }

    pub fn skip(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.skip()
    }

    pub fn uvarint(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.uvarint()
    }

    pub fn varint(&mut self) -> i64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.varint()
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { debug_log_reader: Arc::new(Mutex::new(Some(debugLogReader::default()))), first: Arc::new(Mutex::new(Some(false))), lost: Arc::new(Mutex::new(Some(0))), next_tick: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.debug_log_reader.lock().unwrap().as_ref().unwrap()), (*self.first.lock().unwrap().as_ref().unwrap()), (*self.lost.lock().unwrap().as_ref().unwrap()), (*self.next_tick.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct20 {
    pub base: Arc<Mutex<Option<offAddr>>>,
    pub bound: Arc<Mutex<Option<offAddr>>>,
}
impl AnonymousStruct20 {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bound: { let __guard = self.bound.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct20 {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(offAddr::default()))), bound: Arc::new(Mutex::new(Some(offAddr::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct20 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), (*self.bound.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct20 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct21 {
    pub sema: Arc<Mutex<Option<u32>>>,
    pub active: Arc<Mutex<Option<bool>>>,
    pub offset: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub records: Arc<Mutex<Option<Vec<internal_profilerecord::r#mod::StackRecord>>>>,
    pub labels: Arc<Mutex<Option<Vec<usize>>>>,
}
impl AnonymousStruct21 {
    pub fn __go_value_clone(&self) -> Self {
        Self { sema: { let __guard = self.sema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, active: { let __guard = self.active.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, offset: { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, records: self.records.clone(), labels: self.labels.clone() }
    }
}


impl Default for AnonymousStruct21 {
    fn default() -> Self {
        Self { sema: Arc::new(Mutex::new(Some(0))), active: Arc::new(Mutex::new(Some(false))), offset: Arc::new(Mutex::new(Some(Default::default()))), records: Arc::new(Mutex::new(None)), labels: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct21 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.sema.lock().unwrap().as_ref().unwrap()), (*self.active.lock().unwrap().as_ref().unwrap()), (*self.offset.lock().unwrap().as_ref().unwrap()), format_slice(&self.records), format_slice(&self.labels))
    }
}

impl GoJsonDecode for AnonymousStruct21 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
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
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, newm: { let __guard = self.newm.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waiting: { let __guard = self.waiting.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wake: { let __guard = self.wake.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, have_template_thread: { let __guard = self.have_template_thread.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct22 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), newm: Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0))))))), waiting: Arc::new(Mutex::new(Some(false))), wake: Arc::new(Mutex::new(Some(note::default()))), have_template_thread: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.newm.lock().unwrap().as_ref().unwrap()), (*self.waiting.lock().unwrap().as_ref().unwrap()), (*self.wake.lock().unwrap().as_ref().unwrap()), (*self.have_template_thread.lock().unwrap().as_ref().unwrap()))
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
        Self { signal_lock: { let __guard = self.signal_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hz: { let __guard = self.hz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct23 {
    fn default() -> Self {
        Self { signal_lock: Arc::new(Mutex::new(Some(Default::default()))), hz: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct23 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.signal_lock.lock().unwrap().as_ref().unwrap()), (*self.hz.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct23 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct24 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub seed: Arc<Mutex<Option<[u8; 32]>>>,
    pub state: Arc<Mutex<Option<internal_chacha8rand::chacha8::State>>>,
    pub init: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct24 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, seed: { let __guard = self.seed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, init: { let __guard = self.init.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct24 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), seed: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), state: Arc::new(Mutex::new(Some(Default::default()))), init: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.seed), (*self.state.lock().unwrap().as_ref().unwrap()), (*self.init.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct24 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct25 {
    pub cgocheck: Arc<Mutex<Option<i32>>>,
    pub clobberfree: Arc<Mutex<Option<i32>>>,
    pub disablethp: Arc<Mutex<Option<i32>>>,
    pub dontfreezetheworld: Arc<Mutex<Option<i32>>>,
    pub efence: Arc<Mutex<Option<i32>>>,
    pub gccheckmark: Arc<Mutex<Option<i32>>>,
    pub gcpacertrace: Arc<Mutex<Option<i32>>>,
    pub gcshrinkstackoff: Arc<Mutex<Option<i32>>>,
    pub gcstoptheworld: Arc<Mutex<Option<i32>>>,
    pub gctrace: Arc<Mutex<Option<i32>>>,
    pub invalidptr: Arc<Mutex<Option<i32>>>,
    pub madvdontneed: Arc<Mutex<Option<i32>>>,
    pub runtime_contention_stacks: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub scavtrace: Arc<Mutex<Option<i32>>>,
    pub scheddetail: Arc<Mutex<Option<i32>>>,
    pub schedtrace: Arc<Mutex<Option<i32>>>,
    pub tracebackancestors: Arc<Mutex<Option<i32>>>,
    pub asyncpreemptoff: Arc<Mutex<Option<i32>>>,
    pub harddecommit: Arc<Mutex<Option<i32>>>,
    pub adaptivestackstart: Arc<Mutex<Option<i32>>>,
    pub tracefpunwindoff: Arc<Mutex<Option<i32>>>,
    pub traceadvanceperiod: Arc<Mutex<Option<i32>>>,
    pub trace_check_stack_ownership: Arc<Mutex<Option<i32>>>,
    pub profstackdepth: Arc<Mutex<Option<i32>>>,
    pub dataindependenttiming: Arc<Mutex<Option<i32>>>,
    pub malloc: Arc<Mutex<Option<bool>>>,
    pub inittrace: Arc<Mutex<Option<i32>>>,
    pub sbrk: Arc<Mutex<Option<i32>>>,
    pub traceallocfree: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub panicnil: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub asynctimerchan: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct25 {
    pub fn __go_value_clone(&self) -> Self {
        Self { cgocheck: { let __guard = self.cgocheck.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, clobberfree: { let __guard = self.clobberfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, disablethp: { let __guard = self.disablethp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dontfreezetheworld: { let __guard = self.dontfreezetheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, efence: { let __guard = self.efence.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gccheckmark: { let __guard = self.gccheckmark.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcpacertrace: { let __guard = self.gcpacertrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcshrinkstackoff: { let __guard = self.gcshrinkstackoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcstoptheworld: { let __guard = self.gcstoptheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gctrace: { let __guard = self.gctrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, invalidptr: { let __guard = self.invalidptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, madvdontneed: { let __guard = self.madvdontneed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runtime_contention_stacks: { let __guard = self.runtime_contention_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scavtrace: { let __guard = self.scavtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scheddetail: { let __guard = self.scheddetail.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, schedtrace: { let __guard = self.schedtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracebackancestors: { let __guard = self.tracebackancestors.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, asyncpreemptoff: { let __guard = self.asyncpreemptoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, harddecommit: { let __guard = self.harddecommit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, adaptivestackstart: { let __guard = self.adaptivestackstart.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracefpunwindoff: { let __guard = self.tracefpunwindoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceadvanceperiod: { let __guard = self.traceadvanceperiod.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, trace_check_stack_ownership: { let __guard = self.trace_check_stack_ownership.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, profstackdepth: { let __guard = self.profstackdepth.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dataindependenttiming: { let __guard = self.dataindependenttiming.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, malloc: { let __guard = self.malloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, inittrace: { let __guard = self.inittrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sbrk: { let __guard = self.sbrk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceallocfree: { let __guard = self.traceallocfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, panicnil: { let __guard = self.panicnil.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, asynctimerchan: { let __guard = self.asynctimerchan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct25 {
    fn default() -> Self {
        Self { cgocheck: Arc::new(Mutex::new(Some(0))), clobberfree: Arc::new(Mutex::new(Some(0))), disablethp: Arc::new(Mutex::new(Some(0))), dontfreezetheworld: Arc::new(Mutex::new(Some(0))), efence: Arc::new(Mutex::new(Some(0))), gccheckmark: Arc::new(Mutex::new(Some(0))), gcpacertrace: Arc::new(Mutex::new(Some(0))), gcshrinkstackoff: Arc::new(Mutex::new(Some(0))), gcstoptheworld: Arc::new(Mutex::new(Some(0))), gctrace: Arc::new(Mutex::new(Some(0))), invalidptr: Arc::new(Mutex::new(Some(0))), madvdontneed: Arc::new(Mutex::new(Some(0))), runtime_contention_stacks: Arc::new(Mutex::new(Some(Default::default()))), scavtrace: Arc::new(Mutex::new(Some(0))), scheddetail: Arc::new(Mutex::new(Some(0))), schedtrace: Arc::new(Mutex::new(Some(0))), tracebackancestors: Arc::new(Mutex::new(Some(0))), asyncpreemptoff: Arc::new(Mutex::new(Some(0))), harddecommit: Arc::new(Mutex::new(Some(0))), adaptivestackstart: Arc::new(Mutex::new(Some(0))), tracefpunwindoff: Arc::new(Mutex::new(Some(0))), traceadvanceperiod: Arc::new(Mutex::new(Some(0))), trace_check_stack_ownership: Arc::new(Mutex::new(Some(0))), profstackdepth: Arc::new(Mutex::new(Some(0))), dataindependenttiming: Arc::new(Mutex::new(Some(0))), malloc: Arc::new(Mutex::new(Some(false))), inittrace: Arc::new(Mutex::new(Some(0))), sbrk: Arc::new(Mutex::new(Some(0))), traceallocfree: Arc::new(Mutex::new(Some(Default::default()))), panicnil: Arc::new(Mutex::new(Some(Default::default()))), asynctimerchan: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct25 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.cgocheck.lock().unwrap().as_ref().unwrap()), (*self.clobberfree.lock().unwrap().as_ref().unwrap()), (*self.disablethp.lock().unwrap().as_ref().unwrap()), (*self.dontfreezetheworld.lock().unwrap().as_ref().unwrap()), (*self.efence.lock().unwrap().as_ref().unwrap()), (*self.gccheckmark.lock().unwrap().as_ref().unwrap()), (*self.gcpacertrace.lock().unwrap().as_ref().unwrap()), (*self.gcshrinkstackoff.lock().unwrap().as_ref().unwrap()), (*self.gcstoptheworld.lock().unwrap().as_ref().unwrap()), (*self.gctrace.lock().unwrap().as_ref().unwrap()), (*self.invalidptr.lock().unwrap().as_ref().unwrap()), (*self.madvdontneed.lock().unwrap().as_ref().unwrap()), (*self.runtime_contention_stacks.lock().unwrap().as_ref().unwrap()), (*self.scavtrace.lock().unwrap().as_ref().unwrap()), (*self.scheddetail.lock().unwrap().as_ref().unwrap()), (*self.schedtrace.lock().unwrap().as_ref().unwrap()), (*self.tracebackancestors.lock().unwrap().as_ref().unwrap()), (*self.asyncpreemptoff.lock().unwrap().as_ref().unwrap()), (*self.harddecommit.lock().unwrap().as_ref().unwrap()), (*self.adaptivestackstart.lock().unwrap().as_ref().unwrap()), (*self.tracefpunwindoff.lock().unwrap().as_ref().unwrap()), (*self.traceadvanceperiod.lock().unwrap().as_ref().unwrap()), (*self.trace_check_stack_ownership.lock().unwrap().as_ref().unwrap()), (*self.profstackdepth.lock().unwrap().as_ref().unwrap()), (*self.dataindependenttiming.lock().unwrap().as_ref().unwrap()), (*self.malloc.lock().unwrap().as_ref().unwrap()), (*self.inittrace.lock().unwrap().as_ref().unwrap()), (*self.sbrk.lock().unwrap().as_ref().unwrap()), (*self.traceallocfree.lock().unwrap().as_ref().unwrap()), (*self.panicnil.lock().unwrap().as_ref().unwrap()), (*self.asynctimerchan.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct25 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct26 {
    pub g_list: Arc<Mutex<Option<gList>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct26 {
    pub fn __go_value_clone(&self) -> Self {
        Self { g_list: { let __guard = self.g_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct26 {
    pub fn empty(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.empty()
    }

    pub fn pop(&mut self) -> GoPtr<crate::runtime2::g> {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.pop()
    }

    pub fn push(&self, gp: GoPtr<crate::runtime2::g>) {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.push(gp)
    }

    pub fn push_all(&mut self, q: Arc<Mutex<Option<gQueue>>>) {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.push_all(q)
    }
}


impl Default for AnonymousStruct26 {
    fn default() -> Self {
        Self { g_list: Arc::new(Mutex::new(Some(gList::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct26 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.g_list.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct26 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct27 {
    pub len: Arc<Mutex<Option<i32>>>,
    pub buf: Arc<Mutex<Option<[GoPtr<crate::mheap::mspan>; 128]>>>,
}
impl AnonymousStruct27 {
    pub fn __go_value_clone(&self) -> Self {
        Self { len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, buf: { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct27 {
    fn default() -> Self {
        Self { len: Arc::new(Mutex::new(Some(0))), buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| GoPtr::nil())))) }
    }
}

impl std::fmt::Display for AnonymousStruct27 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.len.lock().unwrap().as_ref().unwrap()), { let __guard = self.buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", __v.iter().map(|__p| if __p.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() }).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct27 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct28 {
    pub user: Arc<Mutex<Option<bool>>>,
    pub runnable: Arc<Mutex<Option<gQueue>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct28 {
    pub fn __go_value_clone(&self) -> Self {
        Self { user: { let __guard = self.user.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runnable: { let __guard = self.runnable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct28 {
    fn default() -> Self {
        Self { user: Arc::new(Mutex::new(Some(false))), runnable: Arc::new(Mutex::new(Some(gQueue::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct28 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.user.lock().unwrap().as_ref().unwrap()), (*self.runnable.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct28 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct29 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub stack: Arc<Mutex<Option<gList>>>,
    pub no_stack: Arc<Mutex<Option<gList>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct29 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stack: { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, no_stack: { let __guard = self.no_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct29 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), stack: Arc::new(Mutex::new(Some(gList::default()))), no_stack: Arc::new(Mutex::new(Some(gList::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct29 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.stack.lock().unwrap().as_ref().unwrap()), (*self.no_stack.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct29 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub tick: Arc<Mutex<Option<u64>>>,
    pub i: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        Self { tick: { let __guard = self.tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, i: { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { tick: Arc::new(Mutex::new(Some(0))), i: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tick.lock().unwrap().as_ref().unwrap()), (*self.i.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct3 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct30 {
    pub root: Arc<Mutex<Option<semaRoot>>>,
    pub pad: Arc<Mutex<Option<[u8; 104]>>>,
}
impl AnonymousStruct30 {
    pub fn __go_value_clone(&self) -> Self {
        Self { root: { let __guard = self.root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct30 {
    fn default() -> Self {
        Self { root: Arc::new(Mutex::new(Some(semaRoot::default()))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for AnonymousStruct30 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.root.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad))
    }
}

impl GoJsonDecode for AnonymousStruct30 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct31 {
    pub note: Arc<Mutex<Option<note>>>,
    pub mask: Arc<Mutex<Option<[u32; 1]>>>,
    pub wanted: Arc<Mutex<Option<[u32; 1]>>>,
    pub ignored: Arc<Mutex<Option<[u32; 1]>>>,
    pub recv: Arc<Mutex<Option<[u32; 1]>>>,
    pub state: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub delivering: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub inuse: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct31 {
    pub fn __go_value_clone(&self) -> Self {
        Self { note: { let __guard = self.note.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mask: { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wanted: { let __guard = self.wanted.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ignored: { let __guard = self.ignored.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: { let __guard = self.recv.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, delivering: { let __guard = self.delivering.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, inuse: { let __guard = self.inuse.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct31 {
    fn default() -> Self {
        Self { note: Arc::new(Mutex::new(Some(note::default()))), mask: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), wanted: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), ignored: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), recv: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), state: Arc::new(Mutex::new(Some(Default::default()))), delivering: Arc::new(Mutex::new(Some(Default::default()))), inuse: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct31 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.note.lock().unwrap().as_ref().unwrap()), format_slice(&self.mask), format_slice(&self.wanted), format_slice(&self.ignored), format_slice(&self.recv), (*self.state.lock().unwrap().as_ref().unwrap()), (*self.delivering.lock().unwrap().as_ref().unwrap()), (*self.inuse.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct31 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct32 {
    pub item: Arc<Mutex<Option<stackpoolItem>>>,
    pub __blank_1_0: Arc<Mutex<Option<[u8; 104]>>>,
}
impl AnonymousStruct32 {
    pub fn __go_value_clone(&self) -> Self {
        Self { item: { let __guard = self.item.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_1_0: { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct32 {
    fn default() -> Self {
        Self { item: Arc::new(Mutex::new(Some(stackpoolItem::default()))), __blank_1_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for AnonymousStruct32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.item.lock().unwrap().as_ref().unwrap()), format_slice(&self.__blank_1_0))
    }
}

impl GoJsonDecode for AnonymousStruct32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct33 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: Arc<Mutex<Option<[mSpanList; 35]>>>,
}
impl AnonymousStruct33 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: { let __guard = self.free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct33 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), free: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for AnonymousStruct33 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.free))
    }
}

impl GoJsonDecode for AnonymousStruct33 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct4 {
    pub mutex: Arc<Mutex<Option<mutex>>>,
    pub persistent_alloc: Arc<Mutex<Option<persistentAlloc>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: { let __guard = self.mutex.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, persistent_alloc: { let __guard = self.persistent_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        Self { mutex: Arc::new(Mutex::new(Some(mutex::default()))), persistent_alloc: Arc::new(Mutex::new(Some(persistentAlloc::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.mutex.lock().unwrap().as_ref().unwrap()), (*self.persistent_alloc.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct4 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct5 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub data: Arc<Mutex<Option<u8>>>,
}
impl AnonymousStruct5 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), data: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { let __guard = self.data.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct5 {
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
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: { let __guard = self.free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, busy: { let __guard = self.busy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), free: Arc::new(Mutex::new(Some(mSpanList::default()))), busy: Arc::new(Mutex::new(Some(mSpanList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.free.lock().unwrap().as_ref().unwrap()), (*self.busy.lock().unwrap().as_ref().unwrap()))
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
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, q: { let __guard = self.q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), q: Arc::new(Mutex::new(Some(gQueue::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.q.lock().unwrap().as_ref().unwrap()))
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
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, list: { let __guard = self.list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct8 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), list: Arc::new(Mutex::new(Some(gList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.list.lock().unwrap().as_ref().unwrap()))
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
        Self { block: { let __guard = self.block.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, q: { let __guard = self.q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct9 {
    fn default() -> Self {
        Self { block: Arc::new(Mutex::new(Some(false))), lock: Arc::new(Mutex::new(Some(mutex::default()))), q: Arc::new(Mutex::new(Some(gQueue::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.block.lock().unwrap().as_ref().unwrap()), (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.q.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct9 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type debug = AnonymousStruct25;


pub(crate) type debugPtrmask = AnonymousStruct5;


pub(crate) type gcBitsArenas = AnonymousStruct19;


pub(crate) type gcDebugMarkDone = AnonymousStruct11;


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) type globalRand = AnonymousStruct24;


pub(crate) type goroutineProfile = AnonymousStruct21;


pub(crate) type newmHandoff = AnonymousStruct22;


pub(crate) type prof = AnonymousStruct23;


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) type sig = AnonymousStruct31;


pub(crate) type stackLarge = AnonymousStruct33;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static pinnedTypemaps: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<BTreeMap<internal_abi::r#type::TypeOff, Arc<Mutex<Option<internal_abi::r#type::Type>>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static aixStaticDataBase: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static firstmoduledata: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<moduledata>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lastmoduledatap: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<moduledata>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static modulesSlice: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Vec<Arc<Mutex<Option<moduledata>>>>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *pinnedTypemaps.lock().unwrap() = Some(vec![]);
    *aixStaticDataBase.lock().unwrap() = Some(0);
    *firstmoduledata.lock().unwrap() = Some(Default::default());
    *lastmoduledatap.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *modulesSlice.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_zero_globals() {
    *pinnedTypemaps.lock().unwrap() = Some(vec![]);
    *aixStaticDataBase.lock().unwrap() = Some(0);
    *firstmoduledata.lock().unwrap() = Some(Default::default());
    *lastmoduledatap.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *modulesSlice.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


impl Frames {
    /// Next returns a [Frame] representing the next call frame in the slice
    /// of PC values. If it has already returned all call frames, Next
    /// returns a zero [Frame].
    ///
    /// The more result indicates whether the next call to Next will return
    /// a valid [Frame]. It does not necessarily indicate whether this call
    /// returned one.
    ///
    /// See the [Frames] example for idiomatic usage.
    pub fn next(&mut self) -> (Arc<Mutex<Option<Frame>>>, bool) {
    let mut frame: Arc<Mutex<Option<Frame>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut more: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        while { let __tmp_x = (({ let __len_target = { let __field = self.frames.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
                // Find the next frame.
                // We need to look for 2 frames so we know what
                // to return for the "more" result.
        if { let __tmp_x = (({ let __len_target = { let __field = self.callers.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        break
    }
        let mut pc: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = (*self.next_p_c.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let __tmp_0 = { let __selector_holder = self.next_p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_1 = 0; *pc.lock().unwrap() = Some(__tmp_0); *self.next_p_c.lock().unwrap() = Some(__tmp_1 as usize); };
    } else {
        { let __tmp_0 = { let __seq = { let __seq_holder = self.callers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_1 = Arc::new(Mutex::new(Some({ let __seq_holder = self.callers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); *pc.lock().unwrap() = Some(__tmp_0); *self.callers.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
        let mut funcInfo = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if !(*funcInfo.lock().unwrap().as_ref().unwrap()).valid() {
        if { let __nil_result = (*cgoSymbolizer.lock().unwrap()).is_some(); __nil_result } {
                // Pre-expand cgo frames. We could do this
                // incrementally, too, but there's no way to
                // avoid allocation in this case anyway.
        { let new_val = { let __append_target = self.frames.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = expand_cgo_frames(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; self.frames = new_val; };
    }
                // Pre-expand cgo frames. We could do this
                // incrementally, too, but there's no way to
                // avoid allocation in this case anyway.
        continue
    }
                // Pre-expand cgo frames. We could do this
                // incrementally, too, but there's no way to
                // avoid allocation in this case anyway.
        let mut f: GoPtr<Func> = (*funcInfo.lock().unwrap().as_ref().unwrap()).__func();
        let mut entry = { let __recv_value = f.borrow(); let __result = (*__recv_value.as_ref().unwrap()).entry(); __result };

                // We store the pc of the start of the instruction following
                // the instruction in question (the call or the inline mark).
                // This is done for historical reasons, and to make FuncForPC
                // work correctly for entries in the result of runtime.Callers.
                // Decrement to get back to the instruction we care about.
                //
                // It is not possible to get pc == entry from runtime.Callers,
                // but if the caller does provide one, provide best-effort
                // results by avoiding backing out of the function entirely.
        if { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = entry; __tmp_x > __tmp_y } {
        { let mut guard = pc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

                // It's important that interpret pc non-strictly as cgoTraceback may
                // have added bogus PCs with a valid funcInfo but invalid PCDATA.
        let (mut u, mut uf) = new_inline_unwinder(Arc::new(Mutex::new(Some({ let __arg_holder = funcInfo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut sf = (*u.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*u.lock().unwrap().as_ref().unwrap()).is_inlined(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // Note: entry is not modified. It always refers to a real frame, not an inlined one.
                // File/line from funcline1 below are already correct.
        f = GoPtr::nil();
                // When CallersFrame is invoked using the PC list returned by Callers,
                // the PC list includes virtual PCs corresponding to each outer frame
                // around an innermost real inlined PC.
                // We also want to support code passing in a PC list extracted from a
                // stack trace, and there only the real PCs are printed, not the virtual ones.
                // So check to see if the implied virtual PC for this PC (obtained from the
                // unwinder itself) is the next PC in ci.callers. If not, insert it.
                // The +1 here correspond to the pc-- above: the output of Callers
                // and therefore the input to CallersFrames is return PCs from the stack;
                // The pc-- backs up into the CALL instruction (not the first byte of the CALL
                // instruction, but good enough to find it nonetheless).
                // There are no cycles in implied virtual PCs (some number of frames were
                // inlined, but that number is finite), so this unpacking cannot cause an infinite loop.
        let mut unext = (*u.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    while (*unext.lock().unwrap().as_ref().unwrap()).valid() && { let __tmp_x = (({ let __len_target = { let __field = self.callers.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = self.callers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = { let __tmp_x = (*{ let __field = (*unext.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x + __tmp_y }; __tmp_x != __tmp_y } {
        let mut snext = (*u.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = unext.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = { let __selector_holder = (*snext.lock().unwrap().as_ref().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_WRAPPER as u8)))); __tmp_x == __tmp_y } && elide_wrapper_calling(Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some((*(*(*sf.lock().unwrap().as_ref().unwrap()).func_i_d.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))) {
                // Skip, because tracebackPCs (inside runtime.Callers) would too.
        { let new_val = (*u.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = unext.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *unext.lock().unwrap() = __moved_val; };; continue
    }
                // Skip, because tracebackPCs (inside runtime.Callers) would too.
        { let new_val = { let __tmp_x = (*{ let __field = (*unext.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x + __tmp_y }; *self.next_p_c.lock().unwrap() = Some(new_val); };
        break;
        { let new_val = (*u.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = unext.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *unext.lock().unwrap() = __moved_val; };
    }
    }
                // Note: entry is not modified. It always refers to a real frame, not an inlined one.
                // File/line from funcline1 below are already correct.
                // When CallersFrame is invoked using the PC list returned by Callers,
                // the PC list includes virtual PCs corresponding to each outer frame
                // around an innermost real inlined PC.
                // We also want to support code passing in a PC list extracted from a
                // stack trace, and there only the real PCs are printed, not the virtual ones.
                // So check to see if the implied virtual PC for this PC (obtained from the
                // unwinder itself) is the next PC in ci.callers. If not, insert it.
                // The +1 here correspond to the pc-- above: the output of Callers
                // and therefore the input to CallersFrames is return PCs from the stack;
                // The pc-- backs up into the CALL instruction (not the first byte of the CALL
                // instruction, but good enough to find it nonetheless).
                // There are no cycles in implied virtual PCs (some number of frames were
                // inlined, but that number is finite), so this unpacking cannot cause an infinite loop.
                // Skip, because tracebackPCs (inside runtime.Callers) would too.
        { let new_val = { let __append_target = self.frames.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Frame { p_c: Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), func: f.clone(), function: func_name_for_print((*sf.lock().unwrap().as_ref().unwrap()).name()), entry: Arc::new(Mutex::new(Some(entry))), start_line: Arc::new(Mutex::new(Some({ let __selector_holder = (*sf.lock().unwrap().as_ref().unwrap()).start_line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))), func_info: Arc::new(Mutex::new(Some({ let __arg_holder = funcInfo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }); __append_target.clone() }; self.frames = new_val; };
    }
                // Find the next frame.
                // We need to look for 2 frames so we know what
                // to return for the "more" result.
                // Pre-expand cgo frames. We could do this
                // incrementally, too, but there's no way to
                // avoid allocation in this case anyway.
                // We store the pc of the start of the instruction following
                // the instruction in question (the call or the inline mark).
                // This is done for historical reasons, and to make FuncForPC
                // work correctly for entries in the result of runtime.Callers.
                // Decrement to get back to the instruction we care about.
                //
                // It is not possible to get pc == entry from runtime.Callers,
                // but if the caller does provide one, provide best-effort
                // results by avoiding backing out of the function entirely.
                // It's important that interpret pc non-strictly as cgoTraceback may
                // have added bogus PCs with a valid funcInfo but invalid PCDATA.
                // Note: entry is not modified. It always refers to a real frame, not an inlined one.
                // File/line from funcline1 below are already correct.
                // When CallersFrame is invoked using the PC list returned by Callers,
                // the PC list includes virtual PCs corresponding to each outer frame
                // around an innermost real inlined PC.
                // We also want to support code passing in a PC list extracted from a
                // stack trace, and there only the real PCs are printed, not the virtual ones.
                // So check to see if the implied virtual PC for this PC (obtained from the
                // unwinder itself) is the next PC in ci.callers. If not, insert it.
                // The +1 here correspond to the pc-- above: the output of Callers
                // and therefore the input to CallersFrames is return PCs from the stack;
                // The pc-- backs up into the CALL instruction (not the first byte of the CALL
                // instruction, but good enough to find it nonetheless).
                // There are no cycles in implied virtual PCs (some number of frames were
                // inlined, but that number is finite), so this unpacking cannot cause an infinite loop.
                // Skip, because tracebackPCs (inside runtime.Callers) would too.
                // Note: File,Line set below
                // Pop one frame from the frame list. Keep the rest.
                // Avoid allocation in the common case, which is 1 or 2 frames.
        { let _switch_val = ({ let __len_target = { let __field = self.frames.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) });
    if _switch_val == (0) {
            return (frame.clone(), (*more.lock().unwrap().as_ref().unwrap()));
        } else if _switch_val == (1) {
            { let new_val = { let __seq = { let __seq_holder = self.frames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; *frame.lock().unwrap() = Some(new_val); };
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.frame_store.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.frames = new_val; };
        } else if _switch_val == (2) {
            { let new_val = { let __seq = { let __seq_holder = self.frames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; *frame.lock().unwrap() = Some(new_val); };
            (*self.frame_store.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __seq = { let __seq_holder = self.frames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() };
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.frame_store.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (1) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.frames = new_val; };
        } else {
            { let new_val = { let __seq = { let __seq_holder = self.frames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; *frame.lock().unwrap() = Some(new_val); };
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.frames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.frames = new_val; };
        }
    }
                // In the rare case when there are no frames at all, we return Frame{}.
        { let new_val = { let __tmp_x = (({ let __len_target = { let __field = self.frames.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y }; *more.lock().unwrap() = Some(new_val); };
        if (*(*frame.lock().unwrap().as_ref().unwrap()).func_info.lock().unwrap().as_ref().unwrap()).valid() {
                // Compute file/line just before we need to return it,
                // as it can be expensive. This avoids computing file/line
                // for the Frame we find but don't return. See issue 32093.
        let (mut file, mut line) = funcline1(Arc::new(Mutex::new(Some((*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).func_info.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false))));
        { let __tmp_0 = (*file.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = Arc::new(Mutex::new(Some(line as i32))); *(*frame.lock().unwrap().as_ref().unwrap()).file.lock().unwrap() = Some(__tmp_0); *(*frame.lock().unwrap().as_ref().unwrap()).line.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
                // Compute file/line just before we need to return it,
                // as it can be expensive. This avoids computing file/line
                // for the Frame we find but don't return. See issue 32093.
        return (frame.clone(), (*more.lock().unwrap().as_ref().unwrap()));
    }
}

impl Func {
    pub fn raw(&self) -> GoPtr<crate::runtime2::_func> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    pub fn func_info(&self) -> Arc<Mutex<Option<funcInfo>>> {
        { let __recv = self.raw(); let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).func_info(); __result }
    }

    /// Name returns the name of the function.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        if false {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let mut r#fn: GoPtr<crate::runtime2::_func> = self.raw();
        if { let __recv_value = r#fn.borrow(); let __result = (*__recv_value.as_ref().unwrap()).is_inlined(); __result } {
        let mut fi: GoPtr<crate::runtime2::funcinl> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(r#fn.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        return func_name_for_print(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = fi.with_mut(|__ptr_value| __ptr_value.name.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        func_name_for_print(funcname(self.func_info()))
    }

    /// Entry returns the entry address of the function.
    pub fn entry(&self) -> usize {
        let mut r#fn: GoPtr<crate::runtime2::_func> = self.raw();
        if { let __recv_value = r#fn.borrow(); let __result = (*__recv_value.as_ref().unwrap()).is_inlined(); __result } {
        let mut fi: GoPtr<crate::runtime2::funcinl> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(r#fn.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        return (*{ let __ptr_value = fi.with_mut(|__ptr_value| __ptr_value.entry.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap());
    }
        { let __recv = { let __result = r#fn.with_mut(|__recv_value| __recv_value.func_info()); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).entry(); __result }
    }

    /// FileLine returns the file name and line number of the
    /// source code corresponding to the program counter pc.
    /// The result will not be accurate if pc is not a program
    /// counter within f.
    pub fn file_line(&self, pc: Arc<Mutex<Option<usize>>>) -> (Arc<Mutex<Option<String>>>, i32) {
    let mut file: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut line: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        let mut r#fn: GoPtr<crate::runtime2::_func> = self.raw();
        if { let __recv_value = r#fn.borrow(); let __result = (*__recv_value.as_ref().unwrap()).is_inlined(); __result } {
        let mut fi: GoPtr<crate::runtime2::funcinl> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(r#fn.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        return ({ let __return_value_0 = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = fi.with_mut(|__ptr_value| __ptr_value.file.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); __return_value_0 }, (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = fi.with_mut(|__ptr_value| __ptr_value.line.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()));
    }
                // Pass strict=false here, because anyone can call this function,
                // and they might just be wrong about targetpc belonging to f.
        let (__tmp_0, mut line32) = funcline1(self.func_info(), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *file.lock().unwrap() = __moved_tmp_0;;
        return ({ let __owned = file.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, (*Arc::new(Mutex::new(Some(line32 as i32))).lock().unwrap().as_ref().unwrap()));
    }

    /// startLine returns the starting line number of the function. i.e., the line
    /// number of the func keyword.
    pub fn start_line(&self) -> i32 {
        let mut r#fn: GoPtr<crate::runtime2::_func> = self.raw();
        if { let __recv_value = r#fn.borrow(); let __result = (*__recv_value.as_ref().unwrap()).is_inlined(); __result } {
        let mut fi: GoPtr<crate::runtime2::funcinl> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(r#fn.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        return (*{ let __ptr_value = fi.with_mut(|__ptr_value| __ptr_value.start_line.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap());
    }
        return (*(*{ let __result = r#fn.with_mut(|__recv_value| __recv_value.func_info()); __result }.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap().start_line.lock().unwrap().as_ref().unwrap());
    }
}

impl crate::runtime2::_func {
    pub fn func_info(&self) -> Arc<Mutex<Option<funcInfo>>> {
                // Find the module containing fn. fn is located in the pclntable.
                // The unsafe.Pointer to uintptr conversions and arithmetic
                // are safe because we are working with module addresses.
        let mut ptr = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self as *const _ as usize))).lock().unwrap().as_ref().unwrap()) as usize)));
        let mut r#mod: Arc<Mutex<Option<moduledata>>> = Arc::new(Mutex::new(None));
        let mut datap = firstmoduledata.clone();
    while { let __nil_result = (*datap.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = (({ let __len_target = { let __field = (*datap.lock().unwrap().as_ref().unwrap()).pclntable.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = (*datap.lock().unwrap().as_ref().unwrap()).next.clone(); datap = new_val; };; continue
    }
        let mut base = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = (*datap.lock().unwrap().as_ref().unwrap()).pclntable.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize)));
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = (*datap.lock().unwrap().as_ref().unwrap()).pclntable.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; __tmp_x < __tmp_y } {
        { let new_val = datap.clone(); r#mod = new_val; };
        break
    }
        { let new_val = (*datap.lock().unwrap().as_ref().unwrap()).next.clone(); datap = new_val; };
    }
        return Arc::new(Mutex::new(Some(funcInfo { _func: Arc::new(Mutex::new(Some(self.clone()))), datap: r#mod.clone(), ..Default::default() })));
    }

    /// isInlined reports whether f should be re-interpreted as a *funcinl.
    pub fn is_inlined(&self) -> bool {
        return { let __tmp_x = (*self.entry_off.lock().unwrap().as_ref().unwrap()); let __tmp_y = !(0 as u32) as u32; __tmp_x == __tmp_y };
    }
}

impl moduledata {
    /// textAddr returns md.text + off, with special handling for multiple text sections.
    /// off is a (virtual) offset computed at internal linking time,
    /// before the external linker adjusts the sections' base addresses.
    ///
    /// The text, or instruction stream is generated as one large buffer.
    /// The off (offset) for a function is its offset within this buffer.
    /// If the total text size gets too large, there can be issues on platforms like ppc64
    /// if the target of calls are too far for the call instruction.
    /// To resolve the large text issue, the text is split into multiple text sections
    /// to allow the linker to generate long calls when necessary.
    /// When this happens, the vaddr for each text section is set to its offset within the text.
    /// Each function's offset is compared against the section vaddrs and ends to determine the containing section.
    /// Then the section relative offset is added to the section's
    /// relocated baseaddr to compute the function address.
    ///
    /// It is nosplit because it is part of the findfunc implementation.
    ///
    ///go:nosplit
    pub fn text_addr(&self, off32: Arc<Mutex<Option<u32>>>) -> usize {
        let mut off = Arc::new(Mutex::new(Some((*off32.lock().unwrap().as_ref().unwrap()) as usize)));
        let mut res = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.text.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        if { let __tmp_x = (({ let __len_target = { let __field = self.textsectmap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
        { let __range_holder = self.textsectmap.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, sect) in __range_values.iter().enumerate() {
                // For the last section, include the end address (etext), as it is included in the functab.
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*sect.vaddr.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*sect.end.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || ({ let __tmp_x = (i as i32); let __tmp_y = ({ let __tmp_x = (({ let __len_target = { let __field = self.textsectmap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*sect.end.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y }) {
        { let new_val = { let __tmp_x = { let __tmp_x = (*sect.baseaddr.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = (*sect.vaddr.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; *res.lock().unwrap() = Some(new_val); };
        break
    }
    } }
                // For the last section, include the end address (etext), as it is included in the functab.
        if { let __tmp_x = { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.etext.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } && { let __tmp_x = "arm64".to_string(); let __tmp_y = "wasm".to_string(); __tmp_x != __tmp_y } {
        eprintln!("{} {} {} {} {} {}", format!("{}", "runtime: textAddr".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*res.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "out of range".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = self.text.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "-".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = self.etext.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))));
        throw(Arc::new(Mutex::new(Some("runtime: text offset out of range".to_string()))));
    }
    }
                // For the last section, include the end address (etext), as it is included in the functab.
                // on wasm, functions do not live in the same address space as the linear memory
        return { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// textOff is the opposite of textAddr. It converts a PC to a (virtual) offset
    /// to md.text, and returns if the PC is in any Go text section.
    ///
    /// It is nosplit because it is part of the findfunc implementation.
    ///
    ///go:nosplit
    pub fn text_off(&self, pc: Arc<Mutex<Option<usize>>>) -> (u32, bool) {
        let mut res = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.text.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as u32)));
        if { let __tmp_x = (({ let __len_target = { let __field = self.textsectmap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
        { let __range_holder = self.textsectmap.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, sect) in __range_values.iter().enumerate() {
        if { let __tmp_x = (*sect.baseaddr.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // pc is not in any section.
        return (0, false);
    }
                // pc is not in any section.
        let mut end = Arc::new(Mutex::new(Some({ let __tmp_x = (*sect.baseaddr.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (*sect.end.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*sect.vaddr.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x + __tmp_y })));
                // For the last section, include the end address (etext), as it is included in the functab.
        if { let __tmp_x = (i as i32); let __tmp_y = ({ let __tmp_x = (({ let __len_target = { let __field = self.textsectmap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x == __tmp_y } {
        { let mut guard = end.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*sect.baseaddr.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = (*sect.vaddr.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        break
    }
    } }
    }
                // pc is not in any section.
                // For the last section, include the end address (etext), as it is included in the functab.
        return ({ let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }

    /// funcName returns the string at nameOff in the function name table.
    pub fn func_name(&self, nameOff: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = { let __v = (*nameOff.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        gostringnocopy(GoPtr::slice_elem(GoSliceElemPtr::new(self.funcnametab.clone(), ({ let __v = (*nameOff.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)))
    }
}

impl funcInfo {
    pub fn valid(&self) -> bool {
        return { let __nil_target = self._func.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
    }

    pub fn __func(&self) -> GoPtr<Func> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&self._func.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    /// entry returns the entry PC for f.
    ///
    /// entry should be an internal detail,
    /// but widely used packages access it using linkname.
    /// Notable members of the hall of shame include:
    ///   - github.com/phuslu/log
    ///
    /// Do not remove or change the type signature.
    /// See go.dev/issue/67401.
    pub fn entry(&self) -> usize {
        (*self.datap.lock().unwrap().as_ref().unwrap()).text_addr(Arc::new(Mutex::new(Some({ let __selector_holder = (*self._func.lock().unwrap().as_ref().unwrap()).entry_off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
    }

    pub fn src_func(&self) -> Arc<Mutex<Option<srcFunc>>> {
        if !self.valid() {
        return Arc::new(Mutex::new(Some(srcFunc { datap: Default::default(), name_off: Arc::new(Mutex::new(Some(0))), start_line: Arc::new(Mutex::new(Some(0))), func_i_d: Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(0))))))) })));
    }
        Arc::new(Mutex::new(Some(srcFunc { datap: { let __field = self.datap.clone(); __field }, name_off: Arc::new(Mutex::new(Some({ let __selector_holder = (*self._func.lock().unwrap().as_ref().unwrap()).name_off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), start_line: Arc::new(Mutex::new(Some({ let __selector_holder = (*self._func.lock().unwrap().as_ref().unwrap()).start_line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), func_i_d: Arc::new(Mutex::new(Some({ let __selector_holder = (*self._func.lock().unwrap().as_ref().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })))
    }

    pub fn func_info(&self) -> Arc<Mutex<Option<funcInfo>>> {
        // Forward to embedded type's method
        let embedded = self._func.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_info()
    }

    pub fn is_inlined(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self._func.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_inlined()
    }
}

impl srcFunc {
    /// name should be an internal detail,
    /// but widely used packages access it using linkname.
    /// Notable members of the hall of shame include:
    ///   - github.com/phuslu/log
    ///
    /// Do not remove or change the type signature.
    /// See go.dev/issue/67401.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        if { let __nil_target = self.datap.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        (*self.datap.lock().unwrap().as_mut().unwrap()).func_name(Arc::new(Mutex::new(Some({ let __selector_holder = self.name_off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
    }
}

/// CallersFrames takes a slice of PC values returned by [Callers] and
/// prepares to return function/file/line information.
/// Do not change the slice until you are done with the [Frames].
pub fn callers_frames(callers: Arc<Mutex<Option<Vec<usize>>>>) -> Arc<Mutex<Option<Frames>>> {
    let mut f = Arc::new(Mutex::new(Some(Frames { callers: callers.clone(), ..Default::default() })));
    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = (*f.lock().unwrap().as_ref().unwrap()).frame_store.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); (*f.lock().unwrap().as_mut().unwrap()).frames = new_val; };
    return f.clone();
}

/// expandCgoFrames expands frame information for pc, known to be
/// a non-Go function, using the cgoSymbolizer hook. expandCgoFrames
/// returns nil if pc could not be expanded.
pub fn expand_cgo_frames(pc: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<Frame>>>> {
    let mut arg = Arc::new(Mutex::new(Some(cgoSymbolizerArg { pc: Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    call_cgo_symbolizer(arg.clone());

    if { let __nil_target = (*arg.lock().unwrap().as_ref().unwrap()).file.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && { let __nil_target = (*arg.lock().unwrap().as_ref().unwrap()).func_name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
                // No useful information from symbolizer.
        return Arc::new(Mutex::new(None));
    }

        // No useful information from symbolizer.
    let mut frames: Arc<Mutex<Option<Vec<Frame>>>> = Arc::new(Mutex::new(None));
    loop {
        { let new_val = { let __append_target = frames.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Frame { p_c: Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), func: GoPtr::nil(), function: gostring({ let __field = (*arg.lock().unwrap().as_ref().unwrap()).func_name.clone(); __field }), file: gostring({ let __field = (*arg.lock().unwrap().as_ref().unwrap()).file.clone(); __field }), line: Arc::new(Mutex::new(Some({ let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).lineno.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))), entry: Arc::new(Mutex::new(Some({ let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).entry.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }); __append_target.clone() }; frames = new_val; };
                // funcInfo is zero, which implies !funcInfo.valid().
                // That ensures that we use the File/Line info given here.
        if { let __tmp_x = (*{ let __field = (*arg.lock().unwrap().as_ref().unwrap()).more.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        break
    }
        call_cgo_symbolizer(arg.clone());
    }

        // funcInfo is zero, which implies !funcInfo.valid().
        // That ensures that we use the File/Line info given here.
        // No more frames for this PC. Tell the symbolizer we are done.
        // We don't try to maintain a single cgoSymbolizerArg for the
        // whole use of Frames, because there would be no good way to tell
        // the symbolizer when we are done.
    { let new_val = 0 as usize; *(*arg.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
    call_cgo_symbolizer(arg.clone());

    return frames.clone();
}

/// activeModules returns a slice of active modules.
///
/// A module is active once its gcdatamask and gcbssmask have been
/// assembled and it is usable by the GC.
///
/// This is nosplit/nowritebarrier because it is called by the
/// cgo pointer checking code.
///
///go:nosplit
///go:nowritebarrier
pub fn active_modules() -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<moduledata>>>>>>> {
    let mut p: GoPtr<Vec<Arc<Mutex<Option<moduledata>>>>> = GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(modulesSlice.clone())))) as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    if p.is_nil() {
        return Arc::new(Mutex::new(None));
    }
    Arc::new(Mutex::new(Some({ let __ptr_value = p.borrow(); __ptr_value.as_ref().unwrap().clone() })))
}

/// FuncForPC returns a *[Func] describing the function that contains the
/// given program counter address, or else nil.
///
/// If pc represents multiple functions because of inlining, it returns
/// the *Func describing the innermost function, but with an entry of
/// the outermost function.
pub fn func_for_p_c(pc: Arc<Mutex<Option<usize>>>) -> GoPtr<Func> {
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        return GoPtr::nil();
    }

        // This must interpret PC non-strictly so bad PCs (those between functions) don't crash the runtime.
        // We just report the preceding function in that situation. See issue 29735.
        // TODO: Perhaps we should report no function at all in that case.
        // The runtime currently doesn't have function end info, alas.
    let (mut u, mut uf) = new_inline_unwinder(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if !(*u.lock().unwrap().as_ref().unwrap()).is_inlined(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return (*f.lock().unwrap().as_ref().unwrap()).__func();
    }
    let mut sf = (*u.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut file, mut line) = (*u.lock().unwrap().as_ref().unwrap()).file_line(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut fi = Arc::new(Mutex::new(Some(funcinl { ones: Arc::new(Mutex::new(Some(!(0 as u32) as u32))), entry: Arc::new(Mutex::new(Some((*f.lock().unwrap().as_ref().unwrap()).entry()))), name: (*sf.lock().unwrap().as_ref().unwrap()).name(), file: Arc::new(Mutex::new(Some({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), line: Arc::new(Mutex::new(Some(line as i32))), start_line: Arc::new(Mutex::new(Some({ let __selector_holder = (*sf.lock().unwrap().as_ref().unwrap()).start_line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
        // entry of the real (the outermost) function.
    return GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&fi) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
}

/// findmoduledatap looks up the moduledata for a PC.
///
/// It is nosplit because it's part of the isgoexception
/// implementation.
///
///go:nosplit
pub fn findmoduledatap(pc: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<moduledata>>> {
    let mut datap = firstmoduledata.clone();
    while { let __nil_result = (*datap.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).minpc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).maxpc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return datap.clone();
    }
        { let new_val = (*datap.lock().unwrap().as_ref().unwrap()).next.clone(); datap = new_val; };
    }
    return Arc::new(Mutex::new(None));
}

/// findfunc looks up function metadata for a PC.
///
/// It is nosplit because it's part of the isgoexception
/// implementation.
///
/// findfunc should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/phuslu/log
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:nosplit
///go:linkname findfunc
pub fn findfunc(pc: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<funcInfo>>> {
    let mut datap = findmoduledatap(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __nil_result = (*datap.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(funcInfo { _func: Default::default(), datap: Default::default() })));
    }
    const nsub: usize = (16 as usize);


    let (mut pcOff, mut ok) = { let __recv = datap.clone(); let __recv_ptr: *const moduledata = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const moduledata }; let __result = unsafe { &*__recv_ptr }.text_off(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    if !ok {
        return Arc::new(Mutex::new(Some(funcInfo { _func: Default::default(), datap: Default::default() })));
    }

    let mut x = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(pcOff as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).text.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).minpc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
    let mut b = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_abi::FUNC_TAB_BUCKET_SIZE as usize; __tmp_x / __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_abi::FUNC_TAB_BUCKET_SIZE as usize; __tmp_x % __tmp_y }; let __tmp_y = ((internal_abi::FUNC_TAB_BUCKET_SIZE as usize) / (nsub as usize)) as usize; __tmp_x / __tmp_y })));

    let mut ffb: GoPtr<findfuncbucket> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __selector_holder = (*datap.lock().unwrap().as_ref().unwrap()).findfunctab.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<findfuncbucket>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut idx = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = ffb.borrow(); __ptr_value.as_ref().unwrap().idx.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = { let __ptr_value = ffb.with_mut(|__ptr_value| __ptr_value.subbuckets.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));

        // Find the ftab entry.
    while { let __tmp_x = (*{ let __seq = { let __seq_holder = (*datap.lock().unwrap().as_ref().unwrap()).ftab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }) as usize].clone() }.entryoff.lock().unwrap().as_ref().unwrap()); let __tmp_y = pcOff; __tmp_x <= __tmp_y } {
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut funcoff = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = (*datap.lock().unwrap().as_ref().unwrap()).ftab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.funcoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    return Arc::new(Mutex::new(Some(funcInfo { _func: Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = (*datap.lock().unwrap().as_ref().unwrap()).pclntable.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __v = (*funcoff.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<_func>(unimplemented!("unsafe.Pointer conversion to _func")) } })).clone(), datap: datap.clone(), ..Default::default() })));
}

/// pcvalueCacheKey returns the outermost index in a pcvalueCache to use for targetpc.
/// It must be very cheap to calculate.
/// For now, align to goarch.PtrSize and reduce mod the number of entries.
/// In practice, this appears to be fairly randomly and evenly distributed.
pub fn pcvalue_cache_key(targetpc: Arc<Mutex<Option<usize>>>) -> usize {
    return { let __tmp_x = ({ let __tmp_x = { let __v = (*targetpc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*pcvalueCache { entries: Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| Default::default()))))), in_use: Arc::new(Mutex::new(Some(0))) }.entries.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x % __tmp_y };
}

/// Returns the PCData value, and the PC where this value starts.
pub fn pcvalue(f: Arc<Mutex<Option<funcInfo>>>, off: Arc<Mutex<Option<u32>>>, targetpc: Arc<Mutex<Option<usize>>>, strict: Arc<Mutex<Option<bool>>>) -> (i32, usize) {
        // If true, when we get a cache hit, still look up the data and make sure it
        // matches the cached contents.
    const debugCheckCache: bool = false;


    if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return (-(1), 0);
    }

        // Check the cache. This speeds up walks of deep stacks, which
        // tend to have the same recursive functions over and over,
        // or repetitive stacks between goroutines.
    let mut checkVal: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut checkPC: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    let mut ck = pcvalue_cache_key(Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    {
        let mut mp = acquirem();
        let mut cache = (*mp.lock().unwrap().as_ref().unwrap()).pcvalue_cache.clone();

                // The cache can be used by the signal handler on this M. Avoid
                // re-entrant use of the cache. The signal handler can also write inUse,
                // but will always restore its value, so we can use a regular increment
                // even if we get signaled in the middle of it.
        { let __target = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*{ let __field = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x == __tmp_y } {
        for i in 0..({ let __seq = { let __seq_holder = (*cache.lock().unwrap().as_ref().unwrap()).entries.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(ck) as usize].clone() }.len()) {
                // We check off first because we're more
                // likely to have multiple entries with
                // different offsets for the same targetpc
                // than the other way around, so we'll usually
                // fail in the first clause.
        let mut ent: Option<GoArrayElemPtr<pcvalueCacheEnt, 8>> = Some(GoArrayElemPtr::nested((*cache.lock().unwrap().as_ref().unwrap()).entries.clone(), (ck) as usize, (i) as usize));
        if { let __tmp_x = (*{ let __field = (*ent.as_ref().unwrap().borrow().as_ref().unwrap()).off.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*ent.as_ref().unwrap().borrow().as_ref().unwrap()).targetpc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*targetpc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        let (mut val, mut pc) = (Arc::new(Mutex::new(Some({ let __selector_holder = (*ent.as_ref().unwrap().borrow().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*ent.as_ref().unwrap().borrow().as_ref().unwrap()).val_p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if debugCheckCache {
        { let __tmp_0 = { let __selector_holder = (*ent.as_ref().unwrap().borrow().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_1 = { let __selector_holder = (*ent.as_ref().unwrap().borrow().as_ref().unwrap()).val_p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *checkVal.lock().unwrap() = Some(__tmp_0); *checkPC.lock().unwrap() = Some(__tmp_1); };
        break
    } else {
        { let __target = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        releasem(GoPtr::local(mp.clone()));
        return ({ let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    }
    }
    } else if debugCheckCache && ({ let __tmp_x = (*{ let __field = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x > __tmp_y }) {
        throw(Arc::new(Mutex::new(Some("cache.inUse out of range".to_string()))));
    }
                // We check off first because we're more
                // likely to have multiple entries with
                // different offsets for the same targetpc
                // than the other way around, so we'll usually
                // fail in the first clause.
                // Catch accounting errors or deeply reentrant use. In principle
                // "inUse" should never exceed 2.
        { let __target = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        releasem(GoPtr::local(mp.clone()));
    }

        // The cache can be used by the signal handler on this M. Avoid
        // re-entrant use of the cache. The signal handler can also write inUse,
        // but will always restore its value, so we can use a regular increment
        // even if we get signaled in the middle of it.
        // We check off first because we're more
        // likely to have multiple entries with
        // different offsets for the same targetpc
        // than the other way around, so we'll usually
        // fail in the first clause.
        // Catch accounting errors or deeply reentrant use. In principle
        // "inUse" should never exceed 2.
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        if { let __v = (*strict.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        eprintln!("{} {}", format!("{}", "runtime: no module data for".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*f.lock().unwrap().as_ref().unwrap()).entry() as u64))))));
        throw(Arc::new(Mutex::new(Some("no module data".to_string()))));
    }
        return (-(1), 0);
    }
    let mut datap = (*f.lock().unwrap().as_ref().unwrap()).datap.clone();
    let mut p = Arc::new(Mutex::new(Some({ let __seq_holder = (*datap.lock().unwrap().as_ref().unwrap()).pctab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    let mut pc = (*f.lock().unwrap().as_ref().unwrap()).entry();
    let mut prevpc = Arc::new(Mutex::new(Some(pc)));
    let mut val = Arc::new(Mutex::new(Some(-(1) as i32)));
    loop {
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = step(p.clone(), Arc::new(Mutex::new(Some(pc.clone()))), val.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = pc; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x == __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        break
    }
        if { let __tmp_x = { let __v = (*targetpc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = pc; __tmp_x < __tmp_y } {
                // Replace a random entry in the cache. Random
                // replacement prevents a performance cliff if
                // a recursive stack's cycle is slightly
                // larger than the cache.
                // Put the new element at the beginning,
                // since it is the most likely to be newly used.
        if debugCheckCache && { let __tmp_x = { let __v = (*checkPC.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        if { let __tmp_x = { let __v = (*checkVal.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*checkPC.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*prevpc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: table value ".to_string()), format!("{}", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "@".to_string()), format!("{}", { let __v = (*prevpc.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " != cache value ".to_string()), format!("{}", { let __v = (*checkVal.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "@".to_string()), format!("{}", { let __v = (*checkPC.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " at PC ".to_string()), format!("{}", { let __v = (*targetpc.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " off ".to_string()), format!("{}", { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad pcvalue cache".to_string()))));
    }
    } else {
        let mut mp = acquirem();
        let mut cache = (*mp.lock().unwrap().as_ref().unwrap()).pcvalue_cache.clone();
        { let __target = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*{ let __field = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x == __tmp_y } {
        let mut e: Option<GoArrayElemPtr<[pcvalueCacheEnt; 8], 2>> = Some(GoArrayElemPtr::new((*cache.lock().unwrap().as_ref().unwrap()).entries.clone(), (ck) as usize));
        let mut ci = cheaprandn(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*cache.lock().unwrap().as_ref().unwrap()).entries.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(ck) as usize].clone() }.len() as u32))));
        { let new_val = { let __seq = e.as_ref().unwrap().borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() }; (*e.as_ref().unwrap().borrow_mut().as_mut().unwrap())[(ci) as usize] = new_val; };
        { let new_val = pcvalueCacheEnt { targetpc: Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), off: Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), val: Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), val_p_c: Arc::new(Mutex::new(Some({ let __arg_holder = prevpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }; (*e.as_ref().unwrap().borrow_mut().as_mut().unwrap())[(0) as usize] = new_val; };
    }
        { let __target = (*cache.lock().unwrap().as_ref().unwrap()).in_use.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        releasem(GoPtr::local(mp.clone()));
    }
        return ({ let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*prevpc.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
                // Replace a random entry in the cache. Random
                // replacement prevents a performance cliff if
                // a recursive stack's cycle is slightly
                // larger than the cache.
                // Put the new element at the beginning,
                // since it is the most likely to be newly used.
        { let new_val = pc; *prevpc.lock().unwrap() = Some(new_val); };
    }

        // Replace a random entry in the cache. Random
        // replacement prevents a performance cliff if
        // a recursive stack's cycle is slightly
        // larger than the cache.
        // Put the new element at the beginning,
        // since it is the most likely to be newly used.
        // If there was a table, it should have covered all program counters.
        // If not, something is wrong.
    if { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } || !{ let __v = (*strict.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (-(1), 0);
    }

    eprint!("{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: invalid pc-encoded table f=".to_string()), format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", " pc=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(pc as u64))))), format!("{}", " targetpc=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*targetpc.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " tab=".to_string()), format!("{}", format_slice(&p)), format!("{}", "\n".to_string()));

    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = (*datap.lock().unwrap().as_ref().unwrap()).pctab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };
    { let new_val = (*f.lock().unwrap().as_ref().unwrap()).entry(); pc = new_val; };
    { let new_val = -1 as i32; *val.lock().unwrap() = Some(new_val); };
    loop {
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = step(p.clone(), Arc::new(Mutex::new(Some(pc.clone()))), val.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = pc; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x == __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        break
    }
        eprint!("{}{}{}{}{}", format!("{}", "\tvalue=".to_string()), format!("{}", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " until pc=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(pc as u64))))), format!("{}", "\n".to_string()));
    }

    throw(Arc::new(Mutex::new(Some("invalid runtime symbol table".to_string()))));
    (-(1), 0)
}

pub fn funcname(f: Arc<Mutex<Option<funcInfo>>>) -> Arc<Mutex<Option<String>>> {
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    (*(*f.lock().unwrap().as_ref().unwrap()).datap.lock().unwrap().as_mut().unwrap()).func_name(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).name_off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
}

pub fn funcfile(f: Arc<Mutex<Option<funcInfo>>>, fileno: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    let mut datap = (*f.lock().unwrap().as_ref().unwrap()).datap.clone();
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        return Arc::new(Mutex::new(Some("?".to_string())));
    }

        // Make sure the cu index and file offset are valid
    {
        let mut fileoff = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*datap.lock().unwrap().as_ref().unwrap()).cutab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).cu_offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*fileno.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize].clone() })));;
        if { let __tmp_x = { let __v = (*fileoff.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as u32) as u32; __tmp_x != __tmp_y } {
            return gostringnocopy(GoPtr::slice_elem(GoSliceElemPtr::new((*datap.lock().unwrap().as_ref().unwrap()).filetab.clone(), ({ let __v = (*fileoff.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));;
        }
    }

        // pcln section is corrupt.
    Arc::new(Mutex::new(Some("?".to_string())))
}

/// funcline1 should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/phuslu/log
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname funcline1
pub fn funcline1(f: Arc<Mutex<Option<funcInfo>>>, targetpc: Arc<Mutex<Option<usize>>>, strict: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<String>>>, i32) {
    let mut file: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut line: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    let mut datap = (*f.lock().unwrap().as_ref().unwrap()).datap.clone();
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        return (Arc::new(Mutex::new(Some("?".to_string()))), 0);
    }
    let (mut fileno, _) = pcvalue(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).pcfile.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = strict.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let (__tmp_0, __tmp_1) = pcvalue(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).pcln.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = strict.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *line.lock().unwrap() = Some(__tmp_0); };
    if { let __tmp_x = fileno; let __tmp_y = -1 as i32; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1 as i32; __tmp_x == __tmp_y } || { let __tmp_x = ((*Arc::new(Mutex::new(Some(fileno as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = (({ let __len_target = { let __field = (*datap.lock().unwrap().as_ref().unwrap()).filetab.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
                // print("looking for ", hex(targetpc), " in ", funcname(f), " got file=", fileno, " line=", lineno, "\n")
        return (Arc::new(Mutex::new(Some("?".to_string()))), 0);
    }
        // print("looking for ", hex(targetpc), " in ", funcname(f), " got file=", fileno, " line=", lineno, "\n")
    { let new_val = funcfile(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(fileno)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *file.lock().unwrap() = __moved_val; };
    return (file.clone(), (*line.lock().unwrap().as_ref().unwrap()));
}

pub fn funcline(f: Arc<Mutex<Option<funcInfo>>>, targetpc: Arc<Mutex<Option<usize>>>) -> (Arc<Mutex<Option<String>>>, i32) {
    let mut file: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut line: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    funcline1(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))))
}

pub fn funcspdelta(f: Arc<Mutex<Option<funcInfo>>>, targetpc: Arc<Mutex<Option<usize>>>) -> i32 {
    let (mut x, _) = pcvalue(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).pcsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
    if DEBUG_PCLN && { let __tmp_x = { let __tmp_x = x; let __tmp_y = ({ let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 1; __tmp_x - __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "invalid spdelta ".to_string()), format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*f.lock().unwrap().as_ref().unwrap()).entry() as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*targetpc.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).pcsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " ".to_string()), format!("{}", x), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad spdelta".to_string()))));
    }
    x
}

/// funcMaxSPDelta returns the maximum spdelta at any point in f.
pub fn func_max_s_p_delta(f: Arc<Mutex<Option<funcInfo>>>) -> i32 {
    let mut datap = (*f.lock().unwrap().as_ref().unwrap()).datap.clone();
    let mut p = Arc::new(Mutex::new(Some({ let __seq_holder = (*datap.lock().unwrap().as_ref().unwrap()).pctab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ((*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).pcsp.lock().unwrap().as_ref().unwrap())) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    let mut pc = (*f.lock().unwrap().as_ref().unwrap()).entry();
    let mut val = Arc::new(Mutex::new(Some(-(1) as i32)));
    let mut most = Arc::new(Mutex::new(Some(0 as i32)));
    loop {
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = step(p.clone(), Arc::new(Mutex::new(Some(pc.clone()))), val.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = pc; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x == __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return { let __v = (*most.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let new_val = std::cmp::max(({ let __v = (*most.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32), ({ let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32)); *most.lock().unwrap() = Some(new_val); };
    }
}

pub fn pcdatastart(f: Arc<Mutex<Option<funcInfo>>>, table: Arc<Mutex<Option<u32>>>) -> u32 {
    { let __v = (*Arc::new(Mutex::new({ let __ptr = add(Arc::new(Mutex::new(Some(Arc::as_ptr(&(*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).nfuncdata.clone()) as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<u8>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*table.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 4 as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u32>(unimplemented!("unsafe.Pointer conversion to u32")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }
}

pub fn pcdatavalue(f: Arc<Mutex<Option<funcInfo>>>, table: Arc<Mutex<Option<u32>>>, targetpc: Arc<Mutex<Option<usize>>>) -> i32 {
    if { let __tmp_x = { let __v = (*table.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).npcdata.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return -(1);
    }
    let (mut r, _) = pcvalue(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(pcdatastart(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = table.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
    r
}

pub fn pcdatavalue1(f: Arc<Mutex<Option<funcInfo>>>, table: Arc<Mutex<Option<u32>>>, targetpc: Arc<Mutex<Option<usize>>>, strict: Arc<Mutex<Option<bool>>>) -> i32 {
    if { let __tmp_x = { let __v = (*table.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).npcdata.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return -(1);
    }
    let (mut r, _) = pcvalue(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(pcdatastart(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = table.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = strict.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    r
}

/// Like pcdatavalue, but also return the start PC of this PCData value.
pub fn pcdatavalue2(f: Arc<Mutex<Option<funcInfo>>>, table: Arc<Mutex<Option<u32>>>, targetpc: Arc<Mutex<Option<usize>>>) -> (i32, usize) {
    if { let __tmp_x = { let __v = (*table.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).npcdata.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (-(1), 0);
    }
    pcvalue(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(pcdatastart(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = table.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))), Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))))
}

/// funcdata returns a pointer to the ith funcdata for f.
/// funcdata should be kept in sync with cmd/link:writeFuncs.
pub fn funcdata(f: Arc<Mutex<Option<funcInfo>>>, i: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<usize>>> {
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).nfuncdata.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    let mut base = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_ref().unwrap()).datap.lock().unwrap().as_ref().unwrap()).gofunc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).nfuncdata.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<u8>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).npcdata.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 4 as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 4 as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    let mut off = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u32>(unimplemented!("unsafe.Pointer conversion to u32")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));

        // Return off == ^uint32(0) ? 0 : f.datap.gofunc + uintptr(off), but without branches.
        // The compiler calculates mask on most architectures using conditional assignment.
    let mut mask: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as u32) as u32; __tmp_x == __tmp_y } {
        { let new_val = 1 as usize; *mask.lock().unwrap() = Some(new_val); };
    }
    { let mut guard = mask.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    let mut raw = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
    return Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*raw.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y })));
}

/// step advances to the next pc, value pair in the encoded table.
pub fn step(mut p: Arc<Mutex<Option<Vec<u8>>>>, pc: Arc<Mutex<Option<usize>>>, val: Arc<Mutex<Option<i32>>>, first: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, bool) {
    let mut newp: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        // For both uvdelta and pcdelta, the common case (~70%)
        // is that they are a single byte. If so, avoid calling readvarint.
    let mut uvdelta = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as u32)));
    if { let __tmp_x = { let __v = (*uvdelta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } && !{ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(None)), false);
    }
    let mut n = Arc::new(Mutex::new(Some(1 as u32)));
    if { let __tmp_x = { let __tmp_x = { let __v = (*uvdelta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = readvarint(p.clone()); *n.lock().unwrap() = Some(__tmp_0); *uvdelta.lock().unwrap() = Some(__tmp_1); };
    }
    { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (({ let __tmp_x = { let __v = (*uvdelta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x & __tmp_y })).wrapping_neg(); let __tmp_y = ({ let __tmp_x = { let __v = (*uvdelta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }); __tmp_x ^ __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let mut guard = val.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };

    let mut pcdelta = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as u32)));
    { let new_val = 1 as u32; *n.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __tmp_x = { let __v = (*pcdelta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = readvarint(p.clone()); *n.lock().unwrap() = Some(__tmp_0); *pcdelta.lock().unwrap() = Some(__tmp_1); };
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };
    { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*pcdelta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_runtime_sys::P_C_QUANTUM as u32; __tmp_x * __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()); let mut guard = pc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    return (p.clone(), true);
}

/// readvarint reads a varint from p.
pub fn readvarint(p: Arc<Mutex<Option<Vec<u8>>>>) -> (u32, u32) {
    let mut read: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut val: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

    let mut v: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));let mut shift: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));let mut n: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut b = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x7F as u8; __tmp_x & __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 31 as u32; __tmp_x & __tmp_y }); __tmp_x << __tmp_y }; let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        break
    }
        { let __rhs = 7 as u32; let mut guard = shift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

///go:nowritebarrier
pub fn stackmapdata(stkmap: GoPtr<stackmap>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::stack::bitvector>>> {
        // Check this invariant only when stackDebug is on at all.
        // The invariant is already checked by many of stackmapdata's callers,
        // and disabling it by default allows stackmapdata to be inlined.
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 0; __tmp_x > __tmp_y } && ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = stkmap.borrow(); __ptr_value.as_ref().unwrap().n.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y }) {
        throw(Arc::new(Mutex::new(Some("stackmapdata: index out of range".to_string()))));
    }
    Arc::new(Mutex::new(Some(crate::stack::bitvector { n: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = stkmap.with_mut(|__ptr_value| __ptr_value.nbit.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), bytedata: addb(GoPtr::array_elem(GoArrayElemPtr::new({ let __ptr_value = stkmap.with_mut(|__ptr_value| __ptr_value.bytedata.clone()); __ptr_value }.clone(), (0) as usize)), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (*{ let __ptr_value = stkmap.borrow(); __ptr_value.as_ref().unwrap().nbit.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 7 as i32; __tmp_x + __tmp_y }); let __tmp_y = 3; __tmp_x >> __tmp_y }); __tmp_x * __tmp_y }) as usize)))), ..Default::default() })))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Frames {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Frame {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Func {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pcHeader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for moduledata {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for modulehash {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for functab {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for textsect {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for findfuncbucket {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for funcInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for srcFunc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pcvalueCache {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pcvalueCacheEnt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stackmap {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
