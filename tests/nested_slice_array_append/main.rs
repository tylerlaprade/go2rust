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
trait GoNestedSliceRow {
    fn go_format_nested_row(&self) -> String;
}

impl<T: Display> GoNestedSliceRow for Vec<T> {
    fn go_format_nested_row(&self) -> String { format_slice_values(self.as_slice()) }
}

impl<T: Display, const N: usize> GoNestedSliceRow for [T; N] {
    fn go_format_nested_row(&self) -> String { format_slice_values(self.as_slice()) }
}

impl<T: Display> GoNestedSliceRow for Arc<Mutex<Option<Vec<T>>>> {
    fn go_format_nested_row(&self) -> String {
        let guard = self.lock().unwrap();
        match guard.as_ref() { Some(row) => format_slice_values(row.as_slice()), None => "[]".to_string() }
    }
}

fn format_nested_slice<C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: GoNestedSliceRow,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        format_nested_slice_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice_values<Inner>(slice: &[Inner]) -> String
where
    Inner: GoNestedSliceRow,
{
    let formatted: Vec<String> = slice.iter().map(|inner| inner.go_format_nested_row()).collect();
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
    pub elems: Arc<Mutex<Option<[Arc<Mutex<Option<Vec<String>>>>; 2]>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.elems.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            elems: __go_clone_0_0,
        }
    }
}


impl Default for Holder {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 2, _>(|_| Arc::new(Mutex::new(None))))));
        Self {
            elems: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_nested_slice(&self.elems));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn replacement() -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some("delta".to_string())))
}

fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut h = Arc::new(Mutex::new(Some(Holder { elems: Arc::new(Mutex::new(Some([Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None))]))), ..Default::default() })));
    { let __append_slice = { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __append_value = "alpha".to_string(); (*__append_slice.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_slice.clone() };
    { let __append_slice = { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __append_value = "beta".to_string(); (*__append_slice.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_slice.clone() };
    { let __append_slice = { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __append_value = "gamma".to_string(); (*__append_slice.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_slice.clone() };
    { let __outer_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = (*replacement().lock().unwrap().as_ref().unwrap()).clone(); let __outer_guard = __outer_holder.lock().unwrap(); let __nested_holder_0 = __outer_guard.as_ref().unwrap()[__outer_idx].clone(); drop(__outer_guard); let mut __seq_guard = __nested_holder_0.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __rhs; };

    println!("{} {} {}", format!("{}", (*{ let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    println!("{} {}", format!("{}", (*{ let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*h.lock().unwrap().as_ref().unwrap()).elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));
}