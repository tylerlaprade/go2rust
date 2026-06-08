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
    defs_darwin_arm64::{__M_A_D_V__F_R_E_E__R_E_U_S_A_B_L_E, __M_A_D_V__F_R_E_E__R_E_U_S_E, __M_A_P__A_N_O_N, __M_A_P__F_I_X_E_D, __M_A_P__P_R_I_V_A_T_E, __P_R_O_T__N_O_N_E, __P_R_O_T__R_E_A_D, __P_R_O_T__W_R_I_T_E},
    panic::{throw},
    sys_darwin::{madvise, mmap, munmap},
};

use std::sync::{Arc, Mutex};

pub(crate) const __E_N_O_M_E_M: i32 = 12;


/// Don't split the stack as this function may be invoked without a valid G,
/// which prevents us from allocating more stack.
///
///go:nosplit
pub fn sys_alloc_o_s(n: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    let (mut v, mut err) = mmap(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = __P_R_O_T__R_E_A_D; let __tmp_y = __P_R_O_T__W_R_I_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some({ let __tmp_x = __M_A_P__A_N_O_N; let __tmp_y = __M_A_P__P_R_I_V_A_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some(-1 as i32))), Arc::new(Mutex::new(Some(0 as u32))));
    if { let __tmp_x = err; let __tmp_y = 0; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn sys_unused_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
        // MADV_FREE_REUSABLE is like MADV_FREE except it also propagates
        // accounting information about the process to task_info.
    madvise(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__M_A_D_V__F_R_E_E__R_E_U_S_A_B_L_E as i32))));
}

pub fn sys_used_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
        // MADV_FREE_REUSE is necessary to keep the kernel's accounting
        // accurate. If called on any memory region that hasn't been
        // MADV_FREE_REUSABLE'd, it's a no-op.
    madvise(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__M_A_D_V__F_R_E_E__R_E_U_S_E as i32))));
}

pub fn sys_huge_page_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
}

pub fn sys_no_huge_page_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
}

/// Don't split the stack as this function may be invoked without a valid G,
/// which prevents us from allocating more stack.
///
///go:nosplit
pub fn sys_free_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    munmap(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

pub fn sys_fault_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    mmap(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__P_R_O_T__N_O_N_E as i32))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __M_A_P__A_N_O_N; let __tmp_y = __M_A_P__P_R_I_V_A_T_E; __tmp_x | __tmp_y }; let __tmp_y = __M_A_P__F_I_X_E_D; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some(-1 as i32))), Arc::new(Mutex::new(Some(0 as u32))));
}

pub fn sys_reserve_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    let (mut p, mut err) = mmap(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__P_R_O_T__N_O_N_E as i32))), Arc::new(Mutex::new(Some({ let __tmp_x = __M_A_P__A_N_O_N; let __tmp_y = __M_A_P__P_R_I_V_A_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some(-1 as i32))), Arc::new(Mutex::new(Some(0 as u32))));
    if { let __tmp_x = err; let __tmp_y = 0; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    return { let __owned = p.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn sys_map_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    let (mut p, mut err) = mmap(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = __P_R_O_T__R_E_A_D; let __tmp_y = __P_R_O_T__W_R_I_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __M_A_P__A_N_O_N; let __tmp_y = __M_A_P__F_I_X_E_D; __tmp_x | __tmp_y }; let __tmp_y = __M_A_P__P_R_I_V_A_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some(-1 as i32))), Arc::new(Mutex::new(Some(0 as u32))));
    if { let __tmp_x = err; let __tmp_y = 12; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: out of memory".to_string()))));
    }
    if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*v.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } || { let __tmp_x = err; let __tmp_y = 0; __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: mmap(".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", ", ".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", ") returned ".to_string());
            let __go_print_arg_5 = format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_6 = format!("{}", ", ".to_string());
            let __go_print_arg_7 = format!("{}", err);
            let __go_print_arg_8 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
        throw(Arc::new(Mutex::new(Some("runtime: cannot map pages in arena address space".to_string()))));
    }
}