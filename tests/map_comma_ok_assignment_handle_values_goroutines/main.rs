use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


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

/// This is the concurrent-wrapper twin of the ordinary fixture. The map's
/// entry must keep its handle when comma-ok assignment writes an existing
/// variable.
fn main() {
    let mut functions = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("function".to_string())))
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>))))]))));
    let mut function: Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = functions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; function = __tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {}", format!("{}", "function".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_result = (*function.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*{ let __map = { let __map_holder = functions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).is_some(); __nil_result }));

    let mut interfaces = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(Box::new("interface".to_string()) as Box<dyn Any + Send + Sync>))))]))));
    let mut iface: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = interfaces.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; iface = __tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {}", format!("{}", "interface".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_result = (*iface.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*{ let __map = { let __map_holder = interfaces.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).is_some(); __nil_result }));

    let mut errorsByName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("error".to_string()))))))]))));
    let mut err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = errorsByName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; err = __tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {}", format!("{}", "error".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*{ let __map = { let __map_holder = errorsByName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).is_some(); __nil_result }));

    let mut channels = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<GoChannel<i32>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(GoChannel::<i32>::new_buffered(1 as usize)))))]))));
    let mut channel = GoChannel::<i32>::new();
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = channels.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.lock().unwrap().as_ref().unwrap().clone(), true), None => (Default::default(), false) } }; channel = __tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {}", format!("{}", "channel".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", !channel.is_nil()), format!("{}", !{ let __map = { let __map_holder = channels.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| { let __guard = __v.lock().unwrap(); __guard.as_ref().cloned().unwrap_or_default() }).unwrap_or_else(|| Default::default()) }.is_nil()));
}