pub fn worker(id: std::sync::Arc<std::sync::Mutex<Option<i32>>>, wg: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    // defer (*wg.lock().unwrap().as_mut().unwrap()).done() // TODO: defer not yet supported
    print!("Worker {} starting\n", (*id.lock().unwrap().as_mut().unwrap()));
    (*time.lock().unwrap().as_mut().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some((*time.lock().unwrap().as_mut().unwrap()).second))));
    print!("Worker {} done\n", (*id.lock().unwrap().as_mut().unwrap()));
}

fn main() {
    let mut wg;
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 3 {
        (*wg.lock().unwrap().as_mut().unwrap()).add(std::sync::Arc::new(std::sync::Mutex::new(Some(1))));
        // TODO: Unhandled statement type: GoStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*wg.lock().unwrap().as_mut().unwrap()).wait();
    println!("{}", "All workers done".to_string());
}