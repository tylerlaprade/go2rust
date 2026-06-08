use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{mem_darwin::{sys_alloc_o_s, sys_fault_o_s, sys_free_o_s, sys_huge_page_o_s, sys_map_o_s, sys_no_huge_page_o_s, sys_reserve_o_s, sys_unused_o_s, sys_used_o_s}, mgcpacer::{gcController}, mstats::{sysMemStat}};

use std::sync::{Arc, Mutex};

/// sysAlloc transitions an OS-chosen region of memory from None to Ready.
/// More specifically, it obtains a large chunk of zeroed memory from the
/// operating system, typically on the order of a hundred kilobytes
/// or a megabyte. This memory is always immediately available for use.
///
/// sysStat must be non-nil.
///
/// Don't split the stack as this function may be invoked without a valid G,
/// which prevents us from allocating more stack.
///
///go:nosplit
pub fn sys_alloc(n: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> Arc<Mutex<Option<usize>>> {
    { let __recv = sysStat.clone(); let __recv_ptr: *const crate::mstats::sysMemStat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mstats::sysMemStat }; let __result = unsafe { &*__recv_ptr }.add(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i64)))); __result };
    (*(*gcController.lock().unwrap().as_ref().unwrap()).mapped_ready.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i64))));
    sys_alloc_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// sysUnused transitions a memory region from Ready to Prepared. It notifies the
/// operating system that the physical pages backing this memory region are no
/// longer needed and can be reused for other purposes. The contents of a
/// sysUnused memory region are considered forfeit and the region must not be
/// accessed again until sysUsed is called.
pub fn sys_unused(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    (*(*gcController.lock().unwrap().as_ref().unwrap()).mapped_ready.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-((*n.lock().unwrap().as_ref().unwrap()) as i64)))));
    sys_unused_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// sysUsed transitions a memory region from Prepared to Ready. It notifies the
/// operating system that the memory region is needed and ensures that the region
/// may be safely accessed. This is typically a no-op on systems that don't have
/// an explicit commit step and hard over-commit limits, but is critical on
/// Windows, for example.
///
/// This operation is idempotent for memory already in the Prepared state, so
/// it is safe to refer, with v and n, to a range of memory that includes both
/// Prepared and Ready memory. However, the caller must provide the exact amount
/// of Prepared memory for accounting purposes.
pub fn sys_used(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>, prepared: Arc<Mutex<Option<usize>>>) {
    (*(*gcController.lock().unwrap().as_ref().unwrap()).mapped_ready.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some((*prepared.lock().unwrap().as_ref().unwrap()) as i64))));
    sys_used_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// sysHugePage does not transition memory regions, but instead provides a
/// hint to the OS that it would be more efficient to back this memory region
/// with pages of a larger size transparently.
pub fn sys_huge_page(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    sys_huge_page_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// sysNoHugePage does not transition memory regions, but instead provides a
/// hint to the OS that it would be less efficient to back this memory region
/// with pages of a larger size transparently.
pub fn sys_no_huge_page(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    sys_no_huge_page_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// sysFree transitions a memory region from any state to None. Therefore, it
/// returns memory unconditionally. It is used if an out-of-memory error has been
/// detected midway through an allocation or to carve out an aligned section of
/// the address space. It is okay if sysFree is a no-op only if sysReserve always
/// returns a memory region aligned to the heap allocator's alignment
/// restrictions.
///
/// sysStat must be non-nil.
///
/// Don't split the stack as this function may be invoked without a valid G,
/// which prevents us from allocating more stack.
///
///go:nosplit
pub fn sys_free(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) {
    { let __recv = sysStat.clone(); let __recv_ptr: *const crate::mstats::sysMemStat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mstats::sysMemStat }; let __result = unsafe { &*__recv_ptr }.add(Arc::new(Mutex::new(Some(-((*n.lock().unwrap().as_ref().unwrap()) as i64))))); __result };
    (*(*gcController.lock().unwrap().as_ref().unwrap()).mapped_ready.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-((*n.lock().unwrap().as_ref().unwrap()) as i64)))));
    sys_free_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// sysFault transitions a memory region from Ready to Reserved. It
/// marks a region such that it will always fault if accessed. Used only for
/// debugging the runtime.
///
/// TODO(mknyszek): Currently it's true that all uses of sysFault transition
/// memory from Ready to Reserved, but this may not be true in the future
/// since on every platform the operation is much more general than that.
/// If a transition from Prepared is ever introduced, create a new function
/// that elides the Ready state accounting.
pub fn sys_fault(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    (*(*gcController.lock().unwrap().as_ref().unwrap()).mapped_ready.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-((*n.lock().unwrap().as_ref().unwrap()) as i64)))));
    sys_fault_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// sysReserve transitions a memory region from None to Reserved. It reserves
/// address space in such a way that it would cause a fatal fault upon access
/// (either via permissions or not committing the memory). Such a reservation is
/// thus never backed by physical memory.
///
/// If the pointer passed to it is non-nil, the caller wants the
/// reservation there, but sysReserve can still choose another
/// location if that one is unavailable.
///
/// NOTE: sysReserve returns OS-aligned memory, but the heap allocator
/// may use larger alignment, so the caller must be careful to realign the
/// memory obtained by sysReserve.
pub fn sys_reserve(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    sys_reserve_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// sysMap transitions a memory region from Reserved to Prepared. It ensures the
/// memory region can be efficiently transitioned to Ready.
///
/// sysStat must be non-nil.
pub fn sys_map(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) {
    { let __recv = sysStat.clone(); let __recv_ptr: *const crate::mstats::sysMemStat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mstats::sysMemStat }; let __result = unsafe { &*__recv_ptr }.add(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i64)))); __result };
    sys_map_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}