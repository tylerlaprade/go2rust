use go2rust_stdlib_stubs::*;

use crate::atob::*;
use crate::atoc::*;
use crate::atof::*;
use crate::atoi::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::itoa::*;
use crate::quote::*;

use std::sync::{Arc, Mutex};

/// index returns the index of the first instance of c in s, or -1 if missing.
pub fn index(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}