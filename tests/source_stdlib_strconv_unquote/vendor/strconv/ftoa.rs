use go2rust_stdlib_stubs::*;

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
        let __go_clone_0_0 = { let __guard = self.mantbits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.expbits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.bias.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            mantbits: __go_clone_0_0,
            expbits: __go_clone_1_0,
            bias: __go_clone_2_0,
        }
    }
}


impl Default for floatInfo {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            mantbits: __go_default_0_0,
            expbits: __go_default_1_0,
            bias: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for floatInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.mantbits.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.expbits.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.bias.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
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
