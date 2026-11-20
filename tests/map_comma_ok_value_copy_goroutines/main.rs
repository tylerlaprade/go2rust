use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
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


#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub by_name: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Point>>>>>>>,
    pub counts: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>,
}

impl Registry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.by_name.clone();
        let __go_clone_1_0 = self.counts.clone();
        Self {
            by_name: __go_clone_0_0,
            counts: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.by_name));
        let __go_fmt_1 = format!("{}", format_map(&self.counts));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Scores(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>);

impl Display for Scores {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


/// "v, ok := m[k]" gives v its own copy of the value. Writing to v afterwards
/// must not change the map, and reading into existing variables must not take
/// the value out of the map.
fn main() {
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut counts = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1))))]))));
    let (mut n, mut ok) = { let __map = { let __map_holder = counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
    { let new_val = 5; *n.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "int".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map = { let __map_holder = counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut names = Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<String>>>>::from([(1, Arc::new(Mutex::new(Some("one".to_string()))))]))));
    let (mut name, mut ok) = { let __map = { let __map_holder = names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&1)) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(String::new()))), false) } };
    { (*name.lock().unwrap().as_mut().unwrap()).push_str(&"!".to_string()); };
    println!("{} {} {} {}", format!("{}", "string".to_string()), format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map = { let __map_holder = names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));

    {
        let (mut v, mut ok) = { let __map = { let __map_holder = counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            { let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
            println!("{} {} {}", format!("{}", "in an if".to_string()), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map = { let __map_holder = counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));;
        }
    }

    let mut m: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *m.lock().unwrap() = __moved_tmp_0; *found.lock().unwrap() = Some(__tmp_1); };
    { let __rhs = 10; let mut guard = m.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {} {} {}", format!("{}", "int into variables".to_string()), format!("{}", { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*found.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map = { let __map_holder = counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", (*counts.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&1)) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(String::new()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_tmp_0; ok = __tmp_1; };
    println!("{} {} {} {}", format!("{}", "string into variables".to_string()), format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", { let __map = { let __map_holder = names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&2)) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(String::new()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_tmp_0; ok = __tmp_1; };
    println!("{} {} {} {}", format!("{}", "missing key into variables".to_string()), format!("{}", { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y }), format!("{}", ok), format!("{}", { let __map = { let __map_holder = names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));

    let mut points = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Point>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }))))]))));
    let (mut p, mut ok) = { let __map = { let __map_holder = points.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().__go_value_clone()))), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };
    { let new_val = 9; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct".to_string()), format!("{}", ok), format!("{}", (*{ let __map = { let __map_holder = points.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut q: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = points.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().__go_value_clone()))), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_tmp_0; *found.lock().unwrap() = Some(__tmp_1); };
    { let new_val = 8; *(*q.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct into variables".to_string()), format!("{}", { let __v = (*found.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __map = { let __map_holder = points.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*q.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    {
        let (mut r, mut ok) = { let __map = { let __map_holder = points.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().__go_value_clone()))), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };;
        if ok {
            { let new_val = 7; *(*r.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };;
            println!("{} {} {}", format!("{}", "struct in an if".to_string()), format!("{}", (*{ let __map = { let __map_holder = points.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));;
        }
    }

    let (mut missing, mut ok) = { let __map = { let __map_holder = points.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"zzz".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().__go_value_clone()))), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };
    { let new_val = 5; *(*missing.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "struct, missing key".to_string()), format!("{}", ok), format!("{}", (*{ let __field = (*missing.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*points.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    let mut reg = Arc::new(Mutex::new(Some(Registry { by_name: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Point>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }))))])))), counts: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1))))])))), ..Default::default() })));
    let (mut rp, mut ok) = { let __map = { let __map_holder = (*reg.lock().unwrap().as_ref().unwrap()).by_name.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().__go_value_clone()))), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };
    { let new_val = 7; *(*rp.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    let (mut c, mut ok2) = { let __map = { let __map_holder = (*reg.lock().unwrap().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
    { let mut guard = c.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{} {} {} {} {} {} {}", format!("{}", "map in a field".to_string()), format!("{}", ok), format!("{}", ok2), format!("{}", (*{ let __map = { let __map_holder = (*reg.lock().unwrap().as_ref().unwrap()).by_name.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*rp.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", { let __map = { let __map_holder = (*reg.lock().unwrap().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut c2: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = (*reg.lock().unwrap().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *c2.lock().unwrap() = __moved_tmp_0; ok = __tmp_1; };
    { let __rhs = 5; let mut guard = c2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {} {}", format!("{}", "map in a field into variables".to_string()), format!("{}", ok), format!("{}", { let __map = { let __map_holder = (*reg.lock().unwrap().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*c2.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut sc = Arc::new(Mutex::new(Some(Scores(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1))))]))))))));
    let (mut v, mut ok) = { let __map = { let __map_holder = { let __named_map = (*sc.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
    { let new_val = 3; *v.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "named map type".to_string()), format!("{}", ok), format!("{}", { let __map = { let __map_holder = { let __named_map = (*sc.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let (_, mut has) = { let __map = { let __map_holder = { let __named_map = (*sc.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"zzz".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
    let (mut z, _) = { let __map = { let __map_holder = { let __named_map = (*sc.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
    { let new_val = 4; *z.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "blank results".to_string()), format!("{}", has), format!("{}", { let __map = { let __map_holder = { let __named_map = (*sc.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut nested = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(1))))])))))]))));
    let (mut x, mut ok) = { let __map = { let __map_holder = { let __map = { let __map_holder = nested.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"x".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
    { let new_val = 99; *x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "nested lookup".to_string()), format!("{}", ok), format!("{}", { let __map = { let __map_holder = { let __map = { let __map_holder = nested.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }));

        // A pointer, a slice and a map are shared with the map entry, as in Go.
    let mut ptrs = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Point>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }))).clone())]))));
    let (mut pp, mut ok) = { let __map = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    { let new_val = 9; *(*pp.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "pointer value".to_string()), format!("{}", ok), format!("{}", (*(*{ let __map = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*pp.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut pq: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; pq = __tmp_0.clone(); ok = __tmp_1; };
    { let new_val = 10; *(*pq.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "pointer value into variables".to_string()), format!("{}", ok), format!("{}", (*(*{ let __map = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*pq.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut lists = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i32>>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(vec![1, 2]))))]))));
    let (mut l, mut ok) = { let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    { let __idx = (0) as usize; let __rhs = 50; (*l.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "slice value".to_string()), format!("{}", ok), format!("{}", format_slice(&{ let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&l)));

    let mut l2: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; l2 = __tmp_0; ok = __tmp_1; };
    { let __idx = (1) as usize; let __rhs = 60; (*l2.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "slice value into variables".to_string()), format!("{}", ok), format!("{}", format_slice(&{ let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&l2)));

    let (mut inner, mut ok) = { let __map = { let __map_holder = nested.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
    { let __map_key = "x".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*inner.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {} {}", format!("{}", "map value".to_string()), format!("{}", ok), format!("{}", { let __map = { let __map_holder = { let __map = { let __map_holder = nested.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map = { let __map_holder = inner.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut inner2: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = nested.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; inner2 = __tmp_0; ok = __tmp_1; };
    { let __map_key = "x".to_string(); let __map_value = Arc::new(Mutex::new(Some(3))); (*inner2.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {} {}", format!("{}", "map value into variables".to_string()), format!("{}", ok), format!("{}", { let __map = { let __map_holder = { let __map = { let __map_holder = nested.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map = { let __map_holder = inner2.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
}