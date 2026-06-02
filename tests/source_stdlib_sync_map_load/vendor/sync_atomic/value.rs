use go2rust_stdlib_stubs::*;

use crate::doc::*;
use crate::doc_64::*;
use crate::r#type::*;

pub(crate) static firstStoreInProgress: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u8>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *firstStoreInProgress.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *firstStoreInProgress.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
