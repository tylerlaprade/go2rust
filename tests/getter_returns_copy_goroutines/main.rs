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
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub name: Arc<Mutex<Option<String>>>,
    pub pos: Arc<Mutex<Option<Point>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            name: __go_clone_0_0,
            pos: __go_clone_1_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Point::default())));
        Self {
            name: __go_default_0_0,
            pos: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pos.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl User {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }

    pub fn pos(&self) -> Arc<Mutex<Option<Point>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); drop(__selector_guard); __cloned })));
    }

    pub fn label(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

pub fn name_of(u: Arc<Mutex<Option<User>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

/// A method that returns a field returns a copy of it. The caller's variable
/// is its own: changing it does not change the struct.
fn main() {
        // A goroutine makes every shared value a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut u = Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("ann".to_string()))), pos: Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }))), ..Default::default() })));

    let mut a = { let __recv = u.clone(); let __recv_ptr: *const User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const User }; let __result = unsafe { &*__recv_ptr }.name(); __result };
    { let new_val = Arc::new(Mutex::new(Some({ let __s = (*a.lock().unwrap().as_ref().unwrap()).clone(); __s.to_uppercase() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *a.lock().unwrap() = __moved_val; };
    println!("{} {}", format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));

    let mut b = { let __recv = u.clone(); let __recv_ptr: *const User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const User }; let __result = unsafe { &*__recv_ptr }.name(); __result };
    { (*b.lock().unwrap().as_mut().unwrap()).push_str(&"!".to_string()); };
    println!("{} {}", format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));

    let mut c = Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    { (*c.lock().unwrap().as_mut().unwrap()).push_str(&"?".to_string()); };
    println!("{} {}", format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));

    let mut d = name_of(u.clone());
    { let new_val = "other".to_string(); *d.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));

    let mut e = { let __recv = u.clone(); let __recv_ptr: *const User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const User }; let __result = unsafe { &*__recv_ptr }.label(); __result };
    { let new_val = "label".to_string(); *e.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));

    let mut p = { let __recv = u.clone(); let __recv_ptr: *const User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const User }; let __result = unsafe { &*__recv_ptr }.pos(); __result };
    { let new_val = 100; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).pos.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
}