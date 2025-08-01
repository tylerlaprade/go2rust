pub fn worker(id: std::sync::Arc<std::sync::Mutex<Option<i32>>>, jobs: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>, results: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    for j in 0..(*jobs.lock().unwrap().as_mut().unwrap()).len() {
        println!("{} {} {} {}", "worker".to_string(), (*id.lock().unwrap().as_mut().unwrap()), "started  job".to_string(), j);
        (*time.lock().unwrap().as_mut().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some((*time.lock().unwrap().as_mut().unwrap()).second))));
        println!("{} {} {} {}", "worker".to_string(), (*id.lock().unwrap().as_mut().unwrap()), "finished job".to_string(), j);
        // TODO: Unhandled statement type: SendStmt
    }
}

fn main() {
    const numJobs: i32 = 5;

    let mut jobs = vec![0; numJobs];
    let mut results = vec![0; numJobs];
    let mut w = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*w.lock().unwrap().as_mut().unwrap()) <= 3 {
        // TODO: Unhandled statement type: GoStmt
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut j = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_mut().unwrap()) <= numJobs {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(std::sync::Arc::new(std::sync::Mutex::new(Some((*jobs.lock().unwrap().as_mut().unwrap())))));
    let mut a = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*a.lock().unwrap().as_mut().unwrap()) <= numJobs {
        <-(*results.lock().unwrap().as_mut().unwrap());
        { let mut guard = a.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}