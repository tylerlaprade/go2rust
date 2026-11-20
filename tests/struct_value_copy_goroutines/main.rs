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
pub struct Line {
    pub a: Arc<Mutex<Option<Point>>>,
    pub b: Arc<Mutex<Option<Point>>>,
}

impl Line {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_0_1 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_0_1,
        }
    }
}


impl Default for Line {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Point::default())));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(Point::default())));
        Self {
            a: __go_default_0_0,
            b: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.b.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Arc<Mutex<Option<[Point; 2]>>>,
}

impl Grid {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cells.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))) };
        Self {
            cells: __go_clone_0_0,
        }
    }
}


impl Default for Grid {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default()))));
        Self {
            cells: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.cells));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Node {
    pub p: Arc<Mutex<Option<Point>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.p.clone();
        Self {
            p: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.p.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Config {
    pub name: Arc<Mutex<Option<String>>>,
    pub port: Arc<Mutex<Option<i64>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.port.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            port: __go_clone_1_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            port: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.port.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Location {
    pub p: Arc<Mutex<Option<Point>>>,
}

impl Location {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            p: __go_clone_0_0,
        }
    }
}


impl Default for Location {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Point::default())));
        Self {
            p: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub l: Arc<Mutex<Option<Location>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.l.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            l: __go_clone_0_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Location::default())));
        Self {
            l: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.l.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Config {
    /// A value receiver works on a copy of the struct.
    pub fn with_name(&self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Config>>> {
        let mut __self = self.__go_value_clone();
        { let new_val = name.lock().unwrap().as_ref().unwrap().clone(); *__self.name.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    pub fn bump(&self) {
        let mut __self = self.__go_value_clone();
        { let __target = __self.port.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl User {
    pub fn location(&self) -> Arc<Mutex<Option<Location>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.l.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); drop(__selector_guard); __cloned })));
    }
}

pub fn r#move(p: Arc<Mutex<Option<Point>>>) -> Arc<Mutex<Option<Point>>> {
    { let __target = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); let __rhs = 10; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    return { let __owned = p.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn stretch(l: Arc<Mutex<Option<Line>>>) {
    { let new_val = 100; *(*(*l.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
}

/// A struct is a value. Each of these makes a copy, and changing the copy
/// leaves the original alone.
fn main() {
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut a = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() })));
    let mut b = { let __owned = a.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 9; *(*b.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assign".to_string()), format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut c: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = a.lock().unwrap().as_ref().unwrap().__go_value_clone(); *c.lock().unwrap() = Some(new_val); };
    { let new_val = 7; *(*c.lock().unwrap().as_ref().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assign to declared".to_string()), format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut d = r#move(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {} {}", format!("{}", "parameter and result".to_string()), format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*d.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut l1 = Arc::new(Mutex::new(Some(Line { a: Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }))), b: Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }))), ..Default::default() })));
    let mut l2 = { let __owned = l1.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 9; *(*(*l2.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "struct in struct".to_string()), format!("{}", { let __v = (*l1.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*l2.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    stretch(Arc::new(Mutex::new(Some({ let __arg_holder = l1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {}", format!("{}", "struct in struct, parameter".to_string()), format!("{}", { let __v = (*l1.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut points = Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    let mut e = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = points.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.__go_value_clone())));
    { let new_val = 50; *(*e.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "slice element".to_string()), format!("{}", format_slice(&points)), format!("{}", { let __v = (*e.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    { let __range_holder = points.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 60; *p.x.lock().unwrap() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range value".to_string()), format!("{}", format_slice(&points)));

    let mut arr = Arc::new(Mutex::new(Some([Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    let mut arr2 = { let __owned = arr.lock().unwrap().as_ref().unwrap().each_ref().map(|__go_elem| __go_elem.__go_value_clone()); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 7; *{ let __seq = { let __seq_holder = arr2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "array".to_string()), format!("{}", format_slice(&arr)), format!("{}", format_slice(&arr2)));

    let mut g1 = Arc::new(Mutex::new(Some(Grid { cells: Arc::new(Mutex::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default())))) })));
    let mut g2 = { let __owned = g1.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 5; *{ let __seq = { let __seq_holder = (*g2.lock().unwrap().as_ref().unwrap()).cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "array in struct".to_string()), format!("{}", { let __v = (*g1.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*g2.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut base = Arc::new(Mutex::new(Some(Config { name: Arc::new(Mutex::new(Some("base".to_string()))), port: Arc::new(Mutex::new(Some(80))), ..Default::default() })));
    let mut dev = (*base.lock().unwrap().as_ref().unwrap()).with_name(Arc::new(Mutex::new(Some("dev".to_string()))));
    println!("{} {} {}", format!("{}", "value receiver".to_string()), format!("{}", { let __v = (*base.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*dev.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));
    (*base.lock().unwrap().as_ref().unwrap()).bump();
    println!("{} {}", format!("{}", "value receiver, no result".to_string()), format!("{}", { let __v = (*base.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut u = Arc::new(Mutex::new(Some(User { l: Arc::new(Mutex::new(Some(Location { p: Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }))), ..Default::default() }))), ..Default::default() })));
    let mut v = { let __recv = u.clone(); let __recv_ptr: *const User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const User }; let __result = unsafe { &*__recv_ptr }.location(); __result };
    { let new_val = 9; *(*(*v.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "getter".to_string()), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).l.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

        // A pointer inside the struct is copied as a pointer: both structs
        // point at the same Point until one of them gets a new pointer.
    let mut n1 = Arc::new(Mutex::new(Some(Node {
        p: Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }))).clone(),
        ..Default::default()
    })));
    let mut n2 = { let __owned = n1.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 3; *(*(*n2.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "pointer field shared".to_string()), format!("{}", { let __v = (*(*n1.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*(*n2.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    { let new_val = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(8))), y: Arc::new(Mutex::new(Some(8))), ..Default::default() }))).clone(); (*n2.lock().unwrap().as_mut().unwrap()).p = new_val; };
    println!("{} {} {}", format!("{}", "pointer field replaced".to_string()), format!("{}", { let __v = (*(*n1.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*(*n2.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).clone(); __v }));

        // A range over a slice reads each element when its turn comes, so it
        // sees a field that the loop itself wrote. The loop variable is still a
        // copy: writing to it does not change the slice.
    let mut seen = Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    { let __range_holder = seen.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 9; *{ let __seq = { let __seq_holder = seen.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
        println!("{} {}", format!("{}", "range over a slice, field written in the loop".to_string()), format!("{}", (*p.x.lock().unwrap().as_ref().unwrap())));
        { let new_val = 99; *p.x.lock().unwrap() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range over a slice, the slice afterwards".to_string()), format!("{}", format_slice(&seen)));

        // A range over an array works on a copy of the array.
    let mut fixed = Arc::new(Mutex::new(Some([Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    { let __range_holder = fixed.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 9; *{ let __seq = { let __seq_holder = fixed.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
        println!("{} {}", format!("{}", "range over an array, field written in the loop".to_string()), format!("{}", (*p.x.lock().unwrap().as_ref().unwrap())));
        { let new_val = 99; *p.x.lock().unwrap() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range over an array, the array afterwards".to_string()), format!("{}", format_slice(&fixed)));
}