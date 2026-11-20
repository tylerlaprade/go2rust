use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
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

#[derive(Debug, Clone)]
pub struct Cache__string__User {
    pub data: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<User>>>>>>>,
    pub order: Arc<Mutex<Option<Vec<String>>>>,
    pub limit: Arc<Mutex<Option<i32>>>,
}

impl Cache__string__User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.order.clone();
        let __go_clone_2_0 = { let __guard = self.limit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            order: __go_clone_1_0,
            limit: __go_clone_2_0,
        }
    }
}


impl Default for Cache__string__User {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            data: __go_default_0_0,
            order: __go_default_1_0,
            limit: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Cache__string__User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.data));
        let __go_fmt_1 = format!("{}", format_slice(&self.order));
        let __go_fmt_2 = format!("{}", (*self.limit.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Cache__int__slice_string {
    pub data: Arc<Mutex<Option<BTreeMap<i32, Arc<Mutex<Option<Vec<String>>>>>>>>,
    pub order: Arc<Mutex<Option<Vec<i32>>>>,
    pub limit: Arc<Mutex<Option<i32>>>,
}

impl Cache__int__slice_string {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.order.clone();
        let __go_clone_2_0 = { let __guard = self.limit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            order: __go_clone_1_0,
            limit: __go_clone_2_0,
        }
    }
}


impl Default for Cache__int__slice_string {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            data: __go_default_0_0,
            order: __go_default_1_0,
            limit: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Cache__int__slice_string {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<map>");
        let __go_fmt_1 = format!("{}", format_slice(&self.order));
        let __go_fmt_2 = format!("{}", (*self.limit.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub name: Arc<Mutex<Option<String>>>,
    pub age: Arc<Mutex<Option<i32>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub trait Number: std::fmt::Display + Any {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool;
}

impl Clone for Box<dyn Number + Send + Sync> {
    fn clone(&self) -> Self {
        Number::__go_clone_box_number(self.as_ref())
    }
}

impl Number for i8 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i16 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for i64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for isize {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<isize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u8 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u16 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for u64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for usize {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<usize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for f32 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for f64 {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Number for String {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<String>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct Celsius(pub Arc<Mutex<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Celsius(Arc::new(Mutex::new(__value)))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}


impl Cache__string__User {
    pub fn put(&mut self, k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<User>>>) {
        {
        let (_, mut ok) = { let __map = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*k.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };;
        if !ok {
            { let new_val = { let __append_target = self.order.clone(); let __append_value = (*k.lock().unwrap().as_ref().unwrap()).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.order = new_val; };;
        }
    }
        { let __map_key = (*k.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = { let __owned = v.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) }; (*self.data.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        if {
            let __tmp_x = (({ let __len_target = { let __field = self.order.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
            let __tmp_y = ((*self.limit.clone().lock().unwrap().as_ref().unwrap()) as i32);
            __tmp_x > __tmp_y
        } {
        let mut oldest = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.order.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.order.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = (1) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); self.order = new_val; };
        { let __map_handle = self.data.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&(*oldest.lock().unwrap().as_ref().unwrap()).clone()); };
    }
    }

    pub fn get(&self, k: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<User>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut v, mut ok) = { let __map = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*k.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().__go_value_clone()))), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };
        if !ok {
        let mut zero: Arc<Mutex<Option<User>>> = Arc::new(Mutex::new(Some(Default::default())));
        return (
            { let __owned = zero.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) },
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("miss".to_string())))))
        );
    }
        return ({ let __owned = v.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }
}

impl Cache__int__slice_string {
    pub fn put(&mut self, k: Arc<Mutex<Option<i32>>>, v: Arc<Mutex<Option<Vec<String>>>>) {
        {
        let (_, mut ok) = { let __map = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v })) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if !ok {
            { let new_val = { let __append_target = self.order.clone(); let __append_value = (*k.lock().unwrap().as_ref().unwrap()).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.order = new_val; };;
        }
    }
        { let __map_key = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __map_value = v.clone(); (*self.data.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        if {
            let __tmp_x = (({ let __len_target = { let __field = self.order.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
            let __tmp_y = ((*self.limit.clone().lock().unwrap().as_ref().unwrap()) as i32);
            __tmp_x > __tmp_y
        } {
        let mut oldest = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.order.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = self.order.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = (1) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); self.order = new_val; };
        { let __map_handle = self.data.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&{ let __v = (*oldest.lock().unwrap().as_ref().unwrap()).clone(); __v }); };
    }
    }

    pub fn get(&self, k: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut v, mut ok) = { let __map = { let __map_holder = self.data.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v })) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        if !ok {
        let mut zero: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        return (
            zero.clone(),
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("miss".to_string())))))
        );
    }
        return (v.clone(), Arc::new(Mutex::new(None)));
    }
}

impl Celsius {
}

impl Number for Celsius {
    fn __go_clone_box_number(&self) -> Box<dyn Number + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Number + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_number(&self, other: &(dyn Number + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Celsius>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn new_cache__string___user(limit: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Cache__string__User>>> {
    Arc::new(Mutex::new(Some(Cache__string__User { data: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<User>>>>::new()))), limit: Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn new_cache__int__slice_string(limit: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Cache__int__slice_string>>> {
    Arc::new(Mutex::new(Some(Cache__int__slice_string { data: Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<Vec<String>>>>>::new()))), limit: Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn sum_by___user__int(xs: Arc<Mutex<Option<Vec<User>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i32 + Send + Sync>>>>) -> i32 {
    let mut total: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        { let __rhs = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i32 + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i32 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some((*x).__go_value_clone())))
            )
        }; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn sum_by___user__float64(xs: Arc<Mutex<Option<Vec<User>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> f64 + Send + Sync>>>>) -> f64 {
    let mut total: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        { let __rhs = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> f64 + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> f64 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some((*x).__go_value_clone())))
            )
        }; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn sum_by___celsius___celsius(xs: Arc<Mutex<Option<Vec<Celsius>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<Celsius>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Celsius>>> {
    let mut total: Arc<Mutex<Option<Celsius>>> = Arc::new(Mutex::new(Some(Celsius(Arc::new(Mutex::new(Some(0.0)))))));

    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().cloned() {
        { let __rhs = (*{
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<Celsius>>> + Send + Sync> = {
                let mut __f_guard = f.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<Celsius>>> + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some(x.clone())))
            )
        }.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + __rhs); };
    } }
    return { let __owned = total.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn group_by___user__int(xs: Arc<Mutex<Option<Vec<User>>>>, key: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i32 + Send + Sync>>>>) -> Arc<Mutex<Option<BTreeMap<i32, Arc<Mutex<Option<Vec<User>>>>>>>> {
    let mut out = Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<Vec<User>>>>>::new())));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        let x = &x.__go_value_clone();
        let mut k = {
            let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i32 + Send + Sync> = {
                let mut __f_guard = key.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i32 + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(Mutex::new(Some((*x).__go_value_clone())))
            )
        };
        { let __map_key = k; let __map_value = { let __slice = { let __map_holder = out.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&k).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = (*x).__go_value_clone(); (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*out.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    return out.clone();
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();
    let mut c = new_cache__string___user(Arc::new(Mutex::new(Some(2))));
    { let __recv = c.clone(); let __recv_ptr: *mut Cache__string__User = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Cache__string__User }; let __result = unsafe { &mut *__recv_ptr }.put(
        Arc::new(Mutex::new(Some("a".to_string()))),
        Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("Ann".to_string()))), age: Arc::new(Mutex::new(Some(30))), ..Default::default() }))),
    ); __result };
    { let __recv = c.clone(); let __recv_ptr: *mut Cache__string__User = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Cache__string__User }; let __result = unsafe { &mut *__recv_ptr }.put(
        Arc::new(Mutex::new(Some("b".to_string()))),
        Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(25))), ..Default::default() }))),
    ); __result };
    { let __recv = c.clone(); let __recv_ptr: *mut Cache__string__User = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Cache__string__User }; let __result = unsafe { &mut *__recv_ptr }.put(
        Arc::new(Mutex::new(Some("c".to_string()))),
        Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("Cy".to_string()))), age: Arc::new(Mutex::new(Some(40))), ..Default::default() }))),
    ); __result };
    let (_, mut err) = { let __recv = c.clone(); let __recv_ptr: *const Cache__string__User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Cache__string__User }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some("a".to_string())))); __result };
    let (mut u, mut err2) = { let __recv = c.clone(); let __recv_ptr: *const Cache__string__User = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Cache__string__User }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some("c".to_string())))); __result };
    println!("{} {} {} {} {}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*u.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __text = match err2.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", ({ let __len_target = { let __field = (*c.lock().unwrap().as_ref().unwrap()).data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })), format!("{}", format_slice_values(&(*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).order.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())));

    let mut users = Arc::new(Mutex::new(Some(vec![
        User { name: Arc::new(Mutex::new(Some("Ann".to_string()))), age: Arc::new(Mutex::new(Some(30))), ..Default::default() },
        User { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(25))), ..Default::default() },
        User { name: Arc::new(Mutex::new(Some("Cy".to_string()))), age: Arc::new(Mutex::new(Some(40))), ..Default::default() },
        User { name: Arc::new(Mutex::new(Some("Di".to_string()))), age: Arc::new(Mutex::new(Some(25))), ..Default::default() },
    ])));
    println!("{}", format!("{}", sum_by___user__int(users.clone(), Arc::new(Mutex::new(Some(Box::new(move |u: Arc<Mutex<Option<User>>>| -> i32 {
        return (*(*u.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap());
    }) as Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i32 + Send + Sync>))))));
    println!("{}", format!("{}", sum_by___user__float64(
        users.clone(),
        Arc::new(Mutex::new(Some(Box::new(move |u: Arc<Mutex<Option<User>>>| -> f64 {
        return {
            let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).age.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as f64))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = 2.0;
            __tmp_x / __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> f64 + Send + Sync>)))
    )));
    let mut temps = Arc::new(Mutex::new(Some(vec![Celsius(Arc::new(Mutex::new(Some(20.5)))), Celsius(Arc::new(Mutex::new(Some(22.0)))), Celsius(Arc::new(Mutex::new(Some(19.5))))])));
    println!("{}", format!("{}", (*sum_by___celsius___celsius(temps.clone(), Arc::new(Mutex::new(Some(Box::new(move |c: Arc<Mutex<Option<Celsius>>>| -> Arc<Mutex<Option<Celsius>>> {
        return { let __owned = c.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Celsius>>>) -> Arc<Mutex<Option<Celsius>>> + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut groups = group_by___user__int(users.clone(), Arc::new(Mutex::new(Some(Box::new(move |u: Arc<Mutex<Option<User>>>| -> i32 {
        return (*(*u.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap());
    }) as Box<dyn FnMut(Arc<Mutex<Option<User>>>) -> i32 + Send + Sync>))));
    let mut ages = Arc::new(Mutex::new(Some(Vec::<i32>::with_capacity(((*groups.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (age, _) in { let __range_holder = groups.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = ages.clone(); let __append_value = age; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; ages = new_val; };
    }
    { let __sort_target = ages.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = ages.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for age in __range_values.iter().copied() {
        let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        { let __range_holder = { let __map = { let __map_holder = groups.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&age)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for u in __range_values.iter() {
        let u = &u.__go_value_clone();
        { let new_val = { let __append_target = names.clone(); let __append_value = { let __selector_holder = u.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; names = new_val; };
    } }
        println!("{} {}", format!("{}", age), format!("{}", (*Arc::new(Mutex::new(Some({ let __parts = (*names.lock().unwrap()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) }))).lock().unwrap().as_ref().unwrap())));
    } }
    let mut ic = new_cache__int__slice_string(Arc::new(Mutex::new(Some(3))));
    { let __recv = ic.clone(); let __recv_ptr: *mut Cache__int__slice_string = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Cache__int__slice_string }; let __result = unsafe { &mut *__recv_ptr }.put(
        Arc::new(Mutex::new(Some(1))),
        Arc::new(Mutex::new(Some(vec!["x".to_string()]))),
    ); __result };
    let (mut got, _) = { let __recv = ic.clone(); let __recv_ptr: *const Cache__int__slice_string = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Cache__int__slice_string }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some(1)))); __result };
    println!("{} {}", format!("{}", format_slice(&got)), format!("{}", (*got.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}