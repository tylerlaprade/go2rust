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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
        }
    }
}


impl Default for CacheLinePad {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            __blank_0_0: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for CacheLinePad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.__blank_0_0));
        write!(f, "{{{}}}", __go_fmt_0)
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
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.feature.clone();
        let __go_clone_2_0 = { let __guard = self.specified.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.enable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            feature: __go_clone_1_0,
            specified: __go_clone_2_0,
            enable: __go_clone_3_0,
        }
    }
}


impl Default for option {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            name: __go_default_0_0,
            feature: __go_default_1_0,
            specified: __go_default_2_0,
            enable: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for option {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.feature.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.specified.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.enable.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.has_a_e_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.has_a_d_x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.has_a_v_x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.has_a_v_x2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.has_a_v_x512_f.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.has_a_v_x512_b_w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.has_a_v_x512_v_l.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.has_b_m_i1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.has_b_m_i2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.has_e_r_m_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.has_f_s_r_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.has_f_m_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.has_o_s_x_s_a_v_e.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.has_p_c_l_m_u_l_q_d_q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.has_p_o_p_c_n_t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.has_r_d_t_s_c_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.has_s_h_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.has_s_s_e3.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.has_s_s_s_e3.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.has_s_s_e41.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.has_s_s_e42.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = { let __guard = self.__blank_22_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            has_a_e_s: __go_clone_1_0,
            has_a_d_x: __go_clone_2_0,
            has_a_v_x: __go_clone_3_0,
            has_a_v_x2: __go_clone_4_0,
            has_a_v_x512_f: __go_clone_5_0,
            has_a_v_x512_b_w: __go_clone_6_0,
            has_a_v_x512_v_l: __go_clone_7_0,
            has_b_m_i1: __go_clone_8_0,
            has_b_m_i2: __go_clone_9_0,
            has_e_r_m_s: __go_clone_10_0,
            has_f_s_r_m: __go_clone_11_0,
            has_f_m_a: __go_clone_12_0,
            has_o_s_x_s_a_v_e: __go_clone_13_0,
            has_p_c_l_m_u_l_q_d_q: __go_clone_14_0,
            has_p_o_p_c_n_t: __go_clone_15_0,
            has_r_d_t_s_c_p: __go_clone_16_0,
            has_s_h_a: __go_clone_17_0,
            has_s_s_e3: __go_clone_18_0,
            has_s_s_s_e3: __go_clone_19_0,
            has_s_s_e41: __go_clone_20_0,
            has_s_s_e42: __go_clone_21_0,
            __blank_22_0: __go_clone_22_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_19_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_21_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_22_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            has_a_e_s: __go_default_1_0,
            has_a_d_x: __go_default_2_0,
            has_a_v_x: __go_default_3_0,
            has_a_v_x2: __go_default_4_0,
            has_a_v_x512_f: __go_default_5_0,
            has_a_v_x512_b_w: __go_default_6_0,
            has_a_v_x512_v_l: __go_default_7_0,
            has_b_m_i1: __go_default_8_0,
            has_b_m_i2: __go_default_9_0,
            has_e_r_m_s: __go_default_10_0,
            has_f_s_r_m: __go_default_11_0,
            has_f_m_a: __go_default_12_0,
            has_o_s_x_s_a_v_e: __go_default_13_0,
            has_p_c_l_m_u_l_q_d_q: __go_default_14_0,
            has_p_o_p_c_n_t: __go_default_15_0,
            has_r_d_t_s_c_p: __go_default_16_0,
            has_s_h_a: __go_default_17_0,
            has_s_s_e3: __go_default_18_0,
            has_s_s_s_e3: __go_default_19_0,
            has_s_s_e41: __go_default_20_0,
            has_s_s_e42: __go_default_21_0,
            __blank_22_0: __go_default_22_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.has_a_e_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.has_a_d_x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.has_a_v_x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.has_a_v_x2.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.has_a_v_x512_f.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.has_a_v_x512_b_w.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.has_a_v_x512_v_l.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.has_b_m_i1.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.has_b_m_i2.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.has_e_r_m_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.has_f_s_r_m.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.has_f_m_a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.has_o_s_x_s_a_v_e.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.has_p_c_l_m_u_l_q_d_q.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.has_p_o_p_c_n_t.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.has_r_d_t_s_c_p.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.has_s_h_a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.has_s_s_e3.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", (*self.has_s_s_s_e3.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_20 = format!("{}", (*self.has_s_s_e41.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_21 = format!("{}", (*self.has_s_s_e42.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_22 = format!("{}", (*self.__blank_22_0.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22
        )
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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.has_v_f_pv4.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.has_i_d_i_v_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.has_v7_atomics.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.__blank_4_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            has_v_f_pv4: __go_clone_1_0,
            has_i_d_i_v_a: __go_clone_2_0,
            has_v7_atomics: __go_clone_3_0,
            __blank_4_0: __go_clone_4_0,
        }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            has_v_f_pv4: __go_default_1_0,
            has_i_d_i_v_a: __go_default_2_0,
            has_v7_atomics: __go_default_3_0,
            __blank_4_0: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.has_v_f_pv4.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.has_i_d_i_v_a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.has_v7_atomics.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.__blank_4_0.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.has_a_e_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.has_p_m_u_l_l.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.has_s_h_a1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.has_s_h_a2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.has_s_h_a512.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.has_c_r_c32.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.has_a_t_o_m_i_c_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.has_c_p_u_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.has_d_i_t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.is_neoverse.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.__blank_11_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            has_a_e_s: __go_clone_1_0,
            has_p_m_u_l_l: __go_clone_2_0,
            has_s_h_a1: __go_clone_3_0,
            has_s_h_a2: __go_clone_4_0,
            has_s_h_a512: __go_clone_5_0,
            has_c_r_c32: __go_clone_6_0,
            has_a_t_o_m_i_c_s: __go_clone_7_0,
            has_c_p_u_i_d: __go_clone_8_0,
            has_d_i_t: __go_clone_9_0,
            is_neoverse: __go_clone_10_0,
            __blank_11_0: __go_clone_11_0,
        }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            has_a_e_s: __go_default_1_0,
            has_p_m_u_l_l: __go_default_2_0,
            has_s_h_a1: __go_default_3_0,
            has_s_h_a2: __go_default_4_0,
            has_s_h_a512: __go_default_5_0,
            has_c_r_c32: __go_default_6_0,
            has_a_t_o_m_i_c_s: __go_default_7_0,
            has_c_p_u_i_d: __go_default_8_0,
            has_d_i_t: __go_default_9_0,
            is_neoverse: __go_default_10_0,
            __blank_11_0: __go_default_11_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.has_a_e_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.has_p_m_u_l_l.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.has_s_h_a1.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.has_s_h_a2.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.has_s_h_a512.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.has_c_r_c32.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.has_a_t_o_m_i_c_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.has_c_p_u_i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.has_d_i_t.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.is_neoverse.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.__blank_11_0.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11
        )
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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.has_l_s_x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.has_c_r_c32.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.has_l_a_m_c_a_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.has_l_a_m__b_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.__blank_5_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            has_l_s_x: __go_clone_1_0,
            has_c_r_c32: __go_clone_2_0,
            has_l_a_m_c_a_s: __go_clone_3_0,
            has_l_a_m__b_h: __go_clone_4_0,
            __blank_5_0: __go_clone_5_0,
        }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            has_l_s_x: __go_default_1_0,
            has_c_r_c32: __go_default_2_0,
            has_l_a_m_c_a_s: __go_default_3_0,
            has_l_a_m__b_h: __go_default_4_0,
            __blank_5_0: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.has_l_s_x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.has_c_r_c32.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.has_l_a_m_c_a_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.has_l_a_m__b_h.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.__blank_5_0.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5)
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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.has_m_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.__blank_2_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            has_m_s_a: __go_clone_1_0,
            __blank_2_0: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            has_m_s_a: __go_default_1_0,
            __blank_2_0: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.has_m_s_a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.__blank_2_0.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.has_d_a_r_n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.has_s_c_v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.is_p_o_w_e_r8.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.is_p_o_w_e_r9.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.is_p_o_w_e_r10.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.__blank_6_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            has_d_a_r_n: __go_clone_1_0,
            has_s_c_v: __go_clone_2_0,
            is_p_o_w_e_r8: __go_clone_3_0,
            is_p_o_w_e_r9: __go_clone_4_0,
            is_p_o_w_e_r10: __go_clone_5_0,
            __blank_6_0: __go_clone_6_0,
        }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            has_d_a_r_n: __go_default_1_0,
            has_s_c_v: __go_default_2_0,
            is_p_o_w_e_r8: __go_default_3_0,
            is_p_o_w_e_r9: __go_default_4_0,
            is_p_o_w_e_r10: __go_default_5_0,
            __blank_6_0: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.has_d_a_r_n.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.has_s_c_v.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.is_p_o_w_e_r8.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.is_p_o_w_e_r9.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.is_p_o_w_e_r10.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.__blank_6_0.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
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
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.has_z_a_r_c_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.has_s_t_f_l_e.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.has_l_d_i_s_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.has_e_i_m_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.has_d_f_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.has_e_t_f3_e_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.has_m_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.has_a_e_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.has_a_e_s_c_b_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.has_a_e_s_c_t_r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.has_a_e_s_g_c_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.has_g_h_a_s_h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.has_s_h_a1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.has_s_h_a256.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.has_s_h_a512.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.has_s_h_a3.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.has_v_x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.has_v_x_e.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.has_k_d_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.has_e_c_d_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.has_e_d_d_s_a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = { let __guard = self.__blank_22_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            has_z_a_r_c_h: __go_clone_1_0,
            has_s_t_f_l_e: __go_clone_2_0,
            has_l_d_i_s_p: __go_clone_3_0,
            has_e_i_m_m: __go_clone_4_0,
            has_d_f_p: __go_clone_5_0,
            has_e_t_f3_e_h: __go_clone_6_0,
            has_m_s_a: __go_clone_7_0,
            has_a_e_s: __go_clone_8_0,
            has_a_e_s_c_b_c: __go_clone_9_0,
            has_a_e_s_c_t_r: __go_clone_10_0,
            has_a_e_s_g_c_m: __go_clone_11_0,
            has_g_h_a_s_h: __go_clone_12_0,
            has_s_h_a1: __go_clone_13_0,
            has_s_h_a256: __go_clone_14_0,
            has_s_h_a512: __go_clone_15_0,
            has_s_h_a3: __go_clone_16_0,
            has_v_x: __go_clone_17_0,
            has_v_x_e: __go_clone_18_0,
            has_k_d_s_a: __go_clone_19_0,
            has_e_c_d_s_a: __go_clone_20_0,
            has_e_d_d_s_a: __go_clone_21_0,
            __blank_22_0: __go_clone_22_0,
        }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_19_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_21_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_22_0 = Arc::new(Mutex::new(Some(CacheLinePad::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            has_z_a_r_c_h: __go_default_1_0,
            has_s_t_f_l_e: __go_default_2_0,
            has_l_d_i_s_p: __go_default_3_0,
            has_e_i_m_m: __go_default_4_0,
            has_d_f_p: __go_default_5_0,
            has_e_t_f3_e_h: __go_default_6_0,
            has_m_s_a: __go_default_7_0,
            has_a_e_s: __go_default_8_0,
            has_a_e_s_c_b_c: __go_default_9_0,
            has_a_e_s_c_t_r: __go_default_10_0,
            has_a_e_s_g_c_m: __go_default_11_0,
            has_g_h_a_s_h: __go_default_12_0,
            has_s_h_a1: __go_default_13_0,
            has_s_h_a256: __go_default_14_0,
            has_s_h_a512: __go_default_15_0,
            has_s_h_a3: __go_default_16_0,
            has_v_x: __go_default_17_0,
            has_v_x_e: __go_default_18_0,
            has_k_d_s_a: __go_default_19_0,
            has_e_c_d_s_a: __go_default_20_0,
            has_e_d_d_s_a: __go_default_21_0,
            __blank_22_0: __go_default_22_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.has_z_a_r_c_h.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.has_s_t_f_l_e.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.has_l_d_i_s_p.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.has_e_i_m_m.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.has_d_f_p.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.has_e_t_f3_e_h.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.has_m_s_a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.has_a_e_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.has_a_e_s_c_b_c.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.has_a_e_s_c_t_r.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.has_a_e_s_g_c_m.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.has_g_h_a_s_h.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.has_s_h_a1.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.has_s_h_a256.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.has_s_h_a512.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.has_s_h_a3.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.has_v_x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.has_v_x_e.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", (*self.has_k_d_s_a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_20 = format!("{}", (*self.has_e_c_d_s_a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_21 = format!("{}", (*self.has_e_d_d_s_a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_22 = format!("{}", (*self.__blank_22_0.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22
        )
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
