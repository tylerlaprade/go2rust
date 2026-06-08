use go2rust_stdlib_stubs::*;

pub(crate) static _zero: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_zero.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *_zero.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
