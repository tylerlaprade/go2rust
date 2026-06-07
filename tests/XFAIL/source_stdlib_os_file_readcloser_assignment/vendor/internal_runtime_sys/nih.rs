use go2rust_stdlib_stubs::*;

use crate::consts::*;
use crate::consts_norace::*;
use crate::dit_arm64::*;
use crate::intrinsics::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// NOTE: keep in sync with cmd/compile/internal/types.CalcSize
/// to make the compiler recognize this as an intrinsic type.
#[derive(Debug, Clone, Default)]
pub struct nih {
}

impl nih {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for nih {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for nih {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// NotInHeap is a type must never be allocated from the GC'd heap or on the stack,
/// and is called not-in-heap.
///
/// Other types can embed NotInHeap to make it not-in-heap. Specifically, pointers
/// to these types must always fail the `runtime.inheap` check. The type may be used
/// for global variables, or for objects in unmanaged memory (e.g., allocated with
/// `sysAlloc`, `persistentalloc`, `fixalloc`, or from a manually-managed span).
///
/// Specifically:
///
/// 1. `new(T)`, `make([]T)`, `append([]T, ...)` and implicit heap
/// allocation of T are disallowed. (Though implicit allocations are
/// disallowed in the runtime anyway.)
///
/// 2. A pointer to a regular type (other than `unsafe.Pointer`) cannot be
/// converted to a pointer to a not-in-heap type, even if they have the
/// same underlying type.
///
/// 3. Any type that containing a not-in-heap type is itself considered as not-in-heap.
///
/// - Structs and arrays are not-in-heap if their elements are not-in-heap.
/// - Maps and channels contains no-in-heap types are disallowed.
///
/// 4. Write barriers on pointers to not-in-heap types can be omitted.
///
/// The last point is the real benefit of NotInHeap. The runtime uses
/// it for low-level internal structures to avoid memory barriers in the
/// scheduler and the memory allocator where they are illegal or simply
/// inefficient. This mechanism is reasonably safe and does not compromise
/// the readability of the runtime.
#[derive(Debug, Clone)]
pub struct NotInHeap {
    pub __blank_0_0: Arc<Mutex<Option<nih>>>,
}

impl NotInHeap {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for NotInHeap {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(nih::default()))) }
    }
}

impl std::fmt::Display for NotInHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for NotInHeap {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl GoValueClone for nih {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for NotInHeap {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
