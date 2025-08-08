use std::sync::{Arc, Mutex};

pub fn f(from: Arc<Mutex<Option<String>>>) {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        println!("{} {} {}", (*from.lock().unwrap().as_mut().unwrap()), ":".to_string(), (*i.lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    f(Arc::new(Mutex::new(Some("direct".to_string()))));

    // TODO: Unhandled statement type: GoStmt

    // TODO: Unhandled statement type: GoStmt

    (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some(500 * (*time.lock().unwrap().as_mut().unwrap())::millisecond))));
    println!("{}", "done".to_string());
}