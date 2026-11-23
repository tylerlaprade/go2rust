use std::collections::BTreeMap;
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
pub struct Point {
    pub x: Arc<Mutex<Option<i32>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Point {
    pub fn inc(&mut self) {
        { let __target = self.x.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut numbers = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1))))]))));
    for (_, mut v) in { let __range_holder = numbers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let v = Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone())));
        { let new_val = 5; *v.lock().unwrap() = Some(new_val); };
        println!("{} {} {}", format!("{}", "int".to_string()), format!("{}", (*v.lock().unwrap().as_mut().unwrap())), format!("{}", { let __map = { let __map_holder = numbers.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    }
    let mut sum = Arc::new(Mutex::new(Some(0)));
    let mut max = Arc::new(Mutex::new(Some(0)));
    for (_, v) in { let __range_holder = numbers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let v = Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone())));
        { let __rhs = v.lock().unwrap().as_ref().unwrap().clone(); let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = (*v.lock().unwrap().as_mut().unwrap()); let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *max.lock().unwrap() = Some(new_val); };
    }
    }
    println!("{} {} {}", format!("{}", "aggregate".to_string()), format!("{}", { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut weights = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<f64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1.5)))), ("b".to_string(), Arc::new(Mutex::new(Some(2.25))))]))));
    let mut totalWeight = Arc::new(Mutex::new(Some(0.0)));
    for (_, weight) in { let __range_holder = weights.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let weight = Arc::new(Mutex::new(Some(weight.lock().unwrap().as_ref().unwrap().clone())));
        { let __rhs = weight.lock().unwrap().as_ref().unwrap().clone(); let mut guard = totalWeight.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    println!("{} {}", format!("{}", "weight".to_string()), format!("{}", { let __v = (*totalWeight.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut words = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some("one".to_string()))))]))));
    for (_, mut word) in { let __range_holder = words.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let word = Arc::new(Mutex::new(Some(word.lock().unwrap().as_ref().unwrap().clone())));
        { (*word.lock().unwrap().as_mut().unwrap()).push_str(&"!".to_string()); };
        println!("{} {} {}", format!("{}", "string".to_string()), format!("{}", (*word.lock().unwrap().as_mut().unwrap())), format!("{}", { let __map = { let __map_holder = words.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
    }

    let mut points = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Point>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }))))]))));
    for (_, p) in { let __range_holder = points.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let p = Arc::new(Mutex::new(Some(p.lock().unwrap().as_ref().unwrap().__go_value_clone())));
        (*p.lock().unwrap().as_mut().unwrap()).inc();
        println!("{} {} {}", format!("{}", "struct".to_string()), format!("{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __map = { let __map_holder = points.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.lock().unwrap().as_ref().unwrap())));
    }

    let mut lists = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i32>>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(vec![1]))))]))));
    for (_, list) in { let __range_holder = lists.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __idx = (0) as usize; let __rhs = 2; (*list.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    println!("{} {}", format!("{}", "slice".to_string()), format!("{}", { let __seq = { let __seq_holder = { let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));

    let mut n = Arc::new(Mutex::new(Some(3)));
    let mut ptrs = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("a".to_string(), n.clone().clone())]))));
    for (_, p) in { let __range_holder = ptrs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = 4; *p.lock().unwrap() = Some(new_val); };
    }
    println!("{} {}", format!("{}", "pointer".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut nilPtrs = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("a".to_string(), Arc::new(Mutex::new(None)))]))));
    for (_, p) in { let __range_holder = nilPtrs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        println!("{} {}", format!("{}", "nil-pointer".to_string()), format!("{}", { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result }));
    }
}