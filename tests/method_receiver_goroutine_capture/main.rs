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

#[derive(Debug, Clone, Default)]
pub struct Runner {
    pub name: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for Runner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}


impl Runner {
    pub fn after(&mut self) {
        let _ = self.name.clone();
    }

    pub fn run(&mut self, done: GoChannel<String>) {
        let done_thread = done.clone(); let mut r_thread = self.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let mut r_defer_captured = r_thread.clone(); __defer_stack.push(Box::new(move || {
        r_defer_captured.after();
    }));;
        done_thread.send(r_thread.name.clone().lock().unwrap().as_ref().unwrap().clone());;
        while let Some(f) = __defer_stack.pop() {
            f();
        };
    });
    }
}

fn main() {
    let mut done = GoChannel::<String>::new();
    let mut r = Arc::new(Mutex::new(Some(Runner { name: Arc::new(Mutex::new(Some("ok".to_string()))), ..Default::default() })));
    (*r.lock().unwrap().as_mut().unwrap()).run(done.clone());
    println!("{}", done.recv().unwrap());
}