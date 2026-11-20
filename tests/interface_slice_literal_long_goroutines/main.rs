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

pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool;
    fn area(&self) -> f64;
}

impl Clone for Box<dyn Shape + Send + Sync> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub w: Arc<Mutex<Option<f64>>>,
    pub h: Arc<Mutex<Option<f64>>>,
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
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0.0)));
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


#[derive(Debug, Clone)]
pub struct Square {
    pub rect: Arc<Mutex<Option<Rect>>>,
}

impl Square {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.rect.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            rect: __go_clone_0_0,
        }
    }
}


impl Default for Square {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Rect::default())));
        Self {
            rect: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.rect.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Tri {
    pub b: Arc<Mutex<Option<f64>>>,
    pub h: Arc<Mutex<Option<f64>>>,
}

impl Tri {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            b: __go_clone_0_0,
            h: __go_clone_0_1,
        }
    }
}


impl Default for Tri {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            b: __go_default_0_0,
            h: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Tri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.b.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.h.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Rect {
    pub fn area(&self) -> f64 {
        return { let __tmp_x = (*self.w.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
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
    fn area(&self) -> f64 {
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

impl Tri {
    pub fn area(&self) -> f64 {
        return {
            let __tmp_x = { let __tmp_x = (*self.b.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
            let __tmp_y = 2.0;
            __tmp_x / __tmp_y
        };
    }
}

impl Shape for Tri {
    fn area(&self) -> f64 {
        Tri::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Tri>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TriPtr(pub Arc<Mutex<Option<Tri>>>);

impl std::fmt::Display for TriPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for TriPtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Tri::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TriPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Square {
    pub fn area(&self) -> f64 {
        // Forward to embedded type's method
        let embedded = self.rect.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.area()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        Square::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Square>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SquarePtr(pub Arc<Mutex<Option<Square>>>);

impl std::fmt::Display for SquarePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for SquarePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Square::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SquarePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn new_square(s: Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<Square>>> {
    Arc::new(Mutex::new(Some(Square { rect: Arc::new(Mutex::new(Some(Rect { w: Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), h: Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), ..Default::default() })))
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();
    let mut shapes = Arc::new(Mutex::new(Some(vec![
        Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(3.0))), h: Arc::new(Mutex::new(Some(4.0))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))),
        Arc::new(Mutex::new(Some(Box::new((*new_square(Arc::new(Mutex::new(Some(2.0)))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Shape + Send + Sync>))),
        Arc::new(Mutex::new(Some(Box::new(Tri { b: Arc::new(Mutex::new(Some(4.0))), h: Arc::new(Mutex::new(Some(5.0))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))),
        Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(1.0))), h: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))),
        Arc::new(Mutex::new(Some(Box::new((*new_square(Arc::new(Mutex::new(Some(1.5)))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Shape + Send + Sync>))),
    ])));
    let mut total = Arc::new(Mutex::new(Some(0.0)));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, s) in __range_values.iter().enumerate() {
        print!("{} {:.2}\n", i, { let __arg_value = (*s.lock().unwrap().as_ref().unwrap()).area(); __arg_value });
        { let __rhs = (*s.lock().unwrap().as_ref().unwrap()).area(); let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    print!("total {:.2}\n", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v });
}