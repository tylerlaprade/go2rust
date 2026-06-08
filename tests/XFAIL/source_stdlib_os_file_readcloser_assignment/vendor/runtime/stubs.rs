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

use crate::{map_swiss::{LOAD_FACTOR_DEN, LOAD_FACTOR_NUM}, r#type::{_type}, runtime2::{g, gobuf}};

use std::any::Any;
use std::sync::{Arc, Mutex};

pub(crate) const HASH_LOAD: f32 = (LOAD_FACTOR_NUM as f32) / (LOAD_FACTOR_DEN as f32);


pub(crate) static intArgRegs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *intArgRegs.lock().unwrap() = Some(0);
    *intArgRegs.lock().unwrap() = Some(internal_abi::INT_ARG_REGS);
}


pub(crate) fn __go_zero_globals() {
    *intArgRegs.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_79() {
    *intArgRegs.lock().unwrap() = Some(internal_abi::INT_ARG_REGS);
}


/// Should be a built-in for unsafe.Pointer?
///
/// add should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - fortio.org/log
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname add
///go:nosplit
pub fn add(p: Arc<Mutex<Option<usize>>>, x: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))
}

/// getg returns the pointer to the current g.
/// The compiler rewrites calls to this function into instructions
/// that fetch the g directly (from TLS or from the dedicated register).
pub fn getg() -> Arc<Mutex<Option<crate::runtime2::g>>> {
    unimplemented!("Go function declaration has no body");
}


/// mcall switches from the g to the g0 stack and invokes fn(g),
/// where g is the goroutine that made the call.
/// mcall saves g's current PC/SP in g->sched so that it can be restored later.
/// It is up to fn to arrange for that later execution, typically by recording
/// g in a data structure, causing something to call ready(g) later.
/// mcall returns to the original goroutine g later, when g has been rescheduled.
/// fn must not return at all; typically it ends by calling schedule, to let the m
/// run other goroutines.
///
/// mcall can only be called from g stacks (not g0, not gsignal).
///
/// This must NOT be go:noescape: if fn is a stack-allocated closure,
/// fn puts g on a run queue, and g executes before fn returns, the
/// closure will be invalidated while it is still executing.
pub fn mcall(r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}


/// systemstack runs fn on a system stack.
/// If systemstack is called from the per-OS-thread (g0) stack, or
/// if systemstack is called from the signal handling (gsignal) stack,
/// systemstack calls fn directly and returns.
/// Otherwise, systemstack is being called from the limited stack
/// of an ordinary goroutine. In this case, systemstack switches
/// to the per-OS-thread stack, calls fn, and switches back.
/// It is common to use a func literal as the argument, in order
/// to share inputs and outputs with the code around the call
/// to system stack:
///
///	... set up y ...
///	systemstack(func() {
///		x = bigcall(y)
///	})
///	... use x ...
///
///go:noescape
pub fn systemstack(r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}


/// memclrNoHeapPointers clears n bytes starting at ptr.
///
/// Usually you should use typedmemclr. memclrNoHeapPointers should be
/// used only when the caller knows that *ptr contains no heap pointers
/// because either:
///
/// *ptr is initialized memory and its type is pointer-free, or
///
/// *ptr is uninitialized memory (e.g., memory that's being reused
/// for a new allocation) and hence contains only "junk".
///
/// memclrNoHeapPointers ensures that if ptr is pointer-aligned, and n
/// is a multiple of the pointer size, then any pointer-aligned,
/// pointer-sized portion is cleared atomically. Despite the function
/// name, this is necessary because this function is the underlying
/// implementation of typedmemclr and memclrHasPointers. See the doc of
/// memmove for more details.
///
/// The (CPU-specific) implementations of this function are in memclr_*.s.
///
/// memclrNoHeapPointers should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/sonic
///   - github.com/chenzhuoyu/iasm
///   - github.com/dgraph-io/ristretto
///   - github.com/outcaste-io/ristretto
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname memclrNoHeapPointers
///go:noescape
pub fn memclr_no_heap_pointers(ptr: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}


/// memmove copies n bytes from "from" to "to".
///
/// memmove ensures that any pointer in "from" is written to "to" with
/// an indivisible write, so that racy reads cannot observe a
/// half-written pointer. This is necessary to prevent the garbage
/// collector from observing invalid pointers, and differs from memmove
/// in unmanaged languages. However, memmove is only required to do
/// this if "from" and "to" may contain pointers, which can only be the
/// case if "from", "to", and "n" are all be word-aligned.
///
/// Implementations are in memmove_*.s.
///
/// Outside assembly calls memmove.
///
/// memmove should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/sonic
///   - github.com/cloudwego/dynamicgo
///   - github.com/ebitengine/purego
///   - github.com/tetratelabs/wazero
///   - github.com/ugorji/go/codec
///   - gvisor.dev/gvisor
///   - github.com/sagernet/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname memmove
///go:noescape
pub fn memmove(to: Arc<Mutex<Option<usize>>>, from: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}


/// in internal/bytealg/equal_*.s
///
/// memequal should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/sonic
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname memequal
///go:noescape
pub fn memequal(a: Arc<Mutex<Option<usize>>>, b: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


/// noescape hides a pointer from escape analysis.  noescape is
/// the identity function but escape analysis doesn't think the
/// output depends on the input.  noescape is inlined and currently
/// compiles down to zero instructions.
/// USE CAREFULLY!
///
/// noescape should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/gopkg
///   - github.com/ebitengine/purego
///   - github.com/hamba/avro/v2
///   - github.com/puzpuzpuz/xsync/v3
///   - github.com/songzhibin97/gkit
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname noescape
///go:nosplit
pub fn noescape(p: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    let mut x = Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize)));
    return Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x ^ __tmp_y })));
}

