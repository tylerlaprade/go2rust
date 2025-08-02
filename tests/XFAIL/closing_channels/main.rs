fn main() {
    let mut jobs = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 5])));
    let mut done = ;
    // TODO: Unhandled statement type: GoStmt
    let mut j = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_mut().unwrap()) <= 3 {
        // TODO: Unhandled statement type: SendStmt
        println!("{} {}", "sent job".to_string(), (*j.lock().unwrap().as_mut().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(jobs.clone());
    println!("{}", "sent all jobs".to_string());
    <-(*done.lock().unwrap().as_mut().unwrap());
}