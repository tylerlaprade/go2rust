use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::exit::*;

use std::any::Any;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

/// Interface is the interface required of test loggers.
/// The os package will invoke the interface's methods to indicate that
/// it is inspecting the given environment variables or files.
/// Multiple goroutines may call these methods simultaneously.
pub trait Interface: std::fmt::Display + Any {
    fn __go_clone_box_interface(&self) -> Box<dyn Interface + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_interface(&self, other: &(dyn Interface + Send + Sync)) -> bool;
    fn getenv(&self, key: Arc<Mutex<Option<String>>>);
    fn stat(&self, file: Arc<Mutex<Option<String>>>);
    fn open(&self, file: Arc<Mutex<Option<String>>>);
    fn chdir(&self, dir: Arc<Mutex<Option<String>>>);
}

impl Clone for Box<dyn Interface + Send + Sync> {
    fn clone(&self) -> Self {
        Interface::__go_clone_box_interface(self.as_ref())
    }
}

pub(crate) static logger_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync_atomic::r#type::Pointer<Box<dyn Interface + Send + Sync>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *logger_1.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *logger_1.lock().unwrap() = Some(Default::default());
}


/// Logger returns the current test logger implementation.
/// It returns nil if there is no logger.
pub fn logger() -> Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>> {
    let mut r#impl: GoPtr<Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>> = { let __go_ptr = (*logger_1.lock().unwrap().as_mut().unwrap()).load().clone(); match __go_ptr { sync_atomic::GoPtr::Nil => GoPtr::nil(), sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()), sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr), sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
    if r#impl.is_nil() {
        return Arc::new(Mutex::new(None));
    }
    r#impl.clone()
}

/// Getenv calls Logger().Getenv, if a logger has been set.
pub fn getenv(name: Arc<Mutex<Option<String>>>) {
    {
        let mut log = logger();;
        if { let __nil_result = (*log.lock().unwrap()).is_some(); __nil_result } {
            (*log.lock().unwrap().as_ref().unwrap()).getenv(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
}

/// Open calls Logger().Open, if a logger has been set.
pub fn open(name: Arc<Mutex<Option<String>>>) {
    {
        let mut log = logger();;
        if { let __nil_result = (*log.lock().unwrap()).is_some(); __nil_result } {
            (*log.lock().unwrap().as_ref().unwrap()).open(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
}

/// Stat calls Logger().Stat, if a logger has been set.
pub fn stat(name: Arc<Mutex<Option<String>>>) {
    {
        let mut log = logger();;
        if { let __nil_result = (*log.lock().unwrap()).is_some(); __nil_result } {
            (*log.lock().unwrap().as_ref().unwrap()).stat(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
