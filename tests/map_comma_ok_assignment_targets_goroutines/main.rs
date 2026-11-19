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
pub struct Numbers(pub Arc<Mutex<Option<Vec<i32>>>>);

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Index(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>);

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct Holder {
    pub nums: Arc<Mutex<Option<Numbers>>>,
    pub plain: Arc<Mutex<Option<Vec<i32>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.nums.clone();
        let __go_clone_1_0 = self.plain.clone();
        Self {
            nums: __go_clone_0_0,
            plain: __go_clone_1_0,
        }
    }
}


impl Default for Holder {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            nums: __go_default_0_0,
            plain: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.nums.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.plain));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static global: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *global.lock().unwrap() = Some(vec![]);
}


/// "v, ok = m[k]" into an existing target must leave the value in the map,
/// whatever the target is: a named slice, a field, a package variable, an
/// element, or a variable captured by a closure.
fn main() {
    __go_init_all();
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Numbers>>>>::new())));
    let mut src = Arc::new(Mutex::new(Some(Numbers(Arc::new(Mutex::new(Some(vec![0; (2) as usize])))))));
    {
        let __tmp_0 = 1;
        let __tmp_1 = 2;
        { let __idx = (0) as usize; (*{ let __named_slice = (*src.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __tmp_0; }
        { let __idx = (1) as usize; (*{ let __named_slice = (*src.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __tmp_1; }
    };
    { let __map_key = "a".to_string(); let __map_value = src.clone(); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut numbers: Arc<Mutex<Option<Numbers>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *numbers.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    { let __idx = (0) as usize; let __rhs = 9; (*{ let __named_slice = (*numbers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "named slice".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __seq_holder = { let __named_slice = (*{ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }), format!("{}", { let __seq_holder = { let __named_slice = (*numbers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }));
    { let new_val = { let __base = { let __named_slice = (*numbers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(5); Arc::new(Mutex::new(Some(Numbers(Arc::new(Mutex::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *numbers.lock().unwrap() = __moved_val; };
    println!("{} {} {}", format!("{}", "named slice, appended to".to_string()), format!("{}", { let __slice_holder = { let __named_slice = (*{ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __slice_holder = { let __named_slice = (*numbers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));

    let mut h: Arc<Mutex<Option<Holder>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*h.lock().unwrap().as_ref().unwrap()).nums.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    { let __idx = (1) as usize; let __rhs = 8; (*{ let __named_slice = (*(*h.lock().unwrap().as_ref().unwrap()).nums.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "named slice field".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __seq_holder = { let __named_slice = (*{ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }), format!("{}", { let __seq_holder = { let __named_slice = (*(*h.lock().unwrap().as_ref().unwrap()).nums.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }));

    let mut lists = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i32>>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(vec![1, 2]))))]))));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *global.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {} {} {}", format!("{}", "global".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&{ let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", (*global.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __seq = { let __seq_holder = global.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = global.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));

    let mut dst = Arc::new(Mutex::new(Some(vec![vec![9]])));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; { let __idx = (0) as usize; (*dst.lock().unwrap().as_mut().unwrap())[__idx] = __tmp_0.lock().unwrap().take().unwrap_or_default(); } *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {}", format!("{}", "indexed".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&{ let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice_values(&{ let __seq = { let __seq_holder = dst.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));

    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; (*h.lock().unwrap().as_mut().unwrap()).plain = __tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    { let __idx = (0) as usize; let __rhs = 70; (*(*h.lock().unwrap().as_ref().unwrap()).plain.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {} {}", format!("{}", "plain slice field".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&{ let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice_values(&(*{ let __field = (*h.lock().unwrap().as_ref().unwrap()).plain.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())));

    let mut captured: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    let mut captured_closure_clone = captured.clone(); let lists_closure_clone = lists.clone(); let mut set = Arc::new(Mutex::new(Some(Box::new(move || -> bool {
        let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = lists_closure_clone.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *captured_closure_clone.lock().unwrap() = __moved_tmp_0; *found.lock().unwrap() = Some(__tmp_1); };
        return { let __v = (*found.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Box<dyn FnMut() -> bool + Send + Sync>)));
    println!("{} {} {} {}", format!("{}", "captured by a closure".to_string()), format!("{}", {
        let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = {
            let mut __f_guard = set.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }), format!("{}", format_slice(&{ let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&captured)));

    let mut capturedNamed: Arc<Mutex<Option<Numbers>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut capturedNamed_closure_clone = capturedNamed.clone(); let m_closure_clone = m.clone(); let mut setNamed = Arc::new(Mutex::new(Some(Box::new(move || -> bool {
        let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = m_closure_clone.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *capturedNamed_closure_clone.lock().unwrap() = __moved_tmp_0; *found.lock().unwrap() = Some(__tmp_1); };
        return { let __v = (*found.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Box<dyn FnMut() -> bool + Send + Sync>)));
    println!("{} {} {} {}", format!("{}", "named slice captured by a closure".to_string()), format!("{}", {
        let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = {
            let mut __f_guard = setNamed.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }), format!("{}", { let __slice_holder = { let __named_slice = (*{ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __slice_holder = { let __named_slice = (*capturedNamed.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));

    let mut indexes = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Index>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(Index(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(1))))])))))))))]))));
    let mut idx: Arc<Mutex<Option<Index>>> = Arc::new(Mutex::new(Some(Index(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::new())))))));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = indexes.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *idx.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    { let __map_key = "x".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*{ let __named_map = (*idx.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {} {}", format!("{}", "named map".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map = { let __map_holder = { let __named_map = (*{ let __map = { let __map_holder = indexes.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map = { let __map_holder = { let __named_map = (*idx.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
