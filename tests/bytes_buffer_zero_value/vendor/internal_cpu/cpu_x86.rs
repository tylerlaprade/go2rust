use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

pub const CACHE_LINE_PAD_SIZE: i32 = 64;


pub(crate) const CPUID__S_S_E3: i32 = 1 << 0;
pub(crate) const CPUID__P_C_L_M_U_L_Q_D_Q: i32 = 1 << 1;
pub(crate) const CPUID__S_S_S_E3: i32 = 1 << 9;
pub(crate) const CPUID__F_M_A: i32 = 1 << 12;
pub(crate) const CPUID__S_S_E41: i32 = 1 << 19;
pub(crate) const CPUID__S_S_E42: i32 = 1 << 20;
pub(crate) const CPUID__P_O_P_C_N_T: i32 = 1 << 23;
pub(crate) const CPUID__A_E_S: i32 = 1 << 25;
pub(crate) const CPUID__O_S_X_S_A_V_E: i32 = 1 << 27;
pub(crate) const CPUID__A_V_X: i32 = 1 << 28;
pub(crate) const CPUID__B_M_I1: i32 = 1 << 3;
pub(crate) const CPUID__A_V_X2: i32 = 1 << 5;
pub(crate) const CPUID__B_M_I2: i32 = 1 << 8;
pub(crate) const CPUID__E_R_M_S: i32 = 1 << 9;
pub(crate) const CPUID__A_V_X512_F: i32 = 1 << 16;
pub(crate) const CPUID__A_D_X: i32 = 1 << 19;
pub(crate) const CPUID__S_H_A: i32 = 1 << 29;
pub(crate) const CPUID__A_V_X512_B_W: i32 = 1 << 30;
pub(crate) const CPUID__A_V_X512_V_L: i64 = 1 << 31;
pub(crate) const CPUID__F_S_R_M: i32 = 1 << 4;
pub(crate) const CPUID__R_D_T_S_C_P: i32 = 1 << 27;


pub(crate) static maxExtendedFunctionInformation: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *maxExtendedFunctionInformation.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *maxExtendedFunctionInformation.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
