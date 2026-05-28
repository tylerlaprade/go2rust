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
	
#[derive(Debug, Clone)]
pub struct Counter {
    pub mu: GoMutex,
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { mu: self.mu.clone(), value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self { mu: GoMutex::new(), value: Rc::new(RefCell::new(Some(0))) }
    }
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

    pub fn value(&self) -> i32 {
        let __mutex_guard_source_209 = self.mu.clone(); let __mutex_guard_209 = __mutex_guard_source_209.lock();
        // mu.Unlock() handled by RAII guard
        return (*self.value.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut counter = Rc::new(RefCell::new(Some(Counter { mu: GoMutex::new(), value: Rc::new(RefCell::new(Some(0))) })));
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    (*counter.borrow_mut().as_mut().unwrap()).increment();
    println!("{} {}", format!("{}", "Counter value:".to_string()), format!("{}", (*counter.borrow().as_ref().unwrap()).value()));
}