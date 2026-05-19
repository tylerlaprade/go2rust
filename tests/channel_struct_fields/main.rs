use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


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

#[derive(Debug, Clone, Default)]
pub struct Holder {
    pub ch: GoChannel<i32>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { ch: self.ch.clone() }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Holder {
    pub fn ready(&self) -> Arc<Mutex<Option<bool>>> {
        return Arc::new(Mutex::new(Some(!self.ch.is_nil())));
    }

    pub fn fill(&mut self) {
        self.ch = GoChannel::<i32>::new_buffered(2 as usize);
        self.ch.send(1);
        println!("{} {} {}", format!("{}", !self.ch.is_nil()), format!("{}", self.ch.len()), format!("{}", self.ch.capacity()));
        println!("{}", format!("{}", self.ch.recv().unwrap()));
        self.ch = Default::default();
        println!("{}", format!("{}", self.ch.is_nil()));
    }
}

fn main() {
    let mut h = Arc::new(Mutex::new(Some(Holder { ch: Default::default() })));
    println!("{}", format!("{}", (*(*h.lock().unwrap().as_mut().unwrap()).ready().lock().unwrap().as_ref().unwrap())));
    (*h.lock().unwrap().as_mut().unwrap()).fill();

    let mut h2 = Arc::new(Mutex::new(Some(Holder { ch: GoChannel::<i32>::new_buffered(1 as usize), ..Default::default() })));
    println!("{}", format!("{}", (*(*h2.lock().unwrap().as_mut().unwrap()).ready().lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*h2.lock().unwrap().as_ref().unwrap()).ch.len()));
    println!("{}", format!("{}", (*h2.lock().unwrap().as_ref().unwrap()).ch.capacity()));
    (*h2.lock().unwrap().as_ref().unwrap()).ch.send(7);
    println!("{}", format!("{}", (*h2.lock().unwrap().as_ref().unwrap()).ch.recv().unwrap()));

    let mut h3 = Arc::new(Mutex::new(Some(Holder { ch: Default::default(), ..Default::default() })));
    println!("{}", format!("{}", (*(*h3.lock().unwrap().as_mut().unwrap()).ready().lock().unwrap().as_ref().unwrap())));
}