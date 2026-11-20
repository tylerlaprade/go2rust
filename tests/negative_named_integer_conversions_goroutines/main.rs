use std::any::Any;
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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<Big>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<I8>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Level>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn format_any_slice_values(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_any(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice))
}

fn format_any_variadic(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format_any_slice_values(slice)
}

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
    if let Some(v) = value.downcast_ref::<Big>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<I8>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<Level>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }

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

pub(crate) const STEP: i32 = 5;


#[derive(Debug, Clone, Default)]
pub struct I8(pub Arc<Mutex<Option<i8>>>);

impl Display for I8 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for I8 {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i8> for I8 {
    fn eq(&self, other: &i8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for I8 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i8> for I8 {
    fn partial_cmp(&self, other: &i8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<I8> for i8 {
    fn eq(&self, other: &I8) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<I8> for i8 {
    fn partial_cmp(&self, other: &I8) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for I8 {
    type Output = I8;
    fn add(self, other: Self) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i8> for I8 {
    type Output = I8;
    fn add(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<I8> for i8 {
    type Output = I8;
    fn add(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for I8 {
    type Output = I8;
    fn sub(self, other: Self) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i8> for I8 {
    type Output = I8;
    fn sub(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<I8> for i8 {
    type Output = I8;
    fn sub(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for I8 {
    type Output = I8;
    fn mul(self, other: Self) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i8> for I8 {
    type Output = I8;
    fn mul(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<I8> for i8 {
    type Output = I8;
    fn mul(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for I8 {
    type Output = I8;
    fn div(self, other: Self) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i8> for I8 {
    type Output = I8;
    fn div(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<I8> for i8 {
    type Output = I8;
    fn div(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for I8 {
    type Output = I8;
    fn neg(self) -> I8 {
        I8(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for I8 {
    type Output = I8;
    fn rem(self, other: Self) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i8> for I8 {
    type Output = I8;
    fn rem(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<I8> for i8 {
    type Output = I8;
    fn rem(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for I8 {
    type Output = I8;
    fn bitand(self, other: Self) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i8> for I8 {
    type Output = I8;
    fn bitand(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<I8> for i8 {
    type Output = I8;
    fn bitand(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for I8 {
    type Output = I8;
    fn bitor(self, other: Self) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i8> for I8 {
    type Output = I8;
    fn bitor(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<I8> for i8 {
    type Output = I8;
    fn bitor(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for I8 {
    type Output = I8;
    fn bitxor(self, other: Self) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i8> for I8 {
    type Output = I8;
    fn bitxor(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<I8> for i8 {
    type Output = I8;
    fn bitxor(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for I8 {
    type Output = I8;
    fn not(self) -> I8 {
        I8(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for I8 {
    type Output = I8;
    fn shl(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for I8 {
    type Output = I8;
    fn shl(self, other: i32) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for I8 {
    type Output = I8;
    fn shl(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for I8 {
    type Output = I8;
    fn shl(self, other: i16) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for I8 {
    type Output = I8;
    fn shl(self, other: i64) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for I8 {
    type Output = I8;
    fn shl(self, other: u32) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for I8 {
    type Output = I8;
    fn shl(self, other: u8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for I8 {
    type Output = I8;
    fn shl(self, other: u16) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for I8 {
    type Output = I8;
    fn shl(self, other: u64) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for I8 {
    type Output = I8;
    fn shl(self, other: usize) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for I8 {
    type Output = I8;
    fn shr(self, other: I8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for I8 {
    type Output = I8;
    fn shr(self, other: i32) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for I8 {
    type Output = I8;
    fn shr(self, other: i8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for I8 {
    type Output = I8;
    fn shr(self, other: i16) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for I8 {
    type Output = I8;
    fn shr(self, other: i64) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for I8 {
    type Output = I8;
    fn shr(self, other: u32) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for I8 {
    type Output = I8;
    fn shr(self, other: u8) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for I8 {
    type Output = I8;
    fn shr(self, other: u16) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for I8 {
    type Output = I8;
    fn shr(self, other: u64) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for I8 {
    type Output = I8;
    fn shr(self, other: usize) -> I8 {
        I8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for I8 {}

impl Ord for I8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Level(pub Arc<Mutex<Option<i32>>>);

impl Display for Level {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Level {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Level {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Level> for i32 {
    fn eq(&self, other: &Level) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Level> for i32 {
    fn partial_cmp(&self, other: &Level) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Level {
    type Output = Level;
    fn add(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Level {
    type Output = Level;
    fn add(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Level> for i32 {
    type Output = Level;
    fn add(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Level {
    type Output = Level;
    fn sub(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Level {
    type Output = Level;
    fn sub(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Level> for i32 {
    type Output = Level;
    fn sub(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Level {
    type Output = Level;
    fn mul(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Level {
    type Output = Level;
    fn mul(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Level> for i32 {
    type Output = Level;
    fn mul(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Level {
    type Output = Level;
    fn div(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Level {
    type Output = Level;
    fn div(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Level> for i32 {
    type Output = Level;
    fn div(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Level {
    type Output = Level;
    fn neg(self) -> Level {
        Level(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Level {
    type Output = Level;
    fn rem(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Level {
    type Output = Level;
    fn rem(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Level> for i32 {
    type Output = Level;
    fn rem(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Level {
    type Output = Level;
    fn bitand(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Level {
    type Output = Level;
    fn bitand(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Level> for i32 {
    type Output = Level;
    fn bitand(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Level {
    type Output = Level;
    fn bitor(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Level {
    type Output = Level;
    fn bitor(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Level> for i32 {
    type Output = Level;
    fn bitor(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Level {
    type Output = Level;
    fn bitxor(self, other: Self) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Level {
    type Output = Level;
    fn bitxor(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Level> for i32 {
    type Output = Level;
    fn bitxor(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Level {
    type Output = Level;
    fn not(self) -> Level {
        Level(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Level {
    type Output = Level;
    fn shl(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Level {
    type Output = Level;
    fn shl(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Level {
    type Output = Level;
    fn shl(self, other: i8) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Level {
    type Output = Level;
    fn shl(self, other: i16) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Level {
    type Output = Level;
    fn shl(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Level {
    type Output = Level;
    fn shl(self, other: u32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Level {
    type Output = Level;
    fn shl(self, other: u8) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Level {
    type Output = Level;
    fn shl(self, other: u16) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Level {
    type Output = Level;
    fn shl(self, other: u64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Level {
    type Output = Level;
    fn shl(self, other: usize) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Level {
    type Output = Level;
    fn shr(self, other: Level) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Level {
    type Output = Level;
    fn shr(self, other: i32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Level {
    type Output = Level;
    fn shr(self, other: i8) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Level {
    type Output = Level;
    fn shr(self, other: i16) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Level {
    type Output = Level;
    fn shr(self, other: i64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Level {
    type Output = Level;
    fn shr(self, other: u32) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Level {
    type Output = Level;
    fn shr(self, other: u8) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Level {
    type Output = Level;
    fn shr(self, other: u16) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Level {
    type Output = Level;
    fn shr(self, other: u64) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Level {
    type Output = Level;
    fn shr(self, other: usize) -> Level {
        Level(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Level {}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct U8(pub Arc<Mutex<Option<u8>>>);

impl Display for U8 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for U8 {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for U8 {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for U8 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for U8 {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<U8> for u8 {
    fn eq(&self, other: &U8) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<U8> for u8 {
    fn partial_cmp(&self, other: &U8) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for U8 {
    type Output = U8;
    fn add(self, other: Self) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for U8 {
    type Output = U8;
    fn add(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<U8> for u8 {
    type Output = U8;
    fn add(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for U8 {
    type Output = U8;
    fn sub(self, other: Self) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for U8 {
    type Output = U8;
    fn sub(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<U8> for u8 {
    type Output = U8;
    fn sub(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for U8 {
    type Output = U8;
    fn mul(self, other: Self) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for U8 {
    type Output = U8;
    fn mul(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<U8> for u8 {
    type Output = U8;
    fn mul(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for U8 {
    type Output = U8;
    fn div(self, other: Self) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for U8 {
    type Output = U8;
    fn div(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<U8> for u8 {
    type Output = U8;
    fn div(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for U8 {
    type Output = U8;
    fn rem(self, other: Self) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for U8 {
    type Output = U8;
    fn rem(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<U8> for u8 {
    type Output = U8;
    fn rem(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for U8 {
    type Output = U8;
    fn bitand(self, other: Self) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for U8 {
    type Output = U8;
    fn bitand(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<U8> for u8 {
    type Output = U8;
    fn bitand(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for U8 {
    type Output = U8;
    fn bitor(self, other: Self) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for U8 {
    type Output = U8;
    fn bitor(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<U8> for u8 {
    type Output = U8;
    fn bitor(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for U8 {
    type Output = U8;
    fn bitxor(self, other: Self) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for U8 {
    type Output = U8;
    fn bitxor(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<U8> for u8 {
    type Output = U8;
    fn bitxor(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for U8 {
    type Output = U8;
    fn not(self) -> U8 {
        U8(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for U8 {
    type Output = U8;
    fn shl(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for U8 {
    type Output = U8;
    fn shl(self, other: i32) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for U8 {
    type Output = U8;
    fn shl(self, other: i8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for U8 {
    type Output = U8;
    fn shl(self, other: i16) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for U8 {
    type Output = U8;
    fn shl(self, other: i64) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for U8 {
    type Output = U8;
    fn shl(self, other: u32) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for U8 {
    type Output = U8;
    fn shl(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for U8 {
    type Output = U8;
    fn shl(self, other: u16) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for U8 {
    type Output = U8;
    fn shl(self, other: u64) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for U8 {
    type Output = U8;
    fn shl(self, other: usize) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for U8 {
    type Output = U8;
    fn shr(self, other: U8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for U8 {
    type Output = U8;
    fn shr(self, other: i32) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for U8 {
    type Output = U8;
    fn shr(self, other: i8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for U8 {
    type Output = U8;
    fn shr(self, other: i16) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for U8 {
    type Output = U8;
    fn shr(self, other: i64) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for U8 {
    type Output = U8;
    fn shr(self, other: u32) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for U8 {
    type Output = U8;
    fn shr(self, other: u8) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for U8 {
    type Output = U8;
    fn shr(self, other: u16) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for U8 {
    type Output = U8;
    fn shr(self, other: u64) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for U8 {
    type Output = U8;
    fn shr(self, other: usize) -> U8 {
        U8(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for U8 {}

impl Ord for U8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Big(pub Arc<Mutex<Option<i64>>>);

impl Display for Big {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Big {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Big {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Big {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Big {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Big> for i64 {
    fn eq(&self, other: &Big) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Big> for i64 {
    fn partial_cmp(&self, other: &Big) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Big {
    type Output = Big;
    fn add(self, other: Self) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Big {
    type Output = Big;
    fn add(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Big> for i64 {
    type Output = Big;
    fn add(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Big {
    type Output = Big;
    fn sub(self, other: Self) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Big {
    type Output = Big;
    fn sub(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Big> for i64 {
    type Output = Big;
    fn sub(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Big {
    type Output = Big;
    fn mul(self, other: Self) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Big {
    type Output = Big;
    fn mul(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Big> for i64 {
    type Output = Big;
    fn mul(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Big {
    type Output = Big;
    fn div(self, other: Self) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Big {
    type Output = Big;
    fn div(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Big> for i64 {
    type Output = Big;
    fn div(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Big {
    type Output = Big;
    fn neg(self) -> Big {
        Big(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Big {
    type Output = Big;
    fn rem(self, other: Self) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Big {
    type Output = Big;
    fn rem(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Big> for i64 {
    type Output = Big;
    fn rem(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Big {
    type Output = Big;
    fn bitand(self, other: Self) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Big {
    type Output = Big;
    fn bitand(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Big> for i64 {
    type Output = Big;
    fn bitand(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Big {
    type Output = Big;
    fn bitor(self, other: Self) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Big {
    type Output = Big;
    fn bitor(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Big> for i64 {
    type Output = Big;
    fn bitor(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Big {
    type Output = Big;
    fn bitxor(self, other: Self) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Big {
    type Output = Big;
    fn bitxor(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Big> for i64 {
    type Output = Big;
    fn bitxor(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Big {
    type Output = Big;
    fn not(self) -> Big {
        Big(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Big {
    type Output = Big;
    fn shl(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Big {
    type Output = Big;
    fn shl(self, other: i32) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Big {
    type Output = Big;
    fn shl(self, other: i8) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Big {
    type Output = Big;
    fn shl(self, other: i16) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Big {
    type Output = Big;
    fn shl(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Big {
    type Output = Big;
    fn shl(self, other: u32) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Big {
    type Output = Big;
    fn shl(self, other: u8) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Big {
    type Output = Big;
    fn shl(self, other: u16) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Big {
    type Output = Big;
    fn shl(self, other: u64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Big {
    type Output = Big;
    fn shl(self, other: usize) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Big {
    type Output = Big;
    fn shr(self, other: Big) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Big {
    type Output = Big;
    fn shr(self, other: i32) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Big {
    type Output = Big;
    fn shr(self, other: i8) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Big {
    type Output = Big;
    fn shr(self, other: i16) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Big {
    type Output = Big;
    fn shr(self, other: i64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Big {
    type Output = Big;
    fn shr(self, other: u32) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Big {
    type Output = Big;
    fn shr(self, other: u8) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Big {
    type Output = Big;
    fn shr(self, other: u16) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Big {
    type Output = Big;
    fn shr(self, other: u64) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Big {
    type Output = Big;
    fn shr(self, other: usize) -> Big {
        Big(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Big {}

impl Ord for Big {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


fn main() {
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
        started_thread.send(true);;;
    });
    started.recv().unwrap_or_default();

    let mut a = Arc::new(Mutex::new(Some(I8(Arc::new(Mutex::new(Some(-2 as i8)))))));
    let mut b = Arc::new(Mutex::new(Some(Level(Arc::new(Mutex::new(Some(-3 as i32)))))));
    let mut c: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Level(Arc::new(Mutex::new(Some(-4 as i32))))) as Box<dyn Any + Send + Sync>)));
    let mut levels = Arc::new(Mutex::new(Some(vec![Level(Arc::new(Mutex::new(Some(-5 as i32 as i32)))), Level(Arc::new(Mutex::new(Some(-6 as i32))))])));
    println!("{} {} {} {}", format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __text = format_any(c.lock().unwrap().as_ref().unwrap().as_ref()); __text }), format!("{}", format_slice(&levels)));

    println!("{} {} {} {} {} {} {}", format!("{}", Level(Arc::new(Mutex::new(Some((-3 + 1) as i32))))), format!("{}", I8(Arc::new(Mutex::new(Some(-128 as i8))))), format!("{}", Level(Arc::new(Mutex::new(Some(!0 as i32))))), format!("{}", Level(Arc::new(Mutex::new(Some(-STEP as i32))))), format!("{}", Big(Arc::new(Mutex::new(Some(-(((1 as i64) << (40 as i64))) as i64))))), format!("{}", Level(Arc::new(Mutex::new(Some(7 as i32))))), format!("{}", U8(Arc::new(Mutex::new(Some(!0 as u8 as u8))))));

    let mut n = Arc::new(Mutex::new(Some(4)));
    println!("{} {}", format!("{}", Level(Arc::new(Mutex::new(Some(-((*n.lock().unwrap().as_ref().unwrap())) as i32))))), format!("{}", Level(Arc::new(Mutex::new(Some({ let __tmp_x = -((*n.lock().unwrap().as_ref().unwrap())); let __tmp_y = 1; __tmp_x + __tmp_y } as i32))))));

    print!("{} {} {}\n", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, "main.I8");
    let mut vals = Arc::new(Mutex::new(Some(vec![Box::new(I8(Arc::new(Mutex::new(Some(-2 as i8))))) as Box<dyn Any + Send + Sync>, Box::new(Level(Arc::new(Mutex::new(Some(-9 as i32))))) as Box<dyn Any + Send + Sync>, Box::new(Big(Arc::new(Mutex::new(Some(-1 as i64))))) as Box<dyn Any + Send + Sync>])));
    println!("{}", format_any_variadic(&vals));
    { let __range_holder = vals.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for v in __range_values.iter() {
        print!("{}|", format_any(v.as_ref()));
    } }
    println!();

    if {
        let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = Level(Arc::new(Mutex::new(Some(-2 as i32))));
        __tmp_x < __tmp_y
    } {
        println!("{}", format!("{}", "below -2".to_string()));
    }
}