use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
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

#[derive(Clone, Default)]
pub struct Result_ {
    pub err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>,
    pub next: Arc<Mutex<Option<Result_>>>,
    pub tags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>,
}

impl Result_ {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        let __go_clone_1_0 = self.next.clone();
        let __go_clone_2_0 = self.tags.clone();
        Self {
            err: __go_clone_0_0,
            next: __go_clone_1_0,
            tags: __go_clone_2_0,
        }
    }
}

impl std::fmt::Display for Result_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.lock().unwrap().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", format_map(&self.tags));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct T {
    pub x: Arc<Mutex<Option<i32>>>,
}

impl T {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for T {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub(crate) static lastErr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *lastErr.lock().unwrap() = None;
    *cache.lock().unwrap() = Some(BTreeMap::new());
}


impl Result_ {
    pub fn check(&self) {
        let _ = &self.err;
        let _ = &self.next;
    }

    pub fn check_pointer(&self) {
        let _ = &self.err;
        let _ = &self.tags;
    }
}

pub fn find() -> (Arc<Mutex<Option<T>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
}

fn main() {
    __go_init_all();
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let _ = &err;
    let mut ptr: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));
    let _ = &ptr;
    let mut m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));
    let _ = &m;
    let mut s: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    let _ = &s;
    let mut a: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let _ = &a;
    let (mut q, mut e2) = find();
    let _ = &q;
    let _ = &e2;
    let mut f: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let _ = &f;

    let _ = &lastErr;
    let _ = &cache;
    let mut r: Arc<Mutex<Option<Result_>>> = Arc::new(Mutex::new(Some(Default::default())));
    let _ = &(*r.lock().unwrap().as_ref().unwrap()).err;
    let _ = &(*r.lock().unwrap().as_ref().unwrap()).next;
    let _ = &(*r.lock().unwrap().as_ref().unwrap()).tags;
    let _ = &((*r.lock().unwrap().as_ref().unwrap()).err);
    (*r.lock().unwrap().as_ref().unwrap()).check();
    let mut p = Arc::new(Mutex::new(Some(Result_ { err: Default::default(), next: Default::default(), tags: Default::default() })));
    let _ = &(*p.lock().unwrap().as_ref().unwrap()).err;
    let _ = &(*p.lock().unwrap().as_ref().unwrap()).next;
    { let __recv = p.clone(); let __recv_ptr: *const Result_ = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Result_ }; let __result = unsafe { &*__recv_ptr }.check_pointer(); __result };
    let mut inner = Arc::new(Mutex::new(Some(Result_ {
        next: Arc::new(Mutex::new(Some(Result_ { err: Default::default(), next: Default::default(), tags: Default::default() }))).clone(),
        ..Default::default()
    })));
    let _ = &(*(*inner.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).err;

    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("kept".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *lastErr.lock().unwrap() = new_val; };
    let _ = &lastErr;
    println!("{} {} {}", format!("{}", { let __text = match lastErr.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_target = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __nil_target = (*p.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }));

    let mut count = Arc::new(Mutex::new(Some(0)));
    let mut count_closure_clone = count.clone(); let mut bump = Arc::new(Mutex::new(Some(Box::new(move || -> i32 {
        { let mut guard = count_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return { let __v = (*count_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Box<dyn FnMut() -> i32 + Send + Sync>)));
    let _ = {
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = bump.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    let _ = { let __seq = { let __seq_holder = Arc::new(Mutex::new(Some(vec![{
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = bump.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }]))).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
    println!("{}", format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
