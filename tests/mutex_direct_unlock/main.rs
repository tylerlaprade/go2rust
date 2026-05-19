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
    pub n: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { mu: self.mu.clone(), n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self { mu: GoMutex::new(), n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl Counter {
    pub fn inc(&mut self) {
        let __mutex_guard_source_117 = self.mu.clone(); let __mutex_guard_117 = __mutex_guard_source_117.lock();
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        drop(__mutex_guard_117);
    }

    pub fn add_branch(&mut self, hit: Rc<RefCell<Option<bool>>>) {
        let __mutex_guard_source_195 = self.mu.clone(); let __mutex_guard_195 = __mutex_guard_source_195.lock();
        if (*hit.borrow().as_ref().unwrap()) {
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 10); };
        drop(__mutex_guard_195);
    } else {
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 20); };
        drop(__mutex_guard_195);
    }
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(Counter { mu: GoMutex::new(), n: Rc::new(RefCell::new(Some(0))) })));
    (*c.borrow_mut().as_mut().unwrap()).inc();
    (*c.borrow_mut().as_mut().unwrap()).add_branch(Rc::new(RefCell::new(Some(true))));
    (*c.borrow_mut().as_mut().unwrap()).add_branch(Rc::new(RefCell::new(Some(false))));
    println!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));
}