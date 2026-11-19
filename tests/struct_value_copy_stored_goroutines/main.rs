use std::any::Any;
use std::collections::BTreeMap;
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


pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool;
    fn area(&self) -> i32;
}

impl Clone for Box<dyn Shape + Send + Sync> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub w: Arc<Mutex<Option<i32>>>,
    pub h: Arc<Mutex<Option<i32>>>,
}

impl Rect {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            w: __go_clone_0_0,
            h: __go_clone_0_1,
        }
    }
}


impl Default for Rect {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            w: __go_default_0_0,
            h: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.h.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Rect {
    pub fn area(&self) -> i32 {
        return { let __tmp_x = (*self.w.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }
}

impl Shape for Rect {
    fn area(&self) -> i32 {
        Rect::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rect>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RectPtr(pub Arc<Mutex<Option<Rect>>>);

impl std::fmt::Display for RectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for RectPtr {
    fn area(&self) -> i32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Rect::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// Storing a struct somewhere copies it, and so does reading it back. Each
/// line changes the variable after the store, or the value after the read,
/// and prints both sides.
fn main() {
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut p = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() })));

    let mut s = Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(0))), y: Arc::new(Mutex::new(Some(0))), ..Default::default() }])));
    { let __idx = (0) as usize; let __rhs = { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    { let new_val = 2; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "stored in a slice".to_string()), format!("{}", { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    { let new_val = { let __append_target = s.clone(); let __append_value = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; s = new_val; };
    { let new_val = 3; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "appended".to_string()), format!("{}", { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Point>>>>::from([]))));
    { let __map_key = "a".to_string(); let __map_value = { let __owned = p.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) }; (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let new_val = 4; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "stored in a map".to_string()), format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut got = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) })));
    { let new_val = 40; *(*got.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "read from a map".to_string()), format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) }), format!("{}", { let __v = (*got.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut l = Arc::new(Mutex::new(Some(Line { a: Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), b: Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), ..Default::default() })));
    { let new_val = 5; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "in a struct literal".to_string()), format!("{}", { let __v = (*l.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    { let new_val = p.lock().unwrap().as_ref().unwrap().__go_value_clone(); *(*l.lock().unwrap().as_ref().unwrap()).b.lock().unwrap() = Some(new_val); };
    { let new_val = 6; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned to a field".to_string()), format!("{}", { let __v = (*l.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut arr = Arc::new(Mutex::new(Some([Default::default(), Default::default()])));
    { let __idx = (0) as usize; let __rhs = { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }; (*arr.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    { let new_val = 7; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "stored in an array".to_string()), format!("{}", { let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut r = Arc::new(Mutex::new(Some(Rect { w: Arc::new(Mutex::new(Some(2))), h: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    let mut shape: Arc<Mutex<Option<Box<dyn Shape + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Shape + Send + Sync>)));
    { let new_val = 10; *(*r.lock().unwrap().as_ref().unwrap()).w.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "in an interface".to_string()), format!("{}", (*shape.lock().unwrap().as_ref().unwrap()).area()), format!("{}", (*r.lock().unwrap().as_ref().unwrap()).area()));

    let mut q = p.clone();
    let mut c = Arc::new(Mutex::new(Some({ let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    { let new_val = 100; *(*c.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "read through a pointer".to_string()), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut d: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }.__go_value_clone(); *d.lock().unwrap() = Some(new_val); };
    { let new_val = 200; *(*d.lock().unwrap().as_ref().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned through a pointer".to_string()), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*d.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut e: Arc<Mutex<Option<Point>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.__go_value_clone(); *e.lock().unwrap() = Some(new_val); };
    { let new_val = 300; *(*e.lock().unwrap().as_ref().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assigned from an element".to_string()), format!("{}", { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __v = (*e.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut list = Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    let mut spread = { let __append_target = Arc::new(Mutex::new(Some(Vec::<Point>::new()))).clone(); let __append_values: Vec<_> = { let __slice_holder = list.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().map(|__go_elem| __go_elem.__go_value_clone()).collect(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() };
    { let new_val = 400; *{ let __seq = { let __seq_holder = spread.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "slice copied with append".to_string()), format!("{}", { let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = spread.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));

    let mut copied: Arc<Mutex<Option<Vec<Point>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    let mut n = {
        let _src = { let __copy_src_holder = list.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
        let _n = std::cmp::min((*copied.lock().unwrap().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*copied.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].__go_value_clone();
        }
        Arc::new(Mutex::new(Some(_n as i32)))
    };
    { let new_val = 500; *{ let __seq = { let __seq_holder = copied.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.y.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "slice copied with copy".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = copied.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));

    let mut grown = { let __append_target = spread.clone(); let __append_values: Vec<_> = { let __slice_holder = list.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().map(|__go_elem| __go_elem.__go_value_clone()).collect(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() };
    { let new_val = 600; *{ let __seq = { let __seq_holder = grown.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", "slice appended to another".to_string()), format!("{}", { let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = grown.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }), format!("{}", (*grown.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    let mut ch = GoChannel::<Point>::new_buffered(1 as usize);
    ch.send(p.lock().unwrap().as_ref().unwrap().__go_value_clone());
    { let new_val = 8; *(*p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "sent on a channel".to_string()), format!("{}", ch.recv().unwrap_or_default()), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));
}