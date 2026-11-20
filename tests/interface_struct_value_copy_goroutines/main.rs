use std::any::Any;
use std::error::Error as StdError;
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

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i32>() { return "int" }
    if val.is::<i64>() { return "int64" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u32>() { return "uint" }
    if val.is::<u64>() { return "uint64" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i32>>() { return "[]int" }
    if val.is::<Vec<i64>>() { return "[]int64" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Record>() { return "main.Record" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct Record {
    pub name: Arc<Mutex<Option<String>>>,
}

impl Record {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Record {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


impl Record {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "record ".to_string(), (*self.name.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for Record {}


pub fn run() {
    let mut value = Arc::new(Mutex::new(Some(Record { name: Arc::new(Mutex::new(Some("before".to_string()))), ..Default::default() })));
    let mut direct: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync>)));
    let mut err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new((*value.lock().unwrap().as_ref().unwrap()).__go_value_clone()) as Arc<dyn StdError + Send + Sync>)));

    { let new_val = "after".to_string(); *(*value.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", (*{ let __field = (*value.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*({
        let val = direct.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Record>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Record")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Record")
        }
    }).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));

    let mut asserted = Arc::new(Mutex::new(Some(({
        let val = direct.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Record>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Record")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Record")
        }
    }))));
    { let new_val = "asserted".to_string(); *(*asserted.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", (*{ let __field = (*value.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*({
        let val = direct.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Record>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Record")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Record")
        }
    }).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*{ let __field = (*asserted.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));

    let (mut comma, mut ok) = ({
        let val = direct.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Record>() {
                (Arc::new(Mutex::new(Some(typed_val.__go_value_clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(Default::default()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(Default::default()))), false)
        }
    });
    { let new_val = "comma".to_string(); *(*comma.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {} {}", format!("{}", (*{ let __field = (*value.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*({
        let val = direct.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Record>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Record")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Record")
        }
    }).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*{ let __field = (*comma.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", ok), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));
}

fn main() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        run();;
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();
}