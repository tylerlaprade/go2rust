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

pub(crate) static trace: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *trace.lock().unwrap() = Some(String::new());
}


pub fn target(label: Arc<Mutex<Option<String>>>, values: Arc<Mutex<Option<Vec<i64>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    { (*trace.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", { let __v = (*label.lock().unwrap().as_ref().unwrap()).clone(); __v }, "|".to_string())); };
    return values.clone();
}

pub fn index(label: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<i64>>>) -> i64 {
    { (*trace.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", { let __v = (*label.lock().unwrap().as_ref().unwrap()).clone(); __v }, "|".to_string())); };
    return { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn result(label: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<i64>>>) -> i64 {
    { (*trace.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", { let __v = (*label.lock().unwrap().as_ref().unwrap()).clone(); __v }, "|".to_string())); };
    return { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

fn main() {
    __go_init_all();
    let mut done = GoChannel::<AnonymousStruct1>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(AnonymousStruct1 {  });;;
    });
    done.recv().unwrap_or_default();

    let mut left = Arc::new(Mutex::new(Some(vec![0i64, 0i64])));
    let mut right = Arc::new(Mutex::new(Some(vec![0i64, 0i64])));
    let mut __go_lhs_sequence_0 = Arc::new(Mutex::new(Some(vec![10i64])));
    let mut __go_lhs_index_0 = Arc::new(Mutex::new(Some(0i64)));
    {
        let __go_lhs_sequence_0_ = target(Arc::new(Mutex::new(Some("left sequence".to_string()))), left.clone()).clone();
        let __go_lhs_index_0_ = (index(Arc::new(Mutex::new(Some("left index".to_string()))), Arc::new(Mutex::new(Some(0))))) as usize;
        let __go_lhs_sequence_1 = target(Arc::new(Mutex::new(Some("right sequence".to_string()))), right.clone()).clone();
        let __go_lhs_index_1 = (index(Arc::new(Mutex::new(Some("right index".to_string()))), Arc::new(Mutex::new(Some(1))))) as usize;
        let __tmp_0 = result(
            Arc::new(Mutex::new(Some("left result".to_string()))),
            Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = __go_lhs_sequence_0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*__go_lhs_index_0.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })))
        );
        let __tmp_1 = result(Arc::new(Mutex::new(Some("right result".to_string()))), Arc::new(Mutex::new(Some(20))));
        (*__go_lhs_sequence_0_.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0_] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    println!("{}", format!("{}", { let __v = (*trace.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", format_slice(&left)), format!("{}", format_slice(&right)));
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
