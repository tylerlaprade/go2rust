use std::collections::BTreeMap;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
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

/// Go calls the functions of a statement from left to right: the map or the
/// index of the target first, then the value or what is appended.
fn main() {
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut order = Arc::new(Mutex::new(Some("".to_string())));
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1))))]))));
    let mut order_closure_clone = order.clone(); let mut key = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<String>>> {
        { (*order_closure_clone.lock().unwrap().as_mut().unwrap()).push_str(&"key ".to_string()); };
        Arc::new(Mutex::new(Some("a".to_string())))
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>)));
    let mut order_closure_clone = order.clone(); let mut operand = Arc::new(Mutex::new(Some(Box::new(move || -> i32 {
        { (*order_closure_clone.lock().unwrap().as_mut().unwrap()).push_str(&"operand ".to_string()); };
        5
    }) as Box<dyn FnMut() -> i32 + Send + Sync>)));
    let m_closure_clone = m.clone(); let mut order_closure_clone = order.clone(); let mut pick = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>> {
        { (*order_closure_clone.lock().unwrap().as_mut().unwrap()).push_str(&"map ".to_string()); };
        return m_closure_clone.clone();
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>> + Send + Sync>)));
    { let __map_holder = {
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>> + Send + Sync> = {
            let mut __f_guard = pick.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.clone(); let __key = { let __map_key_holder = {
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync> = {
            let mut __f_guard = key.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __rhs = {
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = operand.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }; let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); };
    println!("{} {} {}", format!("{}", "compound map assignment:".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut lists = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i32>>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(vec![1]))))]))));
    let mut order_closure_clone = order.clone(); let mut value = Arc::new(Mutex::new(Some(Box::new(move || -> i32 {
        { (*order_closure_clone.lock().unwrap().as_mut().unwrap()).push_str(&"value ".to_string()); };
        2
    }) as Box<dyn FnMut() -> i32 + Send + Sync>)));
    { let new_val = "".to_string(); *order.lock().unwrap() = Some(new_val); };
    { let __map_key = { let __map_key_holder = {
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync> = {
            let mut __f_guard = key.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __map_value = { let __slice = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&{ let __map_key_holder = {
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync> = {
            let mut __f_guard = key.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<String>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = {
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = value.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*lists.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {}", format!("{}", "append to a slice in a map:".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&{ let __map = { let __map_holder = lists.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));

    let mut grid = Arc::new(Mutex::new(Some(vec![vec![1]])));
    let mut order_closure_clone = order.clone(); let mut idx = Arc::new(Mutex::new(Some(Box::new(move || -> i32 {
        { (*order_closure_clone.lock().unwrap().as_mut().unwrap()).push_str(&"index ".to_string()); };
        0
    }) as Box<dyn FnMut() -> i32 + Send + Sync>)));
    { let new_val = "".to_string(); *order.lock().unwrap() = Some(new_val); };
    { let _ = ({
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = idx.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }) as usize; { let __append_idx = ({
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = idx.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }) as usize; let __append_value = {
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = value.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }; (*grid.lock().unwrap().as_mut().unwrap())[__append_idx].push(__append_value); (*grid.lock().unwrap().as_ref().unwrap())[__append_idx].clone() }; };
    println!("{} {} {}", format!("{}", "append to an element:".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __seq = { let __seq_holder = grid.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.len()));

    let mut s = Arc::new(Mutex::new(Some(vec![1, 2])));
    { let new_val = "".to_string(); *order.lock().unwrap() = Some(new_val); };
    { let __idx = ({
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = idx.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }) as usize; let __rhs = {
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = value.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "element assignment:".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&s)));

    let mut order_closure_clone = order.clone(); let s_closure_clone = s.clone(); let mut get = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Vec<i32>>>> {
        { (*order_closure_clone.lock().unwrap().as_mut().unwrap()).push_str(&"slice ".to_string()); };
        return s_closure_clone.clone();
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Vec<i32>>>> + Send + Sync>)));
    { let new_val = "".to_string(); *order.lock().unwrap() = Some(new_val); };
    { let __seq = {
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<Vec<i32>>>> + Send + Sync> = {
            let mut __f_guard = get.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<Vec<i32>>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.clone(); let __idx = ({
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = idx.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }) as usize; let __rhs = {
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = value.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }; (*__seq.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "element of what a function returned:".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&s)));

    let mut table = Arc::new(Mutex::new(Some(vec![vec![1, 2], vec![3, 4]])));
    let mut order_closure_clone = order.clone(); let table_closure_clone = table.clone(); let mut getTable = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Vec<Vec<i32>>>>> {
        { (*order_closure_clone.lock().unwrap().as_mut().unwrap()).push_str(&"table ".to_string()); };
        return table_closure_clone.clone();
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Vec<Vec<i32>>>>> + Send + Sync>)));
    { let new_val = "".to_string(); *order.lock().unwrap() = Some(new_val); };
    { let __seq = {
        let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<Vec<Vec<i32>>>>> + Send + Sync> = {
            let mut __f_guard = getTable.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<Vec<Vec<i32>>>>> + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }.clone(); let __outer_idx = ({
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = idx.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }) as usize; let __idx = ({
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = idx.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }) as usize; let __rhs = {
        let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
            let mut __f_guard = value.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }; (*__seq.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "element of a slice of slices that a function returned:".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __seq = { let __seq_holder = table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }[(0) as usize].clone()));

    { let new_val = "".to_string(); *order.lock().unwrap() = Some(new_val); };
    { let new_val = { let __append_target = s.clone(); let __append_values = vec![
        {
            let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
                let mut __f_guard = value.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        },
        {
            let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = {
                let mut __f_guard = idx.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(

            )
        },
    ]; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; s = new_val; };
    println!("{} {} {}", format!("{}", "append:".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&s)));
}