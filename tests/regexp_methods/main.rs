#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub(crate) static modFlagRegexp: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<regexp::r#mod::Regexp>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *modFlagRegexp.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *modFlagRegexp.lock().unwrap() = Some(regexp::must_compile(Arc::new(Mutex::new(Some("-mod[ =](\\w+)".to_string())))));
}


pub fn capture(re: Arc<Mutex<Option<regexp::r#mod::Regexp>>>, text: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    { let __recv = re.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.find_string_submatch(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }
}

fn main() {
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::regexp::__go_init_all();
    ::regexp_syntax::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    __go_init_all();
    let mut modMatches = capture({ let __arg_holder = modFlagRegexp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some("-mod=vendor".to_string()))));
    let mut modMatches = { let __slice_header = GoSliceHeader::go_slice_header(&modMatches); Arc::new(Mutex::new(__slice_header)) };
    println!("{}", format!("{}", { let __seq = { let __seq_holder = modMatches.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    println!("{}", format!("{}", { let __seq = { let __seq_holder = { let __recv_holder = (*modFlagRegexp.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.find_string_submatch(Arc::new(Mutex::new(Some("-mod vendor".to_string())))); __result }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    let mut changed = regexp::must_compile(Arc::new(Mutex::new(Some("go:.*go.mod.*contents have changed".to_string()))));
    println!("{}", format!("{}", { let __recv = changed.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.match_string(Arc::new(Mutex::new(Some("go: updates to go.mod needed, but contents have changed".to_string())))); __result }));
    let mut version = regexp::must_compile(Arc::new(Mutex::new(Some("^go version (go\\S+|devel \\S+)".to_string()))));
    println!("{}", format!("{}", { let __seq = { let __seq_holder = { let __recv = version.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.find_string_submatch(Arc::new(Mutex::new(Some("go version go1.22.0 darwin/arm64".to_string())))); __result }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    let mut currency = regexp::must_compile(Arc::new(Mutex::new(Some("[$,]".to_string()))));
    println!("{}", format!("{}", (*{ let __recv = currency.clone(); let __recv_ptr: *const regexp::r#mod::Regexp = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const regexp::r#mod::Regexp }; let __result = unsafe { &*__recv_ptr }.replace_all_string(Arc::new(Mutex::new(Some("$1,234".to_string()))), Arc::new(Mutex::new(Some("".to_string())))); __result }.lock().unwrap().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
