use std::any::Any;
use std::fmt::{Display, Formatter};
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

#[derive(Clone, Default)]
pub struct entry {
    pub value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.value.clone();
        Self {
            value: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.value.lock().unwrap().as_ref() { Some(__v) => go_format_any_nested(|| format_any(__v.as_ref())), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });
    let mut e = Arc::new(Mutex::new(Some(entry { value: Default::default() })));
    if { let __nil_target = (*e.lock().unwrap().as_ref().unwrap()).value.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        println!("{}", format!("{}", "nil".to_string()));
    }
}