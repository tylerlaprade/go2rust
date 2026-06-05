use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

/// stripGo converts from a "go1.21-bigcorp" version to a "1.21" version.
/// If v does not start with "go", stripGo returns the empty string (a known invalid version).
pub fn strip_go(mut v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    { let (__tmp_0, __tmp_1, __tmp_2) = strings::cut(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("-".to_string())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0; };
    if { let __tmp_x = ((*v.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x < __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __high = (2) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "go".to_string(); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    Arc::new(Mutex::new(Some({ let __s = &((*v.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() })))
}

/// Lang returns the Go language version for version x.
/// If x is not a valid version, Lang returns the empty string.
/// For example:
///
///	Lang("go1.21rc2") = "go1.21"
///	Lang("go1.21.2") = "go1.21"
///	Lang("go1.21") = "go1.21"
///	Lang("go1") = "go1"
///	Lang("bad") = ""
///	Lang("1.21") = ""
pub fn lang(x: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut v = internal_gover::lang(strip_go(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
    if { let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    if strings::has_prefix(Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = 2; let __tmp_y = ((*v.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y }) as usize; __s[..__high].to_string() })));
    } else {
        return Arc::new(Mutex::new(Some(format!("{}{}", "go".to_string(), { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    }
}

/// Compare returns -1, 0, or +1 depending on whether
/// x < y, x == y, or x > y, interpreted as Go versions.
/// The versions x and y must begin with a "go" prefix: "go1.21" not "1.21".
/// Invalid versions, including the empty string, compare less than
/// valid versions and equal to each other.
/// The language version "go1.21" compares less than the
/// release candidate and eventual releases "go1.21rc1" and "go1.21.0".
pub fn compare(x: Arc<Mutex<Option<String>>>, y: Arc<Mutex<Option<String>>>) -> i32 {
    internal_gover::compare(strip_go(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), strip_go(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))))
}