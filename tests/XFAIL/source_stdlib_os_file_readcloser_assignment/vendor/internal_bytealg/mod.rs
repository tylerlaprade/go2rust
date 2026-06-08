use go2rust_stdlib_stubs::*;

pub(crate) const OFFSET_X86_HAS_S_S_E42: usize = std::mem::offset_of!(internal_cpu::X86, has_s_s_e42);
pub(crate) const OFFSET_X86_HAS_A_V_X2: usize = std::mem::offset_of!(internal_cpu::X86, has_a_v_x2);
pub(crate) const OFFSET_X86_HAS_P_O_P_C_N_T: usize = std::mem::offset_of!(internal_cpu::X86, has_p_o_p_c_n_t);
pub(crate) const OFFSET_S390X_HAS_V_X: usize = std::mem::offset_of!(internal_cpu::S390X, has_v_x);
pub(crate) const OFFSET_P_P_C64_HAS_P_O_W_E_R9: usize = std::mem::offset_of!(internal_cpu::PPC64, is_p_o_w_e_r9);


pub const PRIME_R_K: i32 = 16777619;


pub static MaxLen: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *MaxLen.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *MaxLen.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
