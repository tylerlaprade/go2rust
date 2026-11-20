use std::any::Any;
use std::collections::BTreeMap;
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
    if val.is::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>() { return "map[string]int" }
    if val.is::<Arc<Mutex<Option<Vec<String>>>>>() { return "[]string" }
    if val.is::<Arc<Mutex<Option<Vec<i32>>>>>() { return "[]int" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone, Default)]
pub struct Numbers(pub Arc<Mutex<Option<Vec<i32>>>>);

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


fn main() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();
    let mut counts: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1))))])))).clone()) as Box<dyn Any + Send + Sync>)));
    let mut m = ({ let val = counts.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __map_key = "b".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_holder = m.clone(); let __key = "a".to_string(); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
    println!("{} {}", format!("{}", (*({ let val = counts.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __map = { let __map_holder = ({ let val = counts.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut c = ({ let val = counts.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __map_key = "x".to_string(); let __map_value = Arc::new(Mutex::new(Some(7))); (*c.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", { let __map = { let __map_holder = ({ let val = counts.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let counts_closure_clone = counts.clone(); { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let mut inner = ({ let val = counts_closure_clone.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
        { let __map_key = "y".to_string(); let __map_value = Arc::new(Mutex::new(Some(5))); (*inner.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{}", format!("{}", (*({ let val = counts.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    let mut list: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec![1, 2, 3]))).clone()) as Box<dyn Any + Send + Sync>)));
    let mut items = ({ let val = list.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __idx = (0) as usize; let __rhs = 10; (*items.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = ({ let val = list.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", (*items.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

        // A var of another slice type is a value of that type made from the
        // asserted slice.
    let mut typed: Arc<Mutex<Option<Numbers>>> = Arc::new(Mutex::new(Some(Numbers(Arc::new(Mutex::new(Some(({
        let val = list.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Arc<Mutex<Option<Vec<i32>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "[]int")).clone(); let __value = (*__handle.lock().unwrap()).clone().unwrap_or_default(); __value }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "[]int")
        }
    }))))))));
    let mut plain: Arc<Mutex<Option<Vec<i32>>>> = ({ let val = list.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __idx = (1) as usize; let __rhs = 20; (*plain.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", { let __slice_holder = { let __named_slice = (*typed.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __seq = { let __seq_holder = ({ let val = list.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));

        // Put into another interface value, the asserted slice or map is
        // still the same one.
    let mut again: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(({ let val = list.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone()) as Box<dyn Any + Send + Sync>)));
    let mut again2 = ({ let val = again.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __idx = (2) as usize; let __rhs = 30; (*again2.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{}", format!("{}", { let __seq = { let __seq_holder = ({ let val = list.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }));
    let mut reboxed = Arc::new(Mutex::new(Some(Box::new(({ let val = counts.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone()) as Box<dyn Any + Send + Sync>)));
    { let __map_key = "a".to_string(); let __map_value = Arc::new(Mutex::new(Some(50))); (*({ let val = reboxed.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", { let __map = { let __map_holder = ({ let val = counts.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut names: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec!["x".to_string()]))).clone()) as Box<dyn Any + Send + Sync>)));
    let mut ns = ({ let val = names.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __idx = (0) as usize; let __rhs = "z".to_string(); (*ns.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{}", format!("{}", format_slice(&({ let val = names.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }))));
}