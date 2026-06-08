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

pub const INT_ARG_REGS: i32 = 16;
pub const FLOAT_ARG_REGS: i32 = 16;
pub const EFFECTIVE_FLOAT_REG_SIZE: i32 = 8;
