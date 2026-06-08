use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

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
        Self { ss_sp: self.ss_sp.clone(), ss_size: { let __guard = self.ss_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ss_flags: { let __guard = self.ss_flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad_cgo_0: { let __guard = self.pad_cgo_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for stackt {
    fn default() -> Self {
        Self { ss_sp: Arc::new(Mutex::new(None)), ss_size: Arc::new(Mutex::new(Some(0))), ss_flags: Arc::new(Mutex::new(Some(0))), pad_cgo_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for stackt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", { let __guard = self.ss_sp.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.ss_size.lock().unwrap().as_ref().unwrap()), (*self.ss_flags.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad_cgo_0))
    }
}

impl GoJsonDecode for stackt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { __sigaction_u: { let __guard = self.__sigaction_u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sa_mask: { let __guard = self.sa_mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sa_flags: { let __guard = self.sa_flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for usigactiont {
    fn default() -> Self {
        Self { __sigaction_u: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), sa_mask: Arc::new(Mutex::new(Some(0))), sa_flags: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for usigactiont {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.__sigaction_u), (*self.sa_mask.lock().unwrap().as_ref().unwrap()), (*self.sa_flags.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for usigactiont {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { si_signo: { let __guard = self.si_signo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, si_errno: { let __guard = self.si_errno.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, si_code: { let __guard = self.si_code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, si_pid: { let __guard = self.si_pid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, si_uid: { let __guard = self.si_uid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, si_status: { let __guard = self.si_status.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, si_addr: self.si_addr.clone(), si_value: { let __guard = self.si_value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, si_band: { let __guard = self.si_band.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __pad: { let __guard = self.__pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for siginfo {
    fn default() -> Self {
        Self { si_signo: Arc::new(Mutex::new(Some(0))), si_errno: Arc::new(Mutex::new(Some(0))), si_code: Arc::new(Mutex::new(Some(0))), si_pid: Arc::new(Mutex::new(Some(0))), si_uid: Arc::new(Mutex::new(Some(0))), si_status: Arc::new(Mutex::new(Some(0))), si_addr: Arc::new(Mutex::new(None)), si_value: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), si_band: Arc::new(Mutex::new(Some(0))), __pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for siginfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {}}}", (*self.si_signo.lock().unwrap().as_ref().unwrap()), (*self.si_errno.lock().unwrap().as_ref().unwrap()), (*self.si_code.lock().unwrap().as_ref().unwrap()), (*self.si_pid.lock().unwrap().as_ref().unwrap()), (*self.si_uid.lock().unwrap().as_ref().unwrap()), (*self.si_status.lock().unwrap().as_ref().unwrap()), { let __guard = self.si_addr.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice(&self.si_value), (*self.si_band.lock().unwrap().as_ref().unwrap()), format_slice(&self.__pad))
    }
}

impl GoJsonDecode for siginfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct timespec {
    pub tv_sec: Arc<Mutex<Option<i64>>>,
    pub tv_nsec: Arc<Mutex<Option<i64>>>,
}

impl timespec {
    pub fn __go_value_clone(&self) -> Self {
        Self { tv_sec: { let __guard = self.tv_sec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tv_nsec: { let __guard = self.tv_nsec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for timespec {
    fn default() -> Self {
        Self { tv_sec: Arc::new(Mutex::new(Some(0))), tv_nsec: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for timespec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tv_sec.lock().unwrap().as_ref().unwrap()), (*self.tv_nsec.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for timespec {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { far: { let __guard = self.far.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, esr: { let __guard = self.esr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, exc: { let __guard = self.exc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for exceptionstate64 {
    fn default() -> Self {
        Self { far: Arc::new(Mutex::new(Some(0))), esr: Arc::new(Mutex::new(Some(0))), exc: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for exceptionstate64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.far.lock().unwrap().as_ref().unwrap()), (*self.esr.lock().unwrap().as_ref().unwrap()), (*self.exc.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for exceptionstate64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { x: { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fp: { let __guard = self.fp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lr: { let __guard = self.lr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sp: { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pc: { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cpsr: { let __guard = self.cpsr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __pad: { let __guard = self.__pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for regs64 {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), fp: Arc::new(Mutex::new(Some(0))), lr: Arc::new(Mutex::new(Some(0))), sp: Arc::new(Mutex::new(Some(0))), pc: Arc::new(Mutex::new(Some(0))), cpsr: Arc::new(Mutex::new(Some(0))), __pad: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for regs64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", format_slice(&self.x), (*self.fp.lock().unwrap().as_ref().unwrap()), (*self.lr.lock().unwrap().as_ref().unwrap()), (*self.sp.lock().unwrap().as_ref().unwrap()), (*self.pc.lock().unwrap().as_ref().unwrap()), (*self.cpsr.lock().unwrap().as_ref().unwrap()), (*self.__pad.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for regs64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { v: { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fpsr: { let __guard = self.fpsr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fpcr: { let __guard = self.fpcr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for neonstate64 {
    fn default() -> Self {
        Self { v: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), fpsr: Arc::new(Mutex::new(Some(0))), fpcr: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for neonstate64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.v), (*self.fpsr.lock().unwrap().as_ref().unwrap()), (*self.fpcr.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for neonstate64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { es: { let __guard = self.es.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ss: { let __guard = self.ss.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ns: { let __guard = self.ns.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mcontext64 {
    fn default() -> Self {
        Self { es: Arc::new(Mutex::new(Some(exceptionstate64::default()))), ss: Arc::new(Mutex::new(Some(regs64::default()))), ns: Arc::new(Mutex::new(Some(neonstate64::default()))) }
    }
}

impl std::fmt::Display for mcontext64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.es.lock().unwrap().as_ref().unwrap()), (*self.ss.lock().unwrap().as_ref().unwrap()), (*self.ns.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mcontext64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { uc_onstack: { let __guard = self.uc_onstack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, uc_sigmask: { let __guard = self.uc_sigmask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, uc_stack: { let __guard = self.uc_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, uc_link: self.uc_link.clone(), uc_mcsize: { let __guard = self.uc_mcsize.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, uc_mcontext: self.uc_mcontext.clone() }
    }
}


impl Default for ucontext {
    fn default() -> Self {
        Self { uc_onstack: Arc::new(Mutex::new(Some(0))), uc_sigmask: Arc::new(Mutex::new(Some(0))), uc_stack: Arc::new(Mutex::new(Some(stackt::default()))), uc_link: Arc::new(Mutex::new(None)), uc_mcsize: Arc::new(Mutex::new(Some(0))), uc_mcontext: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for ucontext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.uc_onstack.lock().unwrap().as_ref().unwrap()), (*self.uc_sigmask.lock().unwrap().as_ref().unwrap()), (*self.uc_stack.lock().unwrap().as_ref().unwrap()), { let __guard = self.uc_link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.uc_mcsize.lock().unwrap().as_ref().unwrap()), { let __guard = self.uc_mcontext.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for ucontext {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { ident: { let __guard = self.ident.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, filter: { let __guard = self.filter.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fflags: { let __guard = self.fflags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, udata: self.udata.clone() }
    }
}


impl Default for keventt {
    fn default() -> Self {
        Self { ident: Arc::new(Mutex::new(Some(0))), filter: Arc::new(Mutex::new(Some(0))), flags: Arc::new(Mutex::new(Some(0))), fflags: Arc::new(Mutex::new(Some(0))), data: Arc::new(Mutex::new(Some(0))), udata: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for keventt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.ident.lock().unwrap().as_ref().unwrap()), (*self.filter.lock().unwrap().as_ref().unwrap()), (*self.flags.lock().unwrap().as_ref().unwrap()), (*self.fflags.lock().unwrap().as_ref().unwrap()), (*self.data.lock().unwrap().as_ref().unwrap()), { let __guard = self.udata.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for keventt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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
        Self { x__sig: { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x__opaque: { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pthreadattr {
    fn default() -> Self {
        Self { x__sig: Arc::new(Mutex::new(Some(0))), x__opaque: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for pthreadattr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x__sig.lock().unwrap().as_ref().unwrap()), format_slice(&self.x__opaque))
    }
}

impl GoJsonDecode for pthreadattr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("X__sig") {
            out.x__sig = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("X__opaque") {
            out.x__opaque = <Arc<Mutex<Option<[i8; 56]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadmutex {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 56]>>>,
}

impl pthreadmutex {
    pub fn __go_value_clone(&self) -> Self {
        Self { x__sig: { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x__opaque: { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pthreadmutex {
    fn default() -> Self {
        Self { x__sig: Arc::new(Mutex::new(Some(0))), x__opaque: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for pthreadmutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x__sig.lock().unwrap().as_ref().unwrap()), format_slice(&self.x__opaque))
    }
}

impl GoJsonDecode for pthreadmutex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("X__sig") {
            out.x__sig = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("X__opaque") {
            out.x__opaque = <Arc<Mutex<Option<[i8; 56]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadmutexattr {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 8]>>>,
}

impl pthreadmutexattr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x__sig: { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x__opaque: { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pthreadmutexattr {
    fn default() -> Self {
        Self { x__sig: Arc::new(Mutex::new(Some(0))), x__opaque: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for pthreadmutexattr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x__sig.lock().unwrap().as_ref().unwrap()), format_slice(&self.x__opaque))
    }
}

impl GoJsonDecode for pthreadmutexattr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("X__sig") {
            out.x__sig = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("X__opaque") {
            out.x__opaque = <Arc<Mutex<Option<[i8; 8]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadcond {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 40]>>>,
}

impl pthreadcond {
    pub fn __go_value_clone(&self) -> Self {
        Self { x__sig: { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x__opaque: { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pthreadcond {
    fn default() -> Self {
        Self { x__sig: Arc::new(Mutex::new(Some(0))), x__opaque: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for pthreadcond {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x__sig.lock().unwrap().as_ref().unwrap()), format_slice(&self.x__opaque))
    }
}

impl GoJsonDecode for pthreadcond {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("X__sig") {
            out.x__sig = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("X__opaque") {
            out.x__opaque = <Arc<Mutex<Option<[i8; 40]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct pthreadcondattr {
    pub x__sig: Arc<Mutex<Option<i64>>>,
    pub x__opaque: Arc<Mutex<Option<[i8; 8]>>>,
}

impl pthreadcondattr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x__sig: { let __guard = self.x__sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x__opaque: { let __guard = self.x__opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pthreadcondattr {
    fn default() -> Self {
        Self { x__sig: Arc::new(Mutex::new(Some(0))), x__opaque: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for pthreadcondattr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x__sig.lock().unwrap().as_ref().unwrap()), format_slice(&self.x__opaque))
    }
}

impl GoJsonDecode for pthreadcondattr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("X__sig") {
            out.x__sig = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("X__opaque") {
            out.x__opaque = <Arc<Mutex<Option<[i8; 8]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
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
