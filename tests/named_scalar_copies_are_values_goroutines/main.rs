use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
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

#[derive(Debug, Default)]
pub struct Counter(pub Arc<Mutex<Option<i64>>>);

impl Clone for Counter {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Counter(Arc::new(Mutex::new(__value)))
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Counter {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Counter {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Counter> for i64 {
    fn eq(&self, other: &Counter) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Counter> for i64 {
    fn partial_cmp(&self, other: &Counter) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Counter {
    type Output = Counter;
    fn add(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Counter {
    type Output = Counter;
    fn add(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Counter> for i64 {
    type Output = Counter;
    fn add(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Counter {
    type Output = Counter;
    fn sub(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Counter {
    type Output = Counter;
    fn sub(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Counter> for i64 {
    type Output = Counter;
    fn sub(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Counter {
    type Output = Counter;
    fn mul(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Counter {
    type Output = Counter;
    fn mul(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Counter> for i64 {
    type Output = Counter;
    fn mul(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Counter {
    type Output = Counter;
    fn div(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Counter {
    type Output = Counter;
    fn div(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Counter> for i64 {
    type Output = Counter;
    fn div(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Counter {
    type Output = Counter;
    fn neg(self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Counter {
    type Output = Counter;
    fn rem(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Counter {
    type Output = Counter;
    fn rem(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Counter> for i64 {
    type Output = Counter;
    fn rem(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Counter {
    type Output = Counter;
    fn bitand(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Counter {
    type Output = Counter;
    fn bitand(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Counter> for i64 {
    type Output = Counter;
    fn bitand(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Counter {
    type Output = Counter;
    fn bitor(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Counter {
    type Output = Counter;
    fn bitor(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Counter> for i64 {
    type Output = Counter;
    fn bitor(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Counter {
    type Output = Counter;
    fn bitxor(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Counter {
    type Output = Counter;
    fn bitxor(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Counter> for i64 {
    type Output = Counter;
    fn bitxor(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Counter {
    type Output = Counter;
    fn not(self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Counter {
    type Output = Counter;
    fn shl(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Counter {
    type Output = Counter;
    fn shl(self, other: i32) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Counter {
    type Output = Counter;
    fn shl(self, other: i8) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Counter {
    type Output = Counter;
    fn shl(self, other: i16) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Counter {
    type Output = Counter;
    fn shl(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Counter {
    type Output = Counter;
    fn shl(self, other: u32) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Counter {
    type Output = Counter;
    fn shl(self, other: u8) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Counter {
    type Output = Counter;
    fn shl(self, other: u16) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Counter {
    type Output = Counter;
    fn shl(self, other: u64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Counter {
    type Output = Counter;
    fn shl(self, other: usize) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Counter {
    type Output = Counter;
    fn shr(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Counter {
    type Output = Counter;
    fn shr(self, other: i32) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Counter {
    type Output = Counter;
    fn shr(self, other: i8) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Counter {
    type Output = Counter;
    fn shr(self, other: i16) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Counter {
    type Output = Counter;
    fn shr(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Counter {
    type Output = Counter;
    fn shr(self, other: u32) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Counter {
    type Output = Counter;
    fn shr(self, other: u8) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Counter {
    type Output = Counter;
    fn shr(self, other: u16) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Counter {
    type Output = Counter;
    fn shr(self, other: u64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Counter {
    type Output = Counter;
    fn shr(self, other: usize) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Counter {}

impl Ord for Counter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
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


#[derive(Debug, Default)]
pub struct Name(pub Arc<Mutex<Option<String>>>);

impl Clone for Name {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Name(Arc::new(Mutex::new(__value)))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Eq for Name {}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self.0.lock().unwrap().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for Name {
    type Output = Name;
    fn add(self, other: Self) -> Name {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other.0.lock().unwrap().as_ref().unwrap());
        Name(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for Name {
    type Output = Name;
    fn add(self, other: String) -> Name {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        Name(Arc::new(Mutex::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for Name {
    type Output = Name;
    fn add(self, other: &str) -> Name {
        let __joined = format!("{}{}", self.0.lock().unwrap().as_ref().unwrap(), other);
        Name(Arc::new(Mutex::new(Some(__joined))))
    }
}


#[derive(Debug, Default)]
pub struct Flag(pub Arc<Mutex<Option<bool>>>);

impl Clone for Flag {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Flag(Arc::new(Mutex::new(__value)))
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Debug, Clone)]
pub struct Stats {
    pub hits: Arc<Mutex<Option<Counter>>>,
    pub temp: Arc<Mutex<Option<Celsius>>>,
}

impl Stats {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.hits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.temp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            hits: __go_clone_0_0,
            temp: __go_clone_1_0,
        }
    }
}


impl Default for Stats {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Counter(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Celsius(Arc::new(Mutex::new(Some(0.0)))))));
        Self {
            hits: __go_default_0_0,
            temp: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.hits.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.temp.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Counter {
    pub fn inc(&mut self) {
        { let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as i64); }
    }
}

impl Celsius {
    pub fn warm(&mut self) {
        { let __rhs = 1.5; let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + __rhs); };
    }
}

impl Name {
    pub fn shout(&mut self) {
        { (*self.0.lock().unwrap().as_mut().unwrap()).push_str(&"!".to_string()); };
    }
}

impl Flag {
    pub fn toggle(&mut self) {
        { let new_val = Flag(Arc::new(Mutex::new(Some(!((*self.0.lock().unwrap().as_ref().unwrap())))))); *self = new_val; };
    }
}

pub fn bump(c: Arc<Mutex<Option<Counter>>>) -> Arc<Mutex<Option<Counter>>> {
    (*c.lock().unwrap().as_mut().unwrap()).inc();
    return { let __owned = c.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();

    let mut a = Arc::new(Mutex::new(Some(Counter(Arc::new(Mutex::new(Some(1 as i64)))))));
    let mut b = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    (*b.lock().unwrap().as_mut().unwrap()).inc();
    println!("{} {} {} {}", format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*bump(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut x = Arc::new(Mutex::new(Some(Celsius(Arc::new(Mutex::new(Some(1.0 as f64)))))));
    let mut y = { let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    (*y.lock().unwrap().as_mut().unwrap()).warm();
    println!("{} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut n = Arc::new(Mutex::new(Some(Name(Arc::new(Mutex::new(Some("go".to_string())))))));
    let mut m = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    (*m.lock().unwrap().as_mut().unwrap()).shout();
    println!("{} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut f = Arc::new(Mutex::new(Some(Flag(Arc::new(Mutex::new(Some(false)))))));
    let mut g = { let __owned = f.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    (*g.lock().unwrap().as_mut().unwrap()).toggle();
    println!("{} {}", format!("{}", { let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*g.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut s = Arc::new(Mutex::new(Some(Stats { hits: Arc::new(Mutex::new(Some(Counter(Arc::new(Mutex::new(Some(5 as i64))))))), temp: Arc::new(Mutex::new(Some(Celsius(Arc::new(Mutex::new(Some(20.0))))))), ..Default::default() })));
    let mut t = { let __owned = s.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    (*(*t.lock().unwrap().as_ref().unwrap()).hits.lock().unwrap().as_mut().unwrap()).inc();
    (*(*t.lock().unwrap().as_ref().unwrap()).temp.lock().unwrap().as_mut().unwrap()).warm();
    println!("{} {}", format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*t.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));

    let mut counters = Arc::new(Mutex::new(Some(vec![Counter(Arc::new(Mutex::new(Some(1 as i64)))), Counter(Arc::new(Mutex::new(Some(2 as i64)))), Counter(Arc::new(Mutex::new(Some(3 as i64))))])));
    let mut first = Arc::new(Mutex::new(Some(Counter(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = counters.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
    (*first.lock().unwrap().as_mut().unwrap()).inc();
    { let __range_holder = counters.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut c in __range_values.iter().cloned() {
        c.inc();
    } }
    println!("{} {}", format!("{}", format_slice(&counters)), format!("{}", { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }));

    let mut p = a.clone();
    { let __recv = p.clone(); let __recv_ptr: *mut Counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Counter }; let __result = unsafe { &mut *__recv_ptr }.inc(); __result };
    println!("{} {}", format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut byName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Counter>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(Counter(Arc::new(Mutex::new(Some(10 as i64))))))))]))));
    let mut v = Arc::new(Mutex::new(Some(Counter(Arc::new(Mutex::new(Some((*{ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| Counter(Arc::new(Mutex::new(Some(0))))) }.0.lock().unwrap().as_ref().unwrap()))))))));
    (*v.lock().unwrap().as_mut().unwrap()).inc();
    println!("{} {}", format!("{}", { let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| Counter(Arc::new(Mutex::new(Some(0))))) }), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}