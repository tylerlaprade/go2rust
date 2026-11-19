use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread;


thread_local! {
    static GO_FORMAT_ANY_DEPTH: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

// Runs format one level further inside the value being printed.
fn go_format_any_nested(format: impl FnOnce() -> String) -> String {
    struct Depth;
    impl Drop for Depth {
        fn drop(&mut self) {
            GO_FORMAT_ANY_DEPTH.with(|depth| depth.set(depth.get() - 1));
        }
    }
    GO_FORMAT_ANY_DEPTH.with(|depth| depth.set(depth.get() + 1));
    let _depth = Depth;
    format()
}

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

fn format_any_slice_values(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_any(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    go_format_any_nested(|| format!("[{}]", format_any_slice_values(slice)))
}

fn format_any_variadic(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format_any_slice_values(slice)
}

pub fn log(values: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
    println!("{}", format_any_variadic(&values));
}

fn main() {
    std::thread::spawn(move || {
        ;
    });
    log(Arc::new(Mutex::new(Some(vec![Box::new("x".to_string()) as Box<dyn Any + Send + Sync>, Box::new(7) as Box<dyn Any + Send + Sync>, Box::new(true) as Box<dyn Any + Send + Sync>]))));
}