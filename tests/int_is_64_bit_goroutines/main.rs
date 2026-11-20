use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
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

pub(crate) static memo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<i64>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *memo.lock().unwrap() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<i64, Arc<Mutex<Option<i64>>>>::new();
        *memo.lock().unwrap() = Some(__go_map);
    }
}


pub fn fib(n: Arc<Mutex<Option<i64>>>) -> i64 {
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    {
        let (mut v, mut ok) = { let __map = { let __map_holder = memo.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v })) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };;
        }
    }
    let mut v = Arc::new(Mutex::new(Some({ let __tmp_x = fib(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))); let __tmp_y = fib(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y })))); __tmp_x + __tmp_y })));
    { let __map_key = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*memo.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn factorial(n: Arc<Mutex<Option<i64>>>) -> i64 {
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x <= __tmp_y } {
        return 1;
    }
    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = factorial(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))); __tmp_x * __tmp_y };
}

fn main() {
    __go_init_all();
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();
    println!("{} {}", format!("{}", fib(Arc::new(Mutex::new(Some(90))))), format!("{}", factorial(Arc::new(Mutex::new(Some(20))))));
    let mut big = Arc::new(Mutex::new(Some(1099511627776i64)));
    println!("{} {} {} {}", format!("{}", { let __v = (*big.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = { let __v = (*big.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x * __tmp_y }), format!("{}", 9223372036854775807i64), format!("{}", i64::MIN));
    let mut total: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100000; __tmp_x < __tmp_y } {
        { let __rhs = { let __bin_i = (*i.lock().unwrap().as_ref().unwrap()).clone(); __bin_i * __bin_i }; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let (mut n, _) = { let __atoi_input = "9000000000".to_string(); match __atoi_input.parse::<i64>() { Ok(n) => (n, Arc::new(Mutex::new(None))), Err(_) => (0 as i64, Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input))))))) } };
    println!("{} {}", format!("{}", { let __tmp_x = n; let __tmp_y = 1; __tmp_x + __tmp_y }), format!("{}", (*Arc::new(Mutex::new(Some((({ let __tmp_x = n; let __tmp_y = 2; __tmp_x * __tmp_y })).to_string()))).lock().unwrap().as_ref().unwrap())));
    let mut wrapped = Arc::new(Mutex::new(Some(9223372036854775807i64)));
    { let mut guard = wrapped.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{}", format!("{}", { let __tmp_x = { let __v = (*wrapped.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i64::MIN; __tmp_x == __tmp_y }));
    let mut x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(7i64) as Box<dyn Any + Send + Sync>)));
    let (_, mut isInt) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i64>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(0))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), false)
        }
    });
    print!("{} {} {}\n", format!("{}", { let __type_name = __go_type_name(&**x.lock().unwrap().as_ref().unwrap()); __type_name }), format!("{}", isInt), format!("{}", 5000000000i64));
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(('A' as i32))));
    print!("{} {} {}\n", format!("{}", "int32"), format!("{}", { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as u8 as char));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
