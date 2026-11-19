use std::any::Any;


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

fn format_any(value: &dyn Any) -> String {
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
