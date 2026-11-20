use std::any::Any;
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

fn go_embedded_owner_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_register_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, owner: Arc<Mutex<Option<T>>>) {
    go_embedded_owner_registry().lock().unwrap().insert(embedded_key, Box::new(owner));
}

fn go_lookup_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, target: &str) -> Arc<Mutex<Option<T>>> {
    let registry = go_embedded_owner_registry().lock().unwrap();
    let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
    owner
        .downcast_ref::<Arc<Mutex<Option<T>>>>()
        .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
        .clone()
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
pub struct Basic {
    pub id: Arc<Mutex<Option<String>>>,
    pub balance: Arc<Mutex<Option<f64>>>,
    pub history: Arc<Mutex<Option<Vec<String>>>>,
}

impl Basic {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.balance.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.history.clone();
        Self {
            id: __go_clone_0_0,
            balance: __go_clone_1_0,
            history: __go_clone_2_0,
        }
    }
}


impl Default for Basic {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            id: __go_default_0_0,
            balance: __go_default_1_0,
            history: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.id.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.balance.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.history));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Savings {
    pub basic: Arc<Mutex<Option<Basic>>>,
    pub rate: Arc<Mutex<Option<f64>>>,
}

impl Savings {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.basic.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.rate.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            basic: __go_clone_0_0,
            rate: __go_clone_1_0,
        }
    }
}


impl Default for Savings {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Basic::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            basic: __go_default_0_0,
            rate: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Savings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.basic.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.rate.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Premium {
    pub savings: Arc<Mutex<Option<Savings>>>,
    pub bonus: Arc<Mutex<Option<f64>>>,
}

impl Premium {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.savings.clone();
        let __go_clone_1_0 = { let __guard = self.bonus.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            savings: __go_clone_0_0,
            bonus: __go_clone_1_0,
        }
    }
}


impl Default for Premium {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            savings: __go_default_0_0,
            bonus: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Premium {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.savings.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.bonus.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Basic {
    pub fn deposit(&mut self, a: Arc<Mutex<Option<f64>>>) {
        { let __target = self.balance.clone(); let __rhs = (*a.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = { let __append_target = self.history.clone(); let __append_value = (*Arc::new(Mutex::new(Some(format!("+{:.2}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v })))).lock().unwrap().as_ref().unwrap()).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.history = new_val; };
    }
}

impl Savings {
    pub fn add_interest(&mut self) {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*self.basic.lock().unwrap().as_ref().unwrap()).balance.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.rate.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))); self.deposit(__method_arg0) };
    }

    pub fn deposit(&mut self, a: Arc<Mutex<Option<f64>>>) {
        // Forward to embedded type's method
        let embedded = self.basic.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.deposit(a)
    }
}

impl Premium {
    pub fn reward(&mut self) {
        { let __promoted_recv = self.savings.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.add_interest(); __result };
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.bonus.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); self.deposit(__method_arg0) };
    }

    pub fn add_interest(&mut self) {
        // Forward to embedded type's method
        let embedded = self.savings.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.add_interest()
    }

    pub fn deposit(&mut self, a: Arc<Mutex<Option<f64>>>) {
        // Forward to embedded type's method
        let embedded = self.savings.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.deposit(a)
    }
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();
    let mut s = {
        let __owner = Arc::new(Mutex::new(Some(Savings { basic: Arc::new(Mutex::new(Some(Basic { id: Arc::new(Mutex::new(Some("bob".to_string()))), ..Default::default() }))), rate: Arc::new(Mutex::new(Some(0.5))), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().basic.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __recv = s.clone(); let __recv_ptr: *mut Savings = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Savings }; let __result = unsafe { &mut *__recv_ptr }.deposit(Arc::new(Mutex::new(Some(100.0)))); __result };
    { let __recv = s.clone(); let __recv_ptr: *mut Savings = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Savings }; let __result = unsafe { &mut *__recv_ptr }.add_interest(); __result };
    println!("{} {} {}", format!("{}", (*(*(*s.lock().unwrap().as_ref().unwrap()).basic.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*(*(*s.lock().unwrap().as_ref().unwrap()).basic.lock().unwrap().as_ref().unwrap()).balance.lock().unwrap().as_ref().unwrap())), format!("{}", format_slice_values(&(*(*(*s.lock().unwrap().as_ref().unwrap()).basic.lock().unwrap().as_ref().unwrap()).history.lock().unwrap().as_ref().unwrap()).clone())));
    let mut p = {
        let __owner = Arc::new(Mutex::new(Some(Premium {
            savings: {
                let __owner = Arc::new(Mutex::new(Some(Savings { basic: Arc::new(Mutex::new(Some(Basic { id: Arc::new(Mutex::new(Some("ann".to_string()))), ..Default::default() }))), rate: Arc::new(Mutex::new(Some(0.1))), ..Default::default() })));
                let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().basic.clone() };
                let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
                go_register_embedded_owner(__embedded_key, __owner.clone());
                __owner
            }.clone(),
            bonus: Arc::new(Mutex::new(Some(5.0))),
            ..Default::default()
        })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().savings.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __recv = p.clone(); let __recv_ptr: *mut Premium = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Premium }; let __result = unsafe { &mut *__recv_ptr }.deposit(Arc::new(Mutex::new(Some(50.0)))); __result };
    { let __recv = p.clone(); let __recv_ptr: *mut Premium = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Premium }; let __result = unsafe { &mut *__recv_ptr }.reward(); __result };
    print!("{} {} {}\n", format!("{}", (*(*(*p.lock().unwrap().as_ref().unwrap()).savings.lock().unwrap().as_ref().unwrap().basic.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()).clone()), format!("{:.2}", (*(*(*p.lock().unwrap().as_ref().unwrap()).savings.lock().unwrap().as_ref().unwrap().basic.lock().unwrap().as_ref().unwrap()).balance.lock().unwrap().as_ref().unwrap())), format!("{}", format_slice_values(&(*(*(*p.lock().unwrap().as_ref().unwrap()).savings.lock().unwrap().as_ref().unwrap().basic.lock().unwrap().as_ref().unwrap()).history.lock().unwrap().as_ref().unwrap()).clone())));
}