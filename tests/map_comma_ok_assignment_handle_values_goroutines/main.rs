use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
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

/// This is the concurrent-wrapper twin of the ordinary fixture. The map's
/// entry must keep its handle when comma-ok assignment writes an existing
/// variable.
fn main() {
    let mut functions = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(Box::new(move |prefix: Arc<Mutex<Option<String>>>| -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __v }, "function".to_string()))));
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>))))]))));
    let mut function = Arc::new(Mutex::new(Some(Box::new(move |prefix: Arc<Mutex<Option<String>>>| -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __v }, "old function".to_string()))));
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
    let mut functionPointer = function.clone();
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = functions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __target_callable = v.clone(); Arc::new(Mutex::new(Some(Box::new(move |__arg0| { let mut __callable = __target_callable.lock().unwrap(); (*__callable.as_mut().unwrap())(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>))) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *function.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {} {}", format!("{}", "function".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = (*{
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = {
            let mut __f_guard = function.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            Arc::new(Mutex::new(Some("new ".to_string())))
        )
    }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "new function".to_string(); __tmp_x == __tmp_y }), format!("{}", { let __tmp_x = (*{ let __f_holder = { let __map = { let __map_holder = functions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("map ".to_string())))) }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "map function".to_string(); __tmp_x == __tmp_y }), format!("{}", {
        let __left = function.clone();
        let __right = functionPointer.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100000; __tmp_x < __tmp_y } {
        { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = functions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __target_callable = v.clone(); Arc::new(Mutex::new(Some(Box::new(move |__arg0| { let mut __callable = __target_callable.lock().unwrap(); (*__callable.as_mut().unwrap())(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>))) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *function.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        std::panic::panic_any(Box::new("function disappeared during repeated map reads".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {} {} {}", format!("{}", "function repeated".to_string()), format!("{}", { let __tmp_x = (*{
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = {
            let mut __f_guard = function.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            Arc::new(Mutex::new(Some("new ".to_string())))
        )
    }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "new function".to_string(); __tmp_x == __tmp_y }), format!("{}", { let __tmp_x = (*{ let __f_holder = { let __map = { let __map_holder = functions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("map ".to_string())))) }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "map function".to_string(); __tmp_x == __tmp_y }), format!("{}", {
        let __left = function.clone();
        let __right = functionPointer.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = functions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"missing".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __target_callable = v.clone(); Arc::new(Mutex::new(Some(Box::new(move |__arg0| { let mut __callable = __target_callable.lock().unwrap(); (*__callable.as_mut().unwrap())(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>))) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *function.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {} {}", format!("{}", "function missing".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_result = (*function.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __tmp_x = (*{ let __f_holder = { let __map = { let __map_holder = functions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("map ".to_string())))) }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "map function".to_string(); __tmp_x == __tmp_y }), format!("{}", {
        let __left = function.clone();
        let __right = functionPointer.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));

    let mut interfaces = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(Box::new("interface".to_string()) as Box<dyn Any + Send + Sync>))))]))));
    let mut iface: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("old interface".to_string()) as Box<dyn Any + Send + Sync>)));
    let mut interfacePointer = iface.clone();
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = interfaces.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().as_ref().map(|__value| go_any_clone(__value.as_ref())); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *iface.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {} {}", format!("{}", "interface".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ({
        let val = iface.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    })), format!("{}", ({
        let val = { let __map = { let __map_holder = interfaces.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    })), format!("{}", {
        let __left = iface.clone();
        let __right = interfacePointer.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = interfaces.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"missing".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().as_ref().map(|__value| go_any_clone(__value.as_ref())); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *iface.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {} {}", format!("{}", "interface missing".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_result = (*iface.lock().unwrap()).is_none(); __nil_result }), format!("{}", ({
        let val = { let __map = { let __map_holder = interfaces.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    })), format!("{}", {
        let __left = iface.clone();
        let __right = interfacePointer.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));

    let mut errorsByName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("error".to_string()))))))]))));
    let mut err = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("old error".to_string())))));
    let mut errorPointer = err.clone();
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = errorsByName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {} {}", format!("{}", "error".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", { let __map = { let __map_holder = errorsByName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())), format!("{}", {
        let __left = err.clone();
        let __right = errorPointer.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = errorsByName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"missing".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.lock().unwrap().clone(); Arc::new(Mutex::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {} {}", format!("{}", "error missing".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", { let __map = { let __map_holder = errorsByName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())), format!("{}", {
        let __left = err.clone();
        let __right = errorPointer.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));

    let mut channels = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<GoChannel<i32>>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(GoChannel::<i32>::new_buffered(1 as usize)))))]))));
    let mut channel = GoChannel::<i32>::new();
    { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = channels.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&"k".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.lock().unwrap().as_ref().unwrap().clone(), true), None => (Default::default(), false) } }; channel = __tmp_0; *ok.lock().unwrap() = Some(__tmp_1); };
    println!("{} {} {} {}", format!("{}", "channel".to_string()), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", !channel.is_nil()), format!("{}", !{ let __map = { let __map_holder = channels.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| { let __guard = __v.lock().unwrap(); __guard.as_ref().cloned().unwrap_or_default() }).unwrap_or_else(|| Default::default()) }.is_nil()));
}