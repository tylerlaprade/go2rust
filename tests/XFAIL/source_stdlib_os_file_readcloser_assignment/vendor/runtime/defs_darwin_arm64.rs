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

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const __E_I_N_T_R: i32 = 0x4;
pub(crate) const __E_F_A_U_L_T: i32 = 0xe;
pub(crate) const __E_A_G_A_I_N: i32 = 0x23;
pub(crate) const __E_T_I_M_E_D_O_U_T: i32 = 0x3c;
pub(crate) const __P_R_O_T__N_O_N_E: i32 = 0x0;
pub(crate) const __P_R_O_T__R_E_A_D: i32 = 0x1;
pub(crate) const __P_R_O_T__W_R_I_T_E: i32 = 0x2;
pub(crate) const __P_R_O_T__E_X_E_C: i32 = 0x4;
pub(crate) const __M_A_P__A_N_O_N: i32 = 0x1000;
pub(crate) const __M_A_P__P_R_I_V_A_T_E: i32 = 0x2;
pub(crate) const __M_A_P__F_I_X_E_D: i32 = 0x10;
pub(crate) const __M_A_D_V__D_O_N_T_N_E_E_D: i32 = 0x4;
pub(crate) const __M_A_D_V__F_R_E_E: i32 = 0x5;
pub(crate) const __M_A_D_V__F_R_E_E__R_E_U_S_A_B_L_E: i32 = 0x7;
pub(crate) const __M_A_D_V__F_R_E_E__R_E_U_S_E: i32 = 0x8;
pub(crate) const __S_A__S_I_G_I_N_F_O: i32 = 0x40;
pub(crate) const __S_A__R_E_S_T_A_R_T: i32 = 0x2;
pub(crate) const __S_A__O_N_S_T_A_C_K: i32 = 0x1;
pub(crate) const __S_A__U_S_E_R_T_R_A_M_P: i32 = 0x100;
pub(crate) const __S_A_64_R_E_G_S_E_T: i32 = 0x200;
pub(crate) const __S_I_G_H_U_P: i32 = 0x1;
pub(crate) const __S_I_G_I_N_T: i32 = 0x2;
pub(crate) const __S_I_G_Q_U_I_T: i32 = 0x3;
pub(crate) const __S_I_G_I_L_L: i32 = 0x4;
pub(crate) const __S_I_G_T_R_A_P: i32 = 0x5;
pub(crate) const __S_I_G_A_B_R_T: i32 = 0x6;
pub(crate) const __S_I_G_E_M_T: i32 = 0x7;
pub(crate) const __S_I_G_F_P_E: i32 = 0x8;
pub(crate) const __S_I_G_K_I_L_L: i32 = 0x9;
pub(crate) const __S_I_G_B_U_S: i32 = 0xa;
pub(crate) const __S_I_G_S_E_G_V: i32 = 0xb;
pub(crate) const __S_I_G_S_Y_S: i32 = 0xc;
pub(crate) const __S_I_G_P_I_P_E: i32 = 0xd;
pub(crate) const __S_I_G_A_L_R_M: i32 = 0xe;
pub(crate) const __S_I_G_T_E_R_M: i32 = 0xf;
pub(crate) const __S_I_G_U_R_G: i32 = 0x10;
pub(crate) const __S_I_G_S_T_O_P: i32 = 0x11;
pub(crate) const __S_I_G_T_S_T_P: i32 = 0x12;
pub(crate) const __S_I_G_C_O_N_T: i32 = 0x13;
pub(crate) const __S_I_G_C_H_L_D: i32 = 0x14;
pub(crate) const __S_I_G_T_T_I_N: i32 = 0x15;
pub(crate) const __S_I_G_T_T_O_U: i32 = 0x16;
pub(crate) const __S_I_G_I_O: i32 = 0x17;
pub(crate) const __S_I_G_X_C_P_U: i32 = 0x18;
pub(crate) const __S_I_G_X_F_S_Z: i32 = 0x19;
pub(crate) const __S_I_G_V_T_A_L_R_M: i32 = 0x1a;
pub(crate) const __S_I_G_P_R_O_F: i32 = 0x1b;
pub(crate) const __S_I_G_W_I_N_C_H: i32 = 0x1c;
pub(crate) const __S_I_G_I_N_F_O: i32 = 0x1d;
pub(crate) const __S_I_G_U_S_R1: i32 = 0x1e;
pub(crate) const __S_I_G_U_S_R2: i32 = 0x1f;
pub(crate) const __F_P_E__I_N_T_D_I_V: i32 = 0x7;
pub(crate) const __F_P_E__I_N_T_O_V_F: i32 = 0x8;
pub(crate) const __F_P_E__F_L_T_D_I_V: i32 = 0x1;
pub(crate) const __F_P_E__F_L_T_O_V_F: i32 = 0x2;
pub(crate) const __F_P_E__F_L_T_U_N_D: i32 = 0x3;
pub(crate) const __F_P_E__F_L_T_R_E_S: i32 = 0x4;
pub(crate) const __F_P_E__F_L_T_I_N_V: i32 = 0x5;
pub(crate) const __F_P_E__F_L_T_S_U_B: i32 = 0x6;
pub(crate) const __B_U_S__A_D_R_A_L_N: i32 = 0x1;
pub(crate) const __B_U_S__A_D_R_E_R_R: i32 = 0x2;
pub(crate) const __B_U_S__O_B_J_E_R_R: i32 = 0x3;
pub(crate) const __S_E_G_V__M_A_P_E_R_R: i32 = 0x1;
pub(crate) const __S_E_G_V__A_C_C_E_R_R: i32 = 0x2;
pub(crate) const __I_T_I_M_E_R__R_E_A_L: i32 = 0x0;
pub(crate) const __I_T_I_M_E_R__V_I_R_T_U_A_L: i32 = 0x1;
pub(crate) const __I_T_I_M_E_R__P_R_O_F: i32 = 0x2;
pub(crate) const __E_V__A_D_D: i32 = 0x1;
pub(crate) const __E_V__D_E_L_E_T_E: i32 = 0x2;
pub(crate) const __E_V__E_N_A_B_L_E: i32 = 0x4;
pub(crate) const __E_V__D_I_S_A_B_L_E: i32 = 0x8;
pub(crate) const __E_V__C_L_E_A_R: i32 = 0x20;
pub(crate) const __E_V__R_E_C_E_I_P_T: i32 = 0x40;
pub(crate) const __E_V__E_R_R_O_R: i32 = 0x4000;
pub(crate) const __E_V__E_O_F: i32 = 0x8000;
pub(crate) const __E_V_F_I_L_T__R_E_A_D: i32 = -0x1;
pub(crate) const __E_V_F_I_L_T__W_R_I_T_E: i32 = -0x2;
pub(crate) const __E_V_F_I_L_T__U_S_E_R: i32 = -0xa;
pub(crate) const __N_O_T_E__T_R_I_G_G_E_R: i32 = 0x1000000;
pub(crate) const __P_T_H_R_E_A_D__C_R_E_A_T_E__D_E_T_A_C_H_E_D: i32 = 0x2;
pub(crate) const __P_T_H_R_E_A_D__K_E_Y_S__M_A_X: i32 = 512;
pub(crate) const __F__G_E_T_F_L: i32 = 0x3;
pub(crate) const __F__S_E_T_F_L: i32 = 0x4;
pub(crate) const __O__W_R_O_N_L_Y: i32 = 0x1;
pub(crate) const __O__N_O_N_B_L_O_C_K: i32 = 0x4;
pub(crate) const __O__C_R_E_A_T: i32 = 0x200;
pub(crate) const __O__T_R_U_N_C: i32 = 0x400;
pub(crate) const __V_M__R_E_G_I_O_N__B_A_S_I_C__I_N_F_O__C_O_U_N_T_64: i32 = 0x9;
pub(crate) const __V_M__R_E_G_I_O_N__B_A_S_I_C__I_N_F_O_64: i32 = 0x9;


