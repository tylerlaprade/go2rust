use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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
pub struct Numbers(pub Arc<Mutex<Option<Vec<i64>>>>);

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Table(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>);

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn sum(xs: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    let mut total = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let __rhs = x; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

fn main() {
        // Appending to the copy of a slice leaves the original's length alone.
    let mut prev = Arc::new(Mutex::new(Some(vec![1i64, 2i64])));
    let mut next = Arc::new(Mutex::new(Some((*prev.lock().unwrap().as_ref().unwrap()).clone())));
    { let new_val = { let __append_target = next.clone(); let __append_value = 3; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; next = new_val; };
    println!("{} {} {}", format!("{}", format_slice(&prev)), format!("{}", format_slice(&next)), format!("{}", sum(next.clone())));

    let mut a = Arc::new(Mutex::new(Some((*prev.lock().unwrap().as_ref().unwrap()).clone())));
    let mut b: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(Some((*prev.lock().unwrap().as_ref().unwrap()).clone())));
    { let new_val = { let __append_target = a.clone(); let __append_value = 4; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; a = new_val; };
    println!("{} {} {} {}", format!("{}", (*prev.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));

        // Setting the original to nil leaves the copy alone: the usual way to
        // walk a tree one level at a time.
    let mut children = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<Vec<i64>>>>>::from([(1i64, Arc::new(Mutex::new(Some(vec![2i64, 3i64])))), (2i64, Arc::new(Mutex::new(Some(vec![4i64])))), (3i64, Arc::new(Mutex::new(Some(vec![5i64, 6i64]))))]))));
    let mut queue = Arc::new(Mutex::new(Some(vec![1i64])));
    let mut level = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ((*queue.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut cur = Arc::new(Mutex::new(Some((*queue.lock().unwrap().as_ref().unwrap()).clone())));
        *queue.lock().unwrap() = None;
        println!("{} {}", format!("{}", { let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&cur)));
        { let __range_holder = cur.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter().copied() {
        { let new_val = { let __append_target = queue.clone(); let __append_values: Vec<_> = { let __slice_holder = { let __map = { let __map_holder = children.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&n)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; queue = new_val; };
    } }
        { let mut guard = level.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // A map variable copied into a new one is the same map.
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    let mut d = m.clone();
    { let __map_key = "a".to_string(); let __map_value = Arc::new(Mutex::new(Some(7))); (*d.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "b".to_string(); let __map_value = Arc::new(Mutex::new(Some(8))); (*d.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {}", format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    let mut e = m.clone();
    { let __map_key = "c".to_string(); let __map_value = Arc::new(Mutex::new(Some(9))); (*e.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"c".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))); d = new_val; };
    println!("{} {}", format!("{}", (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*d.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

        // Named slice and map types.
    let mut n = Arc::new(Mutex::new(Some(Numbers(Arc::new(Mutex::new(Some(vec![4i64, 5i64])))))));
    let mut n2 = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).clone())));
    { let new_val = { let __base = { let __named_slice = (*n2.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(6); Arc::new(Mutex::new(Some(Numbers(Arc::new(Mutex::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n2.lock().unwrap() = __moved_val; };
    println!("{} {} {}", format!("{}", { let __slice_holder = { let __named_slice = (*n.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __slice_holder = { let __named_slice = (*n2.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __seq_holder = { let __named_slice = (*n2.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(2) as usize].clone() }));
    let mut tab = Arc::new(Mutex::new(Some(Table(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(1i64))))]))))))));
    let mut tab2 = tab.clone();
    { let __map_key = "x".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*{ let __named_map = (*tab2.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", { let __map = { let __map_holder = { let __named_map = (*tab.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

        // A slice of structs: the copy reaches the same structs.
    let mut ps = Arc::new(Mutex::new(Some(vec![Point { x: Arc::new(Mutex::new(Some(1))), ..Default::default() }, Point { x: Arc::new(Mutex::new(Some(2))), ..Default::default() }])));
    let mut qs = Arc::new(Mutex::new(Some((*ps.lock().unwrap().as_ref().unwrap()).clone())));
    { let new_val = 22; *{ let __seq = { let __seq_holder = qs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{}", format!("{}", (*{ let __seq = { let __seq_holder = ps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

        // A channel variable copied into a new one is the same channel.
    let mut ch = GoChannel::<i64>::new_buffered(2 as usize);
    let mut ch2 = ch.clone();
    let mut ch3 = ch.clone();
    ch2.send(4);
    ch3.send(5);
    println!("{} {}", format!("{}", ch.recv().unwrap_or_default()), format!("{}", ch.recv().unwrap_or_default()));
}