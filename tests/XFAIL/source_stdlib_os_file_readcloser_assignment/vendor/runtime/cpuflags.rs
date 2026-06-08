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

pub(crate) const OFFSET_X86_HAS_A_V_X: usize = std::mem::offset_of!(internal_cpu::X86, has_a_v_x);
pub(crate) const OFFSET_X86_HAS_A_V_X2: usize = std::mem::offset_of!(internal_cpu::X86, has_a_v_x2);
pub(crate) const OFFSET_X86_HAS_E_R_M_S: usize = std::mem::offset_of!(internal_cpu::X86, has_e_r_m_s);
pub(crate) const OFFSET_X86_HAS_R_D_T_S_C_P: usize = std::mem::offset_of!(internal_cpu::X86, has_r_d_t_s_c_p);
pub(crate) const OFFSET_A_R_M_HAS_I_D_I_V_A: usize = std::mem::offset_of!(internal_cpu::ARM, has_i_d_i_v_a);
pub(crate) const OFFSET_M_I_P_S64_X_HAS_M_S_A: usize = std::mem::offset_of!(internal_cpu::MIPS64X, has_m_s_a);
pub(crate) const OFFSET_L_O_O_N_G64_HAS_L_S_X: usize = std::mem::offset_of!(internal_cpu::Loong64, has_l_s_x);


pub(crate) static x86HasPOPCNT: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static x86HasSSE41: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static x86HasFMA: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static armHasVFPv4: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static arm64HasATOMICS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static loong64HasLAMCAS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static loong64HasLAM_BH: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static loong64HasLSX: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *x86HasPOPCNT.lock().unwrap() = Some(false);
    *x86HasSSE41.lock().unwrap() = Some(false);
    *x86HasFMA.lock().unwrap() = Some(false);
    *armHasVFPv4.lock().unwrap() = Some(false);
    *arm64HasATOMICS.lock().unwrap() = Some(false);
    *loong64HasLAMCAS.lock().unwrap() = Some(false);
    *loong64HasLAM_BH.lock().unwrap() = Some(false);
    *loong64HasLSX.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *x86HasPOPCNT.lock().unwrap() = Some(false);
    *x86HasSSE41.lock().unwrap() = Some(false);
    *x86HasFMA.lock().unwrap() = Some(false);
    *armHasVFPv4.lock().unwrap() = Some(false);
    *arm64HasATOMICS.lock().unwrap() = Some(false);
    *loong64HasLAMCAS.lock().unwrap() = Some(false);
    *loong64HasLAM_BH.lock().unwrap() = Some(false);
    *loong64HasLSX.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
