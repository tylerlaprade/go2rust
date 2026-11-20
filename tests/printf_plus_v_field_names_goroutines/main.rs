use std::error::Error as StdError;
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
    pub x: Arc<Mutex<Option<i32>>>,
    pub y: Arc<Mutex<Option<i32>>>,
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
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.x.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.x.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.y.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.y.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{X:{} Y:{}}}", __go_fmt_0, __go_fmt_1);
        }
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Base {
    pub i_d: Arc<Mutex<Option<i32>>>,
}

impl Base {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            i_d: __go_clone_0_0,
        }
    }
}


impl Default for Base {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            i_d: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.i_d.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.i_d.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{ID:{}}}", __go_fmt_0);
        }
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Line {
    pub base: Arc<Mutex<Option<Base>>>,
    pub from: Arc<Mutex<Option<Point>>>,
    pub to: Arc<Mutex<Option<Point>>>,
    pub label: Arc<Mutex<Option<String>>>,
    pub tags: Arc<Mutex<Option<Vec<String>>>>,
    pub weight: Arc<Mutex<Option<f64>>>,
    pub ok: Arc<Mutex<Option<bool>>>,
}

impl Line {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.from.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_1 = { let __guard = self.to.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_2_0 = { let __guard = self.label.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.tags.clone();
        let __go_clone_4_0 = { let __guard = self.weight.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.ok.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            from: __go_clone_1_0,
            to: __go_clone_1_1,
            label: __go_clone_2_0,
            tags: __go_clone_3_0,
            weight: __go_clone_4_0,
            ok: __go_clone_5_0,
        }
    }
}


impl Default for Line {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Base::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Point::default())));
        let __go_default_1_1 = Arc::new(Mutex::new(Some(Point::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            base: __go_default_0_0,
            from: __go_default_1_0,
            to: __go_default_1_1,
            label: __go_default_2_0,
            tags: __go_default_3_0,
            weight: __go_default_4_0,
            ok: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.base.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.base.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.from.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.from.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_2 = if __go_plus { format!("{:#}", (*self.to.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.to.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_3 = if __go_plus { format!("{:#}", (*self.label.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.label.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_4 = format!("{}", format_slice(&self.tags));
        let __go_fmt_5 = if __go_plus { format!("{:#}", (*self.weight.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.weight.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_6 = if __go_plus { format!("{:#}", (*self.ok.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.ok.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{Base:{} From:{} To:{} Label:{} Tags:{} Weight:{} Ok:{}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6);
        }
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}


#[derive(Debug, Clone)]
pub struct Named {
    pub n: Arc<Mutex<Option<i32>>>,
}

impl Named {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for Named {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Config {
    pub name: Arc<Mutex<Option<String>>>,
    pub retries: Arc<Mutex<Option<i32>>>,
    pub verbose: Arc<Mutex<Option<bool>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.retries.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.verbose.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            retries: __go_clone_1_0,
            verbose: __go_clone_2_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            name: __go_default_0_0,
            retries: __go_default_1_0,
            verbose: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.name.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.name.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.retries.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.retries.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_2 = if __go_plus { format!("{:#}", (*self.verbose.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.verbose.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{Name:{} Retries:{} Verbose:{}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2);
        }
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl Named {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("N{}", (*self.n.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Line {
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();

    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() })));
    let mut l = Arc::new(Mutex::new(Some(Line {
        base: Arc::new(Mutex::new(Some(Base { i_d: Arc::new(Mutex::new(Some(7))), ..Default::default() }))),
        from: Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(0))), y: Arc::new(Mutex::new(Some(0))), ..Default::default() }))),
        to: Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() }))),
        label: Arc::new(Mutex::new(Some("diag".to_string()))),
        tags: Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string()]))),
        weight: Arc::new(Mutex::new(Some(2.5))),
        ok: Arc::new(Mutex::new(Some(true))),
        ..Default::default()
    })));
    print!("{:#}\n", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v });
    print!("{:#}\n", { let __v = (*l.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v });
    print!("{}\n", { let __v = (*l.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v });
    let mut s = Arc::new(Mutex::new(Some(format!("p={} n={} i={} d={} str={}", format!("{:#}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{:#}", Named { n: Arc::new(Mutex::new(Some(3))), ..Default::default() }), format!("{:#}", 5), format!("{:+}", 6), format!("{:#}", "x".to_string())))));
    println!("{}", format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    print!("{:#}\n", { let __text = match Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("boom".to_string()))))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    let mut pts = Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() }])));
    print!("{} {}\n", format!("{:#}", { let __plus_holder = pts.clone(); let __plus_guard = __plus_holder.lock().unwrap(); match __plus_guard.as_ref() { Some(__s) => format!("[{}]", __s.iter().map(|__v| format!("{:#}", __v)).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } }), format!("{}", format_slice(&pts)));
    let mut pp = p.clone();
    print!("{} {}\n", format!("{:#}", { let __text = match pp.lock().unwrap().as_ref() { Some(__value) => format!("&{:#}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match pp.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{:#}\n", AnonymousStruct1 { a: Arc::new(Mutex::new(Some(1))), b: Arc::new(Mutex::new(Some(2))) });
    let mut cfg = Arc::new(Mutex::new(Some(Config { name: Arc::new(Mutex::new(Some("svc".to_string()))), retries: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    print!("config: {:#}\n", { let __v = (*cfg.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v });
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(format!("{}|{}", format!("{}", { let __v = (*cfg.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{:#}", { let __v = (*cfg.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }))))).lock().unwrap().as_ref().unwrap())));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub a: Arc<Mutex<Option<i32>>>,
    pub b: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_0_1,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            a: __go_default_0_0,
            b: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.a.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.a.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.b.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.b.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{A:{} B:{}}}", __go_fmt_0, __go_fmt_1);
        }
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
