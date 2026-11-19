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
pub struct Node {
    pub value: Arc<Mutex<Option<i32>>>,
    pub next: Arc<Mutex<Option<Node>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            value: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            value: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Node {
    pub fn link(&self) -> Arc<Mutex<Option<Node>>> {
        self.next.clone()
    }
}

fn main() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut a = Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(1))), ..Default::default() })));
    let mut b = a.clone();
    { let new_val = Arc::new(Mutex::new(None)); b = new_val; };
    println!("{} {}", format!("{}", { let __nil_result = (*a.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*b.lock().unwrap()).is_none(); __nil_result }));

    let mut shared = Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(2))), ..Default::default() })));
    let mut first = Arc::new(Mutex::new(Some(Node { next: shared.clone(), ..Default::default() })));
    let mut second = Arc::new(Mutex::new(Some(Node { next: shared.clone(), ..Default::default() })));
    { let new_val = Arc::new(Mutex::new(None)); (*first.lock().unwrap().as_mut().unwrap()).next = new_val; };
    println!("{} {}", format!("{}", { let __nil_target = (*first.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __nil_target = (*second.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result }));

    let mut c = Arc::new(Mutex::new(Some(Node {
        next: Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(3))), ..Default::default() }))).clone(),
        ..Default::default()
    })));
    let mut p = { let __recv = c.clone(); let __recv_ptr: *const Node = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Node }; let __result = unsafe { &*__recv_ptr }.link(); __result };
    { let new_val = Arc::new(Mutex::new(None)); p = new_val; };
    println!("{} {}", format!("{}", { let __nil_target = (*c.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result }));

    let mut parallel = a.clone();
    let mut count = Arc::new(Mutex::new(Some(0)));
    {
        let __tmp_0 = Arc::new(Mutex::new(None));
        let __tmp_1 = 2;
        parallel = __tmp_0.clone();
        *count.lock().unwrap() = Some(__tmp_1);
    };
    println!("{} {} {}", format!("{}", { let __nil_result = (*a.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*parallel.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    {
        let __tmp_0 = Arc::new(Mutex::new(None));
        let __tmp_1 = 3;
        (*second.lock().unwrap().as_mut().unwrap()).next = __tmp_0.clone();
        *count.lock().unwrap() = Some(__tmp_1);
    };
    println!("{} {} {}", format!("{}", { let __nil_result = (*shared.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_target = (*second.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut captured = Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(4))), ..Default::default() })));
    let mut captured_closure_clone = captured.clone(); let mut clear = Arc::new(Mutex::new(Some(Box::new(move || {
        *captured_closure_clone.lock().unwrap() = None;
    }) as Box<dyn FnMut() -> () + Send + Sync>)));
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = clear.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    println!("{}", format!("{}", { let __nil_result = (*captured.lock().unwrap()).is_none(); __nil_result }));
}