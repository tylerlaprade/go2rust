use go2rust_stdlib_stubs::*;

use crate::path::*;
use crate::path_nonwindows::*;

use std::sync::{Arc, Mutex};

pub const SEPARATOR: i32 = ('/' as i32);
pub const LIST_SEPARATOR: i32 = (':' as i32);


/// IsAbs reports whether the path is absolute.
pub fn is_abs(path: Arc<Mutex<Option<String>>>) -> bool {
    internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("/".to_string()))))
}