use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::builder::*;
use crate::clone::*;
use crate::iter::*;
use crate::reader::*;
use crate::replace::*;
use crate::search::*;
use crate::r#mod::*;

use std::sync::{Arc, Mutex};

/// Compare returns an integer comparing two strings lexicographically.
/// The result will be 0 if a == b, -1 if a < b, and +1 if a > b.
///
/// Use Compare when you need to perform a three-way comparison (with
/// [slices.SortFunc], for example). It is usually clearer and always faster
/// to use the built-in string comparison operators ==, <, >, and so on.
pub fn compare(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> i32 {
    internal_bytealg::compare_string(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}