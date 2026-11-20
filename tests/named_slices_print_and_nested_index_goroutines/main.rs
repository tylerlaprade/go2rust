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
        format_nested_slice_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice_values<T, Inner>(slice: &[Inner]) -> String
where
    Inner: AsRef<[T]>,
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|inner| format_slice_values(inner.as_ref())).collect();
    format!("[{}]", formatted.join(" "))
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

#[derive(Debug, Clone, Default)]
pub struct Matrix(pub Arc<Mutex<Option<Vec<Vec<f64>>>>>);


#[derive(Debug, Clone, Default)]
pub struct Ints(pub Arc<Mutex<Option<Vec<i32>>>>);

impl Display for Ints {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Words(pub Arc<Mutex<Option<Vec<String>>>>);

impl Display for Words {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


impl Matrix {
    pub fn transpose(&self) -> Arc<Mutex<Option<Matrix>>> {
        let mut t = Arc::new(Mutex::new(Some(Matrix(Arc::new(Mutex::new(Some(vec![vec![]; ({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.len()) as usize])))))));
        for i in 0..(({ let __range_holder = { let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (*Arc::new(Mutex::new(Some(vec![0.0; ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize]))).lock().unwrap().as_ref().unwrap()).clone(); (*{ let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        for j in 0..(({ let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __seq = { let __named_slice = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __outer_idx = (i) as usize; let __idx = (j) as usize; let __rhs = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(j) as usize].clone() }[(i) as usize].clone(); (*__seq.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
    }
        return t.clone();
    }
}

pub fn mk() -> Arc<Mutex<Option<Ints>>> {
    Arc::new(Mutex::new(Some(Ints(Arc::new(Mutex::new(Some(vec![1, 2])))))))
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();

    let mut m = Arc::new(Mutex::new(Some(Matrix(Arc::new(Mutex::new(Some(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]])))))));
    let mut xs = Arc::new(Mutex::new(Some(Ints(Arc::new(Mutex::new(Some(vec![5, 6])))))));
    let mut ws = Arc::new(Mutex::new(Some(Words(Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string()])))))));
    println!("{} {} {}", format!("{}", format_nested_slice(&{ let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_slice(&{ let __named_slice = (*xs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_slice(&{ let __named_slice = (*ws.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })));
    println!("{} {}", format!("{}", format_nested_slice(&{ let __named_slice = (*(*m.lock().unwrap().as_ref().unwrap()).transpose().lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_slice(&{ let __named_slice = (*mk().lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })));
    print!("{} {}\n", format!("{}", format_nested_slice(&{ let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_slice(&{ let __named_slice = (*mk().lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })));
    { let __seq = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __outer_idx = (1) as usize; let __idx = (2) as usize; let __rhs = 9.0; (*__seq.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    let mut grid = Arc::new(Mutex::new(Some(Matrix(Arc::new(Mutex::new(Some(vec![vec![]; (2) as usize])))))));
    for i in 0..(({ let __range_holder = { let __named_slice = (*grid.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (*Arc::new(Mutex::new(Some(vec![0.0; (2) as usize]))).lock().unwrap().as_ref().unwrap()).clone(); (*{ let __named_slice = (*grid.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        { let __seq = { let __named_slice = (*grid.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __outer_idx = (i) as usize; let __idx = (i) as usize; let __rhs = 1.0; (*__seq.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
    println!("{} {} {}", format!("{}", format_nested_slice(&{ let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_nested_slice(&{ let __named_slice = (*grid.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", { let __slice_holder = { let __named_slice = (*(*m.lock().unwrap().as_ref().unwrap()).transpose().lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
}