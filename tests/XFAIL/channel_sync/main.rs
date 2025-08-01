pub fn worker(done: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some("working...".to_string()))));
    (*time.lock().unwrap().as_mut().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some((*time.lock().unwrap().as_mut().unwrap()).second))));
    println!("{}", "done".to_string());
    // TODO: Unhandled statement type: SendStmt
}

fn main() {
    let mut done = vec![0; 1];
    // TODO: Unhandled statement type: GoStmt
    <-(*done.lock().unwrap().as_mut().unwrap());
}