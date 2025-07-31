pub fn f(from: std::sync::Arc<std::sync::Mutex<Option<String>>>) {
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 3 {
        println!("{} {} {}", (*from.lock().unwrap().as_ref().unwrap()), ":".to_string(), (*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    f(std::sync::Arc::new(std::sync::Mutex::new(Some("direct".to_string()))));
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    (*time.lock().unwrap().as_ref().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some((*time.lock().unwrap().as_ref().unwrap()).second))));
    println!("{}", "done".to_string());
}