#[derive(Debug, Clone)]
pub struct stackt {
    pub ss_sp: Arc<Mutex<Option<u8>>>,
    pub ss_size: Arc<Mutex<Option<usize>>>,
    pub ss_flags: Arc<Mutex<Option<i32>>>,
    pub pad_cgo_0: Arc<Mutex<Option<[u8; 4]>>>,
}

impl stackt {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.ss_sp.clone();
        let __go_clone_1_0 = { let __guard = self.ss_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.ss_flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            ss_sp: __go_clone_0_0,
            ss_size: __go_clone_1_0,
            ss_flags: __go_clone_2_0,
            pad_cgo_0: __go_clone_3_0,
        }
    }
}


impl Default for stackt {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            ss_sp: __go_default_0_0,
            ss_size: __go_default_1_0,
            ss_flags: __go_default_2_0,
            pad_cgo_0: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for stackt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.ss_sp.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.ss_size.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.ss_flags.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", format_slice(&self.pad_cgo_0));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct usigactiont {
    pub __sigaction_u: Arc<Mutex<Option<[u8; 8]>>>,
    pub sa_mask: Arc<Mutex<Option<u32>>>,
    pub sa_flags: Arc<Mutex<Option<i32>>>,
}

impl usigactiont {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__sigaction_u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.sa_mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.sa_flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __sigaction_u: __go_clone_0_0,
            sa_mask: __go_clone_1_0,
            sa_flags: __go_clone_2_0,
        }
    }
}


