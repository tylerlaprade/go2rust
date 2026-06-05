use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone, go_strconv_format_float, go_strconv_format_int};

use crate::accuracy_string::*;
use crate::arith::*;
use crate::arith_decl::*;
use crate::decimal::*;
use crate::float::*;
use crate::floatconv::*;
use crate::floatmarsh::*;
use crate::ftoa::*;
use crate::int::*;
use crate::intconv::*;
use crate::intmarsh::*;
use crate::nat::*;
use crate::natconv::*;
use crate::natdiv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::sqrt::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const __ROUNDING_MODE_NAME: &'static str = "ToNearestEvenToNearestAwayToZeroAwayFromZeroToNegativeInfToPositiveInf";


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub mutex: sync::mutex::Mutex,
    pub table: Arc<Mutex<Option<[divisor; 64]>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), table: { let __guard = self.table.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct1 {
    pub fn lock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.lock()
    }

    pub fn try_lock(&mut self) -> bool {
        let embedded_ref = &mut self.mutex;
        embedded_ref.try_lock()
    }

    pub fn unlock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.unlock()
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { mutex: Default::default(), table: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.table))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type cacheBase10 = AnonymousStruct1;


pub(crate) static _RoundingMode_index: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 7]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_RoundingMode_index.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_RoundingMode_index.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 13 as u8, 26 as u8, 32 as u8, 44 as u8, 57 as u8, 70 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *_RoundingMode_index.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_18() {
    *_RoundingMode_index.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 13 as u8, 26 as u8, 32 as u8, 44 as u8, 57 as u8, 70 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::float::RoundingMode {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::float::RoundingMode(Arc::new(Mutex::new(Some({ let __tmp_x = 7; let __tmp_y = 1; __tmp_x - __tmp_y } as u8)))); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "RoundingMode(".to_string())); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(go_strconv_format_int((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()) as i64, 10 as i32)))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s })));
    }
        Arc::new(Mutex::new(Some({ let __s = &(__ROUNDING_MODE_NAME); let __low = ({ let __seq = { let __seq_holder = _RoundingMode_index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }) as usize; let __high = ({ let __seq = { let __seq_holder = _RoundingMode_index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*self.0.lock().unwrap().as_ref().unwrap()) + 1) as usize].clone() }) as usize; __s[__low..__high].to_string() })))
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
