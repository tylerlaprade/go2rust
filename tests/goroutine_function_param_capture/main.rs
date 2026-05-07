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

pub fn run(f: Arc<Mutex<Option<Box<dyn Fn() -> Arc<Mutex<Option<String>>> + Send + Sync>>>>, done: GoChannel<String>) {
    let done_thread = done.clone(); let f_thread = f.clone(); std::thread::spawn(move || {
        done_thread.send((*{ let __f_guard = f_thread.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.lock().unwrap().as_ref().unwrap()).clone());;;
    });
}

fn main() {
    let mut done = GoChannel::<String>::new();
    run(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some("ok".to_string())));
    }) as Box<dyn Fn() -> Arc<Mutex<Option<String>>> + Send + Sync>))), done.clone());
    println!("{}", done.recv().unwrap());
}