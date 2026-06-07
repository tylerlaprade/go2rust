use go2rust_stdlib_stubs::*;

use crate::r#match::*;
use crate::path::*;
use crate::symlink::*;
use crate::symlink_unix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn split_list_1(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Vec::<String>::new())));
    }
    strings::split(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(char::from_u32((LIST_SEPARATOR) as u32).unwrap().to_string()))))
}

pub fn abs_1(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unix_abs(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn join_1(elem: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<String>>> {
        // If there's a bug here, fix the logic in ./path_plan9.go too.
    { let __range_holder = elem.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, e) in __range_values.iter().enumerate() {
        if { let __tmp_x = (*e).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        return clean(strings::join(Arc::new(Mutex::new(Some({ let __seq_holder = elem.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (i) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some(char::from_u32((SEPARATOR) as u32).unwrap().to_string())))));
    }
    } }
    Arc::new(Mutex::new(Some("".to_string())))
}