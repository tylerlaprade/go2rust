use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// errorString is a trivial implementation of error.
#[derive(Debug, Clone)]
pub struct errorString {
    pub s: Arc<Mutex<Option<String>>>,
}

impl errorString {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            s: __go_clone_0_0,
        }
    }
}


impl Default for errorString {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            s: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for errorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


pub static ErrUnsupported: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrUnsupported.lock().unwrap() = None;
    { let __rhs_holder = new(Arc::new(Mutex::new(Some("unsupported operation".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrUnsupported.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrUnsupported.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    { let __rhs_holder = new(Arc::new(Mutex::new(Some("unsupported operation".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrUnsupported.lock().unwrap() = new_val; }
}


impl errorString {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return self.s.clone();
    }
}

impl StdError for errorString {}


/// New returns an error that formats as the given text.
/// Each call to New returns a distinct error value even if the text is identical.
pub fn new(text: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(errorString { s: Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }) as Box<dyn StdError + Send + Sync>)))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for errorString {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
