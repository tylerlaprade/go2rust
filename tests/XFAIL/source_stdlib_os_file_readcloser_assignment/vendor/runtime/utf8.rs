use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

pub(crate) const RUNE_ERROR: i32 = ('\u{fffd}' as i32);
pub(crate) const RUNE_SELF: i32 = 0x80;
pub(crate) const MAX_RUNE: i32 = ('\u{10ffff}' as i32);


pub(crate) const SURROGATE_MIN: i32 = 0xD800;
pub(crate) const SURROGATE_MAX: i32 = 0xDFFF;


pub(crate) const T1: i32 = 0x00;
pub(crate) const TX: i32 = 0x80;
pub(crate) const T2: i32 = 0xC0;
pub(crate) const T3: i32 = 0xE0;
pub(crate) const T4: i32 = 0xF0;
pub(crate) const T5: i32 = 0xF8;
pub(crate) const MASKX: i32 = 0x3F;
pub(crate) const MASK2: i32 = 0x1F;
pub(crate) const MASK3: i32 = 0x0F;
pub(crate) const MASK4: i32 = 0x07;
pub(crate) const RUNE1_MAX: i32 = (((1 as i32) << (7 as i32)) - (1 as i32));
pub(crate) const RUNE2_MAX: i32 = (((1 as i32) << (11 as i32)) - (1 as i32));
pub(crate) const RUNE3_MAX: i32 = (((1 as i32) << (16 as i32)) - (1 as i32));
pub(crate) const LOCB: i32 = 0x80;
pub(crate) const HICB: i32 = 0xBF;
