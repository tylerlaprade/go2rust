package main

import (
	"fmt"
	"strings"
)

// PERMANENT: not scaffold. sync/atomic is native tier (docs/bridge_debt.md):
// Rust's std::sync::atomic is the long-term implementation of Go's typed
// atomics, which the runtime provides and Go source cannot express.
//
// writeAtomicIntegerTypeStub writes a typed integer atomic such as
// atomic.Int64: Add, Load, Store, Swap and CompareAndSwap on one shared
// atomic, so a copy of the struct that holds it sees the same value, as the
// atomic.Int32 and atomic.Uint64 stubs do. Results are the bare integer.
func writeAtomicIntegerTypeStub(out *strings.Builder, stubName, rustAtomic, intType string) {
	helper := "__go_atomic_" + intType + "_arg"
	fmt.Fprintf(out, `#[derive(Debug, Clone)]
pub struct %[1]s {
    __go_value: std::sync::Arc<std::sync::atomic::%[2]s>,
}

impl Default for %[1]s {
    fn default() -> Self {
        Self { __go_value: std::sync::Arc::new(std::sync::atomic::%[2]s::new(0)) }
    }
}

impl std::fmt::Display for %[1]s {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<%[1]s>")
    }
}

fn %[4]s<T: 'static>(arg: &T) -> %[3]s {
    let any = arg as &dyn std::any::Any;
    if let Some(v) = any.downcast_ref::<%[3]s>() {
        *v
    } else if let Some(v) = any.downcast_ref::<i64>() {
        *v as %[3]s
    } else if let Some(v) = any.downcast_ref::<i32>() {
        *v as %[3]s
    } else if let Some(v) = any.downcast_ref::<u64>() {
        *v as %[3]s
    } else if let Some(v) = any.downcast_ref::<u32>() {
        *v as %[3]s
    } else if let Some(v) = any.downcast_ref::<usize>() {
        *v as %[3]s
    } else if let Some(v) = any.downcast_ref::<isize>() {
        *v as %[3]s
    } else if let Some(v) = any.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<%[3]s>>>>() {
        v.lock().unwrap().as_ref().copied().unwrap_or_default()
    } else if let Some(v) = any.downcast_ref::<std::rc::Rc<std::cell::RefCell<Option<%[3]s>>>>() {
        v.borrow().as_ref().copied().unwrap_or_default()
    } else {
        panic!("%[1]s helper: unsupported argument type")
    }
}

impl %[1]s {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn add<T0: 'static>(&self, arg0: T0) -> %[3]s {
        let delta = %[4]s(&arg0);
        let previous = self.__go_value.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);
        previous.wrapping_add(delta)
    }

    pub fn load(&self) -> %[3]s {
        self.__go_value.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn store<T0: 'static>(&self, arg0: T0) {
        self.__go_value.store(%[4]s(&arg0), std::sync::atomic::Ordering::SeqCst);
    }

    pub fn swap<T0: 'static>(&self, arg0: T0) -> %[3]s {
        self.__go_value.swap(%[4]s(&arg0), std::sync::atomic::Ordering::SeqCst)
    }

    pub fn compare_and_swap<T0: 'static, T1: 'static>(&self, old: T0, new: T1) -> bool {
        let old = %[4]s(&old);
        let new = %[4]s(&new);
        self.__go_value.compare_exchange(old, new, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_ok()
    }
}
`, stubName, rustAtomic, intType, helper)
}

// PERMANENT: not scaffold; see writeAtomicIntegerTypeStub. atomic.Bool:
// Load, Store, Swap and CompareAndSwap on one shared AtomicBool.
func writeAtomicBoolStub(out *strings.Builder) {
	out.WriteString(`#[derive(Debug, Clone)]
pub struct atomic_Bool {
    __go_value: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Default for atomic_Bool {
    fn default() -> Self {
        Self { __go_value: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)) }
    }
}

impl std::fmt::Display for atomic_Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<atomic_Bool>")
    }
}

fn __go_atomic_bool_arg<T: 'static>(arg: &T) -> bool {
    let any = arg as &dyn std::any::Any;
    if let Some(v) = any.downcast_ref::<bool>() {
        *v
    } else if let Some(v) = any.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<bool>>>>() {
        v.lock().unwrap().as_ref().copied().unwrap_or_default()
    } else if let Some(v) = any.downcast_ref::<std::rc::Rc<std::cell::RefCell<Option<bool>>>>() {
        v.borrow().as_ref().copied().unwrap_or_default()
    } else {
        panic!("atomic_Bool helper: unsupported argument type")
    }
}

impl atomic_Bool {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn load(&self) -> bool {
        self.__go_value.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn store<T0: 'static>(&self, arg0: T0) {
        self.__go_value.store(__go_atomic_bool_arg(&arg0), std::sync::atomic::Ordering::SeqCst);
    }

    pub fn swap<T0: 'static>(&self, arg0: T0) -> bool {
        self.__go_value.swap(__go_atomic_bool_arg(&arg0), std::sync::atomic::Ordering::SeqCst)
    }

    pub fn compare_and_swap<T0: 'static, T1: 'static>(&self, old: T0, new: T1) -> bool {
        let old = __go_atomic_bool_arg(&old);
        let new = __go_atomic_bool_arg(&new);
        self.__go_value.compare_exchange(old, new, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_ok()
    }
}
`)
}
