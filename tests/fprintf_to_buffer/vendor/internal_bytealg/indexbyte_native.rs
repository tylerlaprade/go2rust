use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

///go:noescape
pub fn index_byte(b: Arc<Mutex<Option<Vec<u8>>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    unimplemented!("Go function declaration has no body");
}
