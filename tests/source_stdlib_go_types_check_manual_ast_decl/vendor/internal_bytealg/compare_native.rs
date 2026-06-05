use go2rust_stdlib_stubs::*;

use crate::{GoByteSequence};

use crate::r#mod::*;
use crate::count_native::*;
use crate::equal_generic::*;
use crate::equal_native::*;
use crate::index_arm64::*;
use crate::index_native::*;
use crate::indexbyte_native::*;
use crate::lastindexbyte_generic::*;

use std::sync::{Arc, Mutex};

pub fn compare_string(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> i32 {
    abigen_runtime_cmpstring(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

///go:linkname abigen_runtime_cmpstring runtime.cmpstring
pub fn abigen_runtime_cmpstring(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> i32 {
    let __a = (*a.lock().unwrap().as_ref().unwrap()).clone();
    let __b = (*b.lock().unwrap().as_ref().unwrap()).clone();
    match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 }
}
