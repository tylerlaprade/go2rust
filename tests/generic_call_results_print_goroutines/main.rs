#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn local__string(v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn local__int(v: Arc<Mutex<Option<i64>>>) -> i64 {
    return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

fn main() {
    ::example_com_genericcallresultsprintgoroutines_dep::__go_init_all();

    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send("started".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
    println!("{} {}", format!("{}", (*example_com_genericcallresultsprintgoroutines_dep::pick::<String>(Arc::new(Mutex::new(Some("qualified".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_genericcallresultsprintgoroutines_dep::pick::<String>(Arc::new(Mutex::new(Some("inferred".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", (*local__string(Arc::new(Mutex::new(Some("local".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", local__int(Arc::new(Mutex::new(Some(3))))));
    println!("{} {}", format!("{}", (*example_com_genericcallresultsprintgoroutines_dep::join::<String, i64>(Arc::new(Mutex::new(Some("a".to_string()))), Arc::new(Mutex::new(Some(12))), Arc::new(Mutex::new(Some("-".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_genericcallresultsprintgoroutines_dep::join::<String, String>(Arc::new(Mutex::new(Some("b".to_string()))), Arc::new(Mutex::new(Some("c".to_string()))), Arc::new(Mutex::new(Some("+".to_string())))).lock().unwrap().as_ref().unwrap())));
    print!("{}|{}\n", format!("{}", (*example_com_genericcallresultsprintgoroutines_dep::pick::<String>(Arc::new(Mutex::new(Some("formatted".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*example_com_genericcallresultsprintgoroutines_dep::join::<i64, i64>(Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(":".to_string())))).lock().unwrap().as_ref().unwrap())));
}