use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
}

fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
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

pub fn receive(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        {
        { let new_val = ch.recv().unwrap_or_default(); *err.lock().unwrap() = new_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            err.clone()
        }
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
    unreachable!()
}

pub fn local_receive(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err = Arc::new(Mutex::new(ch.recv().unwrap_or_default()));
    return err.clone();
}

pub fn comma_receive(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> (bool, bool) {
    let (mut err, mut ok) = match ch.recv() { Some(v) => (Arc::new(Mutex::new(v)), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)), Arc::new(Mutex::new(Some(false)))) };
    return (
        { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result },
        { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }
    );
}

pub fn comma_assign(ch: GoChannel<Option<Box<dyn StdError + Send + Sync>>>) -> (bool, bool) {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    match ch.recv() { Some(v) => { *err.lock().unwrap() = v; *ok.lock().unwrap() = Some(true); }, None => { *err.lock().unwrap() = None::<Box<dyn StdError + Send + Sync>>; *ok.lock().unwrap() = Some(false); } };
    return (
        { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result },
        { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }
    );
}

fn main() {
    let mut ch = GoChannel::<Option<Box<dyn StdError + Send + Sync>>>::new_buffered(2 as usize);
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    ch.send({ let __err_handle = err.clone(); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    println!("{}", format!("{}", { let __nil_result = (*receive(ch.clone()).lock().unwrap()).is_none(); __nil_result }));
    ch.send({ let __err_handle = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("boom".to_string())))); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    println!("{}", format!("{}", { let __nil_result = (*select_receive(ch.clone()).lock().unwrap()).is_some(); __nil_result }));
    ch.send({ let __err_handle = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("local".to_string())))); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    println!("{}", format!("{}", { let __nil_result = (*local_receive(ch.clone()).lock().unwrap()).is_some(); __nil_result }));
    ch.send({ let __err_handle = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("comma".to_string())))); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    let (mut hasErr, mut ok) = comma_receive(ch.clone());
    println!("{} {}", format!("{}", hasErr), format!("{}", ok));
    ch.send({ let __err_handle = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("assign".to_string())))); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() });
    { let (__tmp_0, __tmp_1) = comma_assign(ch.clone()); hasErr = __tmp_0; ok = __tmp_1; };
    println!("{} {}", format!("{}", hasErr), format!("{}", ok));
}