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

pub fn worker(id: Arc<Mutex<Option<i32>>>, jobs: GoChannel<i32>, results: GoChannel<i32>) {
    let _ = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v };
    for j in jobs.clone() {
        results.send({ let __tmp_x = j; let __tmp_y = 2; __tmp_x * __tmp_y });
    }
}

fn main() {
    const numJobs: i32 = 5;

    let mut jobs = GoChannel::<i32>::new_buffered((numJobs) as usize);
    let mut results = GoChannel::<i32>::new_buffered((numJobs) as usize);

    let mut w = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        let jobs_thread = jobs.clone(); let results_thread = results.clone(); let w_thread = Arc::new(Mutex::new(Some((*w.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        worker(Arc::new(Mutex::new(Some((*w_thread.lock().unwrap().as_ref().unwrap()).clone()))), jobs_thread.clone(), results_thread.clone());
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut j = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = numJobs; __tmp_x <= __tmp_y } {
        jobs.send((*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    jobs.close();

    let mut total = Arc::new(Mutex::new(Some(0)));
    let mut a = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = numJobs; __tmp_x <= __tmp_y } {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + results.recv().unwrap()); };
        { let mut guard = a.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    println!("{} {}", format!("{}", "Processed jobs:".to_string()), format!("{}", numJobs));
    println!("{} {}", format!("{}", "Result total:".to_string()), format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}