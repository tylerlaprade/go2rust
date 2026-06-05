use go2rust_stdlib_stubs::*;

use crate::r#match::*;
use crate::path::*;
use crate::symlink::*;
use crate::symlink_unix::*;

use std::sync::{Arc, Mutex};

pub fn join_1(elem: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> {
        // If there's a bug here, fix the logic in ./path_plan9.go too.
    { let __range_holder = elem.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, e) in __range_values.iter().enumerate() {
        if { let __tmp_x = (*e).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        return clean(Arc::new(Mutex::new(Some({ let __parts = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = elem.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize..].to_vec() }))).lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = (*Arc::new(Mutex::new(Some(char::from_u32((SEPARATOR) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap()).clone(); __parts.join(&__sep) }))));
    }
    } }
    Arc::new(Mutex::new(Some("".to_string())))
}