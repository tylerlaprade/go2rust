fn main() {
    let mut jobs = vec![0; 5];
    let mut done = ;
    // TODO: Unhandled statement type: GoStmt
    let mut j = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_ref().unwrap()) <= 3 {
        // TODO: Unhandled statement type: SendStmt
        println!("{} {}", "sent job".to_string(), (*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(std::sync::Arc::new(std::sync::Mutex::new(Some((*jobs.lock().unwrap().as_ref().unwrap())))));
    println!("{}", "sent all jobs".to_string());
    <-(*done.lock().unwrap().as_ref().unwrap());
}