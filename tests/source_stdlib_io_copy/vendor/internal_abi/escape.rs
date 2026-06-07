use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::r#mod::*;
use crate::abi_arm64::*;
use crate::compiletype::*;
use crate::funcpc::*;
use crate::iface::*;
use crate::map_noswiss::*;
use crate::map_select_swiss::*;
use crate::map_swiss::*;
use crate::rangefuncconsts::*;
use crate::runtime::*;
use crate::stack::*;
use crate::switch::*;
use crate::symtab::*;
use crate::r#type::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

pub(crate) static alwaysFalse: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static escapeSink: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *alwaysFalse.lock().unwrap() = Some(false);
    *escapeSink.lock().unwrap() = None;
}


pub(crate) fn __go_zero_globals() {
    *alwaysFalse.lock().unwrap() = Some(false);
    *escapeSink.lock().unwrap() = None;
}


/// NoEscape hides the pointer p from escape analysis, preventing it
/// from escaping to the heap. It compiles down to nothing.
///
/// WARNING: This is very subtle to use correctly. The caller must
/// ensure that it's truly safe for p to not escape to the heap by
/// maintaining runtime pointer invariants (for example, that globals
/// and the heap may not generally point into a stack).
///
///go:nosplit
///go:nocheckptr
pub fn no_escape(p: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    let mut x = Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize)));
    return Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x ^ __tmp_y })));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
