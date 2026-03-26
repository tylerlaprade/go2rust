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

fn main() {
    let mut requests = GoChannel::<i32>::new_buffered(5 as usize);
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_ref().unwrap()) <= 5 {
        requests.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    requests.close();

    let mut limiter = (*time.lock().unwrap().as_ref().unwrap())::tick(Arc::new(Mutex::new(Some(100 * (*time.lock().unwrap().as_ref().unwrap())::millisecond))));

    for req in requests.clone() {
        limiter.recv().unwrap();
        println!("{} {} {}", "request".to_string(), req, (*(*time.lock().unwrap().as_ref().unwrap())::now().lock().unwrap().as_ref().unwrap()));
    }

    let mut burstyLimiter = GoChannel::<time_Time>::new_buffered(3 as usize);

    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 3 {
        burstyLimiter.send((*time.lock().unwrap().as_ref().unwrap())::now());
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let burstyLimiter_closure_clone = burstyLimiter.clone(); let burstyLimiter_thread = burstyLimiter.clone(); std::thread::spawn(move || {
        for t in (*time.lock().unwrap().as_ref().unwrap())::tick(Arc::new(Mutex::new(Some(100 * (*time.lock().unwrap().as_ref().unwrap())::millisecond)))).clone() {
        burstyLimiter_thread.send(t);
    };;
    });

    let mut burstyRequests = GoChannel::<i32>::new_buffered(5 as usize);
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_ref().unwrap()) <= 5 {
        burstyRequests.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    burstyRequests.close();
    for req in burstyRequests.clone() {
        burstyLimiter.recv().unwrap();
        println!("{} {} {}", "request".to_string(), req, (*(*time.lock().unwrap().as_ref().unwrap())::now().lock().unwrap().as_ref().unwrap()));
    }
}