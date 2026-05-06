use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


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

fn go_rand_state() -> &'static std::sync::Mutex<u64> {
    static STATE: std::sync::OnceLock<std::sync::Mutex<u64>> = std::sync::OnceLock::new();
    STATE.get_or_init(|| std::sync::Mutex::new(1))
}

fn go_rand_seed(seed: i64) {
    *go_rand_state().lock().unwrap() = seed as u64;
}

fn go_rand_next_u64() -> u64 {
    let mut state = go_rand_state().lock().unwrap();
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

fn go_rand_intn(n: i32) -> i32 {
    if n <= 0 {
        panic!("invalid argument to Intn");
    }
    (go_rand_next_u64() % n as u64) as i32
}

fn go_rand_float64() -> f64 {
    ((go_rand_next_u64() >> 11) as f64) / ((1u64 << 53) as f64)
}

#[derive(Debug, Clone, Default)]
pub struct readOp {
    pub key: Arc<Mutex<Option<i32>>>,
    pub resp: GoChannel<i32>,
}

impl std::fmt::Display for readOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.key.lock().unwrap().as_ref().unwrap()), (*self.resp.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct writeOp {
    pub key: Arc<Mutex<Option<i32>>>,
    pub val: Arc<Mutex<Option<i32>>>,
    pub resp: GoChannel<bool>,
}

impl std::fmt::Display for writeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.key.lock().unwrap().as_ref().unwrap()), (*self.val.lock().unwrap().as_ref().unwrap()), (*self.resp.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut readOps: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut writeOps: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(Default::default())));

    let mut reads = GoChannel::<readOp>::new();
    let mut writes = GoChannel::<writeOp>::new();
    let mut done = GoChannel::<bool>::new();

    let key_closure_clone = key.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writes_closure_clone = writes.clone(); let key_thread = Arc::new(Mutex::new(Some((*key.lock().unwrap().as_ref().unwrap()).clone()))); let reads_thread = reads.clone(); let resp_thread = Arc::new(Mutex::new(Some((*resp.lock().unwrap().as_ref().unwrap()).clone()))); let val_thread = Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()).clone()))); let writes_thread = writes.clone(); std::thread::spawn(move || {
        let mut state = Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<i32>>>>::new())));;
        while true {
        loop {
        if let Some(read) = reads_thread.try_recv() {
            let mut read = Arc::new(Mutex::new(Some(read)));
            (*{ let __field = (*read.lock().unwrap().as_ref().unwrap()).resp.clone(); __field }.lock().unwrap().as_ref().unwrap()).send({ let __map = { let __map_holder = state.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&(*{ let __field = (*read.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) });
            break;
        }
        if let Some(write) = writes_thread.try_recv() {
            let mut write = Arc::new(Mutex::new(Some(write)));
            (*state.lock().unwrap().as_mut().unwrap()).insert((*{ let __field = (*write.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some((*{ let __field = (*write.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap())))));
            (*{ let __field = (*write.lock().unwrap().as_ref().unwrap()).resp.clone(); __field }.lock().unwrap().as_ref().unwrap()).send(true);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    };;
    });

    let atomic_closure_clone = atomic.clone(); let done_closure_clone = done.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let readOp_closure_clone = readOp.clone(); let readOps_closure_clone = readOps.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let mut r = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x < __tmp_y } {
        let atomic_closure_clone = atomic.clone(); let done_closure_clone = done.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let readOp_closure_clone = readOp.clone(); let readOps_closure_clone = readOps.clone(); let reads_closure_clone = reads.clone(); let resp_closure_clone = resp.clone(); let done_thread = done.clone(); let key_thread = Arc::new(Mutex::new(Some((*key.lock().unwrap().as_ref().unwrap()).clone()))); let readOps_thread = Arc::new(Mutex::new(Some((*readOps.lock().unwrap().as_ref().unwrap()).clone()))); let reads_thread = reads.clone(); let resp_thread = Arc::new(Mutex::new(Some((*resp.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        let mut read = Arc::new(Mutex::new(Some(readOp { key: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(go_rand_intn(5 as i32))))))), resp: Arc::new(Mutex::new(Some(GoChannel::<i32>::new()))), ..Default::default() })));;
        reads_thread.send(read.lock().unwrap().as_ref().unwrap().clone());;
        (*{ let __field = (*read.lock().unwrap().as_ref().unwrap()).resp.clone(); __field }.lock().unwrap().as_ref().unwrap()).recv().unwrap();;
        atomic::add_uint64(Arc::new(Mutex::new(Some(readOps_thread.clone()))), Arc::new(Mutex::new(Some(1))));;
        done_thread.send(true);;;
    });
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let atomic_closure_clone = atomic.clone(); let done_closure_clone = done.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writeOp_closure_clone = writeOp.clone(); let writeOps_closure_clone = writeOps.clone(); let writes_closure_clone = writes.clone(); let mut w = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        let atomic_closure_clone = atomic.clone(); let done_closure_clone = done.clone(); let key_closure_clone = key.clone(); let rand_closure_clone = rand.clone(); let resp_closure_clone = resp.clone(); let val_closure_clone = val.clone(); let writeOp_closure_clone = writeOp.clone(); let writeOps_closure_clone = writeOps.clone(); let writes_closure_clone = writes.clone(); let done_thread = done.clone(); let key_thread = Arc::new(Mutex::new(Some((*key.lock().unwrap().as_ref().unwrap()).clone()))); let resp_thread = Arc::new(Mutex::new(Some((*resp.lock().unwrap().as_ref().unwrap()).clone()))); let val_thread = Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()).clone()))); let writeOps_thread = Arc::new(Mutex::new(Some((*writeOps.lock().unwrap().as_ref().unwrap()).clone()))); let writes_thread = writes.clone(); std::thread::spawn(move || {
        let mut write = Arc::new(Mutex::new(Some(writeOp { key: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(go_rand_intn(5 as i32))))))), val: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(go_rand_intn(100 as i32))))))), resp: Arc::new(Mutex::new(Some(GoChannel::<bool>::new()))), ..Default::default() })));;
        writes_thread.send(write.lock().unwrap().as_ref().unwrap().clone());;
        (*{ let __field = (*write.lock().unwrap().as_ref().unwrap()).resp.clone(); __field }.lock().unwrap().as_ref().unwrap()).recv().unwrap();;
        atomic::add_uint64(Arc::new(Mutex::new(Some(writeOps_thread.clone()))), Arc::new(Mutex::new(Some(1))));;
        done_thread.send(true);;;
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 110; __tmp_x < __tmp_y } {
        done.recv().unwrap();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut readOpsFinal = atomic::load_uint64(Arc::new(Mutex::new(Some(readOps.clone()))));
    println!("{} {}", "readOps:".to_string(), { let __v = (*readOpsFinal.lock().unwrap().as_ref().unwrap()).clone(); __v });
    let mut writeOpsFinal = atomic::load_uint64(Arc::new(Mutex::new(Some(writeOps.clone()))));
    println!("{} {}", "writeOps:".to_string(), { let __v = (*writeOpsFinal.lock().unwrap().as_ref().unwrap()).clone(); __v });
}