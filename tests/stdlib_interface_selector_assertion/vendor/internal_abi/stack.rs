use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::r#mod::*;
use crate::abi_arm64::*;
use crate::compiletype::*;
use crate::escape::*;
use crate::funcpc::*;
use crate::iface::*;
use crate::map_noswiss::*;
use crate::map_select_swiss::*;
use crate::map_swiss::*;
use crate::rangefuncconsts::*;
use crate::runtime::*;
use crate::switch::*;
use crate::symtab::*;
use crate::r#type::*;

pub const STACK_NOSPLIT_BASE: i32 = 800;
pub const STACK_SMALL: i32 = 128;
pub const STACK_BIG: i32 = 4096;
