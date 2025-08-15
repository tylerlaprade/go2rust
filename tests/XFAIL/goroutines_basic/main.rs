use std::sync::{Arc, Mutex};

pub fn say_hello(name: Arc<Mutex<Option<String>>>) {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        print!("Hello {}! ({})\n", (*name.lock().unwrap().as_mut().unwrap()), (*i.lock().unwrap().as_mut().unwrap()) + 1);
        time.sleep(Arc::new(Mutex::new(Some(100 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn counter(start: Arc<Mutex<Option<i32>>>) {
    let mut i = Arc::new(Mutex::new(Some((*start.lock().unwrap().as_mut().unwrap()))));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*start.lock().unwrap().as_mut().unwrap()) + 5 {
        print!("Count: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
        time.sleep(Arc::new(Mutex::new(Some(50 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    println!("{}", "Starting goroutines...".to_string());

        // Launch goroutines
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt
    // TODO: Unhandled statement type: GoStmt

        // Anonymous goroutine
    // TODO: Unhandled statement type: GoStmt

        // Wait for goroutines to complete
    time.sleep(Arc::new(Mutex::new(Some(500 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
    println!("{}", "Main function ending".to_string());
}