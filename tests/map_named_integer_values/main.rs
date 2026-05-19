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
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
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
pub struct Index(pub Arc<Mutex<Option<i32>>>);

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.0.lock().unwrap().as_ref().unwrap() == other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Index {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Index {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Index> for i32 {
    fn eq(&self, other: &Index) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Index> for i32 {
    fn partial_cmp(&self, other: &Index) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Index {
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for Index {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Index> for i32 {
    type Output = i32;
    fn add(self, other: Index) -> i32 {
        self + *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Sub for Index {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for Index {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Index> for i32 {
    type Output = i32;
    fn sub(self, other: Index) -> i32 {
        self - *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Index {
    type Output = Index;
    fn bitand(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Index {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Index> for i32 {
    type Output = i32;
    fn bitand(self, other: Index) -> i32 {
        self & *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Index {
    type Output = Index;
    fn bitor(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Index {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Index> for i32 {
    type Output = i32;
    fn bitor(self, other: Index) -> i32 {
        self | *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Index {
    type Output = Index;
    fn bitxor(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Index {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Index> for i32 {
    type Output = i32;
    fn bitxor(self, other: Index) -> i32 {
        self ^ *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl Eq for Index {}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Holder {
    pub values: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Index>>>>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { values: self.values.clone() }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_map(&self.values))
    }
}


fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap();

    let mut h = Arc::new(Mutex::new(Some(Holder { values: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Index>>>>::from([])))), ..Default::default() })));
    let mut idx: Arc<Mutex<Option<Index>>> = Arc::new(Mutex::new(Some(Index(Arc::new(Mutex::new(Some(7)))))));
    { let __map_key = "answer".to_string(); let __map_value = idx.clone(); (*(*h.lock().unwrap().as_ref().unwrap()).values.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

    let (mut missing, mut ok) = { let __map = { let __map_holder = (*h.lock().unwrap().as_ref().unwrap()).values.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; match __map.get(&"missing".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(Index(Arc::new(Mutex::new(Some(0))))))), Arc::new(Mutex::new(Some(false)))) } };
    println!("{} {}", format!("{}", { let __v = (*missing.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", { let __map = { let __map_holder = (*h.lock().unwrap().as_ref().unwrap()).values.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&"answer".to_string()).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| Index(Arc::new(Mutex::new(Some(0))))) }));
}