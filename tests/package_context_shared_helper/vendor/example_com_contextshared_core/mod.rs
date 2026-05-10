use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub type Exporter = Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<GoContext>>>) -> Arc<Mutex<Option<GoContext>>> + Send + Sync>>>>;


pub fn r#use(ctx: Arc<Mutex<Option<GoContext>>>, exporter: Exporter) -> Arc<Mutex<Option<GoContext>>> {

    return { let __f_guard = exporter.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(ctx.clone()) };
}