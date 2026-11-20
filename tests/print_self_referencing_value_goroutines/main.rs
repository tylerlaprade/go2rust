use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
}

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, false)
}

// Prints a value held inside the one being printed: a struct's interface
// field, an element of a []interface{}.
fn format_any_nested(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, true)
}

fn format_any_at(value: &(dyn Any + Send + Sync), nested: bool) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Node>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Point>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Label>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Node>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn format_any_slice_values(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>, format_value: fn(&(dyn Any + Send + Sync)) -> String) -> String {
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_value(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice, format_any_nested))
}

fn format_any_variadic(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format_any_slice_values(slice, format_any)
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

/// Node can point at itself through its interface field.
#[derive(Clone)]
pub struct Node {
    pub name: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            name: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.next.lock().unwrap().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<i32>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// Label prints itself by printing another value. That is a print call of its
/// own, so the pointer it prints is expanded wherever the Label sits.
#[derive(Debug, Clone)]
pub struct Label {
    pub x: Arc<Mutex<Option<i32>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Label {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


impl Label {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        render(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some({ let __selector_holder = self.x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))))
    }
}

pub fn render(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", { let __text = format_any(v.lock().unwrap().as_ref().unwrap().as_ref()); __text }))))
}

/// shape describes text around the addresses in it, which change from run to
/// run: whether it starts with prefix, and how many addresses it holds.
pub fn shape(text: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some({ let __s = (*text.lock().unwrap().as_ref().unwrap()).clone(); let __arg = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __tmp_x = ((*Arc::new(Mutex::new(Some({ let __s = (*text.lock().unwrap().as_ref().unwrap()).clone(); let __sep = "0x".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })))))
}

fn main() {
        // A goroutine makes every shared value a locked one. Printing a value
        // that points back to itself must not wait for a lock it already holds.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut r#loop = Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("loop".to_string()))), ..Default::default() })));
    { let new_val = Box::new(r#loop.clone()) as Box<dyn Any + Send + Sync>; *(*r#loop.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", "pointer, boxed:".to_string()), format!("{}", (*shape(
        render(Arc::new(Mutex::new(Some(Box::new(r#loop.clone()) as Box<dyn Any + Send + Sync>)))),
        Arc::new(Mutex::new(Some("&{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "pointer, typed:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some(format!("{}", { let __text = match r#loop.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text })))),
        Arc::new(Mutex::new(Some("&{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "struct value, boxed:".to_string()), format!("{}", (*shape(
        render(Arc::new(Mutex::new(Some(Box::new({ let __v = (*r#loop.lock().unwrap().as_ref().unwrap()).clone(); __v }) as Box<dyn Any + Send + Sync>)))),
        Arc::new(Mutex::new(Some("{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "struct value, typed:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some(format!("{}", { let __v = (*r#loop.lock().unwrap().as_ref().unwrap()).clone(); __v })))),
        Arc::new(Mutex::new(Some("{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));

    let mut tail = Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("tail".to_string()))), ..Default::default() })));
    let mut text = render(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("holder".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(tail.clone()) as Box<dyn Any + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    println!("{} {} {}", format!("{}", "struct holding a pointer:".to_string()), format!("{}", (*shape(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("{holder 0x".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some({ let __s = (*text.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "tail".to_string(); __s.contains(&__arg) }))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", Node { name: Arc::new(Mutex::new(Some("end".to_string()))), ..Default::default() }));

    println!("{}", format!("{}", Node { name: Arc::new(Mutex::new(Some("row".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Any + Send + Sync>))), ..Default::default() }));
    println!("{}", format!("{}", (*render(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("row".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Any + Send + Sync>))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));

        // The elements of a slice are inside it. Spread into a print call they
        // are its arguments, and a pointer among them is expanded.
    let mut labels: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>> = Arc::new(Mutex::new(None));
    { let new_val = { let __append_target = labels.clone(); let __append_values = vec![
        Box::new(Label { x: Arc::new(Mutex::new(Some(8))), ..Default::default() }) as Box<dyn Any + Send + Sync>,
        Box::new(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(2))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>,
        Box::new(1) as Box<dyn Any + Send + Sync>,
    ]; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; labels = new_val; };
    println!("{} {}", format!("{}", "slice:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some(format!("{}", format_any_slice(&labels))))),
        Arc::new(Mutex::new(Some("[&{8} 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{}", format_any_variadic(&labels));
}