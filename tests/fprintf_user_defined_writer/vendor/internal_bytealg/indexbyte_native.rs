use go2rust_stdlib_stubs::*;

use crate::r#mod::*;
use crate::compare_native::*;
use crate::count_native::*;
use crate::equal_generic::*;
use crate::equal_native::*;
use crate::index_arm64::*;
use crate::index_native::*;
use crate::lastindexbyte_generic::*;

use std::sync::{Arc, Mutex};

///go:noescape
pub fn index_byte(b: Arc<Mutex<Option<Vec<u8>>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    unimplemented!("Go function declaration has no body");
}
