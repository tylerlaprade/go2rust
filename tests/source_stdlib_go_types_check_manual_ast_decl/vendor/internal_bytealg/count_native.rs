use go2rust_stdlib_stubs::*;

use crate::{GoByteSequence};

use crate::r#mod::*;
use crate::compare_native::*;
use crate::equal_generic::*;
use crate::equal_native::*;
use crate::index_arm64::*;
use crate::index_native::*;
use crate::indexbyte_native::*;
use crate::lastindexbyte_generic::*;

use std::sync::{Arc, Mutex};

///go:noescape
pub fn count_string(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    let __needle = (*c.lock().unwrap().as_ref().unwrap()).clone();
    let __haystack = s.lock().unwrap();
    let __count = __haystack.as_ref().map(|__v| __v.as_bytes().iter().filter(|&&__b| __b == __needle).count()).unwrap_or(0) as i32;
    __count
}
