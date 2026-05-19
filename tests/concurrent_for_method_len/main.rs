use std::fmt::{Display, Formatter};
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

#[derive(Debug, Clone)]
pub struct Params {
    pub n: Arc<Mutex<Option<i32>>>,
}

impl Params {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Params {
    fn default() -> Self {
        Self { n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}


impl Params {
    pub fn len(&self) -> Arc<Mutex<Option<i32>>> {
        return self.n.clone();
    }

    pub fn at(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = 1;
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }
}

pub fn total(p: Arc<Mutex<Option<Params>>>) -> Arc<Mutex<Option<i32>>> {

    let mut sum = Arc::new(Mutex::new(Some(0)));
    for mut i in 0..({ let __v = (*p.lock().unwrap().as_mut().unwrap()).len(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) {
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x == __tmp_y } {
        {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*p.lock().unwrap().as_mut().unwrap()).len().lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*(*p.lock().unwrap().as_mut().unwrap()).at(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone())))).lock().unwrap().as_ref().unwrap())); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + i); };
    }
    }
    return sum.clone();
}

fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap();

    let mut p = Arc::new(Mutex::new(Some(Params { n: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    println!("{}", format!("{}", (*total(p.clone()).lock().unwrap().as_ref().unwrap())));
}