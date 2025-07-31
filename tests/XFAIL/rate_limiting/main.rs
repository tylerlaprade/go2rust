fn main() {
    let mut requests = vec![0; 5];
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_ref().unwrap()) <= 5 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(std::sync::Arc::new(std::sync::Mutex::new(Some((*requests.lock().unwrap().as_ref().unwrap())))));
    let mut limiter = (*time.lock().unwrap().as_ref().unwrap()).tick(std::sync::Arc::new(std::sync::Mutex::new(Some(200 * (*time.lock().unwrap().as_ref().unwrap()).millisecond))));
    for req in 0..(*requests.lock().unwrap().as_ref().unwrap()).len() {
        <-(*limiter.lock().unwrap().as_ref().unwrap());
        println!("{} {} {}", "request".to_string(), req, (*time.lock().unwrap().as_ref().unwrap()).now());
    }
    let mut burstyLimiter = vec![0; 3];
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 3 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    // TODO: Unhandled statement type: GoStmt
    let mut burstyRequests = vec![0; 5];
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_ref().unwrap()) <= 5 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(std::sync::Arc::new(std::sync::Mutex::new(Some((*burstyRequests.lock().unwrap().as_ref().unwrap())))));
    for req in 0..(*burstyRequests.lock().unwrap().as_ref().unwrap()).len() {
        <-(*burstyLimiter.lock().unwrap().as_ref().unwrap());
        println!("{} {} {}", "request".to_string(), req, (*time.lock().unwrap().as_ref().unwrap()).now());
    }
}