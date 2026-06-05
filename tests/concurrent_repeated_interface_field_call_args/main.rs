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

pub trait node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn node + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &(dyn node + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn node + Send + Sync> {
    fn clone(&self) -> Self {
        node::__go_clone_box_node(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct item {
    pub name: Arc<Mutex<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct pair {
    pub left: Arc<Mutex<Option<Box<dyn node + Send + Sync>>>>,
    pub right: Arc<Mutex<Option<Box<dyn node + Send + Sync>>>>,
}

impl pair {
    pub fn __go_value_clone(&self) -> Self {
        Self { left: self.left.clone(), right: self.right.clone() }
    }
}

impl std::fmt::Display for pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.left.lock().unwrap().as_ref().unwrap()), (*self.right.lock().unwrap().as_ref().unwrap()))
    }
}


impl item {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }
}

impl node for item {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        item::name(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<item>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct itemPtr(pub Arc<Mutex<Option<item>>>);

impl std::fmt::Display for itemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl node for itemPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        item::name(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<itemPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn print_pair(left: Arc<Mutex<Option<Box<dyn node + Send + Sync>>>>, right: Arc<Mutex<Option<Box<dyn node + Send + Sync>>>>) {
    println!("{} {}", format!("{}", (*(*left.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())), format!("{}", (*(*right.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())));
}

fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut p = Arc::new(Mutex::new(Some(pair { left: Arc::new(Mutex::new(Some(Box::new(itemPtr(Arc::new(Mutex::new(Some(item { name: Arc::new(Mutex::new(Some("left".to_string()))), ..Default::default() }))).clone())) as Box<dyn node + Send + Sync>))), right: Arc::new(Mutex::new(Some(Box::new(itemPtr(Arc::new(Mutex::new(Some(item { name: Arc::new(Mutex::new(Some("right".to_string()))), ..Default::default() }))).clone())) as Box<dyn node + Send + Sync>))), ..Default::default() })));
    print_pair({ let __field = (*p.lock().unwrap().as_ref().unwrap()).left.clone(); __field }, { let __field = (*p.lock().unwrap().as_ref().unwrap()).right.clone(); __field });
}