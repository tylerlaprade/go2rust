use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
        }
    }

    fn send(&self, val: T) {
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            let _ = tx.send(val);
        }
    }

    fn try_send(&self, val: T) -> bool {
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            tx.try_send(val).is_ok()
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        self.rx.lock().unwrap().recv().ok()
    }

    fn try_recv(&self) -> Option<T> {
        self.rx.lock().unwrap().try_recv().ok()
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}

#[derive(Debug, Clone, Default)]
struct readOp {
    key: Arc<Mutex<Option<i32>>>,
    resp: GoChannel<i32>,
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
    resp: GoChannel<bool>,
}

impl std::fmt::Display for writeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.key.borrow().as_ref().unwrap()), (*self.val.borrow().as_ref().unwrap()), (*self.resp.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut readOps: Arc<Mutex<Option<u64>>> = Default::default();
    let mut writeOps: Arc<Mutex<Option<u64>>> = Default::default();

    let mut reads = GoChannel::<readOp>::new();
    let mut writes = GoChannel::<writeOp>::new();

    let key_closure_clone = key.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writes_closure_clone = writes.clone(); let key_thread = Arc::new(Mutex::new(Some((*key.lock().unwrap().as_ref().unwrap()).clone()))); let reads_thread = reads.clone(); let resp_thread = Arc::new(Mutex::new(Some((*resp.lock().unwrap().as_ref().unwrap()).clone()))); let val_thread = Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()).clone()))); let writes_thread = writes.clone(); std::thread::spawn(move || {
        let mut state = Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<i32>>>>::new())));;
        while true {
        loop {
        if let Some(read) = reads_thread.try_recv() {
            let mut read = Arc::new(Mutex::new(Some(read)));
            (*(*read.lock().unwrap().as_ref().unwrap()).resp.lock().unwrap().as_ref().unwrap()).send((*(*state.lock().unwrap().as_ref().unwrap()).get(&(*(*read.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap())).unwrap().lock().unwrap().as_ref().unwrap()));
            break;
        }
        if let Some(write) = writes_thread.try_recv() {
            let mut write = Arc::new(Mutex::new(Some(write)));
            (*state.lock().unwrap().as_mut().unwrap()).insert((*(*write.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some((*(*write.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap())))));
            (*(*write.lock().unwrap().as_ref().unwrap()).resp.lock().unwrap().as_ref().unwrap()).send(true);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    };;
    });

    let atomic_closure_clone = atomic.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let readOp_closure_clone = readOp.clone(); let readOps_closure_clone = readOps.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let mut r = Arc::new(Mutex::new(Some(0)));
    while (*r.lock().unwrap().as_mut().unwrap()) < 100 {
        let atomic_closure_clone = atomic.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let readOp_closure_clone = readOp.clone(); let readOps_closure_clone = readOps.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let key_thread = Arc::new(Mutex::new(Some((*key.lock().unwrap().as_ref().unwrap()).clone()))); let readOps_thread = Arc::new(Mutex::new(Some((*readOps.lock().unwrap().as_ref().unwrap()).clone()))); let reads_thread = reads.clone(); let resp_thread = Arc::new(Mutex::new(Some((*resp.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        while true {
        let mut read = Arc::new(Mutex::new(Some(readOp { key: Arc::new(Mutex::new(Some((*rand.lock().unwrap().as_mut().unwrap())::intn(Arc::new(Mutex::new(Some(5))))))), resp: Arc::new(Mutex::new(Some(GoChannel::<i32>::new()))) })));
        reads_thread.send(read.lock().unwrap().as_ref().unwrap().clone());
        (*(*read.lock().unwrap().as_ref().unwrap()).resp.lock().unwrap().as_ref().unwrap()).recv().unwrap();
        (*atomic.lock().unwrap().as_mut().unwrap())::add_uint64(Arc::new(Mutex::new(Some(readOps_thread.clone()))), Arc::new(Mutex::new(Some(1))));
        std::thread::sleep(std::time::Duration::from_nanos((*time.lock().unwrap().as_mut().unwrap())::millisecond as u64));
    };;
    });
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let atomic_closure_clone = atomic.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writeOp_closure_clone = writeOp.clone(); let writeOps_closure_clone = writeOps.clone(); let writes_closure_clone = writes.clone(); let mut w = Arc::new(Mutex::new(Some(0)));
    while (*w.lock().unwrap().as_mut().unwrap()) < 10 {
        let atomic_closure_clone = atomic.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writeOp_closure_clone = writeOp.clone(); let writeOps_closure_clone = writeOps.clone(); let writes_closure_clone = writes.clone(); let key_thread = Arc::new(Mutex::new(Some((*key.lock().unwrap().as_ref().unwrap()).clone()))); let resp_thread = Arc::new(Mutex::new(Some((*resp.lock().unwrap().as_ref().unwrap()).clone()))); let val_thread = Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()).clone()))); let writeOps_thread = Arc::new(Mutex::new(Some((*writeOps.lock().unwrap().as_ref().unwrap()).clone()))); let writes_thread = writes.clone(); std::thread::spawn(move || {
        while true {
        let mut write = Arc::new(Mutex::new(Some(writeOp { key: Arc::new(Mutex::new(Some((*rand.lock().unwrap().as_mut().unwrap())::intn(Arc::new(Mutex::new(Some(5))))))), val: Arc::new(Mutex::new(Some((*rand.lock().unwrap().as_mut().unwrap())::intn(Arc::new(Mutex::new(Some(100))))))), resp: Arc::new(Mutex::new(Some(GoChannel::<bool>::new()))) })));
        writes_thread.send(write.lock().unwrap().as_ref().unwrap().clone());
        (*(*write.lock().unwrap().as_ref().unwrap()).resp.lock().unwrap().as_ref().unwrap()).recv().unwrap();
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