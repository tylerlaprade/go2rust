use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::{cpu_arm64::{CACHE_LINE_PAD_SIZE}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// CacheLinePad is used to pad structs to avoid false sharing.
#[derive(Debug, Clone)]
pub struct CacheLinePad {
    pub __blank_0_0: Arc<Mutex<Option<[u8; 128]>>>,
}

impl CacheLinePad {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for CacheLinePad {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for CacheLinePad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.__blank_0_0))
    }
}

impl GoJsonDecode for CacheLinePad {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Option names should be lower case. e.g. avx instead of AVX.
#[derive(Debug, Clone)]
pub struct option {
    pub name: Arc<Mutex<Option<String>>>,
    pub feature: Arc<Mutex<Option<bool>>>,
    pub specified: Arc<Mutex<Option<bool>>>,
    pub enable: Arc<Mutex<Option<bool>>>,
}

impl option {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, feature: self.feature.clone(), specified: { let __guard = self.specified.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, enable: { let __guard = self.enable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for option {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), feature: Arc::new(Mutex::new(None)), specified: Arc::new(Mutex::new(Some(false))), enable: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for option {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), { let __guard = self.feature.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.specified.lock().unwrap().as_ref().unwrap()), (*self.enable.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for option {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Feature") {
            out.feature = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Specified") {
            out.specified = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Enable") {
            out.enable = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub static DebugOptions: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static CacheLineSize: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static X86: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct1>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ARM: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct2>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ARM64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct3>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Loong64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct4>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static MIPS64X: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct5>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static PPC64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct6>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static S390X: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct7>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static options: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<option>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *DebugOptions.lock().unwrap() = Some(false);
    *CacheLineSize.lock().unwrap() = Some(0);
    *X86.lock().unwrap() = Some(Default::default());
    *ARM.lock().unwrap() = Some(Default::default());
    *ARM64.lock().unwrap() = Some(Default::default());
    *Loong64.lock().unwrap() = Some(Default::default());
    *MIPS64X.lock().unwrap() = Some(Default::default());
    *PPC64.lock().unwrap() = Some(Default::default());
    *S390X.lock().unwrap() = Some(Default::default());
    *options.lock().unwrap() = Some(vec![]);
    *CacheLineSize.lock().unwrap() = Some(CACHE_LINE_PAD_SIZE as usize);
}


pub(crate) fn __go_zero_globals() {
    *DebugOptions.lock().unwrap() = Some(false);
    *CacheLineSize.lock().unwrap() = Some(0);
    *X86.lock().unwrap() = Some(Default::default());
    *ARM.lock().unwrap() = Some(Default::default());
    *ARM64.lock().unwrap() = Some(Default::default());
    *Loong64.lock().unwrap() = Some(Default::default());
    *MIPS64X.lock().unwrap() = Some(Default::default());
    *PPC64.lock().unwrap() = Some(Default::default());
    *S390X.lock().unwrap() = Some(Default::default());
    *options.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_0() {
    *CacheLineSize.lock().unwrap() = Some(CACHE_LINE_PAD_SIZE as usize);
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub __blank_0_0: Arc<Mutex<Option<CacheLinePad>>>,
    pub has_a_e_s: Arc<Mutex<Option<bool>>>,
    pub has_a_d_x: Arc<Mutex<Option<bool>>>,
    pub has_a_v_x: Arc<Mutex<Option<bool>>>,
    pub has_a_v_x2: Arc<Mutex<Option<bool>>>,
    pub has_a_v_x512_f: Arc<Mutex<Option<bool>>>,
    pub has_a_v_x512_b_w: Arc<Mutex<Option<bool>>>,
    pub has_a_v_x512_v_l: Arc<Mutex<Option<bool>>>,
    pub has_b_m_i1: Arc<Mutex<Option<bool>>>,
    pub has_b_m_i2: Arc<Mutex<Option<bool>>>,
    pub has_e_r_m_s: Arc<Mutex<Option<bool>>>,
    pub has_f_s_r_m: Arc<Mutex<Option<bool>>>,
    pub has_f_m_a: Arc<Mutex<Option<bool>>>,
    pub has_o_s_x_s_a_v_e: Arc<Mutex<Option<bool>>>,
    pub has_p_c_l_m_u_l_q_d_q: Arc<Mutex<Option<bool>>>,
    pub has_p_o_p_c_n_t: Arc<Mutex<Option<bool>>>,
    pub has_r_d_t_s_c_p: Arc<Mutex<Option<bool>>>,
    pub has_s_h_a: Arc<Mutex<Option<bool>>>,
    pub has_s_s_e3: Arc<Mutex<Option<bool>>>,
    pub has_s_s_s_e3: Arc<Mutex<Option<bool>>>,
    pub has_s_s_e41: Arc<Mutex<Option<bool>>>,
    pub has_s_s_e42: Arc<Mutex<Option<bool>>>,
    pub __blank_22_0: Arc<Mutex<Option<CacheLinePad>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_e_s: { let __guard = self.has_a_e_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_d_x: { let __guard = self.has_a_d_x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_v_x: { let __guard = self.has_a_v_x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_v_x2: { let __guard = self.has_a_v_x2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_v_x512_f: { let __guard = self.has_a_v_x512_f.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_v_x512_b_w: { let __guard = self.has_a_v_x512_b_w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_v_x512_v_l: { let __guard = self.has_a_v_x512_v_l.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_b_m_i1: { let __guard = self.has_b_m_i1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_b_m_i2: { let __guard = self.has_b_m_i2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_e_r_m_s: { let __guard = self.has_e_r_m_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_f_s_r_m: { let __guard = self.has_f_s_r_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_f_m_a: { let __guard = self.has_f_m_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_o_s_x_s_a_v_e: { let __guard = self.has_o_s_x_s_a_v_e.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_p_c_l_m_u_l_q_d_q: { let __guard = self.has_p_c_l_m_u_l_q_d_q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_p_o_p_c_n_t: { let __guard = self.has_p_o_p_c_n_t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_r_d_t_s_c_p: { let __guard = self.has_r_d_t_s_c_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_h_a: { let __guard = self.has_s_h_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_s_e3: { let __guard = self.has_s_s_e3.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_s_s_e3: { let __guard = self.has_s_s_s_e3.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_s_e41: { let __guard = self.has_s_s_e41.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_s_e42: { let __guard = self.has_s_s_e42.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_22_0: { let __guard = self.__blank_22_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))), has_a_e_s: Arc::new(Mutex::new(Some(false))), has_a_d_x: Arc::new(Mutex::new(Some(false))), has_a_v_x: Arc::new(Mutex::new(Some(false))), has_a_v_x2: Arc::new(Mutex::new(Some(false))), has_a_v_x512_f: Arc::new(Mutex::new(Some(false))), has_a_v_x512_b_w: Arc::new(Mutex::new(Some(false))), has_a_v_x512_v_l: Arc::new(Mutex::new(Some(false))), has_b_m_i1: Arc::new(Mutex::new(Some(false))), has_b_m_i2: Arc::new(Mutex::new(Some(false))), has_e_r_m_s: Arc::new(Mutex::new(Some(false))), has_f_s_r_m: Arc::new(Mutex::new(Some(false))), has_f_m_a: Arc::new(Mutex::new(Some(false))), has_o_s_x_s_a_v_e: Arc::new(Mutex::new(Some(false))), has_p_c_l_m_u_l_q_d_q: Arc::new(Mutex::new(Some(false))), has_p_o_p_c_n_t: Arc::new(Mutex::new(Some(false))), has_r_d_t_s_c_p: Arc::new(Mutex::new(Some(false))), has_s_h_a: Arc::new(Mutex::new(Some(false))), has_s_s_e3: Arc::new(Mutex::new(Some(false))), has_s_s_s_e3: Arc::new(Mutex::new(Some(false))), has_s_s_e41: Arc::new(Mutex::new(Some(false))), has_s_s_e42: Arc::new(Mutex::new(Some(false))), __blank_22_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.has_a_e_s.lock().unwrap().as_ref().unwrap()), (*self.has_a_d_x.lock().unwrap().as_ref().unwrap()), (*self.has_a_v_x.lock().unwrap().as_ref().unwrap()), (*self.has_a_v_x2.lock().unwrap().as_ref().unwrap()), (*self.has_a_v_x512_f.lock().unwrap().as_ref().unwrap()), (*self.has_a_v_x512_b_w.lock().unwrap().as_ref().unwrap()), (*self.has_a_v_x512_v_l.lock().unwrap().as_ref().unwrap()), (*self.has_b_m_i1.lock().unwrap().as_ref().unwrap()), (*self.has_b_m_i2.lock().unwrap().as_ref().unwrap()), (*self.has_e_r_m_s.lock().unwrap().as_ref().unwrap()), (*self.has_f_s_r_m.lock().unwrap().as_ref().unwrap()), (*self.has_f_m_a.lock().unwrap().as_ref().unwrap()), (*self.has_o_s_x_s_a_v_e.lock().unwrap().as_ref().unwrap()), (*self.has_p_c_l_m_u_l_q_d_q.lock().unwrap().as_ref().unwrap()), (*self.has_p_o_p_c_n_t.lock().unwrap().as_ref().unwrap()), (*self.has_r_d_t_s_c_p.lock().unwrap().as_ref().unwrap()), (*self.has_s_h_a.lock().unwrap().as_ref().unwrap()), (*self.has_s_s_e3.lock().unwrap().as_ref().unwrap()), (*self.has_s_s_s_e3.lock().unwrap().as_ref().unwrap()), (*self.has_s_s_e41.lock().unwrap().as_ref().unwrap()), (*self.has_s_s_e42.lock().unwrap().as_ref().unwrap()), (*self.__blank_22_0.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("HasAES") {
            out.has_a_e_s = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasADX") {
            out.has_a_d_x = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAVX") {
            out.has_a_v_x = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAVX2") {
            out.has_a_v_x2 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAVX512F") {
            out.has_a_v_x512_f = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAVX512BW") {
            out.has_a_v_x512_b_w = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAVX512VL") {
            out.has_a_v_x512_v_l = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasBMI1") {
            out.has_b_m_i1 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasBMI2") {
            out.has_b_m_i2 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasERMS") {
            out.has_e_r_m_s = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasFSRM") {
            out.has_f_s_r_m = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasFMA") {
            out.has_f_m_a = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasOSXSAVE") {
            out.has_o_s_x_s_a_v_e = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasPCLMULQDQ") {
            out.has_p_c_l_m_u_l_q_d_q = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasPOPCNT") {
            out.has_p_o_p_c_n_t = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasRDTSCP") {
            out.has_r_d_t_s_c_p = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSHA") {
            out.has_s_h_a = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSSE3") {
            out.has_s_s_e3 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSSSE3") {
            out.has_s_s_s_e3 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSSE41") {
            out.has_s_s_e41 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSSE42") {
            out.has_s_s_e42 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct2 {
    pub __blank_0_0: Arc<Mutex<Option<CacheLinePad>>>,
    pub has_v_f_pv4: Arc<Mutex<Option<bool>>>,
    pub has_i_d_i_v_a: Arc<Mutex<Option<bool>>>,
    pub has_v7_atomics: Arc<Mutex<Option<bool>>>,
    pub __blank_4_0: Arc<Mutex<Option<CacheLinePad>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_v_f_pv4: { let __guard = self.has_v_f_pv4.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_i_d_i_v_a: { let __guard = self.has_i_d_i_v_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_v7_atomics: { let __guard = self.has_v7_atomics.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_4_0: { let __guard = self.__blank_4_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))), has_v_f_pv4: Arc::new(Mutex::new(Some(false))), has_i_d_i_v_a: Arc::new(Mutex::new(Some(false))), has_v7_atomics: Arc::new(Mutex::new(Some(false))), __blank_4_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.has_v_f_pv4.lock().unwrap().as_ref().unwrap()), (*self.has_i_d_i_v_a.lock().unwrap().as_ref().unwrap()), (*self.has_v7_atomics.lock().unwrap().as_ref().unwrap()), (*self.__blank_4_0.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("HasVFPv4") {
            out.has_v_f_pv4 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasIDIVA") {
            out.has_i_d_i_v_a = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasV7Atomics") {
            out.has_v7_atomics = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub __blank_0_0: Arc<Mutex<Option<CacheLinePad>>>,
    pub has_a_e_s: Arc<Mutex<Option<bool>>>,
    pub has_p_m_u_l_l: Arc<Mutex<Option<bool>>>,
    pub has_s_h_a1: Arc<Mutex<Option<bool>>>,
    pub has_s_h_a2: Arc<Mutex<Option<bool>>>,
    pub has_s_h_a512: Arc<Mutex<Option<bool>>>,
    pub has_c_r_c32: Arc<Mutex<Option<bool>>>,
    pub has_a_t_o_m_i_c_s: Arc<Mutex<Option<bool>>>,
    pub has_c_p_u_i_d: Arc<Mutex<Option<bool>>>,
    pub has_d_i_t: Arc<Mutex<Option<bool>>>,
    pub is_neoverse: Arc<Mutex<Option<bool>>>,
    pub __blank_11_0: Arc<Mutex<Option<CacheLinePad>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_e_s: { let __guard = self.has_a_e_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_p_m_u_l_l: { let __guard = self.has_p_m_u_l_l.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_h_a1: { let __guard = self.has_s_h_a1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_h_a2: { let __guard = self.has_s_h_a2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_h_a512: { let __guard = self.has_s_h_a512.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_c_r_c32: { let __guard = self.has_c_r_c32.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_t_o_m_i_c_s: { let __guard = self.has_a_t_o_m_i_c_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_c_p_u_i_d: { let __guard = self.has_c_p_u_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_d_i_t: { let __guard = self.has_d_i_t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_neoverse: { let __guard = self.is_neoverse.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_11_0: { let __guard = self.__blank_11_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))), has_a_e_s: Arc::new(Mutex::new(Some(false))), has_p_m_u_l_l: Arc::new(Mutex::new(Some(false))), has_s_h_a1: Arc::new(Mutex::new(Some(false))), has_s_h_a2: Arc::new(Mutex::new(Some(false))), has_s_h_a512: Arc::new(Mutex::new(Some(false))), has_c_r_c32: Arc::new(Mutex::new(Some(false))), has_a_t_o_m_i_c_s: Arc::new(Mutex::new(Some(false))), has_c_p_u_i_d: Arc::new(Mutex::new(Some(false))), has_d_i_t: Arc::new(Mutex::new(Some(false))), is_neoverse: Arc::new(Mutex::new(Some(false))), __blank_11_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.has_a_e_s.lock().unwrap().as_ref().unwrap()), (*self.has_p_m_u_l_l.lock().unwrap().as_ref().unwrap()), (*self.has_s_h_a1.lock().unwrap().as_ref().unwrap()), (*self.has_s_h_a2.lock().unwrap().as_ref().unwrap()), (*self.has_s_h_a512.lock().unwrap().as_ref().unwrap()), (*self.has_c_r_c32.lock().unwrap().as_ref().unwrap()), (*self.has_a_t_o_m_i_c_s.lock().unwrap().as_ref().unwrap()), (*self.has_c_p_u_i_d.lock().unwrap().as_ref().unwrap()), (*self.has_d_i_t.lock().unwrap().as_ref().unwrap()), (*self.is_neoverse.lock().unwrap().as_ref().unwrap()), (*self.__blank_11_0.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct3 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("HasAES") {
            out.has_a_e_s = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasPMULL") {
            out.has_p_m_u_l_l = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSHA1") {
            out.has_s_h_a1 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSHA2") {
            out.has_s_h_a2 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSHA512") {
            out.has_s_h_a512 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasCRC32") {
            out.has_c_r_c32 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasATOMICS") {
            out.has_a_t_o_m_i_c_s = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasCPUID") {
            out.has_c_p_u_i_d = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasDIT") {
            out.has_d_i_t = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IsNeoverse") {
            out.is_neoverse = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct4 {
    pub __blank_0_0: Arc<Mutex<Option<CacheLinePad>>>,
    pub has_l_s_x: Arc<Mutex<Option<bool>>>,
    pub has_c_r_c32: Arc<Mutex<Option<bool>>>,
    pub has_l_a_m_c_a_s: Arc<Mutex<Option<bool>>>,
    pub has_l_a_m__b_h: Arc<Mutex<Option<bool>>>,
    pub __blank_5_0: Arc<Mutex<Option<CacheLinePad>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_l_s_x: { let __guard = self.has_l_s_x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_c_r_c32: { let __guard = self.has_c_r_c32.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_l_a_m_c_a_s: { let __guard = self.has_l_a_m_c_a_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_l_a_m__b_h: { let __guard = self.has_l_a_m__b_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_5_0: { let __guard = self.__blank_5_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))), has_l_s_x: Arc::new(Mutex::new(Some(false))), has_c_r_c32: Arc::new(Mutex::new(Some(false))), has_l_a_m_c_a_s: Arc::new(Mutex::new(Some(false))), has_l_a_m__b_h: Arc::new(Mutex::new(Some(false))), __blank_5_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.has_l_s_x.lock().unwrap().as_ref().unwrap()), (*self.has_c_r_c32.lock().unwrap().as_ref().unwrap()), (*self.has_l_a_m_c_a_s.lock().unwrap().as_ref().unwrap()), (*self.has_l_a_m__b_h.lock().unwrap().as_ref().unwrap()), (*self.__blank_5_0.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct4 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("HasLSX") {
            out.has_l_s_x = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasCRC32") {
            out.has_c_r_c32 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasLAMCAS") {
            out.has_l_a_m_c_a_s = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasLAM_BH") {
            out.has_l_a_m__b_h = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct5 {
    pub __blank_0_0: Arc<Mutex<Option<CacheLinePad>>>,
    pub has_m_s_a: Arc<Mutex<Option<bool>>>,
    pub __blank_2_0: Arc<Mutex<Option<CacheLinePad>>>,
}
impl AnonymousStruct5 {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_m_s_a: { let __guard = self.has_m_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_2_0: { let __guard = self.__blank_2_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))), has_m_s_a: Arc::new(Mutex::new(Some(false))), __blank_2_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.has_m_s_a.lock().unwrap().as_ref().unwrap()), (*self.__blank_2_0.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct5 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("HasMSA") {
            out.has_m_s_a = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct6 {
    pub __blank_0_0: Arc<Mutex<Option<CacheLinePad>>>,
    pub has_d_a_r_n: Arc<Mutex<Option<bool>>>,
    pub has_s_c_v: Arc<Mutex<Option<bool>>>,
    pub is_p_o_w_e_r8: Arc<Mutex<Option<bool>>>,
    pub is_p_o_w_e_r9: Arc<Mutex<Option<bool>>>,
    pub is_p_o_w_e_r10: Arc<Mutex<Option<bool>>>,
    pub __blank_6_0: Arc<Mutex<Option<CacheLinePad>>>,
}
impl AnonymousStruct6 {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_d_a_r_n: { let __guard = self.has_d_a_r_n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_c_v: { let __guard = self.has_s_c_v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_p_o_w_e_r8: { let __guard = self.is_p_o_w_e_r8.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_p_o_w_e_r9: { let __guard = self.is_p_o_w_e_r9.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_p_o_w_e_r10: { let __guard = self.is_p_o_w_e_r10.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_6_0: { let __guard = self.__blank_6_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))), has_d_a_r_n: Arc::new(Mutex::new(Some(false))), has_s_c_v: Arc::new(Mutex::new(Some(false))), is_p_o_w_e_r8: Arc::new(Mutex::new(Some(false))), is_p_o_w_e_r9: Arc::new(Mutex::new(Some(false))), is_p_o_w_e_r10: Arc::new(Mutex::new(Some(false))), __blank_6_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.has_d_a_r_n.lock().unwrap().as_ref().unwrap()), (*self.has_s_c_v.lock().unwrap().as_ref().unwrap()), (*self.is_p_o_w_e_r8.lock().unwrap().as_ref().unwrap()), (*self.is_p_o_w_e_r9.lock().unwrap().as_ref().unwrap()), (*self.is_p_o_w_e_r10.lock().unwrap().as_ref().unwrap()), (*self.__blank_6_0.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct6 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("HasDARN") {
            out.has_d_a_r_n = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSCV") {
            out.has_s_c_v = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IsPOWER8") {
            out.is_p_o_w_e_r8 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IsPOWER9") {
            out.is_p_o_w_e_r9 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IsPOWER10") {
            out.is_p_o_w_e_r10 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct7 {
    pub __blank_0_0: Arc<Mutex<Option<CacheLinePad>>>,
    pub has_z_a_r_c_h: Arc<Mutex<Option<bool>>>,
    pub has_s_t_f_l_e: Arc<Mutex<Option<bool>>>,
    pub has_l_d_i_s_p: Arc<Mutex<Option<bool>>>,
    pub has_e_i_m_m: Arc<Mutex<Option<bool>>>,
    pub has_d_f_p: Arc<Mutex<Option<bool>>>,
    pub has_e_t_f3_e_h: Arc<Mutex<Option<bool>>>,
    pub has_m_s_a: Arc<Mutex<Option<bool>>>,
    pub has_a_e_s: Arc<Mutex<Option<bool>>>,
    pub has_a_e_s_c_b_c: Arc<Mutex<Option<bool>>>,
    pub has_a_e_s_c_t_r: Arc<Mutex<Option<bool>>>,
    pub has_a_e_s_g_c_m: Arc<Mutex<Option<bool>>>,
    pub has_g_h_a_s_h: Arc<Mutex<Option<bool>>>,
    pub has_s_h_a1: Arc<Mutex<Option<bool>>>,
    pub has_s_h_a256: Arc<Mutex<Option<bool>>>,
    pub has_s_h_a512: Arc<Mutex<Option<bool>>>,
    pub has_s_h_a3: Arc<Mutex<Option<bool>>>,
    pub has_v_x: Arc<Mutex<Option<bool>>>,
    pub has_v_x_e: Arc<Mutex<Option<bool>>>,
    pub has_k_d_s_a: Arc<Mutex<Option<bool>>>,
    pub has_e_c_d_s_a: Arc<Mutex<Option<bool>>>,
    pub has_e_d_d_s_a: Arc<Mutex<Option<bool>>>,
    pub __blank_22_0: Arc<Mutex<Option<CacheLinePad>>>,
}
impl AnonymousStruct7 {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_z_a_r_c_h: { let __guard = self.has_z_a_r_c_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_t_f_l_e: { let __guard = self.has_s_t_f_l_e.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_l_d_i_s_p: { let __guard = self.has_l_d_i_s_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_e_i_m_m: { let __guard = self.has_e_i_m_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_d_f_p: { let __guard = self.has_d_f_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_e_t_f3_e_h: { let __guard = self.has_e_t_f3_e_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_m_s_a: { let __guard = self.has_m_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_e_s: { let __guard = self.has_a_e_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_e_s_c_b_c: { let __guard = self.has_a_e_s_c_b_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_e_s_c_t_r: { let __guard = self.has_a_e_s_c_t_r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_a_e_s_g_c_m: { let __guard = self.has_a_e_s_g_c_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_g_h_a_s_h: { let __guard = self.has_g_h_a_s_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_h_a1: { let __guard = self.has_s_h_a1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_h_a256: { let __guard = self.has_s_h_a256.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_h_a512: { let __guard = self.has_s_h_a512.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_s_h_a3: { let __guard = self.has_s_h_a3.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_v_x: { let __guard = self.has_v_x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_v_x_e: { let __guard = self.has_v_x_e.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_k_d_s_a: { let __guard = self.has_k_d_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_e_c_d_s_a: { let __guard = self.has_e_c_d_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_e_d_d_s_a: { let __guard = self.has_e_d_d_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_22_0: { let __guard = self.__blank_22_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))), has_z_a_r_c_h: Arc::new(Mutex::new(Some(false))), has_s_t_f_l_e: Arc::new(Mutex::new(Some(false))), has_l_d_i_s_p: Arc::new(Mutex::new(Some(false))), has_e_i_m_m: Arc::new(Mutex::new(Some(false))), has_d_f_p: Arc::new(Mutex::new(Some(false))), has_e_t_f3_e_h: Arc::new(Mutex::new(Some(false))), has_m_s_a: Arc::new(Mutex::new(Some(false))), has_a_e_s: Arc::new(Mutex::new(Some(false))), has_a_e_s_c_b_c: Arc::new(Mutex::new(Some(false))), has_a_e_s_c_t_r: Arc::new(Mutex::new(Some(false))), has_a_e_s_g_c_m: Arc::new(Mutex::new(Some(false))), has_g_h_a_s_h: Arc::new(Mutex::new(Some(false))), has_s_h_a1: Arc::new(Mutex::new(Some(false))), has_s_h_a256: Arc::new(Mutex::new(Some(false))), has_s_h_a512: Arc::new(Mutex::new(Some(false))), has_s_h_a3: Arc::new(Mutex::new(Some(false))), has_v_x: Arc::new(Mutex::new(Some(false))), has_v_x_e: Arc::new(Mutex::new(Some(false))), has_k_d_s_a: Arc::new(Mutex::new(Some(false))), has_e_c_d_s_a: Arc::new(Mutex::new(Some(false))), has_e_d_d_s_a: Arc::new(Mutex::new(Some(false))), __blank_22_0: Arc::new(Mutex::new(Some(CacheLinePad::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.has_z_a_r_c_h.lock().unwrap().as_ref().unwrap()), (*self.has_s_t_f_l_e.lock().unwrap().as_ref().unwrap()), (*self.has_l_d_i_s_p.lock().unwrap().as_ref().unwrap()), (*self.has_e_i_m_m.lock().unwrap().as_ref().unwrap()), (*self.has_d_f_p.lock().unwrap().as_ref().unwrap()), (*self.has_e_t_f3_e_h.lock().unwrap().as_ref().unwrap()), (*self.has_m_s_a.lock().unwrap().as_ref().unwrap()), (*self.has_a_e_s.lock().unwrap().as_ref().unwrap()), (*self.has_a_e_s_c_b_c.lock().unwrap().as_ref().unwrap()), (*self.has_a_e_s_c_t_r.lock().unwrap().as_ref().unwrap()), (*self.has_a_e_s_g_c_m.lock().unwrap().as_ref().unwrap()), (*self.has_g_h_a_s_h.lock().unwrap().as_ref().unwrap()), (*self.has_s_h_a1.lock().unwrap().as_ref().unwrap()), (*self.has_s_h_a256.lock().unwrap().as_ref().unwrap()), (*self.has_s_h_a512.lock().unwrap().as_ref().unwrap()), (*self.has_s_h_a3.lock().unwrap().as_ref().unwrap()), (*self.has_v_x.lock().unwrap().as_ref().unwrap()), (*self.has_v_x_e.lock().unwrap().as_ref().unwrap()), (*self.has_k_d_s_a.lock().unwrap().as_ref().unwrap()), (*self.has_e_c_d_s_a.lock().unwrap().as_ref().unwrap()), (*self.has_e_d_d_s_a.lock().unwrap().as_ref().unwrap()), (*self.__blank_22_0.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct7 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("HasZARCH") {
            out.has_z_a_r_c_h = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSTFLE") {
            out.has_s_t_f_l_e = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasLDISP") {
            out.has_l_d_i_s_p = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasEIMM") {
            out.has_e_i_m_m = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasDFP") {
            out.has_d_f_p = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasETF3EH") {
            out.has_e_t_f3_e_h = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasMSA") {
            out.has_m_s_a = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAES") {
            out.has_a_e_s = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAESCBC") {
            out.has_a_e_s_c_b_c = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAESCTR") {
            out.has_a_e_s_c_t_r = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasAESGCM") {
            out.has_a_e_s_g_c_m = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasGHASH") {
            out.has_g_h_a_s_h = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSHA1") {
            out.has_s_h_a1 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSHA256") {
            out.has_s_h_a256 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSHA512") {
            out.has_s_h_a512 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasSHA3") {
            out.has_s_h_a3 = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasVX") {
            out.has_v_x = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasVXE") {
            out.has_v_x_e = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasKDSA") {
            out.has_k_d_s_a = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasECDSA") {
            out.has_e_c_d_s_a = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("HasEDDSA") {
            out.has_e_d_d_s_a = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub type ARM = AnonymousStruct2;


pub type ARM64 = AnonymousStruct3;


pub type Loong64 = AnonymousStruct4;


pub type MIPS64X = AnonymousStruct5;


pub type PPC64 = AnonymousStruct6;


pub type S390X = AnonymousStruct7;


pub type X86 = AnonymousStruct1;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for CacheLinePad {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for option {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
