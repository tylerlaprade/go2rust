use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped, go_lookup_embedded_owner, go_register_embedded_owner};

use crate::r#mod::*;
use crate::compiletype::*;
use crate::escape::*;
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

pub const INT_ARG_REGS: i32 = 16;
pub const FLOAT_ARG_REGS: i32 = 16;
pub const EFFECTIVE_FLOAT_REG_SIZE: i32 = 8;
