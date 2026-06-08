use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{panic::{throw}, r#extern::{G_O_O_S}, runtime1::{environ}};

use std::sync::{Arc, Mutex};

pub(crate) static _cgo_setenv: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_unsetenv: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_cgo_setenv.lock().unwrap() = Some(0);
    *_cgo_unsetenv.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *_cgo_setenv.lock().unwrap() = Some(0);
    *_cgo_unsetenv.lock().unwrap() = Some(0);
}


pub fn gogetenv(key: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut env = environ();
    if { let __nil_result = (*env.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("getenv before env init".to_string()))));
    }
    { let __range_holder = env.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        if { let __tmp_x = (s.len() as i32); let __tmp_y = ((*key.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &(s); __s.as_bytes()[((*key.lock().unwrap().as_ref().unwrap()).len()) as usize] }; let __tmp_y = ('=' as i32) as u8; __tmp_x == __tmp_y } && env_key_equal(Arc::new(Mutex::new(Some({ let __s = &(s); let __high = ((*key.lock().unwrap().as_ref().unwrap()).len()) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some({ let __s = &(s); let __low = ({ let __tmp_x = ((*key.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() })));
    }
    } }
    Arc::new(Mutex::new(Some("".to_string())))
}

/// envKeyEqual reports whether a == b, with ASCII-only case insensitivity
/// on Windows. The two strings must have the same length.
pub fn env_key_equal(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> bool {
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x == __tmp_y } {
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*a.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let (mut ca, mut cb) = (Arc::new(Mutex::new(Some({ let __s = &((*a.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }))), Arc::new(Mutex::new(Some({ let __s = &((*b.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }))));
        if { let __tmp_x = { let __v = (*ca.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*cb.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } || { let __tmp_x = lower_a_s_c_i_i(Arc::new(Mutex::new(Some({ let __arg_holder = ca.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = lower_a_s_c_i_i(Arc::new(Mutex::new(Some({ let __arg_holder = cb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        return false;
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return true;
    }
    return { let __tmp_x = (*a.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*b.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
}

pub fn lower_a_s_c_i_i(c: Arc<Mutex<Option<u8>>>) -> u8 {
    if { let __tmp_x = ('A' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y } {
        return { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y }) as u8; __tmp_x + __tmp_y };
    }
    return { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
