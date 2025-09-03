use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
struct readOp {
    key: Arc<Mutex<Option<i32>>>,
    resp: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>,
}

impl std::fmt::Display for readOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.key.borrow().as_ref().unwrap()), (*self.resp.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct writeOp {
    key: Arc<Mutex<Option<i32>>>,
    val: Arc<Mutex<Option<i32>>>,
    resp: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>,
}

impl std::fmt::Display for writeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.key.borrow().as_ref().unwrap()), (*self.val.borrow().as_ref().unwrap()), (*self.resp.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut readOps: Arc<Mutex<Option<u64>>> = Default::default();
    let mut writeOps: Arc<Mutex<Option<u64>>> = Default::default();

    let mut reads = ;
    let mut writes = ;

    let key_closure_clone = key.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writes_closure_clone = writes.clone(); let key_thread = key.clone(); let reads_thread = reads.clone(); let resp_thread = resp.clone(); let val_thread = val.clone(); let writes_thread = writes.clone(); std::thread::spawn(move || {
        let mut state = Arc::new(Mutex::new(Some(HashMap::<i32, Arc<Mutex<Option<i32>>>>::new())));;
        while true {
        // TODO: Unhandled statement type: SelectStmt
    };;
    });

    let atomic_closure_clone = atomic.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let readOp_closure_clone = readOp.clone(); let readOps_closure_clone = readOps.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let mut r = Arc::new(Mutex::new(Some(0)));
    while (*r.lock().unwrap().as_mut().unwrap()) < 100 {
        let atomic_closure_clone = atomic.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let readOp_closure_clone = readOp.clone(); let readOps_closure_clone = readOps.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let key_thread = key.clone(); let readOps_thread = readOps.clone(); let reads_thread = reads.clone(); let resp_thread = resp.clone(); std::thread::spawn(move || {
        while true {
        let mut read = Arc::new(Mutex::new(Some(readOp { key: Arc::new(Mutex::new(Some((*rand.lock().unwrap().as_mut().unwrap())::intn(Arc::new(Mutex::new(Some(5))))))), resp: Arc::new(Mutex::new(Some())) })));
        // TODO: Unhandled statement type: SendStmt
        <-(*(*read.lock().unwrap().as_ref().unwrap()).resp.lock().unwrap().as_ref().unwrap());
        (*atomic.lock().unwrap().as_mut().unwrap())::add_uint64(Arc::new(Mutex::new(Some(readOps_thread.clone()))), Arc::new(Mutex::new(Some(1))));
        std::thread::sleep(std::time::Duration::from_nanos((*time.lock().unwrap().as_mut().unwrap())::millisecond as u64));
    };;
    });
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let atomic_closure_clone = atomic.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writeOp_closure_clone = writeOp.clone(); let writeOps_closure_clone = writeOps.clone(); let writes_closure_clone = writes.clone(); let mut w = Arc::new(Mutex::new(Some(0)));
    while (*w.lock().unwrap().as_mut().unwrap()) < 10 {
        let atomic_closure_clone = atomic.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writeOp_closure_clone = writeOp.clone(); let writeOps_closure_clone = writeOps.clone(); let writes_closure_clone = writes.clone(); let key_thread = key.clone(); let resp_thread = resp.clone(); let val_thread = val.clone(); let writeOps_thread = writeOps.clone(); let writes_thread = writes.clone(); std::thread::spawn(move || {
        while true {
        let mut write = Arc::new(Mutex::new(Some(writeOp { key: Arc::new(Mutex::new(Some((*rand.lock().unwrap().as_mut().unwrap())::intn(Arc::new(Mutex::new(Some(5))))))), val: Arc::new(Mutex::new(Some((*rand.lock().unwrap().as_mut().unwrap())::intn(Arc::new(Mutex::new(Some(100))))))), resp: Arc::new(Mutex::new(Some())) })));
        // TODO: Unhandled statement type: SendStmt
        <-(*(*write.lock().unwrap().as_ref().unwrap()).resp.lock().unwrap().as_ref().unwrap());
        (*atomic.lock().unwrap().as_mut().unwrap())::add_uint64(Arc::new(Mutex::new(Some(writeOps_thread.clone()))), Arc::new(Mutex::new(Some(1))));
        std::thread::sleep(std::time::Duration::from_nanos((*time.lock().unwrap().as_mut().unwrap())::millisecond as u64));
    };;
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    std::thread::sleep(std::time::Duration::from_millis(500));

    let mut readOpsFinal = (*atomic.lock().unwrap().as_mut().unwrap())::load_uint64(Arc::new(Mutex::new(Some(readOps.clone()))));
    println!("{} {}", "readOps:".to_string(), (*readOpsFinal.lock().unwrap().as_mut().unwrap()));
    let mut writeOpsFinal = (*atomic.lock().unwrap().as_mut().unwrap())::load_uint64(Arc::new(Mutex::new(Some(writeOps.clone()))));
    println!("{} {}", "writeOps:".to_string(), (*writeOpsFinal.lock().unwrap().as_mut().unwrap()));
}