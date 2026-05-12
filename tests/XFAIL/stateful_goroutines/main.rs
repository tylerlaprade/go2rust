use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
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

pub mod atomic {
    use super::*;
    pub fn add_uint64<T0, T1>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<u64>>> {
        Arc::new(Mutex::new(Some::<u64>(Default::default())))
    }

    pub fn load_uint64<T0>(_arg0: T0) -> Arc<Mutex<Option<u64>>> {
        Arc::new(Mutex::new(Some::<u64>(Default::default())))
    }
}


#[derive(Debug, Clone, Default)]
pub struct readOp {
    pub key: Arc<Mutex<Option<i32>>>,
    pub resp: GoChannel<i32>,
}

impl readOp {
    pub fn __go_value_clone(&self) -> Self {
        Self { key: { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, resp: self.resp.clone() }
    }
}

impl std::fmt::Display for readOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.key.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct writeOp {
    pub key: Arc<Mutex<Option<i32>>>,
    pub val: Arc<Mutex<Option<i32>>>,
    pub resp: GoChannel<bool>,
}

impl writeOp {
    pub fn __go_value_clone(&self) -> Self {
        Self { key: { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, val: { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, resp: self.resp.clone() }
    }
}

impl std::fmt::Display for writeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.key.lock().unwrap().as_ref().unwrap()), (*self.val.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut readOps: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut writeOps: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

    let mut reads = GoChannel::<readOp>::new();
    let mut writes = GoChannel::<writeOp>::new();
    let mut done = GoChannel::<bool>::new();

    let reads_thread = reads.clone(); let writes_thread = writes.clone(); std::thread::spawn(move || {
        let mut state = Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<i32>>>>::new())));;
        loop {
        loop {
        if let Some(read) = reads_thread.try_recv() {
            let mut read = Arc::new(Mutex::new(Some(read)));
            (*read.lock().unwrap().as_ref().unwrap()).resp.send({ let __map = { let __map_holder = state.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&(*{ let __field = (*read.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) });
            break;
        }
        if let Some(write) = writes_thread.try_recv() {
            let mut write = Arc::new(Mutex::new(Some(write)));
            { let __map_key = (*{ let __field = (*write.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __map_value = Arc::new(Mutex::new(Some((*{ let __field = (*write.lock().unwrap().as_ref().unwrap()).val.clone(); __field }.lock().unwrap().as_ref().unwrap())))); (*state.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
            (*write.lock().unwrap().as_ref().unwrap()).resp.send(true);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    };;
    });

    let mut r = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x < __tmp_y } {
        let done_thread = done.clone(); let readOps_thread = Arc::new(Mutex::new(Some((*readOps.lock().unwrap().as_ref().unwrap()).clone()))); let reads_thread = reads.clone(); std::thread::spawn(move || {
        let mut read = Arc::new(Mutex::new(Some(readOp { key: Arc::new(Mutex::new(Some(go_rand_intn(5 as i32)))), resp: GoChannel::<i32>::new(), ..Default::default() })));;
        reads_thread.send(read.lock().unwrap().as_ref().unwrap().clone());;
        (*read.lock().unwrap().as_ref().unwrap()).resp.recv().unwrap();;
        atomic::add_uint64(readOps_thread.clone(), 1);;
        done_thread.send(true);;;
    });
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut w = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        let done_thread = done.clone(); let writeOps_thread = Arc::new(Mutex::new(Some((*writeOps.lock().unwrap().as_ref().unwrap()).clone()))); let writes_thread = writes.clone(); std::thread::spawn(move || {
        let mut write = Arc::new(Mutex::new(Some(writeOp { key: Arc::new(Mutex::new(Some(go_rand_intn(5 as i32)))), val: Arc::new(Mutex::new(Some(go_rand_intn(100 as i32)))), resp: GoChannel::<bool>::new(), ..Default::default() })));;
        writes_thread.send(write.lock().unwrap().as_ref().unwrap().clone());;
        (*write.lock().unwrap().as_ref().unwrap()).resp.recv().unwrap();;
        atomic::add_uint64(writeOps_thread.clone(), 1);;
        done_thread.send(true);;;
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 110; __tmp_x < __tmp_y } {
        done.recv().unwrap();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut readOpsFinal = atomic::load_uint64(readOps.clone());
    println!("{} {}", "readOps:".to_string(), { let __v = (*readOpsFinal.lock().unwrap().as_ref().unwrap()).clone(); __v });
    let mut writeOpsFinal = atomic::load_uint64(writeOps.clone());
    println!("{} {}", "writeOps:".to_string(), { let __v = (*writeOpsFinal.lock().unwrap().as_ref().unwrap()).clone(); __v });
}