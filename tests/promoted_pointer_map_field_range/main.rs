use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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

#[derive(Debug, Clone)]
pub struct Package {
    pub i_d: Arc<Mutex<Option<String>>>,
    pub imports: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Package>>>>>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { i_d: { let __guard = self.i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, imports: self.imports.clone() }
    }
}


impl Default for Package {
    fn default() -> Self {
        Self { i_d: Arc::new(Mutex::new(Some(String::new()))), imports: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.i_d.lock().unwrap().as_ref().unwrap()), format_map(&self.imports))
    }
}


#[derive(Debug, Clone)]
pub struct loaderPackage {
    pub package: Arc<Mutex<Option<Package>>>,
    pub color: Arc<Mutex<Option<i32>>>,
}

impl loaderPackage {
    pub fn __go_value_clone(&self) -> Self {
        Self { package: self.package.clone(), color: { let __guard = self.color.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for loaderPackage {
    fn default() -> Self {
        Self { package: Arc::new(Mutex::new(None)), color: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for loaderPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.package.lock().unwrap().as_ref().unwrap()), (*self.color.lock().unwrap().as_ref().unwrap()))
    }
}


impl loaderPackage {
}

fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let mut base = Arc::new(Mutex::new(Some(Package { i_d: Arc::new(Mutex::new(Some("root".to_string()))), imports: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Package>>>>::from([("dep".to_string(), Arc::new(Mutex::new(Some(Package { i_d: Arc::new(Mutex::new(Some("dep".to_string()))), ..Default::default() }))).clone())])))), ..Default::default() })));
    let mut lpkg = Arc::new(Mutex::new(Some(loaderPackage { package: base.clone(), ..Default::default() })));
    let mut stubs = (*(*lpkg.lock().unwrap().as_mut().unwrap()).package.lock().unwrap().as_mut().unwrap()).imports.clone();
    { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Package>>>>::new()))); (*(*lpkg.lock().unwrap().as_mut().unwrap()).package.lock().unwrap().as_mut().unwrap()).imports = new_val; };
    for (importPath, _) in { let __range_holder = stubs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __map_key = importPath.clone(); let __map_value = Arc::new(Mutex::new(Some(Package { i_d: Arc::new(Mutex::new(Some("dep".to_string()))), ..Default::default() }))); (*(*(*lpkg.lock().unwrap().as_mut().unwrap()).package.lock().unwrap().as_mut().unwrap()).imports.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    println!("{}", format!("{}", "assigned".to_string()));
    done.send(true);
    println!("{}", format!("{}", done.recv().unwrap()));
}