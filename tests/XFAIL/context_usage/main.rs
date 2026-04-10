use std::sync::{Arc, Mutex};
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
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut ctx, mut cancel) = GoContext::with_timeout(Arc::new(Mutex::new(Some(GoContext::background()))), std::time::Duration::from_secs(1));
    __defer_stack.push(Box::new(move || {
        (*cancel.lock().unwrap().as_ref().unwrap())();
    }));

    loop {
        if let Some(_) = go_channel_after(std::time::Duration::from_millis(500)).try_recv() {
            println!("{}", "Operation completed".to_string());
            break;
        }
        if let Some(_) = (*ctx.lock().unwrap().as_ref().unwrap()).done().try_recv() {
            println!("{} {}", "Context cancelled:".to_string(), format!("{}", (*((*ctx.lock().unwrap().as_ref().unwrap()).err()).lock().unwrap().as_ref().unwrap())));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}