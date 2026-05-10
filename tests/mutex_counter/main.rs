use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


struct GoMutex {
    inner: std::sync::Arc<std::sync::Mutex<()>>,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new(std::sync::Mutex::new(())),
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
        GoMutex {
            inner: self.inner.clone(),
        }
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
    }
}

#[derive(Debug, Clone, Default)]
pub struct Counter {
    pub mu: GoMutex,
    pub value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


impl Counter {
    pub fn increment(&mut self) {
        let __mutex_guard_source_129 = self.mu.clone(); let __mutex_guard_129 = __mutex_guard_source_129.lock();
        // mu.Unlock() handled by RAII guard
        { let __target = self.value.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&self) -> Rc<RefCell<Option<i32>>> {
        let __mutex_guard_source_209 = self.mu.clone(); let __mutex_guard_209 = __mutex_guard_source_209.lock();
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