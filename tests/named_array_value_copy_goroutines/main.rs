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

#[derive(Debug, Clone)]
pub struct P {
    pub x: Arc<Mutex<Option<i32>>>,
}

impl P {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for P {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct A(pub Arc<Mutex<Option<[P; 1]>>>);

impl Default for A {
    fn default() -> Self {
        A(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))))
    }
}

impl A {
    pub fn __go_value_clone(&self) -> Self {
        A(Arc::new(Mutex::new(self.0.lock().unwrap().as_ref().map(|__go_array| __go_array.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))))
    }
}


#[derive(Debug, Clone)]
pub struct Grid(pub Arc<Mutex<Option<[A; 2]>>>);

impl Default for Grid {
    fn default() -> Self {
        Grid(Arc::new(Mutex::new(Some(std::array::from_fn(|_| A(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))))))))))
    }
}

impl Grid {
    pub fn __go_value_clone(&self) -> Self {
        Grid(Arc::new(Mutex::new(self.0.lock().unwrap().as_ref().map(|__go_array| __go_array.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))))
    }
}


#[derive(Debug, Clone)]
pub struct Nums(pub Arc<Mutex<Option<[i32; 2]>>>);

impl Default for Nums {
    fn default() -> Self {
        Nums(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl Nums {
    pub fn __go_value_clone(&self) -> Self {
        Nums(Arc::new(Mutex::new(self.0.lock().unwrap().as_ref().map(|__go_array| __go_array.clone()))))
    }
}

impl Display for Nums {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


pub fn change(a: Arc<Mutex<Option<A>>>) {
    { let new_val = 50; *{ let __seq_holder = { let __named_array = (*a.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
}

/// A named array type is a value: assigning it or passing it copies every
/// element.
fn main() {
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut a: Arc<Mutex<Option<A>>> = Arc::new(Mutex::new(Some(A(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))))))));
    { let new_val = 1; *{ let __seq_holder = { let __named_array = (*a.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    let mut b = { let __owned = a.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 9; *{ let __seq_holder = { let __named_array = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "named array of structs".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = (*a.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq_holder = { let __named_array = (*b.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

    let mut c: Arc<Mutex<Option<A>>> = Arc::new(Mutex::new(Some(A(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))))))));
    { let new_val = a.lock().unwrap().as_ref().unwrap().__go_value_clone(); *c.lock().unwrap() = Some(new_val); };
    { let new_val = 8; *{ let __seq_holder = { let __named_array = (*c.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = (*a.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq_holder = { let __named_array = (*c.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

    change(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {}", format!("{}", "parameter".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = (*a.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

    let mut n: Arc<Mutex<Option<Nums>>> = Arc::new(Mutex::new(Some(Nums(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))))))));
    { let __idx = (0) as usize; let __rhs = 1; (*{ let __named_array = (*n.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    { let __idx = (1) as usize; let __rhs = 2; (*{ let __named_array = (*n.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    let mut m = { let __owned = n.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let __idx = (0) as usize; let __rhs = 9; (*{ let __named_array = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "named array of ints".to_string()), format!("{}", { let __seq_holder = { let __named_array = (*n.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }), format!("{}", { let __seq_holder = { let __named_array = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }));

    let mut g: Arc<Mutex<Option<Grid>>> = Arc::new(Mutex::new(Some(Grid(Arc::new(Mutex::new(Some(std::array::from_fn(|_| A(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))))))))))));
    { let new_val = 1; *{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    { let new_val = 2; *{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    let mut h = { let __owned = g.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 7; *{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*h.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "nested named arrays".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq_holder = { let __named_array = { let __seq_holder = { let __named_array = (*h.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }; __named_array.0.clone() }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

    { let __range_holder = { let __named_array = (*a.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 99; *p.x.lock().unwrap() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range over a named array".to_string()), format!("{}", (*{ let __seq_holder = { let __named_array = (*a.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));
}