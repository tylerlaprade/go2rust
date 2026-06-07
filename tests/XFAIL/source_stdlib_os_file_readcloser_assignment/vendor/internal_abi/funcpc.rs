use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::r#mod::*;
use crate::abi_arm64::*;
use crate::compiletype::*;
use crate::escape::*;
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

/// FuncPCABI0 returns the entry PC of the function f, which must be a
/// direct reference of a function defined as ABI0. Otherwise it is a
/// compile-time error.
///
/// Implemented as a compile intrinsic.
pub fn func_p_c_a_b_i0(f: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> usize {
    let __guard = f.lock().unwrap();
    let __value = __guard.as_ref().expect("internal/abi.FuncPCABI0 requires a function value");
    let mut __hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&std::any::Any::type_id(__value.as_ref()), &mut __hasher);
    std::hash::Hasher::finish(&__hasher) as usize
}


/// FuncPCABIInternal returns the entry PC of the function f. If f is a
/// direct reference of a function, it must be defined as ABIInternal.
/// Otherwise it is a compile-time error. If f is not a direct reference
/// of a defined function, it assumes that f is a func value. Otherwise
/// the behavior is undefined.
///
/// Implemented as a compile intrinsic.
pub fn func_p_c_a_b_i_internal(f: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> usize {
    let __guard = f.lock().unwrap();
    let __value = __guard.as_ref().expect("internal/abi.FuncPCABIInternal requires a function value");
    let mut __hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&std::any::Any::type_id(__value.as_ref()), &mut __hasher);
    std::hash::Hasher::finish(&__hasher) as usize
}
