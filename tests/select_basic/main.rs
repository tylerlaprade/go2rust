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

fn main() {
    let mut c1 = GoChannel::<String>::new();
    let mut c2 = GoChannel::<String>::new();

    let c1_closure_clone = c1.clone(); let c1_thread = c1.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));;
        c1_thread.send("one".to_string());;;
    });
    let c2_closure_clone = c2.clone(); let c2_thread = c2.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));;
        c2_thread.send("two".to_string());;;
    });

    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 2 {
        loop {
        if let Some(msg1) = c1.try_recv() {
            let mut msg1 = Arc::new(Mutex::new(Some(msg1)));
            println!("{} {}", "received".to_string(), (*msg1.lock().unwrap().as_mut().unwrap()));
            break;
        }
        if let Some(msg2) = c2.try_recv() {
            let mut msg2 = Arc::new(Mutex::new(Some(msg2)));
            println!("{} {}", "received".to_string(), (*msg2.lock().unwrap().as_mut().unwrap()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}