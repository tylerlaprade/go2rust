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
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
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

    // A select case's receive: Some(Some(v)) for a value, Some(None) when
    // the channel is closed and drained, None when nothing is ready yet.
    fn try_recv_select(&self) -> Option<Option<T>> {
        if self.is_nil() {
            return None;
        }
        match self.rx.lock().unwrap().try_recv() {
            Ok(value) => {
                if self.capacity > 0 {
                    let _ = self.len.fetch_update(
                        std::sync::atomic::Ordering::SeqCst,
                        std::sync::atomic::Ordering::SeqCst,
                        |__go_current| __go_current.checked_sub(1),
                    );
                }
                Some(Some(value))
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(None),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
        }
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

pub mod math {
    use super::*;
    pub const MAX_INT32: i64 = 2147483647;
    pub const MAX_INT64: i64 = 9223372036854775807;
}


pub fn fnv32(s: Arc<Mutex<Option<String>>>) -> u32 {
    let mut h = Arc::new(Mutex::new(Some(2166136261 as u32)));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i64); __tmp_x < __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as u32))).lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
        { let __rhs = 16777619 as u32; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();
    let mut x: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(math::MAX_INT32 as i32)));
    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    let mut y: Arc<Mutex<Option<i8>>> = Arc::new(Mutex::new(Some(-128 as i8)));
    { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    let mut u: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(250)));
    { let __rhs = 10 as u8; let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    let mut z: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(math::MAX_INT64 as i64)));
    { let __rhs = 2 as i64; let mut guard = z.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    println!("{} {} {} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", fnv32(Arc::new(Mutex::new(Some("hello".to_string()))))), format!("{}", fnv32(Arc::new(Mutex::new(Some("go2rust".to_string()))))));
    let mut w: Arc<Mutex<Option<u16>>> = Arc::new(Mutex::new(Some(0)));
    { let __rhs = 1 as u16; let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    println!("{}", format!("{}", { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}