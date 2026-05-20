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

    fn recv(&self) -> Option<T>
    where
        T: Default,
    {
        if self.is_nil() {
            return None;
        }
        let value = match self.rx.lock().unwrap().recv() {
            Ok(value) => Some(value),
            Err(_) => Some(T::default()),
        };
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
}

#[derive(Debug, Clone)]
pub struct source {
    pub dir: Arc<Mutex<Option<String>>>,
    pub a: Arc<Mutex<Option<Vec<String>>>>,
    pub b: Arc<Mutex<Option<Vec<String>>>>,
}

impl source {
    pub fn __go_value_clone(&self) -> Self {
        Self { dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, a: self.a.clone(), b: self.b.clone() }
    }
}


impl Default for source {
    fn default() -> Self {
        Self { dir: Arc::new(Mutex::new(Some(String::new()))), a: Arc::new(Mutex::new(None)), b: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.dir.lock().unwrap().as_ref().unwrap()), format_slice(&self.a), format_slice(&self.b))
    }
}


#[derive(Debug, Clone, Default)]
pub struct result {
    pub first: Arc<Mutex<Option<Vec<String>>>>,
    pub second: Arc<Mutex<Option<Vec<String>>>>,
}

impl result {
    pub fn __go_value_clone(&self) -> Self {
        Self { first: self.first.clone(), second: self.second.clone() }
    }
}

impl std::fmt::Display for result {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice(&self.first), format_slice(&self.second))
    }
}


pub fn join(dir: Arc<Mutex<Option<String>>>, groups: Arc<Mutex<Option<Vec<Vec<String>>>>>) -> Arc<Mutex<Option<Vec<String>>>> {

    let mut out: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = groups.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for group in __range_values.iter() {
        for name in group.iter() {
        { let new_val = { let __append_target = out.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(format!("{}{}", format!("{}{}", { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v }, "/".to_string()), name)); __append_target.clone() }; out = new_val; };
    }
    } }
    return out.clone();
}

fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        let mut src = Arc::new(Mutex::new(Some(source { dir: Arc::new(Mutex::new(Some("root".to_string()))), a: Arc::new(Mutex::new(Some(vec!["a".to_string()]))), b: Arc::new(Mutex::new(Some(vec!["b".to_string()]))), ..Default::default() })));;
        let mut res = Arc::new(Mutex::new(Some(result { first: join({ let __field = (*src.lock().unwrap().as_ref().unwrap()).dir.clone(); __field }, Arc::new(Mutex::new(Some(vec![{ let __slice_holder = (*src.lock().unwrap().as_ref().unwrap()).a.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }])))), second: join({ let __field = (*src.lock().unwrap().as_ref().unwrap()).dir.clone(); __field }, Arc::new(Mutex::new(Some(vec![{ let __slice_holder = (*src.lock().unwrap().as_ref().unwrap()).b.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }])))), ..Default::default() })));;
        println!("{}", format!("{}", { let __seq = { let __seq_holder = (*res.lock().unwrap().as_ref().unwrap()).first.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));;
        println!("{}", format!("{}", { let __seq = { let __seq_holder = (*res.lock().unwrap().as_ref().unwrap()).second.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));;
        done_thread.send(true);;;
    });
    done.recv().unwrap();
}