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
    let mut ticker = (*time.lock().unwrap().as_mut().unwrap())::new_ticker(Arc::new(Mutex::new(Some(250 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
    let mut done = GoChannel::<bool>::new();

    let done_closure_clone = done.clone(); let ticker_closure_clone = ticker.clone(); let C_thread = Arc::new(Mutex::new(Some((*C.lock().unwrap().as_ref().unwrap()).clone()))); let done_thread = done.clone(); let ticker_thread = Arc::new(Mutex::new(Some((*ticker.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        while true {
        loop {
        if let Some(_) = done_thread.try_recv() {
            return;
            break;
        }
        if let Some(t) = (*(*ticker.lock().unwrap().as_ref().unwrap()).c.lock().unwrap().as_ref().unwrap()).try_recv() {
            let mut t = Arc::new(Mutex::new(Some(t)));
            println!("{} {}", "Tick at".to_string(), (*t.lock().unwrap().as_mut().unwrap()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    };;
    });

    std::thread::sleep(std::time::Duration::from_millis(800));
    (*ticker.lock().unwrap().as_mut().unwrap()).stop();
    done.send(true);
    println!("{}", "Ticker stopped".to_string());
}