impl Default for usigactiont {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            __sigaction_u: __go_default_0_0,
            sa_mask: __go_default_1_0,
            sa_flags: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for usigactiont {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.__sigaction_u));
        let __go_fmt_1 = format!("{}", (*self.sa_mask.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.sa_flags.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct siginfo {
    pub si_signo: Arc<Mutex<Option<i32>>>,
    pub si_errno: Arc<Mutex<Option<i32>>>,
    pub si_code: Arc<Mutex<Option<i32>>>,
    pub si_pid: Arc<Mutex<Option<i32>>>,
    pub si_uid: Arc<Mutex<Option<u32>>>,
    pub si_status: Arc<Mutex<Option<i32>>>,
    pub si_addr: Arc<Mutex<Option<u8>>>,
    pub si_value: Arc<Mutex<Option<[u8; 8]>>>,
    pub si_band: Arc<Mutex<Option<i64>>>,
    pub __pad: Arc<Mutex<Option<[u64; 7]>>>,
}

impl siginfo {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.si_signo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.si_errno.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.si_code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.si_pid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.si_uid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.si_status.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = self.si_addr.clone();
        let __go_clone_7_0 = { let __guard = self.si_value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.si_band.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.__pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            si_signo: __go_clone_0_0,
            si_errno: __go_clone_1_0,
            si_code: __go_clone_2_0,
            si_pid: __go_clone_3_0,
            si_uid: __go_clone_4_0,
            si_status: __go_clone_5_0,
            si_addr: __go_clone_6_0,
            si_value: __go_clone_7_0,
            si_band: __go_clone_8_0,
            __pad: __go_clone_9_0,
        }
    }
}


impl Default for siginfo {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(None));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            si_signo: __go_default_0_0,
            si_errno: __go_default_1_0,
            si_code: __go_default_2_0,
            si_pid: __go_default_3_0,
            si_uid: __go_default_4_0,
            si_status: __go_default_5_0,
            si_addr: __go_default_6_0,
            si_value: __go_default_7_0,
            si_band: __go_default_8_0,
            __pad: __go_default_9_0,
        }
    }
}

impl std::fmt::Display for siginfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.si_signo.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.si_errno.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.si_code.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.si_pid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.si_uid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.si_status.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", { let __guard = self.si_addr.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_7 = format!("{}", format_slice(&self.si_value));
        let __go_fmt_8 = format!("{}", (*self.si_band.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", format_slice(&self.__pad));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9
        )
    }
}


#[derive(Debug, Clone)]
pub struct timespec {
    pub tv_sec: Arc<Mutex<Option<i64>>>,
    pub tv_nsec: Arc<Mutex<Option<i64>>>,
}

