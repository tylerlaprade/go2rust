use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


fn format_any(value: &(dyn Any + Send + Sync)) -> String {
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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Missing>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

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

#[derive(Clone, Copy)]
struct GoAnyTypeMetadata {
    pub kind: &'static str,
    pub comparable: bool,
    pub elem_kind: Option<&'static str>,
    pub elem_comparable: bool,
}

fn go_any_type_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_key(value: &(dyn Any + Send + Sync)) -> usize {
    value as *const (dyn Any + Send + Sync) as *const () as usize
}

fn go_register_any_type<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_register_any_type_with_elem<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

// Boxes value as itself and records what kind of Go type it is. Every reader
// of an interface value looks for the value's own type in the box, so the
// box must hold nothing else: a wrapper around the value would be a type no
// reader knows.
fn go_box_any_with_metadata<T: Any + Send + Sync + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any + Send + Sync> {
    go_register_any_type::<T>(kind, comparable);
    Box::new(value) as Box<dyn Any + Send + Sync>
}

fn go_register_any_value_metadata(value: &(dyn Any + Send + Sync), kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_any_type_metadata(value: &(dyn Any + Send + Sync)) -> Option<GoAnyTypeMetadata> {
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_errorString;

impl std::fmt::Display for errors_errorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_errorString>")
    }
}

impl std::error::Error for errors_errorString {}


impl errors_errorString {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("errors_errorString.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_joinError;

impl std::fmt::Display for errors_joinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_joinError>")
    }
}

impl std::error::Error for errors_joinError {}


impl errors_joinError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        panic!("errors_joinError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone)]
pub struct Missing {
    pub name: Arc<Mutex<Option<String>>>,
}

impl Missing {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Missing {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Clone)]
pub struct Reader {
    pub err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>,
    pub read: Arc<Mutex<Option<i32>>>,
}

impl Reader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        let __go_clone_1_0 = { let __guard = self.read.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            err: __go_clone_0_0,
            read: __go_clone_1_0,
        }
    }
}


