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

#[derive(Debug, Clone)]
pub struct Pair__string__int {
    pub key: Arc<Mutex<Option<String>>>,
    pub val: Arc<Mutex<Option<i32>>>,
}

impl Pair__string__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
            val: __go_clone_1_0,
        }
    }
}


impl Default for Pair__string__int {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            key: __go_default_0_0,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Pair__string__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Set__string(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<AnonymousStruct1>>>>>>>);


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


impl Pair__string__int {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{}={}", format!("{}", (*self.key.lock().unwrap().as_ref().unwrap())), format!("{}", (*self.val.lock().unwrap().as_ref().unwrap()))))))
    }
}

impl Set__string {
    pub fn add(&self, v: Arc<Mutex<Option<String>>>) {
        let mut __self = self.clone();
        { let __map_key = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(AnonymousStruct1 {  }))); (*__self.0.clone().lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    pub fn has(&self, v: Arc<Mutex<Option<String>>>) -> bool {
        let (_, mut ok) = { let __map = { let __map_holder = self.0.clone().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*v.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };
        ok
    }

    pub fn len(&self) -> i32 {
        return { let __map_holder = self.0.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32;
    }
}

pub fn keys__string__int(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut keys = Arc::new(Mutex::new(Some(Vec::<String>::with_capacity(((*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); let __append_value = k.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; keys = new_val; };
    }
    let keys_closure_clone = keys.clone(); { let __sort_target = keys_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>| -> bool {
        return {
            let __tmp_x = { let __seq = { let __seq_holder = keys_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = { let __seq = { let __seq_holder = keys_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            __tmp_x < __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i32))), Arc::new(Mutex::new(Some((__sort_j - 1) as i32)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    return keys.clone();
}

pub fn keys__int__bool(m: Arc<Mutex<Option<BTreeMap<i32, Arc<Mutex<Option<bool>>>>>>>) -> Arc<Mutex<Option<Vec<i32>>>> {
    let mut keys = Arc::new(Mutex::new(Some(Vec::<i32>::with_capacity(((*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); let __append_value = k; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; keys = new_val; };
    }
    let keys_closure_clone = keys.clone(); { let __sort_target = keys_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>| -> bool {
        return {
            let __tmp_x = { let __seq = { let __seq_holder = keys_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = { let __seq = { let __seq_holder = keys_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            __tmp_x < __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i32))), Arc::new(Mutex::new(Some((__sort_j - 1) as i32)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    return keys.clone();
}

pub fn max__int(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn max__float64(a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>) -> f64 {
    if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn max__string(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = (*a.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*b.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x > __tmp_y } {
        return { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    return { let __owned = b.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn filter__int(xs: Arc<Mutex<Option<Vec<i32>>>>, keep: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<i32>>>> {
    let mut out: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        if {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> = {
                let mut __f_guard = keep.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some(x)))
            )
        } {
        { let new_val = { let __append_target = out.clone(); let __append_value = x; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; out = new_val; };
    }
    } }
    return out.clone();
}

pub fn reduce__int__int(xs: Arc<Mutex<Option<Vec<i32>>>>, init: Arc<Mutex<Option<i32>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>>>>) -> i32 {
    let mut acc = { let __owned = init.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let new_val = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                acc.clone(),
                Arc::new(Mutex::new(Some(x)))
            )
        }; *acc.lock().unwrap() = Some(new_val); };
    } }
    return { let __v = (*acc.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn reduce__string__string(xs: Arc<Mutex<Option<Vec<String>>>>, init: Arc<Mutex<Option<String>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let mut acc = { let __owned = init.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        { let new_val = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some({ let __arg_holder = acc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(Some((*x).clone())))
            )
        }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *acc.lock().unwrap() = __moved_val; };
    } }
    return { let __owned = acc.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn map__int__string(xs: Arc<Mutex<Option<Vec<i32>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut out = Arc::new(Mutex::new(Some(vec!["".to_string(); ((*xs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().copied().enumerate() {
        { let __idx = (i) as usize; let __rhs = (*{
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some(x)))
            )
        }.lock().unwrap().as_ref().unwrap()).clone(); (*out.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    } }
    return out.clone();
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();
    let mut ages = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("bob".to_string(), Arc::new(Mutex::new(Some(30)))), ("alice".to_string(), Arc::new(Mutex::new(Some(25)))), ("carol".to_string(), Arc::new(Mutex::new(Some(35))))]))));
    let mut pairs: Arc<Mutex<Option<Vec<Pair__string__int>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = keys__string__int(ages.clone()).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for k in __range_values.iter() {
        { let new_val = { let __append_target = pairs.clone(); let __append_value = Pair__string__int { key: Arc::new(Mutex::new(Some((*k).clone()))), val: Arc::new(Mutex::new(Some({ let __map = { let __map_holder = ages.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(k)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }))), ..Default::default() }; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; pairs = new_val; };
    } }
    println!("{}", format!("{}", format_slice(&pairs)));
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = pairs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", (*pairs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    let mut s = Arc::new(Mutex::new(Some(Set__string(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<AnonymousStruct1>>>>::from([]))))))));
    { let __range_holder = Arc::new(Mutex::new(Some({ let __s = "a b a c b".to_string(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for w in __range_values.iter() {
        (*s.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some((*w).clone()))));
    } }
    println!("{} {} {}", format!("{}", (*s.lock().unwrap().as_ref().unwrap()).len()), format!("{}", (*s.lock().unwrap().as_ref().unwrap()).has(Arc::new(Mutex::new(Some("a".to_string()))))), format!("{}", (*s.lock().unwrap().as_ref().unwrap()).has(Arc::new(Mutex::new(Some("z".to_string()))))));
    println!("{} {} {}", format!("{}", max__int(Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(7))))), format!("{}", max__float64(Arc::new(Mutex::new(Some(2.5))), Arc::new(Mutex::new(Some(-1.0))))), format!("{}", (*max__string(Arc::new(Mutex::new(Some("go".to_string()))), Arc::new(Mutex::new(Some("rust".to_string())))).lock().unwrap().as_ref().unwrap())));
    let mut evens = filter__int(
        Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6]))),
        Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| -> bool {
        return { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>)))
    );
    println!("{} {}", format!("{}", format_slice(&evens)), format!("{}", reduce__int__int(
        evens.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, n: Arc<Mutex<Option<i32>>>| -> i32 {
        return { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>)))
    )));
    let mut words = map__int__string(
        Arc::new(Mutex::new(Some(vec![1, 22, 333]))),
        Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some({ let __s = "x".to_string(); let __count = (*Arc::new(Mutex::new(Some(format!("{}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).lock().unwrap().as_ref().unwrap()).len(); __s.repeat(__count as usize) })))
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)))
    );
    println!("{} {}", format!("{}", format_slice(&words)), format!("{}", (*reduce__string__string(
        words.clone(),
        Arc::new(Mutex::new(Some("".to_string()))),
        Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<String>>>, w: Arc<Mutex<Option<String>>>| -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s.push_str(&format!("{}", { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s.push_str(&format!("{}", "|".to_string()));
            __s
        })));
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)))
    ).lock().unwrap().as_ref().unwrap())));
    let mut ids = keys__int__bool(Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<bool>>>>::from([(3, Arc::new(Mutex::new(Some(true)))), (1, Arc::new(Mutex::new(Some(false)))), (2, Arc::new(Mutex::new(Some(true))))])))));
    println!("{}", format!("{}", format_slice(&ids)));
}