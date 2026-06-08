use go2rust_stdlib_stubs::*;

use crate::{path_unix::{SEPARATOR, is_path_separator, volume_name_len_1}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) static errInvalidPath: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errInvalidPath.lock().unwrap() = None;
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("invalid path".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalidPath.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *errInvalidPath.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("invalid path".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalidPath.lock().unwrap() = new_val; }
}


/// FromSlash is filepath.ToSlash.
pub fn from_slash(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = SEPARATOR; let __tmp_y = ('/' as i32); __tmp_x == __tmp_y } {
        return { let __owned = path.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    replace_string_byte(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('/' as i32) as u8))), Arc::new(Mutex::new(Some(SEPARATOR as u8))))
}

pub fn replace_string_byte(s: Arc<Mutex<Option<String>>>, old: Arc<Mutex<Option<u8>>>, new: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = internal_stringslite::index_byte(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = -1; __tmp_x == __tmp_y } {
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    let mut n = Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_vec())));
    for i in 0..(({ let __range_holder = n.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if {
            let __tmp_x = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() };
            let __tmp_y = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v };
            __tmp_x == __tmp_y
        } {
        (*n.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    }
    return Arc::new(Mutex::new(Some(String::from_utf8((*n.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
}

/// Base is filepath.Base.
pub fn base(mut path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(".".to_string())));
    }

        // Strip trailing slashes.
    while { let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] })))) {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *path.lock().unwrap() = __moved_val; };
    }

        // Throw away volume name
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __low = ((*volume_name(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).len()) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *path.lock().unwrap() = __moved_val; };

        // Find the last element
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && !is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))) {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *path.lock().unwrap() = __moved_val; };
    }

        // If empty now, it had only slashes.
    if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(char::from_u32((SEPARATOR) as u32).unwrap().to_string())));
    }
    return { let __owned = path.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// VolumeName is filepath.VolumeName.
pub fn volume_name(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    from_slash(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __high = (volume_name_len_1(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) as usize; __s[..__high].to_string() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
