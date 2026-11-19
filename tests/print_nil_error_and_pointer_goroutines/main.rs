use std::error::Error as StdError;
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
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Status has a String method on the value, so *Status has it too.
#[derive(Debug, Clone)]
pub struct Status {
    pub code: Arc<Mutex<Option<i32>>>,
}

impl Status {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for Status {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


/// Gauge has a String method on the pointer only.
#[derive(Debug, Clone)]
pub struct Gauge {
    pub level: Arc<Mutex<Option<i32>>>,
}

impl Gauge {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.level.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            level: __go_clone_0_0,
        }
    }
}


impl Default for Gauge {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            level: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Gauge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


/// Broken is an error through its pointer.
#[derive(Debug, Clone)]
pub struct Broken {
    pub why: Arc<Mutex<Option<String>>>,
}

impl Broken {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.why.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            why: __go_clone_0_0,
        }
    }
}


impl Default for Broken {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            why: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Broken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


/// Timeout is an error through its value.
#[derive(Debug, Clone)]
pub struct Timeout {
    pub seconds: Arc<Mutex<Option<i32>>>,
}

impl Timeout {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.seconds.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            seconds: __go_clone_0_0,
        }
    }
}


impl Default for Timeout {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            seconds: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


impl Status {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("status {}", (*self.code.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Gauge {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("gauge {}", (*self.level.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Broken {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "broken: ".to_string(), (*self.why.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for Broken {}


impl Timeout {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("timeout after {}s", (*self.seconds.lock().unwrap().as_ref().unwrap())))))
    }
}

impl StdError for Timeout {}


pub fn find(ok: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(None));
    }
    Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("missing".to_string()))))
}

pub fn lookup(ok: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Point>>> {
    if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() })));
    }
    return Arc::new(Mutex::new(None));
}

fn main() {
        // A goroutine makes every shared value a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

        // A nil error prints as <nil>, whichever print call it reaches and
        // whether it comes from a variable or straight from a call.
    let mut err = find(Arc::new(Mutex::new(Some(true))));
    println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "result:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    print!("{} and {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }, 3);
    println!("{}", format!("{}", { let __text = match find(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(format!("got {}", { let __text = match find(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).lock().unwrap().as_ref().unwrap())));

        // An error that is not nil prints its message, as before.
    { let __rhs_holder = find(Arc::new(Mutex::new(Some(false)))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "result:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{} {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }, { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    println!("{}", format!("{}", { let __text = match find(Arc::new(Mutex::new(Some(false)))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A nil pointer prints as <nil>.
    let mut p: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", { let __text = match p.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "point:".to_string()), format!("{}", { let __text = match p.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{}\n", { let __text = match p.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text });
    println!("{}", format!("{}", { let __text = match lookup(Arc::new(Mutex::new(Some(false)))).lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    { let new_val = lookup(Arc::new(Mutex::new(Some(true)))).clone(); p = new_val; };
    println!("{}", format!("{}", { let __text = match p.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", { let __text = match lookup(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));

        // A pointer whose type has a String or Error method prints through the
        // method, with no & in front. A nil one prints as <nil>.
    let mut s = Arc::new(Mutex::new(Some(Status { code: Arc::new(Mutex::new(Some(7))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match s.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{} {}\n", { let __text = match s.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }, { let __text = match s.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    let mut g = Arc::new(Mutex::new(Some(Gauge { level: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match g.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", { let __text = match g.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).lock().unwrap().as_ref().unwrap())));
    let mut b = Arc::new(Mutex::new(Some(Broken { why: Arc::new(Mutex::new(Some("z".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match b.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {} {}", format!("{}", "gauge:".to_string()), format!("{}", { let __text = match g.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", "broken:".to_string()), format!("{}", { let __text = match b.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut none: Arc<Mutex<Option<Status>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", { let __text = match none.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut empty: Arc<Mutex<Option<Gauge>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", { let __text = match empty.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A struct or a pointer that implements error can start an error
        // variable, and prints as the error it is.
    let mut declared: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Broken { why: Arc::new(Mutex::new(Some("declared".to_string()))), ..Default::default() }) as Box<dyn StdError + Send + Sync>)));
    println!("{}", format!("{}", { let __text = match declared.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut plain: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Timeout { seconds: Arc::new(Mutex::new(Some(3))), ..Default::default() }) as Box<dyn StdError + Send + Sync>)));
    println!("{}", format!("{}", { let __text = match plain.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", { let __nil_result = (*declared.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*plain.lock().unwrap()).is_some(); __nil_result }));
    let mut unset: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", { let __text = match unset.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_result = (*unset.lock().unwrap()).is_none(); __nil_result }));
}