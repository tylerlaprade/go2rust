use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::sync::{Arc, Mutex};


fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

/// Source-transpiling sync is required before sync.Map bridge methods can
/// retire. Today the generated internal/sync crate fails to compile on the
/// hashtriemap generic implementation and a Mutex name collision.
fn main() {
    internal_abi::__go_init_all();
    internal_sync::__go_init_all();
    sync::__go_init_all();
    sync_atomic::__go_init_all();

    let mut m: Arc<Mutex<Option<sync::hashtriemap::Map>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*m.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(Box::new("key".to_string()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new("value".to_string()) as Box<dyn Any + Send + Sync>))));
    let (mut value, mut ok) = (*m.lock().unwrap().as_ref().unwrap()).load(Arc::new(Mutex::new(Some(Box::new("key".to_string()) as Box<dyn Any + Send + Sync>))));
    println!("{} {}", format!("{}", format_any(value.lock().unwrap().as_ref().unwrap().as_ref())), format!("{}", ok));
}