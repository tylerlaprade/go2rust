#[derive(Debug)]
struct readOp {
    key: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    resp: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>,
}

#[derive(Debug)]
struct writeOp {
    key: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    val: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    resp: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>,
}

fn main() {
    let mut readOps = Default::default();
    let mut writeOps = Default::default();
    let mut reads = ;
    let mut writes = ;
    // TODO: Unhandled statement type: GoStmt
    let mut r = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*r.lock().unwrap().as_ref().unwrap()) < 100 {
        // TODO: Unhandled statement type: GoStmt
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut w = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*w.lock().unwrap().as_ref().unwrap()) < 10 {
        // TODO: Unhandled statement type: GoStmt
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*time.lock().unwrap().as_ref().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some((*time.lock().unwrap().as_ref().unwrap()).second))));
    let mut readOpsFinal = (*atomic.lock().unwrap().as_ref().unwrap()).load_uint64(std::sync::Arc::new(std::sync::Mutex::new(Some(readOps.clone()))));
    println!("{} {}", "readOps:".to_string(), (*readOpsFinal.lock().unwrap().as_ref().unwrap()));
    let mut writeOpsFinal = (*atomic.lock().unwrap().as_ref().unwrap()).load_uint64(std::sync::Arc::new(std::sync::Mutex::new(Some(writeOps.clone()))));
    println!("{} {}", "writeOps:".to_string(), (*writeOpsFinal.lock().unwrap().as_ref().unwrap()));
}