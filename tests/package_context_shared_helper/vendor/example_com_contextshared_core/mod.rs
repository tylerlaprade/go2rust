use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub type Exporter = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<GoContext>>>) -> Arc<Mutex<Option<GoContext>>> + Send + Sync>>>>;


pub fn r#use(ctx: Arc<Mutex<Option<GoContext>>>, exporter: Exporter) -> Arc<Mutex<Option<GoContext>>> {
    { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<GoContext>>>) -> Arc<Mutex<Option<GoContext>>> + Send + Sync> = { let mut __f_guard = exporter.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<GoContext>>>) -> Arc<Mutex<Option<GoContext>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(ctx.clone()) }
}