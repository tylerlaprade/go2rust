use go2rust_stdlib_stubs::*;
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
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
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
                |current| current.checked_sub(1),
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
                |current| current.checked_sub(1),
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

#[derive(Clone, Default)]
pub struct Reader {
    pub decoder: Arc<Mutex<Option<example_com_importedembed_base::Decoder>>>,
    pub name: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.decoder.lock().unwrap().as_ref().unwrap()), (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct pkgReader {
    pub pkg_decoder: Arc<Mutex<Option<example_com_importedembed_base::PkgDecoder>>>,
}

impl std::fmt::Display for pkgReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.pkg_decoder.lock().unwrap().as_ref().unwrap()))
    }
}


impl pkgReader {
    pub fn new_reader(&self, delta: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Reader>>> {
        return Arc::new(Mutex::new(Some(Reader { decoder: self.new_decoder(Arc::new(Mutex::new(Some((*delta.lock().unwrap().as_ref().unwrap()).clone())))), name: Arc::new(Mutex::new(Some("frompkg".to_string()))), ..Default::default() })));
    }

    pub fn retire_reader(&self, r: Arc<Mutex<Option<Reader>>>) {
        self.retire_decoder((*r.lock().unwrap().as_ref().unwrap()).decoder.clone());
    }

    pub fn new_decoder(&self, _arg0: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<example_com_importedembed_base::Decoder>>> {
        let embedded = self.pkg_decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.new_decoder(_arg0)
    }

    pub fn retire_decoder(&self, _arg0: Arc<Mutex<Option<example_com_importedembed_base::Decoder>>>) {
        let embedded = self.pkg_decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.retire_decoder(_arg0)
    }
}

impl Reader {
    pub fn add(&self, _arg0: Arc<Mutex<Option<i32>>>) {
        let embedded = self.decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.add(_arg0)
    }

    pub fn clone(&self) -> Arc<Mutex<Option<example_com_importedembed_base::Decoder>>> {
        let embedded = self.decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.clone()
    }

    pub fn label(&self, _arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let embedded = self.decoder.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.label(_arg0)
    }

    pub fn snapshot(&self) -> Arc<Mutex<Option<i32>>> {
        let embedded = self.decoder.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.snapshot()
    }
}

pub fn force_concurrent_wrappers() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap();
}

fn main() {
    force_concurrent_wrappers();
    let mut r = Arc::new(Mutex::new(Some(Reader { decoder: Arc::new(Mutex::new(Some(example_com_importedembed_base::Decoder { value: Arc::new(Mutex::new(Some(3))), ..Default::default() }))), name: Arc::new(Mutex::new(Some("reader".to_string()))), ..Default::default() })));
    (*r.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(4))));
    println!("{}", (*(*r.lock().unwrap().as_mut().unwrap()).label(Arc::new(Mutex::new(Some("reader".to_string())))).lock().unwrap().as_ref().unwrap()));
    println!("{}", (*(*r.lock().unwrap().as_ref().unwrap()).snapshot().lock().unwrap().as_ref().unwrap()));
    let mut copied = Arc::new(Mutex::new(Some(Reader { decoder: (*r.lock().unwrap().as_mut().unwrap()).clone(), name: Arc::new(Mutex::new(Some("copy".to_string()))), ..Default::default() })));
    println!("{}", (*(*copied.lock().unwrap().as_mut().unwrap()).label(Arc::new(Mutex::new(Some("copy".to_string())))).lock().unwrap().as_ref().unwrap()));
    let mut pr = Arc::new(Mutex::new(Some(pkgReader { pkg_decoder: Arc::new(Mutex::new(Some(example_com_importedembed_base::PkgDecoder { base: Arc::new(Mutex::new(Some(10))), ..Default::default() }))), ..Default::default() })));
    let mut fromPkg = (*pr.lock().unwrap().as_mut().unwrap()).new_reader(Arc::new(Mutex::new(Some(5))));
    println!("{}", (*(*fromPkg.lock().unwrap().as_mut().unwrap()).label(Arc::new(Mutex::new(Some("frompkg".to_string())))).lock().unwrap().as_ref().unwrap()));
    (*pr.lock().unwrap().as_mut().unwrap()).retire_reader(fromPkg.clone());
    println!("{}", (*(*fromPkg.lock().unwrap().as_mut().unwrap()).label(Arc::new(Mutex::new(Some("retired".to_string())))).lock().unwrap().as_ref().unwrap()));
}