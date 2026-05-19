use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


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
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
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

pub fn receive(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    {
        { let new_val = ch.recv().unwrap(); *err.lock().unwrap() = new_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err
    }
}

pub fn select_receive(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {

    loop {
        if let Some(err) = ch.try_recv() {
            let mut err = Arc::new(Mutex::new(err));
            return err.clone();
        }
        return Arc::new(Mutex::new(None));
    }
}

pub fn local_receive(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {

    let mut err = Arc::new(Mutex::new(ch.recv().unwrap()));
    return err.clone();
}

pub fn comma_receive(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> (Arc<Mutex<Option<bool>>>, Arc<Mutex<Option<bool>>>) {

    let (mut err, mut ok) = match ch.recv() { Some(v) => (Arc::new(Mutex::new(v)), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)), Arc::new(Mutex::new(Some(false)))) };
    return (Arc::new(Mutex::new(Some((*err.lock().unwrap()).is_some()))), ok.clone());
}

pub fn comma_assign(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> (Arc<Mutex<Option<bool>>>, Arc<Mutex<Option<bool>>>) {

    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    match ch.recv() { Some(v) => { *err.lock().unwrap() = v; *ok.lock().unwrap() = Some(true); }, None => { *err.lock().unwrap() = None::<Box<dyn StdError + Send + Sync>>; *ok.lock().unwrap() = Some(false); } };
    return (Arc::new(Mutex::new(Some((*err.lock().unwrap()).is_some()))), ok.clone());
}

fn main() {
    let mut ch = GoChannel::<Option<Box<dyn StdError + Send + Sync>>>::new_buffered(2 as usize);
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    ch.send({ let __err_handle = err.clone(); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    println!("{}", format!("{}", (*receive(ch.clone()).lock().unwrap()).is_none()));
    ch.send({ let __err_handle = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("boom".to_string())))); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    println!("{}", format!("{}", (*select_receive(ch.clone()).lock().unwrap()).is_some()));
    ch.send({ let __err_handle = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("local".to_string())))); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    println!("{}", format!("{}", (*local_receive(ch.clone()).lock().unwrap()).is_some()));
    ch.send({ let __err_handle = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("comma".to_string())))); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    let (mut hasErr, mut ok) = comma_receive(ch.clone());
    println!("{} {}", format!("{}", { let __v = (*hasErr.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    ch.send({ let __err_handle = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("assign".to_string())))); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    { let (__tmp_0, __tmp_1) = comma_assign(ch.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *hasErr.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *ok.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", { let __v = (*hasErr.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}