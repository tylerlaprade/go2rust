use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


struct GoMutex {
    inner: std::sync::Mutex<()>,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Mutex::new(()),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<()> {
        self.inner.lock().unwrap()
    }
}

impl Default for GoMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GoMutex {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
    }
}

#[derive(Debug, Clone, Default)]
struct Counter {
    mu: GoMutex,
    value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


impl Counter {
    pub fn increment(&mut self) {
        let _guard = self.mu.lock();
        // mu.Unlock() handled by RAII guard
        { let mut guard = self.value.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> Rc<RefCell<Option<i32>>> {
        let _guard = self.mu.lock();
        // mu.Unlock() handled by RAII guard
        return self.value.clone();
    }
}

fn main() {
    let mut counter = Rc::new(RefCell::new(Some(Counter { mu: GoMutex::new(), value: Rc::new(RefCell::new(Some(0))) })));
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*(*counter.borrow_mut().as_mut().unwrap()).value().borrow().as_ref().unwrap()));
}