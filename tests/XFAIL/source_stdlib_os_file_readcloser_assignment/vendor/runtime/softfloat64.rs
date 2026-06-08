use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

pub(crate) const MANTBITS64: u64 = 52;
pub(crate) const EXPBITS64: u64 = 11;
pub(crate) const BIAS64: i32 = (((-1 as i32) << ((EXPBITS64 as i32) - (1 as i32))) + (1 as i32));
pub(crate) const NAN64: u64 = (((((1 as u64) << (EXPBITS64 as u64)) - (1 as u64)) << (MANTBITS64 as u64)) + ((1 as u64) << ((MANTBITS64 as u64) - (1 as u64))));
pub(crate) const INF64: u64 = ((((1 as u64) << (EXPBITS64 as u64)) - (1 as u64)) << (MANTBITS64 as u64));
pub(crate) const NEG64: u64 = ((1 as u64) << ((EXPBITS64 as u64) + (MANTBITS64 as u64)));
pub(crate) const MANTBITS32: u64 = 23;
pub(crate) const EXPBITS32: u64 = 8;
pub(crate) const BIAS32: i32 = (((-1 as i32) << ((EXPBITS32 as i32) - (1 as i32))) + (1 as i32));
pub(crate) const NAN32: u32 = (((((1 as u32) << (EXPBITS32 as u32)) - (1 as u32)) << (MANTBITS32 as u32)) + ((1 as u32) << ((MANTBITS32 as u32) - (1 as u32))));
pub(crate) const INF32: u32 = ((((1 as u32) << (EXPBITS32 as u32)) - (1 as u32)) << (MANTBITS32 as u32));
pub(crate) const NEG32: u32 = ((1 as u32) << ((EXPBITS32 as u32) + (MANTBITS32 as u32)));
