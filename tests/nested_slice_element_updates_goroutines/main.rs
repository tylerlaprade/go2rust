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
fn format_nested_slice<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        format_nested_slice_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice_values<T, Inner>(slice: &[Inner]) -> String
where
    Inner: AsRef<[T]>,
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|inner| format_slice_values(inner.as_ref())).collect();
    format!("[{}]", formatted.join(" "))
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

#[derive(Debug, Clone, Default)]
pub struct Grid(pub Arc<Mutex<Option<Vec<Vec<i32>>>>>);


#[derive(Debug, Clone, Default)]
pub struct Sheet {
    pub cells: Arc<Mutex<Option<Vec<Vec<f64>>>>>,
    pub names: Arc<Mutex<Option<Vec<String>>>>,
}

impl Sheet {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.cells.clone();
        let __go_clone_1_0 = self.names.clone();
        Self {
            cells: __go_clone_0_0,
            names: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_nested_slice(&self.cells));
        let __go_fmt_1 = format!("{}", format_slice(&self.names));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Sheet {
    pub fn bump(&mut self, i: Arc<Mutex<Option<i32>>>) {
        { let __outer_idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = 0.5; let __seq_holder = self.cells.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
        { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = "!".to_string(); let __seq_holder = self.names.clone(); let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    }
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();
    let mut a = Arc::new(Mutex::new(Some(vec![vec![1, 2], vec![3, 4]])));
    { let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = 10; let mut __seq_guard = a.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
    {
        let __go_lhs_sequence_0 = a.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = a.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }[(1) as usize].clone();
        let __tmp_1 = { let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }[(0) as usize].clone();
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    { let __outer_idx = (0) as usize; let __idx = (0) as usize; let mut __seq_guard = a.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + 1; }
    { let __outer_idx = (1) as usize; let __idx = (1) as usize; let __rhs = 2; let mut __seq_guard = a.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] - __rhs; };
    let mut g = Arc::new(Mutex::new(Some(Grid(Arc::new(Mutex::new(Some(vec![vec![1, 2], vec![3, 4]])))))));
    { let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = 7; let __seq_holder = { let __named_slice = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] * __rhs; };
    {
        let __go_lhs_sequence_0 = { let __named_slice = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = { let __named_slice = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = { let __seq_holder = { let __named_slice = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }[(1) as usize].clone();
        let __tmp_1 = { let __seq_holder = { let __named_slice = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(1) as usize].clone() }[(0) as usize].clone();
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    { let __outer_idx = (0) as usize; let __idx = (0) as usize; let __seq_holder = { let __named_slice = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] - 1; }
    println!("{} {}", format!("{}", format_nested_slice(&a)), format!("{}", format_nested_slice(&{ let __named_slice = (*g.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })));

    let mut cube = Arc::new(Mutex::new(Some(vec![vec![vec![1, 2], vec![3]], vec![vec![4]]])));
    { let __outer_idx = (0) as usize; let __outer_idx_1 = (1) as usize; let __idx = (0) as usize; let __rhs = 30; (*cube.lock().unwrap().as_mut().unwrap())[__outer_idx][__outer_idx_1][__idx] = __rhs; };
    { let __outer_idx = (1) as usize; let __outer_idx_1 = (0) as usize; let __idx = (0) as usize; let __rhs = 5; let mut __seq_guard = cube.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx][__outer_idx_1]; __seq[__idx] = __seq[__idx] * __rhs; };
    { let __outer_idx = (0) as usize; let __outer_idx_1 = (0) as usize; let __idx = (1) as usize; let mut __seq_guard = cube.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx][__outer_idx_1]; __seq[__idx] = __seq[__idx] - 1; }
    println!("{} {}", format!("{}", { let __formatted = format_nested_slice_values(&{ let __seq = { let __seq_holder = cube.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }); __formatted }), format!("{}", { let __seq = { let __seq_holder = cube.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }[(0) as usize].clone()[(0) as usize].clone()));

    let mut board: Arc<Mutex<Option<[[String; 3]; 3]>>> = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 3, _>(|_| std::array::from_fn::<_, 3, _>(|_| String::new())))));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        { let __outer_idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __idx = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = ".".to_string(); (*board.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __outer_idx = (1) as usize; let __idx = (1) as usize; let __rhs = "X".to_string(); (*board.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    { let __outer_idx = (0) as usize; let __idx = (2) as usize; let __rhs = "O".to_string(); let mut __seq_guard = board.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    println!("{}", format!("{}", format_nested_slice(&board)));

    let mut grid = Arc::new(Mutex::new(Some([std::array::from_fn::<_, 3, _>(|_| 0), std::array::from_fn::<_, 3, _>(|_| 0), std::array::from_fn::<_, 3, _>(|_| 0)])));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        { let __outer_idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; (*grid.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let __outer_idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __idx = ({ let __tmp_x = 2; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize; let __rhs = 10; let mut __seq_guard = grid.lock().unwrap(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] + __rhs; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", format_nested_slice(&grid)));

    let mut paths = Arc::new(Mutex::new(Some(vec![vec![]; (4) as usize])));
    for i in 0..(({ let __range_holder = paths.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (*Arc::new(Mutex::new(Some(vec![0; (4) as usize]))).lock().unwrap().as_ref().unwrap()).clone(); (*paths.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        { let __outer_idx = (i) as usize; let __idx = (0) as usize; let __rhs = 1; (*paths.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
    let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        { let __outer_idx = (0) as usize; let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = 1; (*paths.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        let mut j = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        { let __outer_idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __idx = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = {
            let __tmp_x = { let __seq = { let __seq_holder = paths.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone();
            let __tmp_y = { let __seq = { let __seq_holder = paths.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone();
            __tmp_x + __tmp_y
        }; (*paths.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = paths.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() }[(3) as usize].clone()), format!("{}", format_nested_slice(&paths)));

    let mut s = Arc::new(Mutex::new(Some(Sheet { cells: Arc::new(Mutex::new(Some(vec![vec![1.0, 2.0], vec![3.0, 4.0]]))), names: Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string()]))), ..Default::default() })));
    (*s.lock().unwrap().as_mut().unwrap()).bump(Arc::new(Mutex::new(Some(0))));
    (*s.lock().unwrap().as_mut().unwrap()).bump(Arc::new(Mutex::new(Some(1))));
    {
        let __go_lhs_sequence_0 = (*s.lock().unwrap().as_ref().unwrap()).cells.clone();
        let __go_lhs_outer_index_0_0 = (1) as usize;
        let __go_lhs_index_0 = (0) as usize;
        let __go_lhs_sequence_1 = (*s.lock().unwrap().as_ref().unwrap()).cells.clone();
        let __go_lhs_outer_index_1_0 = (1) as usize;
        let __go_lhs_index_1 = (1) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }[(1) as usize].clone();
        let __tmp_1 = { let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).cells.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }[(0) as usize].clone();
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_outer_index_0_0][__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_outer_index_1_0][__go_lhs_index_1] = __tmp_1;
    };
    let mut words = Arc::new(Mutex::new(Some(vec!["x".to_string(), "y".to_string()])));
    { let __idx = (0) as usize; let __rhs = "z".to_string(); let mut __seq_guard = words.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    { let __idx = (1) as usize; let __rhs = { let __seq = { let __seq_holder = words.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let mut __seq_guard = words.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    let mut tags = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some("v".to_string()))))]))));
    { let __map_holder = tags.clone(); let __key = "k".to_string(); let __rhs = "w".to_string(); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(String::new())))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + &__rhs); };
    let mut score = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<f64>>>>::from([]))));
    { let __map_holder = score.clone(); let __key = "a".to_string(); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0.0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1.0); }
    { let __map_holder = score.clone(); let __key = "a".to_string(); let __rhs = 1.5; let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0.0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); };
    println!("{} {} {} {} {}", format!("{}", { let __formatted = format_nested_slice_values(&(*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).cells.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()); __formatted }), format!("{}", format_slice_values(&(*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).names.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())), format!("{}", format_slice(&words)), format!("{}", format_map(&tags)), format!("{}", format_map(&score)));
}