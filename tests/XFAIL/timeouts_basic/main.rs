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
    let mut c1 = GoChannel::<String>::new_buffered(1 as usize);
    let c1_closure_clone = c1.clone(); let c1_thread = c1.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));;
        c1_thread.send("result 1".to_string());;;
    });

    loop {
        if let Some(res) = c1.try_recv() {
            let mut res = Arc::new(Mutex::new(Some(res)));
            println!("{}", (*res.lock().unwrap().as_ref().unwrap()));
            break;
        }
        if let Some(_) = go_channel_after(std::time::Duration::from_millis(500)).try_recv() {
            println!("{}", "timeout 1".to_string());
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    let mut c2 = GoChannel::<String>::new_buffered(1 as usize);
    let c2_closure_clone = c2.clone(); let c2_thread = c2.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));;
        c2_thread.send("result 2".to_string());;;
    });
    loop {
        if let Some(res) = c2.try_recv() {
            let mut res = Arc::new(Mutex::new(Some(res)));
            println!("{}", (*res.lock().unwrap().as_ref().unwrap()));
            break;
        }
        if let Some(_) = go_channel_after(std::time::Duration::from_millis(1500)).try_recv() {
            println!("{}", "timeout 2".to_string());
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}