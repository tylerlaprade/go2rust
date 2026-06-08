use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

pub const OLD_MAP_BUCKET_COUNT_BITS: i32 = 3;
pub const OLD_MAP_BUCKET_COUNT: i32 = 1 << OLD_MAP_BUCKET_COUNT_BITS;
pub const OLD_MAP_MAX_KEY_BYTES: i32 = 128;
pub const OLD_MAP_MAX_ELEM_BYTES: i32 = 128;