impl timespec {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.tv_sec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.tv_nsec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            tv_sec: __go_clone_0_0,
            tv_nsec: __go_clone_1_0,
        }
    }
}


impl Default for timespec {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            tv_sec: __go_default_0_0,
            tv_nsec: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for timespec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.tv_sec.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.tv_nsec.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct exceptionstate64 {
    pub far: Arc<Mutex<Option<u64>>>,
    pub esr: Arc<Mutex<Option<u32>>>,
    pub exc: Arc<Mutex<Option<u32>>>,
}

impl exceptionstate64 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.far.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.esr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.exc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            far: __go_clone_0_0,
            esr: __go_clone_1_0,
            exc: __go_clone_2_0,
        }
    }
}


impl Default for exceptionstate64 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            far: __go_default_0_0,
            esr: __go_default_1_0,
            exc: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for exceptionstate64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.far.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.esr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.exc.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct regs64 {
    pub x: Arc<Mutex<Option<[u64; 29]>>>,
    pub fp: Arc<Mutex<Option<u64>>>,
    pub lr: Arc<Mutex<Option<u64>>>,
    pub sp: Arc<Mutex<Option<u64>>>,
    pub pc: Arc<Mutex<Option<u64>>>,
    pub cpsr: Arc<Mutex<Option<u32>>>,
    pub __pad: Arc<Mutex<Option<u32>>>,
}

impl regs64 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.fp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.lr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.cpsr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.__pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            fp: __go_clone_1_0,
            lr: __go_clone_2_0,
            sp: __go_clone_3_0,
            pc: __go_clone_4_0,
            cpsr: __go_clone_5_0,
            __pad: __go_clone_6_0,
        }
    }
}


impl Default for regs64 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            fp: __go_default_1_0,
            lr: __go_default_2_0,
            sp: __go_default_3_0,
            pc: __go_default_4_0,
            cpsr: __go_default_5_0,
            __pad: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for regs64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.x));
        let __go_fmt_1 = format!("{}", (*self.fp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.lr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.sp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.pc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.cpsr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.__pad.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}


#[derive(Debug, Clone)]
pub struct neonstate64 {
    pub v: Arc<Mutex<Option<[u64; 64]>>>,
    pub fpsr: Arc<Mutex<Option<u32>>>,
    pub fpcr: Arc<Mutex<Option<u32>>>,
}

impl neonstate64 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.fpsr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.fpcr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            v: __go_clone_0_0,
            fpsr: __go_clone_1_0,
            fpcr: __go_clone_2_0,
        }
    }
}


impl Default for neonstate64 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            v: __go_default_0_0,
            fpsr: __go_default_1_0,
            fpcr: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for neonstate64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.v));
        let __go_fmt_1 = format!("{}", (*self.fpsr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.fpcr.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct mcontext64 {
    pub es: Arc<Mutex<Option<exceptionstate64>>>,
    pub ss: Arc<Mutex<Option<regs64>>>,
    pub ns: Arc<Mutex<Option<neonstate64>>>,
}

impl mcontext64 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.es.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.ss.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.ns.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            es: __go_clone_0_0,
            ss: __go_clone_1_0,
            ns: __go_clone_2_0,
        }
    }
}


