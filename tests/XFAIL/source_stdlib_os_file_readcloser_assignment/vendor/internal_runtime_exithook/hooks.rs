use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Hook is a function to be run at program termination
/// (when someone invokes os.Exit, or when main.main returns).
/// Hooks are run in reverse order of registration:
/// the first hook added is the last one run.
#[derive(Clone)]
pub struct Hook {
    pub f: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>,
    pub run_on_failure: Arc<Mutex<Option<bool>>>,
}

impl Hook {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.f.clone();
        let __go_clone_1_0 = { let __guard = self.run_on_failure.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            f: __go_clone_0_0,
            run_on_failure: __go_clone_1_0,
        }
    }
}


impl Default for Hook {
    fn default() -> Self {
        Self { f: Arc::new(Mutex::new(None)), run_on_failure: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Hook {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", (*self.run_on_failure.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for Hook {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("RunOnFailure") {
            out.run_on_failure = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub(crate) static locked: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Int32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static runGoid: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static hooks: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Hook>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static running: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Gosched: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Goid: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut() -> u64 + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Throw: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *locked.lock().unwrap() = Some(Default::default());
    *runGoid.lock().unwrap() = Some(Default::default());
    *hooks.lock().unwrap() = Some(vec![]);
    *running.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Hook {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
