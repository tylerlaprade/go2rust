use std::any::Any;
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

    fn recv(&self) -> Option<T>
    where
        T: Default,
    {
        if self.is_nil() {
            return None;
        }
        let value = match self.rx.lock().unwrap().recv() {
            Ok(value) => Some(value),
            Err(_) => Some(T::default()),
        };
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
}

#[derive(Debug, Clone)]
pub struct counter {
    pub value: Arc<Mutex<Option<i32>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for counter {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}


pub trait valueReader: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn valueReader + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &(dyn valueReader + Send + Sync)) -> bool;
    fn value(&self) -> Arc<Mutex<Option<i32>>>;
}

impl Clone for Box<dyn valueReader + Send + Sync> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

pub(crate) static current: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<counter>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static fallback: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<counter>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *current.lock().unwrap() = Some(Default::default());
    *fallback.lock().unwrap() = Some(Default::default());
    *fallback.lock().unwrap() = Some(Arc::new(Mutex::new(Some(counter { value: Arc::new(Mutex::new(Some(5 as i32))), ..Default::default() }))));
}


impl counter {
    pub fn value(&self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }
}

impl valueReader for counter {
    fn value(&self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }
    fn __go_clone_box(&self) -> Box<dyn valueReader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn valueReader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &(dyn valueReader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counter>() {
            false
        } else {
            false
        }
    }
}

pub fn new_counter(value: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<counter>>> {

    return Arc::new(Mutex::new(Some(counter { value: value.clone(), ..Default::default() })));
}

pub fn set_counter(c: Arc<Mutex<Option<counter>>>) {
    { let new_val = c.clone(); *current.lock().unwrap() = Some(new_val); };
}

pub fn get_counter() -> Arc<Mutex<Option<counter>>> {

    return (*current.lock().unwrap().as_ref().unwrap()).clone();
}

pub fn current_value() -> Arc<Mutex<Option<i32>>> {

    return Arc::new(Mutex::new(Some({ let __selector_holder = (*(*current.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

pub fn set_current_value(value: Arc<Mutex<Option<i32>>>) {
    { let new_val = value.lock().unwrap().as_ref().unwrap().clone(); *(*(*current.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).value.lock().unwrap() = Some(new_val); };
}

pub fn get_fallback() -> Arc<Mutex<Option<Box<dyn valueReader + Send + Sync>>>> {

    return Arc::new(Mutex::new(Some(Box::new((*(*fallback.lock().unwrap().as_ref().unwrap()).clone().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn valueReader + Send + Sync>)));
}

pub fn clear_counter() {
    *current.lock().unwrap() = Some(Default::default());
}

pub fn mark_concurrent(done: GoChannel<bool>) {
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
}

fn main() {
    __go_init_all();
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    mark_concurrent(done.clone());
    done.recv().unwrap();

    set_counter(new_counter(Arc::new(Mutex::new(Some(7)))));
    println!("{}", format!("{}", (*(*get_counter().lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*current_value().lock().unwrap().as_ref().unwrap())));
    set_current_value(Arc::new(Mutex::new(Some(9))));
    println!("{}", format!("{}", (*(*get_counter().lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap())));
    set_counter(new_counter(Arc::new(Mutex::new(Some(11)))));
    println!("{}", format!("{}", (*(*get_counter().lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap())));
    set_counter(Arc::new(Mutex::new(None)));
    println!("{}", format!("{}", (*get_counter().lock().unwrap()).is_none()));
    set_counter(new_counter(Arc::new(Mutex::new(Some(13)))));
    clear_counter();
    println!("{}", format!("{}", (*get_counter().lock().unwrap()).is_none()));
    println!("{}", format!("{}", (*{ let __recv = get_fallback(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).value(); __result }.lock().unwrap().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
