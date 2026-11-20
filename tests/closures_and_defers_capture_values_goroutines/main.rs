use std::any::Any;
use std::cell::{RefCell};
use std::sync::{Arc, Mutex};
use std::thread;


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
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>));
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

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    __text
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

pub fn deferred() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut x = Arc::new(Mutex::new(Some(1)));
        let x_defer_captured = Arc::new(Mutex::new((*x.lock().unwrap()).clone())); __defer_stack.push(Box::new(move || {
        println!("{} {}", format!("{}", "deferred x:".to_string()), format!("{}", { let __v = (*x_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    }));
        { let new_val = 2; *x.lock().unwrap() = Some(new_val); };
        let mut i = Arc::new(Mutex::new(Some(3)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let i_defer_captured = Arc::new(Mutex::new((*i.lock().unwrap()).clone())); __defer_stack.push(Box::new(move || {
        print!("{}{}", format!("{}", { let __v = (*i_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()));
    }));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let mut name = Arc::new(Mutex::new(Some("first".to_string())));
        let __defer_arg_0 = Arc::new(Mutex::new(Some((*name.lock().unwrap().as_ref().unwrap()).clone()))); __defer_stack.push(Box::new(move || {
        (move |n: Arc<Mutex<Option<String>>>| {
        println!("{} {}", format!("{}", "arg:".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
        })(__defer_arg_0);
    }));
        { let new_val = "second".to_string(); *name.lock().unwrap() = Some(new_val); };
        let name_defer_captured = name.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        println!("{} {}", format!("{}", "closure sees:".to_string()), format!("{}", { let __v = (*name_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        println!("{} {}", format!("{}", "x is".to_string()), format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            std::panic::set_hook(__go_previous_panic_hook);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                    go_store_panic_payload(__go_defer_panic);
                }
            }
            std::panic::set_hook(__go_previous_panic_hook);
            go_resume_unrecovered_panic();
            ()
        }
    }
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();

    let mut funcs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn FnMut() -> i32 + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        let i_closure_clone = Arc::new(Mutex::new((*i.lock().unwrap()).clone())); { let new_val = { let __append_target = funcs.clone(); let __append_value = Arc::new(Mutex::new(Some(Box::new(move || -> i32 {
        return { let __bin_i = (*i_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __bin_i * __bin_i };
    }) as Box<dyn FnMut() -> i32 + Send + Sync>))); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; funcs = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __range_holder = funcs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter().cloned() {
        print!("{}{}", format!("{}", {
            let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        }), format!("{}", " ".to_string()));
    } }
    println!();

    let mut printers: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    let (mut i, mut s) = (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some("a".to_string()))));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        let i_closure_clone = Arc::new(Mutex::new((*i.lock().unwrap()).clone())); let s_closure_clone = Arc::new(Mutex::new((*s.lock().unwrap()).clone())); { let new_val = { let __append_target = printers.clone(); let __append_value = Arc::new(Mutex::new(Some(Box::new(move || {
        print!("{}{}{}", format!("{}", { let __v = (*i_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*s_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; printers = new_val; };
        {
            let __tmp_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y };
            let __tmp_1 = format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "a".to_string());
            *i.lock().unwrap() = Some(__tmp_0);
            *s.lock().unwrap() = Some(__tmp_1);
        };
    }
    { let __range_holder = printers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter().cloned() {
        {
            let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
                let mut __f_guard = p.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        };
    } }
    println!();

    let mut total = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        let i_closure_clone = Arc::new(Mutex::new((*i.lock().unwrap()).clone())); let mut total_closure_clone = total.clone(); let mut add = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __rhs = (*i_closure_clone.lock().unwrap().as_ref().unwrap()); let mut guard = total_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> () + Send + Sync>)));
        {
            let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
                let mut __f_guard = add.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", "total:".to_string()), format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut count = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        let mut i_closure_clone = i.clone(); let mut step = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __rhs = 3; let mut guard = i_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> () + Send + Sync>)));
        {
            let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
                let mut __f_guard = step.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        };
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", "steps:".to_string()), format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    deferred();
    println!();
}