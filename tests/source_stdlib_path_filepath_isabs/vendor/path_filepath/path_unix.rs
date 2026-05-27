use go2rust_stdlib_stubs::*;

use crate::r#match::*;
use crate::path::*;
use crate::symlink::*;
use crate::symlink_unix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

/// HasPrefix exists for historical compatibility and should not be used.
///
/// Deprecated: HasPrefix does not respect path boundaries and
/// does not ignore case when required.
pub fn has_prefix(p: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> bool {

    return (*Arc::new(Mutex::new(Some({ let __s = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __arg = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap());
}

pub fn split_list_1(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {

    if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ""; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Vec::<String>::new())));
    }
    return Arc::new(Mutex::new(Some({ let __s = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __sep = Arc::new(Mutex::new(Some(char::from_u32(((*ListSeparator.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string()))); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() })));
}

pub fn abs_1(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {

    return unix_abs(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

pub fn join_1(elem: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> {

        // If there's a bug here, fix the logic in ./path_plan9.go too.
    { let __range_holder = elem.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, e) in __range_values.iter().enumerate() {
        if { let __tmp_x = (*e).clone(); let __tmp_y = ""; __tmp_x != __tmp_y } {
        return clean(Arc::new(Mutex::new(Some({ let __parts = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = elem.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize..].to_vec() }))).lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = Arc::new(Mutex::new(Some(char::from_u32(((*Separator.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string()))); __parts.join(&__sep) }))));
    }
    } }
    return Arc::new(Mutex::new(Some("".to_string())));
}

pub fn same_word(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> bool {

    return { let __tmp_x = (*a.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*b.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
}