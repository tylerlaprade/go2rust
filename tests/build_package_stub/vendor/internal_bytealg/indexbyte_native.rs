use go2rust_stdlib_stubs::*;

use crate::{GoByteSequence};

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


///go:noescape
pub fn index_byte_string(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    let __s = (*s.lock().unwrap().as_ref().unwrap()).clone();
    let __c = (*c.lock().unwrap().as_ref().unwrap()).clone();
    __s.as_bytes().iter().position(|&__b| __b == __c).map(|__i| __i as i32).unwrap_or(-1)
}
