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

use crate::{
    lockrank_off::{assert_world_stopped},
    malloc::{HEAP_ARENA_BYTES, persistentalloc},
    mbitmap::{markBits},
    mgc::{gc_mark_work_available},
    mgcmark::{gc_dump_object},
    mheap::{arenaIdx, arena_index, heapArena, mheap_},
    mstats::{memstats, sysMemStat},
    panic::{throw},
    print::{hex, printlock},
    runtime2::{g, m},
    stubs::{getg},
};

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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            b: __go_clone_1_0,
        }
    }
}


impl Default for checkmarksMap {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            __blank_0_0: __go_default_0_0,
            b: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for checkmarksMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.b));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
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
        bitmap = GoPtr::raw({ let __ptr = persistentalloc(
            Arc::new(Mutex::new(Some(std::mem::size_of::<checkmarksMap>()))),
            Arc::new(Mutex::new(Some(0 as usize))),
            (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone()
        ).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
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
        {
            let __go_print_arg_0 = format!("{}", "runtime: checkmarks found unexpected unmarked object obj=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*obj.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        {
            let __go_print_arg_0 = format!("{}", "runtime: found obj at *(".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "+".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
                // Dump the source (base) object
        gc_dump_object(Arc::new(Mutex::new(Some("base".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Dump the object
        gc_dump_object(
            Arc::new(Mutex::new(Some("obj".to_string()))),
            Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(!(0 as usize) as usize)))
        );
        { let new_val = 2 as u8; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap() = Some(new_val); };
        throw(Arc::new(Mutex::new(Some("checkmark found unmarked object".to_string()))));
    }

        // Dump the source (base) object
        // Dump the object
    let mut ai = arena_index(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut arena = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
    let mut arenaWord = Arc::new(Mutex::new(Some({
        let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x / __tmp_y }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y });
        let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = (*arena.lock().unwrap().as_ref().unwrap()).checkmarks.with_mut(|__ptr_value| __ptr_value.b.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize;
        __tmp_x % __tmp_y
    })));
    let mut mask = Arc::new(Mutex::new(Some(({ let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x / __tmp_y }); let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }) as u8)));
    let mut bytep: Option<GoArrayElemPtr<u8, 1048576>> = Some(GoArrayElemPtr::new({ let __ptr_value = (*arena.lock().unwrap().as_ref().unwrap()).checkmarks.with_mut(|__ptr_value| __ptr_value.b.clone()); __ptr_value }.clone(), ({ let __v = (*arenaWord.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));

    if { let __tmp_x = { let __tmp_x = internal_runtime_atomic::load8({ match bytep.clone() { Some(__value) => internal_runtime_atomic::GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })), None => internal_runtime_atomic::GoPtr::nil() } }); let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
                // Already checkmarked.
        return true;
    }

        // Already checkmarked.
    internal_runtime_atomic::or8({ match bytep.clone() { Some(__value) => internal_runtime_atomic::GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })), None => internal_runtime_atomic::GoPtr::nil() } }, Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
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
