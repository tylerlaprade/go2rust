use std::sync::{Arc, Mutex};


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
    let mut messages = GoChannel::<String>::new();
    let mut signals = GoChannel::<bool>::new();

    loop {
        if let Some(msg) = messages.try_recv() {
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", "received message".to_string(), (*msg.lock().unwrap().as_mut().unwrap()));
            break;
        }
        println!("{}", "no message received".to_string());
        break;
    }

    let mut msg = Arc::new(Mutex::new(Some("hi".to_string())));
    loop {
        if messages.try_send(msg.lock().unwrap().as_ref().unwrap().clone()) {
            println!("{} {}", "sent message".to_string(), (*msg.lock().unwrap().as_mut().unwrap()));
            break;
        }
        println!("{}", "no message sent".to_string());
        break;
    }

    loop {
        if let Some(msg) = messages.try_recv() {
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", "received message".to_string(), (*msg.lock().unwrap().as_mut().unwrap()));
            break;
        }
        if let Some(sig) = signals.try_recv() {
            let mut sig = Arc::new(Mutex::new(Some(sig)));
            println!("{} {}", "received signal".to_string(), (*sig.lock().unwrap().as_mut().unwrap()));
            break;
        }
        println!("{}", "no activity".to_string());
        break;
    }
}