pub fn gogo(buf_local: Arc<Mutex<Option<gobuf>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn asminit() {
    unimplemented!("Go function declaration has no body");
}


pub fn setg(gg: Arc<Mutex<Option<g>>>) {
    unimplemented!("Go function declaration has no body");
}


/// reflectcall calls fn with arguments described by stackArgs, stackArgsSize,
/// frameSize, and regArgs.
///
/// Arguments passed on the stack and space for return values passed on the stack
/// must be laid out at the space pointed to by stackArgs (with total length
/// stackArgsSize) according to the ABI.
///
/// stackRetOffset must be some value <= stackArgsSize that indicates the
/// offset within stackArgs where the return value space begins.
///
/// frameSize is the total size of the argument frame at stackArgs and must
/// therefore be >= stackArgsSize. It must include additional space for spilling
/// register arguments for stack growth and preemption.
///
/// TODO(mknyszek): Once we don't need the additional spill space, remove frameSize,
/// since frameSize will be redundant with stackArgsSize.
///
/// Arguments passed in registers must be laid out in regArgs according to the ABI.
/// regArgs will hold any return values passed in registers after the call.
///
/// reflectcall copies stack arguments from stackArgs to the goroutine stack, and
/// then copies back stackArgsSize-stackRetOffset bytes back to the return space
/// in stackArgs once fn has completed. It also "unspills" argument registers from
/// regArgs before calling fn, and spills them back into regArgs immediately
/// following the call to fn. If there are results being returned on the stack,
/// the caller should pass the argument frame type as stackArgsType so that
/// reflectcall can execute appropriate write barriers during the copy.
///
/// reflectcall expects regArgs.ReturnIsPtr to be populated indicating which
/// registers on the return path will contain Go pointers. It will then store
/// these pointers in regArgs.Ptrs such that they are visible to the GC.
///
/// Package reflect passes a frame type. In package runtime, there is only
/// one call that copies results back, in callbackWrap in syscall_windows.go, and it
/// does NOT pass a frame type, meaning there are no write barriers invoked. See that
/// call site for justification.
///
/// Package reflect accesses this symbol through a linkname.
///
/// Arguments passed through to reflectcall do not escape. The type is used
/// only in a very limited callee of reflectcall, the stackArgs are copied, and
/// regArgs is only used in the reflectcall frame.
///
///go:noescape
pub fn reflectcall(stackArgsType: Arc<Mutex<Option<internal_abi::r#type::Type>>>, r#fn: Arc<Mutex<Option<usize>>>, stackArgs: Arc<Mutex<Option<usize>>>, stackArgsSize: Arc<Mutex<Option<u32>>>, stackRetOffset: Arc<Mutex<Option<u32>>>, frameSize: Arc<Mutex<Option<u32>>>, regArgs: Arc<Mutex<Option<internal_abi::r#mod::RegArgs>>>) {
    unimplemented!("Go function declaration has no body");
}


/// procyield should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/sagernet/sing-tun
///   - github.com/slackhq/nebula
///   - golang.zx2c4.com/wireguard
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname procyield
pub fn procyield(cycles: Arc<Mutex<Option<u32>>>) {
    unimplemented!("Go function declaration has no body");
}


/// publicationBarrier performs a store/store barrier (a "publication"
/// or "export" barrier). Some form of synchronization is required
/// between initializing an object and making that object accessible to
/// another processor. Without synchronization, the initialization
/// writes and the "publication" write may be reordered, allowing the
/// other processor to follow the pointer and observe an uninitialized
/// object. In general, higher-level synchronization should be used,
/// such as locking or an atomic pointer write. publicationBarrier is
/// for when those aren't an option, such as in the implementation of
/// the memory manager.
///
/// There's no corresponding barrier for the read side because the read
/// side naturally has a data dependency order. All architectures that
/// Go supports or seems likely to ever support automatically enforce
/// data dependency ordering.
pub fn publication_barrier() {
    unimplemented!("Go function declaration has no body");
}


///go:noescape
pub fn asmcgocall(r#fn: Arc<Mutex<Option<usize>>>, arg: Arc<Mutex<Option<usize>>>) -> i32 {
    unimplemented!("Go function declaration has no body");
}


/// alignUp rounds n up to a multiple of a. a must be a power of 2.
///
///go:nosplit
pub fn align_up(n: Arc<Mutex<Option<usize>>>, a: Arc<Mutex<Option<usize>>>) -> usize {
    return { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & ! __tmp_y };
}

/// alignDown rounds n down to a multiple of a. a must be a power of 2.
///
///go:nosplit
pub fn align_down(n: Arc<Mutex<Option<usize>>>, a: Arc<Mutex<Option<usize>>>) -> usize {
    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & ! __tmp_y };
}

/// divRoundUp returns ceil(n / a).
///
///go:nosplit
pub fn div_round_up(n: Arc<Mutex<Option<usize>>>, a: Arc<Mutex<Option<usize>>>) -> usize {
        // a is generally a power of two. This will get inlined and
        // the compiler will optimize the division.
    return { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y };
}

/// bool2int returns 0 if x is false or 1 if x is true.
pub fn bool2int(x: Arc<Mutex<Option<bool>>>) -> i32 {
        // Avoid branches. In the SSA compiler, this compiles to
        // exactly what you would want it to.
    (*Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&x.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v } as i32))).lock().unwrap().as_ref().unwrap())
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