impl Default for mcontext64 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(exceptionstate64::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(regs64::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(neonstate64::default())));
        Self {
            es: __go_default_0_0,
            ss: __go_default_1_0,
            ns: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for mcontext64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.es.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.ss.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.ns.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct ucontext {
    pub uc_onstack: Arc<Mutex<Option<i32>>>,
    pub uc_sigmask: Arc<Mutex<Option<u32>>>,
    pub uc_stack: Arc<Mutex<Option<stackt>>>,
    pub uc_link: Arc<Mutex<Option<ucontext>>>,
    pub uc_mcsize: Arc<Mutex<Option<u64>>>,
    pub uc_mcontext: Arc<Mutex<Option<mcontext64>>>,
}

impl ucontext {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.uc_onstack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.uc_sigmask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.uc_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.uc_link.clone();
        let __go_clone_4_0 = { let __guard = self.uc_mcsize.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = self.uc_mcontext.clone();
        Self {
            uc_onstack: __go_clone_0_0,
            uc_sigmask: __go_clone_1_0,
            uc_stack: __go_clone_2_0,
            uc_link: __go_clone_3_0,
            uc_mcsize: __go_clone_4_0,
            uc_mcontext: __go_clone_5_0,
        }
    }
}


impl Default for ucontext {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(stackt::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(None));
        Self {
            uc_onstack: __go_default_0_0,
            uc_sigmask: __go_default_1_0,
            uc_stack: __go_default_2_0,
            uc_link: __go_default_3_0,
            uc_mcsize: __go_default_4_0,
            uc_mcontext: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for ucontext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.uc_onstack.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.uc_sigmask.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.uc_stack.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", { let __guard = self.uc_link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_4 = format!("{}", (*self.uc_mcsize.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", { let __guard = self.uc_mcontext.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5)
    }
}


#[derive(Debug, Clone)]
pub struct keventt {
    pub ident: Arc<Mutex<Option<u64>>>,
    pub filter: Arc<Mutex<Option<i16>>>,
    pub flags: Arc<Mutex<Option<u16>>>,
    pub fflags: Arc<Mutex<Option<u32>>>,
    pub data: Arc<Mutex<Option<i64>>>,
    pub udata: Arc<Mutex<Option<u8>>>,
}

impl keventt {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.ident.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.filter.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.fflags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = self.udata.clone();
        Self {
            ident: __go_clone_0_0,
            filter: __go_clone_1_0,
            flags: __go_clone_2_0,
            fflags: __go_clone_3_0,
            data: __go_clone_4_0,
            udata: __go_clone_5_0,
        }
    }
}


impl Default for keventt {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(None));
        Self {
            ident: __go_default_0_0,
            filter: __go_default_1_0,
            flags: __go_default_2_0,
            fflags: __go_default_3_0,
            data: __go_default_4_0,
            udata: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for keventt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.ident.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.filter.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.flags.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.fflags.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.data.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", { let __guard = self.udata.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5)
    }
}


#[derive(Debug, Clone, Default)]
pub struct pthread(pub Arc<Mutex<Option<usize>>>);

