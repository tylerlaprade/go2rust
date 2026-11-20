use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
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
    if val.is::<Arc<Mutex<Option<notFound>>>>() { return "*main.notFound" }
    if val.is::<Arc<Mutex<Option<pointerError>>>>() { return "*main.pointerError" }
    if val.is::<notFound>() { return "main.notFound" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct notFound {
    pub name: Arc<Mutex<Option<String>>>,
}

impl notFound {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for notFound {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for notFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct pointerError {
    pub code: Arc<Mutex<Option<i64>>>,
}

impl pointerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for pointerError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


impl notFound {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", (*self.name.clone().lock().unwrap().as_ref().unwrap()), " not found".to_string()))));
    }
}

impl StdError for notFound {}


impl pointerError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("pointer {}", (*self.code.lock().unwrap().as_ref().unwrap())))))
    }
}

impl StdError for pointerError {}


pub fn error_text(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> (bool, Arc<Mutex<Option<String>>>) {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<notFound>>>>()).is_some() {
        let err = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()) { Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<pointerError>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<notFound>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else { unreachable!("type switch error case matched no error payload") };
        drop(_ts_guard);
        return (
            true,
            Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap()))))
        );;
    } else {
        let err = _ts_subject.clone();
        drop(_ts_guard);
        return (false, Arc::new(Mutex::new(Some("".to_string()))));;
    }
    }
    unreachable!()
}

pub fn replace_then_error_text(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, p: Arc<Mutex<Option<pointerError>>>) -> Arc<Mutex<Option<String>>> {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<notFound>>>>()).is_some() {
        let err = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()) { Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<pointerError>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<notFound>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else { unreachable!("type switch error case matched no error payload") };
        drop(_ts_guard);
        { let new_val = pointerError { code: Arc::new(Mutex::new(Some(10))), ..Default::default() }; *p.lock().unwrap() = Some(new_val); };;
        return Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap()))));;
    } else {
        let err = _ts_subject.clone();
        drop(_ts_guard);
        return Arc::new(Mutex::new(Some("not error".to_string())));;
    }
    }
    unreachable!()
}

pub fn is_error(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<notFound>>>>()).is_some() {
        drop(_ts_guard);
        return true;;
    } else {
        drop(_ts_guard);
        return false;;
    }
    }
    unreachable!()
}

pub fn run() {
    let mut value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(notFound { name: Arc::new(Mutex::new(Some("key".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>)));
    let (mut valueErr, mut valueOK) = ({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<notFound>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    let (mut valueCase, mut valueText) = error_text(value.clone());
    println!("{} {} {} {}", format!("{}", valueOK), format!("{}", valueCase), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", valueErr.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", ({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { (*__err_handle.lock().unwrap()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() { Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync> } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<notFound>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))))).lock().unwrap().as_ref().unwrap())));

    let mut pointerValue = Arc::new(Mutex::new(Some(pointerError { code: Arc::new(Mutex::new(Some(7))), ..Default::default() })));
    let mut pointer: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(pointerValue.clone()) as Box<dyn Any + Send + Sync>)));
    let (mut pointerErr, mut pointerOK) = ({
        let val = pointer.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<notFound>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    let (mut pointerCase, mut pointerText) = error_text(pointer.clone());
    println!("{} {} {} {}", format!("{}", pointerOK), format!("{}", pointerCase), format!("{}", { let __v = (*pointerText.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", ({
        let val = pointer.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { (*__err_handle.lock().unwrap()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() { Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync> } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<notFound>>>>() { Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))))).lock().unwrap().as_ref().unwrap())));
    { let new_val = pointerError { code: Arc::new(Mutex::new(Some(8))), ..Default::default() }; *pointerValue.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", "assertion replacement".to_string()), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", pointerErr.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "switch replacement".to_string()), format!("{}", (*replace_then_error_text(pointer.clone(), pointerValue.clone()).lock().unwrap().as_ref().unwrap())));

    let mut valuePointer: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(notFound { name: Arc::new(Mutex::new(Some("pointer key".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>)));
    let (mut valuePointerErr, mut valuePointerOK) = ({
        let val = valuePointer.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<notFound>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    let (mut valuePointerCase, _) = error_text(valuePointer.clone());
    println!("{} {} {}", format!("{}", valuePointerOK), format!("{}", valuePointerCase), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", valuePointerErr.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));

    let mut nilPointer: Arc<Mutex<Option<pointerError>>> = Arc::new(Mutex::new(None));
    let mut typedNil: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(nilPointer.clone()) as Box<dyn Any + Send + Sync>)));
    let (mut typedNilErr, mut typedNilOK) = ({
        let val = typedNil.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<notFound>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    let mut typedNilCase = is_error(typedNil.clone());
    println!("{} {} {}", format!("{}", typedNilOK), format!("{}", typedNilCase), format!("{}", { let __nil_result = (*typedNilErr.lock().unwrap()).is_none(); __nil_result }));

    let mut text: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("plain".to_string()) as Box<dyn Any + Send + Sync>)));
    let (_, mut textOK) = ({
        let val = text.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<notFound>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    let (mut textCase, _) = error_text(text.clone());
    let (mut textErr, mut textErrOK) = ({
        let val = text.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<pointerError>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<notFound>>>>() {
                (Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), false)
        }
    });
    println!("{} {} {} {}", format!("{}", textOK), format!("{}", textCase), format!("{}", textErrOK), format!("{}", { let __nil_result = (*textErr.lock().unwrap()).is_none(); __nil_result }));

    let _ = &valueText;
    let _ = &pointerErr;
}

fn main() {
    let mut done = GoChannel::<AnonymousStruct1>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        run();;
        done_thread.close();;;
    });
    done.recv().unwrap_or_default();
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}
