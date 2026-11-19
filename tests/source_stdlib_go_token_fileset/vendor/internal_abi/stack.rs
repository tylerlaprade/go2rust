use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
};

pub const STACK_NOSPLIT_BASE: i32 = 800;
pub const STACK_SMALL: i32 = 128;
pub const STACK_BIG: i32 = 4096;
