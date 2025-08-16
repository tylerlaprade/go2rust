use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct readOp {
    key: Arc<Mutex<Option<i32>>>,
    resp: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>,
}

#[derive(Debug)]
struct writeOp {
    key: Arc<Mutex<Option<i32>>>,
    val: Arc<Mutex<Option<i32>>>,
    resp: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>,
}

fn main() {
    let mut readOps: Arc<Mutex<Option<u64>>> = Default::default();
    let mut writeOps: Arc<Mutex<Option<u64>>> = Default::default();

    let mut reads = ;
    let mut writes = ;

    // TODO: Unhandled statement type: GoStmt

    let mut r = Arc::new(Mutex::new(Some(0)));
    while (*r.lock().unwrap().as_mut().unwrap()) < 100 {
        // TODO: Unhandled statement type: GoStmt
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut w = Arc::new(Mutex::new(Some(0)));
    while (*w.lock().unwrap().as_mut().unwrap()) < 10 {
        // TODO: Unhandled statement type: GoStmt
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    time.sleep(Arc::new(Mutex::new(Some(500 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));

    let mut readOpsFinal = atomic.load_uint64(Arc::new(Mutex::new(Some(readOps.clone()))));
    println!("{} {}", "readOps:".to_string(), (*readOpsFinal.lock().unwrap().as_mut().unwrap()));
    let mut writeOpsFinal = atomic.load_uint64(Arc::new(Mutex::new(Some(writeOps.clone()))));
    println!("{} {}", "writeOps:".to_string(), (*writeOpsFinal.lock().unwrap().as_mut().unwrap()));
}