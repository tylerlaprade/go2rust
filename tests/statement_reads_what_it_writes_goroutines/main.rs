use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
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

#[derive(Debug, Clone, Default)]
pub struct Box_ {
    pub items: Arc<Mutex<Option<Vec<i64>>>>,
}

impl Box_ {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.items.clone();
        Self {
            items: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Box_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.items));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Pt {
    pub x: Arc<Mutex<Option<i64>>>,
}

impl Pt {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Pt {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Box_ {
    pub fn dup(&mut self) {
        { let new_val = { let __append_target = self.items.clone(); let __append_value = { let __seq = { let __seq_holder = self.items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = self.items.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.items = new_val; };
    }
}

/// A statement may read the slice or map it writes to: in the index, in the
/// value, or in what it appends. The read must be finished before the write
/// takes its lock.
fn main() {
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send("started".to_string());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut s = Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64])));
    { let __idx = ({ let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __rhs = 5; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "index reads the slice".to_string()), format!("{}", format_slice(&s)));
    { let __idx = (0) as usize; let __rhs = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "value reads the slice".to_string()), format!("{}", format_slice(&s)));
    { let __idx = ({
        let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let __tmp_y = 1;
        __tmp_x - __tmp_y
    }) as usize; let __rhs = 7; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "index is an element".to_string()), format!("{}", format_slice(&s)));
    { let __idx = (0) as usize; let __rhs = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let mut __seq_guard = s.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + __rhs; };
    { let __idx = (1) as usize; let mut __seq_guard = s.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
    {
        let __go_lhs_sequence_0 = s.clone();
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = s.clone();
        let __go_lhs_index_1 = (2) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    println!("{} {}", format!("{}", "compound, increment, swap".to_string()), format!("{}", format_slice(&s)));

    let mut s2 = Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64])));
    { let __idx = ({ let __tmp_x = ((*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let mut __seq_guard = s2.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
    { let __idx = ({ let __tmp_x = ((*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __rhs = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let mut __seq_guard = s2.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + __rhs; };
    {
        let __go_lhs_sequence_0 = s2.clone();
        let __go_lhs_index_0 = ({ let __tmp_x = ((*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
        let __go_lhs_sequence_1 = s2.clone();
        let __go_lhs_index_1 = (0) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    println!("{} {}", format!("{}", "index reads the slice in an increment, a compound assignment and a swap".to_string()), format!("{}", format_slice(&s2)));

    let mut grid = Arc::new(Mutex::new(Some(vec![vec![1i64, 2i64], vec![3i64, 4i64]])));
    { let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = { let __seq = { let __seq_holder = grid.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }[(0) as usize].clone(); (*grid.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    { let __outer_idx = ({ let __tmp_x = ((*grid.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __idx = (0) as usize; let __rhs = 5; (*grid.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "slice of slices".to_string()), format!("{}", { let __seq = { let __seq_holder = grid.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }[(1) as usize].clone()), format!("{}", { let __seq = { let __seq_holder = grid.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }[(0) as usize].clone()));

    let mut pts = Arc::new(Mutex::new(Some(vec![Pt { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Pt { x: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    { let __idx = ({ let __tmp_x = ((*pts.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __rhs = { let __seq = { let __seq_holder = pts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.__go_value_clone(); (*pts.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "struct element".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = pts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

    let mut arr = Arc::new(Mutex::new(Some([1i64, 2i64, 3i64])));
    let mut p = arr.clone();
    { let __idx = ({ let __tmp_x = 3; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __rhs = { let __seq = { let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; (*p.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "through a pointer to an array".to_string()), format!("{}", format_slice(&arr)));

    let mut w = Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string()])));
    { let __idx = ({ let __tmp_x = ((*w.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __rhs = format!("{}{}", { let __seq = { let __seq_holder = w.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }, "!".to_string()); (*w.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "strings".to_string()), format!("{}", format_slice(&w)));

    let mut a = Arc::new(Mutex::new(Some([1i64, 2i64, 3i64])));
    { let __idx = ({ let __tmp_x = 3; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __rhs = { let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; (*a.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "array".to_string()), format!("{}", format_slice(&a)));

    let mut b = Arc::new(Mutex::new(Some(Box_ { items: Arc::new(Mutex::new(Some(vec![1i64, 2i64]))), ..Default::default() })));
    { let __idx = ({ let __tmp_x = (({ let __len_target = { let __field = (*b.lock().unwrap().as_ref().unwrap()).items.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __rhs = 9; (*(*b.lock().unwrap().as_ref().unwrap()).items.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    { let __idx = (0) as usize; let __rhs = { let __seq = { let __seq_holder = (*b.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; (*(*b.lock().unwrap().as_ref().unwrap()).items.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", "slice in a field".to_string()), format!("{}", format_slice_values(&(*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).items.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())));

    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64)))), ("b".to_string(), Arc::new(Mutex::new(Some(2i64))))]))));
    { let __map_holder = m.clone(); let __key = "a".to_string(); let __rhs = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); };
    { let __map_key = "c".to_string(); let __map_value = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) } as i64); let __tmp_y = ((*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); __tmp_x + __tmp_y }))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", "map".to_string()), format!("{}", format_map(&m)));

    let mut n = Arc::new(Mutex::new(Some(vec![1i64, 2i64])));
    { let new_val = { let __append_target = n.clone(); let __append_value = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; n = new_val; };
    { let new_val = { let __append_target = n.clone(); let __append_value = (*n.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; n = new_val; };
    { let new_val = { let __append_target = n.clone(); let __append_values = vec![
        { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() },
        { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() },
    ]; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; n = new_val; };
    println!("{} {}", format!("{}", "append reads the slice".to_string()), format!("{}", format_slice(&n)));
    { let new_val = { let __append_target = n.clone(); let __append_values: Vec<_> = { let __slice_holder = n.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; n = new_val; };
    println!("{} {} {}", format!("{}", "append the slice to itself".to_string()), format!("{}", format_slice(&n)), format!("{}", (*n.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    { let new_val = { let __append_target = n.clone(); let __append_values: Vec<_> = { let __slice_holder = Arc::new(Mutex::new(Some({
        let __seq_holder = n.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        drop(__seq_guard);
        let __low = (10) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; n = new_val; };
    println!("{} {} {} {}", format!("{}", "append a part of the slice".to_string()), format!("{}", (*n.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(12) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(13) as usize].clone() }));

    { let new_val = { let __append_target = (*b.lock().unwrap().as_ref().unwrap()).items.clone(); let __append_value = { let __seq = { let __seq_holder = (*b.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; (*b.lock().unwrap().as_mut().unwrap()).items = new_val; };
    (*b.lock().unwrap().as_mut().unwrap()).dup();
    println!("{} {}", format!("{}", "append to a field".to_string()), format!("{}", format_slice_values(&(*{ let __field = (*b.lock().unwrap().as_ref().unwrap()).items.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())));

    let n_closure_clone = n.clone(); let mut total = Arc::new(Mutex::new(Some(Box::new(move || -> i64 {
        return (*n_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64;
    }) as Box<dyn FnMut() -> i64 + Send + Sync>)));
    { let new_val = { let __append_target = n.clone(); let __append_value = {
        let __f_ptr: *mut Box<dyn FnMut() -> i64 + Send + Sync> = {
            let mut __f_guard = total.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(

        )
    }; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; n = new_val; };
    println!("{} {}", format!("{}", "append what a function read from the slice".to_string()), format!("{}", { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*n.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }));

    let mut byKey = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i64>>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(vec![1i64]))))]))));
    { let __map_key = "a".to_string(); let __map_value = { let __slice = { let __map_holder = byKey.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&"a".to_string()).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = { let __seq = { let __seq_holder = { let __map = { let __map_holder = byKey.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*byKey.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "a".to_string(); let __map_value = { let __slice = { let __map_holder = byKey.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&"a".to_string()).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = (*{ let __map = { let __map_holder = byKey.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*byKey.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", "append to a slice in a map".to_string()), format!("{}", format_slice(&{ let __map = { let __map_holder = byKey.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
}