impl Display for pthread {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for pthread {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for pthread {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for pthread {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for pthread {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<pthread> for usize {
    fn eq(&self, other: &pthread) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<pthread> for usize {
    fn partial_cmp(&self, other: &pthread) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for pthread {
    type Output = pthread;
    fn add(self, other: Self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for pthread {
    type Output = pthread;
    fn add(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<pthread> for usize {
    type Output = pthread;
    fn add(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for pthread {
    type Output = pthread;
    fn sub(self, other: Self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for pthread {
    type Output = pthread;
    fn sub(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<pthread> for usize {
    type Output = pthread;
    fn sub(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for pthread {
    type Output = pthread;
    fn mul(self, other: Self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for pthread {
    type Output = pthread;
    fn mul(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<pthread> for usize {
    type Output = pthread;
    fn mul(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for pthread {
    type Output = pthread;
    fn div(self, other: Self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for pthread {
    type Output = pthread;
    fn div(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<pthread> for usize {
    type Output = pthread;
    fn div(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for pthread {
    type Output = pthread;
    fn rem(self, other: Self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for pthread {
    type Output = pthread;
    fn rem(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<pthread> for usize {
    type Output = pthread;
    fn rem(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for pthread {
    type Output = pthread;
    fn bitand(self, other: Self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for pthread {
    type Output = pthread;
    fn bitand(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<pthread> for usize {
    type Output = pthread;
    fn bitand(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for pthread {
    type Output = pthread;
    fn bitor(self, other: Self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for pthread {
    type Output = pthread;
    fn bitor(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<pthread> for usize {
    type Output = pthread;
    fn bitor(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for pthread {
    type Output = pthread;
    fn bitxor(self, other: Self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for pthread {
    type Output = pthread;
    fn bitxor(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<pthread> for usize {
    type Output = pthread;
    fn bitxor(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for pthread {
    type Output = pthread;
    fn not(self) -> pthread {
        pthread(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for pthread {
    type Output = pthread;
    fn shl(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for pthread {
    type Output = pthread;
    fn shl(self, other: i32) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for pthread {
    type Output = pthread;
    fn shl(self, other: i8) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for pthread {
    type Output = pthread;
    fn shl(self, other: i16) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for pthread {
    type Output = pthread;
    fn shl(self, other: i64) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for pthread {
    type Output = pthread;
    fn shl(self, other: u32) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for pthread {
    type Output = pthread;
    fn shl(self, other: u8) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for pthread {
    type Output = pthread;
    fn shl(self, other: u16) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for pthread {
    type Output = pthread;
    fn shl(self, other: u64) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for pthread {
    type Output = pthread;
    fn shl(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for pthread {
    type Output = pthread;
    fn shr(self, other: pthread) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for pthread {
    type Output = pthread;
    fn shr(self, other: i32) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for pthread {
    type Output = pthread;
    fn shr(self, other: i8) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for pthread {
    type Output = pthread;
    fn shr(self, other: i16) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for pthread {
    type Output = pthread;
    fn shr(self, other: i64) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for pthread {
    type Output = pthread;
    fn shr(self, other: u32) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for pthread {
    type Output = pthread;
    fn shr(self, other: u8) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for pthread {
    type Output = pthread;
    fn shr(self, other: u16) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for pthread {
    type Output = pthread;
    fn shr(self, other: u64) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for pthread {
    type Output = pthread;
    fn shr(self, other: usize) -> pthread {
        pthread(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for pthread {}

impl Ord for pthread {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadattr {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 56]>>>,
}

impl pthreadattr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x__sig: __go_clone_0_0,
            x__opaque: __go_clone_1_0,
        }
    }
}


impl Default for pthreadattr {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            x__sig: __go_default_0_0,
            x__opaque: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pthreadattr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x__sig.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.x__opaque));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadmutex {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 56]>>>,
}

impl pthreadmutex {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x__sig: __go_clone_0_0,
            x__opaque: __go_clone_1_0,
        }
    }
}


impl Default for pthreadmutex {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            x__sig: __go_default_0_0,
            x__opaque: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pthreadmutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x__sig.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.x__opaque));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadmutexattr {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 8]>>>,
}

impl pthreadmutexattr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x__sig: __go_clone_0_0,
            x__opaque: __go_clone_1_0,
        }
    }
}


impl Default for pthreadmutexattr {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            x__sig: __go_default_0_0,
            x__opaque: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pthreadmutexattr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x__sig.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.x__opaque));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadcond {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 40]>>>,
}

impl pthreadcond {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x__sig: __go_clone_0_0,
            x__opaque: __go_clone_1_0,
        }
    }
}


impl Default for pthreadcond {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            x__sig: __go_default_0_0,
            x__opaque: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pthreadcond {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x__sig.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.x__opaque));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadcondattr {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 8]>>>,
}

impl pthreadcondattr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x__sig: __go_clone_0_0,
            x__opaque: __go_clone_1_0,
        }
    }
}


impl Default for pthreadcondattr {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            x__sig: __go_default_0_0,
            x__opaque: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pthreadcondattr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x__sig.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.x__opaque));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl timespec {
    ///go:nosplit
    pub fn set_nsec(&mut self, ns: Arc<Mutex<Option<i64>>>) {
        { let new_val = { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as i64; __tmp_x / __tmp_y }; *self.tv_sec.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1e9 as i64; __tmp_x % __tmp_y }; *self.tv_nsec.lock().unwrap() = Some(new_val); };
    }
}

impl GoValueClone for stackt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for usigactiont {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for siginfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for timespec {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for exceptionstate64 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for regs64 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for neonstate64 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mcontext64 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ucontext {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for keventt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pthreadattr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pthreadmutex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pthreadmutexattr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pthreadcond {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pthreadcondattr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
