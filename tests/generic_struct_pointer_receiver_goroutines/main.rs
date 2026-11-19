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


pub struct C<T: Any + Send + Sync + 'static> {
    pub p: Arc<Mutex<Option<P>>>,
    pub items: Arc<Mutex<Option<[P; 2]>>>,
    pub v: Arc<Mutex<Option<T>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}

impl<T: Any + Send + Sync + 'static> C<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.items.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))) };
        let __go_clone_2_0 = self.v.clone();
        let __go_clone_3_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            p: __go_clone_0_0,
            items: __go_clone_1_0,
            v: __go_clone_2_0,
            n: __go_clone_3_0,
        }
    }
}

impl<T: Any + Send + Sync + 'static> Clone for C<T> {
    fn clone(&self) -> Self {
        Self {
            p: self.p.clone(),
            items: self.items.clone(),
            v: self.v.clone(),
            n: self.n.clone(),
        }
    }
}


impl<T: Any + Send + Sync + 'static> Default for C<T> {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(P::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            p: __go_default_0_0,
            items: __go_default_1_0,
            v: __go_default_2_0,
            n: __go_default_3_0,
        }
    }
}

impl<T: Any + Send + Sync + 'static> std::fmt::Display for C<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.items));
        let __go_fmt_2 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


impl<T: Any + Send + Sync + 'static> C<T> {
    pub fn later_nested(&mut self) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> {
        let mut c_closure_clone = (*self).clone(); Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = (*c_closure_clone.p.lock().unwrap().as_ref().unwrap()).x.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> () + Send + Sync>)))
    }

    pub fn later_direct(&mut self) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> {
        let mut c_closure_clone = (*self).clone(); Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = c_closure_clone.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> () + Send + Sync>)))
    }

    pub fn later_items(&mut self) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> {
        let mut c_closure_clone = (*self).clone(); Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = { let __seq = { let __seq_holder = c_closure_clone.items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.clone(); let __rhs = 5; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> () + Send + Sync>)))
    }

    pub fn self_(&self) -> Arc<Mutex<Option<C<T>>>> {
        Arc::new(Mutex::new(Some(self.clone())))
    }
}

/// A pointer method of a struct with type parameters can hand on its
/// receiver: a closure over it and a returned receiver are the same struct.
fn main() {
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut c = Arc::new(Mutex::new(Some(C::<i32> { ..Default::default() })));
    let mut f = { let __recv = c.clone(); let __recv_ptr: *mut C<i32> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut C<i32> }; let __result = unsafe { &mut *__recv_ptr }.later_nested(); __result };
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = f.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = f.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    println!("{} {}", format!("{}", "nested field through a closure".to_string()), format!("{}", (*(*(*c.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())));

    let mut g = { let __recv = c.clone(); let __recv_ptr: *mut C<i32> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut C<i32> }; let __result = unsafe { &mut *__recv_ptr }.later_direct(); __result };
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = g.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    println!("{} {}", format!("{}", "direct field through a closure".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut k = { let __recv = c.clone(); let __recv_ptr: *mut C<i32> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut C<i32> }; let __result = unsafe { &mut *__recv_ptr }.later_items(); __result };
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = k.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    println!("{} {}", format!("{}", "array of structs through a closure".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

    let mut d = { let __recv = c.clone(); let __recv_ptr: *const C<i32> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const C<i32> }; let __result = unsafe { &*__recv_ptr }.self_(); __result };
    { let new_val = 50; *(*d.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
    { let new_val = 60; *(*(*d.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "returned receiver".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*c.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())));

    let mut v = Arc::new(Mutex::new(Some({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    { let new_val = 7; *(*v.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
    { let new_val = 8; *(*(*v.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 9; *{ let __seq = { let __seq_holder = (*v.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {} {} {} {}", format!("{}", "copy".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*c.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*v.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = (*v.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));
}