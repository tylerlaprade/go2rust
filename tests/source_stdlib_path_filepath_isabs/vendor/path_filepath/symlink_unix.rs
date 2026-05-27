use go2rust_stdlib_stubs::*;

use crate::r#match::*;
use crate::path::*;
use crate::path_unix::*;
use crate::symlink::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn eval_symlinks_1(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    walk_symlinks(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}