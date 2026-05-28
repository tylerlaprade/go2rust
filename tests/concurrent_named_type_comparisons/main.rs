use std::fmt::{Display, Formatter};
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

pub const INVALID: i8 = 0;
pub const FIELD: i8 = 1;
pub const METHOD: i8 = 2;


#[derive(Debug, Clone, Default)]
pub struct Kind(pub Arc<Mutex<Option<i8>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        self.0.lock().unwrap().as_ref().unwrap() == other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialEq<i8> for Kind {
    fn eq(&self, other: &i8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialOrd<i8> for Kind {
    fn partial_cmp(&self, other: &i8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for i8 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for i8 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = Kind;
    fn add(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i8> for Kind {
    type Output = Kind;
    fn add(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Kind> for i8 {
    type Output = Kind;
    fn add(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Kind {
    type Output = Kind;
    fn sub(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i8> for Kind {
    type Output = Kind;
    fn sub(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Kind> for i8 {
    type Output = Kind;
    fn sub(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i8> for Kind {
    type Output = Kind;
    fn bitand(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Kind> for i8 {
    type Output = Kind;
    fn bitand(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i8> for Kind {
    type Output = Kind;
    fn bitor(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Kind> for i8 {
    type Output = Kind;
    fn bitor(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i8> for Kind {
    type Output = Kind;
    fn bitxor(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Kind> for i8 {
    type Output = Kind;
    fn bitxor(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Kind {
    type Output = Kind;
    fn not(self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Kind {
    type Output = Kind;
    fn shl(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Kind {
    type Output = Kind;
    fn shl(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Kind {
    type Output = Kind;
    fn shl(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Kind {
    type Output = Kind;
    fn shl(self, other: i16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Kind {
    type Output = Kind;
    fn shl(self, other: i64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Kind {
    type Output = Kind;
    fn shl(self, other: u32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Kind {
    type Output = Kind;
    fn shl(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Kind {
    type Output = Kind;
    fn shl(self, other: u16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Kind {
    type Output = Kind;
    fn shl(self, other: u64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Kind {
    type Output = Kind;
    fn shl(self, other: usize) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Kind {
    type Output = Kind;
    fn shr(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Kind {
    type Output = Kind;
    fn shr(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Kind {
    type Output = Kind;
    fn shr(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Kind {
    type Output = Kind;
    fn shr(self, other: i16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Kind {
    type Output = Kind;
    fn shr(self, other: i64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Kind {
    type Output = Kind;
    fn shr(self, other: u32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Kind {
    type Output = Kind;
    fn shr(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Kind {
    type Output = Kind;
    fn shr(self, other: u16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Kind {
    type Output = Kind;
    fn shr(self, other: u64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Kind {
    type Output = Kind;
    fn shr(self, other: usize) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Kind {}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: Arc<Mutex<Option<Kind>>>,
}

impl Symbol {
    pub fn __go_value_clone(&self) -> Self {
        Self { kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Symbol {
    fn default() -> Self {
        Self { kind: Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.kind.lock().unwrap().as_ref().unwrap()))
    }
}


impl Symbol {
    pub fn is_field(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = Kind(Arc::new(Mutex::new(Some(FIELD as i8)))); __tmp_x == __tmp_y };
    }

    pub fn is_not_method(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = Kind(Arc::new(Mutex::new(Some(METHOD as i8)))); __tmp_x != __tmp_y };
    }

    pub fn has_field_flag(&self) -> bool {
        return { let __tmp_x = Kind(Arc::new(Mutex::new(Some(((*(*self.kind.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & FIELD))))); let __tmp_y = Kind(Arc::new(Mutex::new(Some(0 as i8)))); __tmp_x != __tmp_y } && { let __tmp_x = Kind(Arc::new(Mutex::new(Some(((*(*self.kind.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) | METHOD))))); let __tmp_y = Kind(Arc::new(Mutex::new(Some(INVALID as i8)))); __tmp_x != __tmp_y };
    }

    pub fn kind_name(&self) -> Arc<Mutex<Option<String>>> {
        { let _switch_val = { let __selector_holder = self.kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (FIELD) {
            return Arc::new(Mutex::new(Some("field".to_string())));
        } else if _switch_val == (METHOD) {
            return Arc::new(Mutex::new(Some("method".to_string())));
        } else {
            return Arc::new(Mutex::new(Some("invalid".to_string())));
        }
    }
    }
}

fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let mut sym = Arc::new(Mutex::new(Some(Symbol { kind: Arc::new(Mutex::new(Some(FIELD))), ..Default::default() })));
    let done_thread = done.clone(); let sym_thread = Arc::new(Mutex::new(Some((*sym.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        done_thread.send((*sym_thread.lock().unwrap().as_ref().unwrap()).is_field());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
    println!("{}", format!("{}", (*sym.lock().unwrap().as_ref().unwrap()).is_not_method()));
    println!("{}", format!("{}", (*sym.lock().unwrap().as_ref().unwrap()).has_field_flag()));
    println!("{}", format!("{}", (*(*sym.lock().unwrap().as_ref().unwrap()).kind_name().lock().unwrap().as_ref().unwrap())));
}