impl Default for Reader {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            err: __go_default_0_0,
            read: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.lock().unwrap().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        let __go_fmt_1 = format!("{}", (*self.read.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub static ErrNotFound: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrClosed: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrNotFound.lock().unwrap() = None;
    *ErrClosed.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("not found".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrNotFound.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("closed".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrClosed.lock().unwrap() = new_val; }
}


impl Missing {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "missing ".to_string(), (*self.name.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for Missing {}


impl Reader {
    pub fn err(&self) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        { let __err_source = { let __field = self.err.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }
    }

    pub fn fail(&mut self, err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        { let __rhs_holder = err.clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *self.err.lock().unwrap() = new_val; };
    }
}

pub fn run(fail: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
    if { let __v = (*fail.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("boom".to_string())))));
    }
    return Arc::new(Mutex::new(None));
}

pub fn find(key: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
    if { let __tmp_x = (*key.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    }
    if { let __tmp_x = (*key.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "closed".to_string(); __tmp_x == __tmp_y } {
        return { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    }
    return Arc::new(Mutex::new(None));
}

pub fn lookup(key: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    {
        let mut err = find(Arc::new(Mutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });;
        }
    }
    (
        (*key.lock().unwrap().as_ref().unwrap()).len() as i32,
        Arc::new(Mutex::new(None))
    )
}

pub fn report(mut err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    println!("{} {}", format!("{}", "report:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    *err.lock().unwrap() = None;
}

pub fn classify(err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    { let _switch_val = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    if (*_switch_val.lock().unwrap()).is_none() {
            return Arc::new(Mutex::new(Some("ok".to_string())));
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __right_guard = __right_holder.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } } {
            return Arc::new(Mutex::new(Some("not found".to_string())));
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __right_guard = __right_holder.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } } {
            return Arc::new(Mutex::new(Some("closed".to_string())));
        }
    }
    return Arc::new(Mutex::new(Some(format!("{}{}", "other: ".to_string(), (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())))));
}

/// work replaces its own result in a deferred function.
pub fn work(fail: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut err_defer_captured = err.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        if { let __nil_result = (*err_defer_captured.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("wrapped: {}", { let __text = match err_defer_captured.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *err_defer_captured.lock().unwrap() = new_val; };
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        if { let __v = (*fail.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("inner".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *err.lock().unwrap() = new_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
    }
        {
        *err.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
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
            err.clone()
        }
    }
}

fn main() {
    __go_init_all();
        // A goroutine makes every shared value a locked one.
    let mut done = GoChannel::<Option<Arc<dyn StdError + Send + Sync>>>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send({ let __err_handle = ErrClosed.clone(); let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value });;;
    });
    let mut first = Arc::new(Mutex::new(done.recv().unwrap_or_default()));
    println!("{} {}", format!("{}", { let __left = first.clone(); let __right = ErrClosed.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __text = match first.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // Assigning an error leaves the one it was assigned from.
    let mut last: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut err = run(Arc::new(Mutex::new(Some(true))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = err.clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *last.lock().unwrap() = new_val; };
    }
    println!("{} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*last.lock().unwrap()).is_some(); __nil_result }));
    println!("{} {}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match last.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A copy is a variable of its own.
    let mut e1 = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("first".to_string())))));
    let mut e2 = { let __err_source = e1.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("second".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *e1.lock().unwrap() = new_val; };
    println!("{} {}", format!("{}", { let __text = match e1.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match e2.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A package-level error can be returned, compared and returned again.
    println!("{} {} {}", format!("{}", { let __text = match find(Arc::new(Mutex::new(Some("".to_string())))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match find(Arc::new(Mutex::new(Some("closed".to_string())))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match find(Arc::new(Mutex::new(Some("x".to_string())))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {}", format!("{}", { let __left = find(Arc::new(Mutex::new(Some("".to_string())))).clone(); let __right = ErrNotFound.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __left = find(Arc::new(Mutex::new(Some("".to_string())))).clone(); let __right = ErrClosed.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __nil_result = (*find(Arc::new(Mutex::new(Some("x".to_string())))).lock().unwrap()).is_none(); __nil_result }));
    println!("{} {} {} {}", format!("{}", (*classify(find(Arc::new(Mutex::new(Some("".to_string()))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*classify(find(Arc::new(Mutex::new(Some("closed".to_string()))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*classify(find(Arc::new(Mutex::new(Some("x".to_string()))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*classify({ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }).lock().unwrap().as_ref().unwrap())));
    {
        let (mut n, mut err) = lookup(Arc::new(Mutex::new(Some("".to_string()))));;
        if { let __left = err.clone(); let __right = ErrNotFound.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq } {
            println!("{} {} {}", format!("{}", "lookup:".to_string()), format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
        }
    }
    let mut got = find(Arc::new(Mutex::new(Some("".to_string()))));
    *got.lock().unwrap() = None;
    println!("{} {} {}", format!("{}", { let __nil_result = (*got.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __text = match ErrNotFound.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = find(Arc::new(Mutex::new(Some("".to_string())))).clone(); let __right = ErrNotFound.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));

        // Two errors with the same text are two errors. A copy is the same error.
    let mut a = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("same".to_string())))));
    let mut b = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("same".to_string())))));
    let mut c = { let __err_source = a.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    println!("{} {} {} {}", format!("{}", { let __left = a.clone(); let __right = b.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __left = a.clone(); let __right = c.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __left = a.clone(); let __right = b.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; !__eq }), format!("{}", { let __left = c.clone(); let __right = a.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));

        // Passing an error does not give the variable away, and the function
        // cannot change it.
    report({ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    report({ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // An error in a struct field.
    let mut r = Arc::new(Mutex::new(Some(Reader { err: Default::default(), read: Arc::new(Mutex::new(Some(0))) })));
    { let __recv = r.clone(); let __recv_ptr: *mut Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Reader }; let __result = unsafe { &mut *__recv_ptr }.fail({ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }); __result };
    let mut held = { let __recv = r.clone(); let __recv_ptr: *const Reader = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Reader }; let __result = unsafe { &*__recv_ptr }.err(); __result };
    *held.lock().unwrap() = None;
    println!("{} {} {} {}", format!("{}", { let __nil_result = (*held.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __text = match { let __recv = r.clone(); let __recv_ptr: *const Reader = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Reader }; let __result = unsafe { &*__recv_ptr }.err(); __result }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = (*r.lock().unwrap().as_ref().unwrap()).err.clone(); let __right = err.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let __rhs_holder = ErrClosed.clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *(*r.lock().unwrap().as_ref().unwrap()).err.lock().unwrap() = new_val; };
    println!("{} {}", format!("{}", { let __left = { let __recv = r.clone(); let __recv_ptr: *const Reader = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Reader }; let __result = unsafe { &*__recv_ptr }.err(); __result }.clone(); let __right = ErrClosed.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", (*classify({ let __recv = r.clone(); let __recv_ptr: *const Reader = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Reader }; let __result = unsafe { &*__recv_ptr }.err(); __result }).lock().unwrap().as_ref().unwrap())));

        // Errors in a slice and in a map.
    let mut list = Arc::new(Mutex::new(Some(vec![{ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, { let __err_source = last.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))])));
    { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, item) in __range_values.iter().cloned().enumerate() {
        println!("{} {} {}", format!("{}", i), format!("{}", { let __text = match item.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = item.clone(); let __right = err.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));
    } }
    { let new_val = { let __append_target = list.clone(); let __append_value = { let __err_source = e2.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; list = new_val; };
    println!("{} {} {}", format!("{}", (*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __text = match { let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(4) as usize].clone() }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut byName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>::from([("a".to_string(), { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }), ("b".to_string(), { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) })]))));
    println!("{} {} {}", format!("{}", { let __text = match { let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = { let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __right = ErrClosed.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __nil_result = (*{ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"c".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).is_none(); __nil_result }));

        // A custom error type.
    let mut m: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(Missing { name: Arc::new(Mutex::new(Some("key".to_string()))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut n = { let __err_source = m.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    *m.lock().unwrap() = None;
    println!("{} {}", format!("{}", { let __nil_result = (*m.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __text = match n.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    *last.lock().unwrap() = None;
    println!("{} {}", format!("{}", { let __nil_result = (*last.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result }));

        // An interface value holds a copy as well: it does not follow the
        // variable it was made from.
    let mut boxed: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else { Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } })));
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("changed".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *err.lock().unwrap() = new_val; };
    println!("{} {}", format!("{}", { let __text = format_any(boxed.lock().unwrap().as_ref().unwrap().as_ref()); __text }), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // So does what a closure returns.
    let err_closure_clone = err.clone(); let mut check = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        return { let __err_source = err_closure_clone.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> + Send + Sync>)));
    let mut fromClosure = {
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> + Send + Sync> = {
            let mut __f_guard = check.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    *fromClosure.lock().unwrap() = None;
    println!("{} {} {}", format!("{}", { let __nil_result = (*fromClosure.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = {
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> + Send + Sync> = {
            let mut __f_guard = check.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.clone(); let __right = err.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));

        // A deferred function can still replace a named result.
    println!("{} {}", format!("{}", { let __text = match work(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match work(Arc::new(Mutex::new(Some(false)))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
