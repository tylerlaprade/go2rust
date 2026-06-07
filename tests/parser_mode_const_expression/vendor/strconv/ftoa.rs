use go2rust_stdlib_stubs::*;

use crate::{GoInteger, go_integer_add_one, go_integer_cast, go_integer_from_i128, go_integer_sub_one};

use crate::atob::*;
use crate::atoc::*;
use crate::atof::*;
use crate::atoi::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::itoa::*;
use crate::quote::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// TODO: move elsewhere?
#[derive(Debug, Clone)]
pub struct floatInfo {
    pub mantbits: Arc<Mutex<Option<u64>>>,
    pub expbits: Arc<Mutex<Option<u64>>>,
    pub bias: Arc<Mutex<Option<i32>>>,
}

impl floatInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { mantbits: { let __guard = self.mantbits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, expbits: { let __guard = self.expbits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bias: { let __guard = self.bias.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for floatInfo {
    fn default() -> Self {
        Self { mantbits: Arc::new(Mutex::new(Some(0))), expbits: Arc::new(Mutex::new(Some(0))), bias: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for floatInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.mantbits.lock().unwrap().as_ref().unwrap()), (*self.expbits.lock().unwrap().as_ref().unwrap()), (*self.bias.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for floatInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static float32info: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<floatInfo>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static float64info: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<floatInfo>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *float32info.lock().unwrap() = Some(Default::default());
    *float64info.lock().unwrap() = Some(Default::default());
    *float32info.lock().unwrap() = Some(floatInfo { mantbits: Arc::new(Mutex::new(Some(23 as u64))), expbits: Arc::new(Mutex::new(Some(8 as u64))), bias: Arc::new(Mutex::new(Some(-127))), ..Default::default() });
    *float64info.lock().unwrap() = Some(floatInfo { mantbits: Arc::new(Mutex::new(Some(52 as u64))), expbits: Arc::new(Mutex::new(Some(11 as u64))), bias: Arc::new(Mutex::new(Some(-1023))), ..Default::default() });
}


pub(crate) fn __go_zero_globals() {
    *float32info.lock().unwrap() = Some(Default::default());
    *float64info.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_8() {
    *float32info.lock().unwrap() = Some(floatInfo { mantbits: Arc::new(Mutex::new(Some(23 as u64))), expbits: Arc::new(Mutex::new(Some(8 as u64))), bias: Arc::new(Mutex::new(Some(-127))), ..Default::default() });
}


pub(crate) fn __go_init_order_9() {
    *float64info.lock().unwrap() = Some(floatInfo { mantbits: Arc::new(Mutex::new(Some(52 as u64))), expbits: Arc::new(Mutex::new(Some(11 as u64))), bias: Arc::new(Mutex::new(Some(-1023))), ..Default::default() });
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for floatInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
