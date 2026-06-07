use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

pub(crate) static goexitPanicValue: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *goexitPanicValue.lock().unwrap() = None;
    *goexitPanicValue.lock().unwrap() = Some(Box::new(Arc::new(Mutex::new(Some(i32::default()))).clone()) as Box<dyn Any + Send + Sync>);
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
