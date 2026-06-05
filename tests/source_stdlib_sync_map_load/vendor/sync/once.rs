use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_slice, format_slice_values, format_slice_wrapped, go_any_eq, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::cond::*;
use crate::hashtriemap::*;
use crate::mutex::*;
use crate::oncefunc::*;
use crate::pool::*;
use crate::poolqueue::*;
use crate::runtime::*;
use crate::runtime2::*;
use crate::rwmutex::*;
use crate::waitgroup::*;
