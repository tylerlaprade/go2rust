use go2rust_stdlib_stubs::*;

use crate::{consts_norace::{IS_RACE}};

pub const STACK_GUARD_MULTIPLIER: i32 = 1 + internal_goos::IS_AIX + internal_goos::IS_OPENBSD + IS_RACE;


pub const DEFAULT_PHYS_PAGE_SIZE: i32 = internal_goarch::DEFAULT_PHYS_PAGE_SIZE as i32;


pub const P_C_QUANTUM: i32 = internal_goarch::P_C_QUANTUM as i32;


pub const INT64_ALIGN: i32 = internal_goarch::PTR_SIZE as i32;


pub const MIN_FRAME_SIZE: i32 = internal_goarch::MIN_FRAME_SIZE as i32;


pub const STACK_ALIGN: i32 = internal_goarch::STACK_ALIGN as i32;
