use std::any::Any;
use std::collections::BTreeMap;
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

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i32>() { return "int" }
    if val.is::<i64>() { return "int64" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u32>() { return "uint" }
    if val.is::<u64>() { return "uint64" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i32>>() { return "[]int" }
    if val.is::<Vec<i64>>() { return "[]int64" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>() { return "func()" }
    if val.is::<Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>>>>>() { return "func() string" }
    if val.is::<Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>>>>>() { return "func(int) int" }
    std::any::type_name_of_val(val)
}

pub fn run(f: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = f.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
}

fn main() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();
    let mut count = Arc::new(Mutex::new(Some(0)));
    let mut count_closure_clone = count.clone(); let mut x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Box::new(move || {
        { let mut guard = count_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> () + Send + Sync>))).clone()) as Box<dyn Any + Send + Sync>)));
    let mut g = ({ let val = x.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = g.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    let (_, mut ok) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>>)), false)
        }
    });
    println!("{} {}", format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ok));
    let (mut h, mut ok) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn FnMut() -> () + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn FnMut() -> () + Send + Sync>>)), false)
        }
    });
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = h.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    println!("{} {}", format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ok));
    run(({ let val = x.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }));
    ({
        let __f_holder = { let val = x.clone(); let guard = val.lock().unwrap(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|| {
            let mut __f_guard = __f_holder.lock().unwrap();
            let __f: &mut Box<dyn FnMut() -> () + Send + Sync> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)();
            __f_result
        })()
    });
    let mut k: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let new_val = ({ let val = x.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }); k = new_val; };
    {
        let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
            let mut __f_guard = k.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    };
    println!("{}", format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut handlers = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>>::from([("inc".to_string(), Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| -> i32 {
        return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>))).clone()) as Box<dyn Any + Send + Sync>))))]))));
    let mut inc = ({ let val = { let __map = { let __map_holder = handlers.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"inc".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let guard = val.lock().unwrap(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    println!("{}", format!("{}", {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
            let mut __f_guard = inc.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            Arc::new(Mutex::new(Some(41)))
        )
    }));

    let mut items = Arc::new(Mutex::new(Some(vec![Box::new(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("hi".to_string())))
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>))).clone()) as Box<dyn Any + Send + Sync>])));
    let mut say = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        any_val.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "func() string")).clone()
    });
    println!("{}", format!("{}", (*{
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync> = {
            let mut __f_guard = say.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.lock().unwrap().as_ref().unwrap())));
    let (mut say2, mut ok) = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Arc::new(Mutex::new(None::<Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>>)), false),
        }
    });
    println!("{} {}", format!("{}", (*{
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync> = {
            let mut __f_guard = say2.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.lock().unwrap().as_ref().unwrap())), format!("{}", ok));
    { let (__tmp_0, __tmp_1) = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Arc::new(Mutex::new(None::<Box<dyn FnMut() -> () + Send + Sync>>)), false),
        }
    }); ok = __tmp_1; };
    println!("{}", format!("{}", ok));

    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>()).is_some() {
        let f = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>>()).unwrap().clone();
        drop(_ts_guard);
        {
            let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        };;
        println!("{} {}", format!("{}", "func".to_string()), format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
    }
    }
}