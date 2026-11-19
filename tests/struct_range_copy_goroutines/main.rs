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


/// The variable of a range loop is a copy of the element, also when the loop
/// assigns to a variable declared before it, and for the values of a map.
fn main() {
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut s = Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    let mut p: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut i: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (__range_i, __range_p) in __range_values.iter().enumerate() {
        { let new_val = __range_i as i32; *i.lock().unwrap() = Some(new_val); };
        { let new_val = (*__range_p).__go_value_clone(); *p.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 9; *{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    }
        println!("{} {}", format!("{}", "slice into a variable".to_string()), format!("{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));
        { let new_val = 99; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    } }
    println!("{} {} {} {}", format!("{}", "slice into a variable, afterwards".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut arr = Arc::new(Mutex::new(Some([Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    let mut q: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = arr.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for (__range_i, __range_q) in __range_values.iter().enumerate() {
        { let new_val = __range_i as i32; *i.lock().unwrap() = Some(new_val); };
        { let new_val = (*__range_q).__go_value_clone(); *q.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 9; *{ let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    }
        println!("{} {}", format!("{}", "array into a variable".to_string()), format!("{}", (*{ let __field = (*q.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));
        { let new_val = 99; *(*q.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    } }
    println!("{} {} {} {}", format!("{}", "array into a variable, afterwards".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*q.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut r: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for __range_r in __range_values.iter() {
        { let new_val = (*__range_r).__go_value_clone(); *r.lock().unwrap() = Some(new_val); };
        { let __target = (*r.lock().unwrap().as_ref().unwrap()).x.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } }
    println!("{} {} {} {}", format!("{}", "slice into a variable, no index".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    { let new_val = Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), ..Default::default() }]))); s = new_val; };
    { let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        let mut a = Arc::new(Mutex::new(Some((*e).__go_value_clone())));
        { let new_val = 100; *(*a.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
        let mut b: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = (*e).__go_value_clone(); *b.lock().unwrap() = Some(new_val); };
        { let new_val = 200; *(*b.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
        println!("{} {} {} {}", format!("{}", "from a range variable".to_string()), format!("{}", (*e.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*a.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    } }

    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Point>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }))))]))));
    for (_, v) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let v = Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().__go_value_clone())));
        { let new_val = 50; *(*v.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    }
    println!("{} {}", format!("{}", "range over a map".to_string()), format!("{}", (*{ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.lock().unwrap().as_ref().unwrap())));

    let mut w: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    for (_, __range_w) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = __range_w.lock().unwrap().as_ref().unwrap().__go_value_clone(); *w.lock().unwrap() = Some(new_val); };
        { let new_val = 60; *(*w.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    }
    println!("{} {} {}", format!("{}", "range over a map into a variable".to_string()), format!("{}", (*{ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*w.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut kept: Arc<Mutex<Option<Vec<Point>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        { let new_val = { let __append_target = kept.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*e).__go_value_clone()); __append_target.clone() }; kept = new_val; };
    } }
    { let new_val = 7; *{ let __seq = { let __seq_holder = kept.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "range variable appended".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = kept.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

    let mut byX = Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<Point>>>>::from([]))));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        { let __map_key = (*e.x.lock().unwrap().as_ref().unwrap()); let __map_value = Arc::new(Mutex::new(Some((*e).clone()))); (*byX.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    let mut got = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = byX.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) })));
    { let new_val = 70; *(*got.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "range variable stored in a map".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __map = { let __map_holder = byX.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*got.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut last: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        { let new_val = (*e).__go_value_clone(); *last.lock().unwrap() = Some(new_val); };
    } }
    { let new_val = 90; *(*last.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "range variable kept after the loop".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*last.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut ch = GoChannel::<Point>::new_buffered(2 as usize);
    ch.send({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.__go_value_clone());
    ch.send({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.__go_value_clone());
    ch.close();
    for c in ch.clone() {
        { let __target = c.x.clone(); let __rhs = 1000; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        println!("{} {}", format!("{}", "range over a channel".to_string()), format!("{}", (*c.x.lock().unwrap().as_ref().unwrap())));
    }
    println!("{} {} {}", format!("{}", "the slice afterwards".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));
}