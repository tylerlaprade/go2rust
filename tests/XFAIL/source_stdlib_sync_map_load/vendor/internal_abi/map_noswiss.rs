use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::r#mod::*;
use crate::abi_arm64::*;
use crate::compiletype::*;
use crate::escape::*;
use crate::funcpc::*;
use crate::iface::*;
use crate::map_select_swiss::*;
use crate::map_swiss::*;
use crate::rangefuncconsts::*;
use crate::runtime::*;
use crate::stack::*;
use crate::switch::*;
use crate::symtab::*;
use crate::r#type::*;

pub const OLD_MAP_BUCKET_COUNT_BITS: i32 = 3;
pub const OLD_MAP_BUCKET_COUNT: i32 = 1 << OLD_MAP_BUCKET_COUNT_BITS;
pub const OLD_MAP_MAX_KEY_BYTES: i32 = 128;
pub const OLD_MAP_MAX_ELEM_BYTES: i32 = 128;
