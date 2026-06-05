use go2rust_stdlib_stubs::*;

use crate::{GoByteSequence};

use crate::r#mod::*;
use crate::compare_native::*;
use crate::count_native::*;
use crate::equal_generic::*;
use crate::equal_native::*;
use crate::index_arm64::*;
use crate::indexbyte_native::*;
use crate::lastindexbyte_generic::*;

use std::sync::{Arc, Mutex};

/// IndexString returns the index of the first instance of b in a, or -1 if b is not present in a.
/// Requires 2 <= len(b) <= MaxLen.
///
///go:noescape
pub fn index_string(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> i32 {
    let __a = (*a.lock().unwrap().as_ref().unwrap()).clone();
    let __b = (*b.lock().unwrap().as_ref().unwrap()).clone();
    __a.find(&__b).map(|__i| __i as i32).unwrap_or(-1)
}
