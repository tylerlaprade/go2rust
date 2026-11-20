use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};
use std::thread;


struct GoPointerError<T> {
    handle: Arc<Mutex<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Arc<Mutex<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.lock().unwrap();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + Send + Sync + 'static> std::error::Error for GoPointerError<T> {}

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

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    std::any::type_name_of_val(val)
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


fn main() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut boxed: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some({ let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("x".to_string()))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } })));

    let (mut got, mut ok) = ({
        let val = boxed.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    println!("{} {}", format!("{}", ok), format!("{}", { let __text = match got.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    *got.lock().unwrap() = None;
    let (mut again, mut ok) = ({
        let val = boxed.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    println!("{} {} {}", format!("{}", ok), format!("{}", { let __text = match again.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match got.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut single = Arc::new(Mutex::new(Some(({
        let val = boxed.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { (*__err_handle.lock().unwrap()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))));
    println!("{}", format!("{}", { let __text = match single.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("replaced".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *single.lock().unwrap() = new_val; };
    let (mut third, mut ok) = ({
        let val = boxed.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    println!("{} {} {}", format!("{}", ok), format!("{}", { let __text = match third.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match single.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    {
    let _ts_subject = boxed.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>()).is_some() {
        let mut v = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else { unreachable!("type switch error case matched no error payload") };
        drop(_ts_guard);
        println!("{} {}", format!("{}", "case error:".to_string()), format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
        *v.lock().unwrap() = None;;
        println!("{}", format!("{}", { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result }));;
    }
    }
    let (mut fourth, mut ok) = ({
        let val = boxed.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    println!("{} {}", format!("{}", ok), format!("{}", { let __text = match fourth.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // The copy is still the same error.
    let mut sentinel = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("sentinel".to_string())))));
    let mut held: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some({ let __err_holder = sentinel.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } })));
    let mut back = Arc::new(Mutex::new(Some(({
        let val = held.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { (*__err_handle.lock().unwrap()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))));
    println!("{} {}", format!("{}", { let __left = back.clone(); let __right = sentinel.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __text = match back.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A formatted error takes the same paths.
    let mut wrapped: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some({ let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("wrapped: {}", 7)))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } })));
    let mut w = Arc::new(Mutex::new(Some(({
        let val = wrapped.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { (*__err_handle.lock().unwrap()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))));
    let (mut w2, mut ok) = ({
        let val = wrapped.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    println!("{} {} {}", format!("{}", { let __text = match w.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match w2.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", ok));
    *w.lock().unwrap() = None;
    let mut w3 = Arc::new(Mutex::new(Some(({
        let val = wrapped.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { (*__err_handle.lock().unwrap()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))));
    println!("{} {}", format!("{}", { let __nil_result = (*w.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __text = match w3.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}