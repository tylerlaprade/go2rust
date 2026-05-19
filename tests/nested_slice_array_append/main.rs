use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| format_slice_values(inner.as_ref()))
            .collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

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
pub struct Holder {
    pub elems: Arc<Mutex<Option<[Vec<String>; 2]>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { elems: { let __guard = self.elems.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Holder {
    fn default() -> Self {
        Self { elems: Arc::new(Mutex::new(Some(std::array::from_fn(|_| vec![])))) }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_nested_slice(&self.elems))
    }
}


pub fn replacement() -> Arc<Mutex<Option<String>>> {

    return Arc::new(Mutex::new(Some("delta".to_string())));
}

fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap();

    let mut h = Arc::new(Mutex::new(Some(Holder { elems: Arc::new(Mutex::new(Some([vec![], vec![]]))), ..Default::default() })));
    { (*(*h.lock().unwrap().as_ref().unwrap()).elems.lock().unwrap().as_mut().unwrap())[(0) as usize].push("alpha".to_string()); (*(*h.lock().unwrap().as_ref().unwrap()).elems.lock().unwrap().as_ref().unwrap())[(0) as usize].clone() };
    { (*(*h.lock().unwrap().as_ref().unwrap()).elems.lock().unwrap().as_mut().unwrap())[(0) as usize].push("beta".to_string()); (*(*h.lock().unwrap().as_ref().unwrap()).elems.lock().unwrap().as_ref().unwrap())[(0) as usize].clone() };
    { (*(*h.lock().unwrap().as_ref().unwrap()).elems.lock().unwrap().as_mut().unwrap())[(1) as usize].push("gamma".to_string()); (*(*h.lock().unwrap().as_ref().unwrap()).elems.lock().unwrap().as_ref().unwrap())[(1) as usize].clone() };
    (*(*h.lock().unwrap().as_ref().unwrap()).elems.lock().unwrap().as_mut().unwrap())[(0) as usize][(1) as usize] = (*replacement().lock().unwrap().as_ref().unwrap()).clone();

    println!("{} {} {}", format!("{}", { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.len()), format!("{}", { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }[(0) as usize].clone()), format!("{}", { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }[(1) as usize].clone()));
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.len()), format!("{}", { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }[(0) as usize].clone()));
}