pub fn say_hello(name: std::sync::Arc<std::sync::Mutex<Option<String>>>) {
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 3 {
        print!("Hello {}! ({})\n", (*name.lock().unwrap().as_ref().unwrap()), (*i.lock().unwrap().as_ref().unwrap()) + 1);
        (*time.lock().unwrap().as_ref().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(100 * (*time.lock().unwrap().as_ref().unwrap()).millisecond))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn counter(start: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some((*start.lock().unwrap().as_ref().unwrap()))));
    while (*i.lock().unwrap().as_ref().unwrap()) < (*start.lock().unwrap().as_ref().unwrap()) + 5 {
        print!("Count: {}\n", (*i.lock().unwrap().as_ref().unwrap()));
        (*time.lock().unwrap().as_ref().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(50 * (*time.lock().unwrap().as_ref().unwrap()).millisecond))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    println!("{}", "Starting goroutines...".to_string());
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    (*time.lock().unwrap().as_ref().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(1 * (*time.lock().unwrap().as_ref().unwrap()).second))));
    println!("{}", "Main function ending".to_string());
}