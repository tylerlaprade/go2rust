use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


struct GoMutex {
    inner: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
}

struct GoMutexGuard {
    mutex: GoMutex,
    active: bool,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
        }
    }

    fn lock(&self) {
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        while *locked {
            locked = ready.wait(locked).unwrap();
        }
        *locked = true;
    }

    fn unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        if !*locked {
            panic!("sync.Mutex: unlock of unlocked mutex");
        }
        *locked = false;
        ready.notify_one();
    }

    fn guard(&self) -> GoMutexGuard {
        self.lock();
        GoMutexGuard {
            mutex: self.clone(),
            active: true,
        }
    }
}

impl Drop for GoMutexGuard {
    fn drop(&mut self) {
        if self.active {
            self.mutex.unlock();
            self.active = false;
        }
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

#[derive(Clone)]
pub struct Counter {
    pub mu: GoMutex,
    pub n: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.mu.clone();
        let __go_clone_1_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            mu: __go_clone_0_0,
            n: __go_clone_1_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = GoMutex::new();
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            mu: __go_default_0_0,
            n: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Counter {
    pub fn inc(&mut self) {
        let __mutex_guard_source_116 = self.mu.clone(); let __mutex_guard_116 = __mutex_guard_source_116.guard();
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        drop(__mutex_guard_116);
    }

    pub fn add_branch(&mut self, hit: Rc<RefCell<Option<bool>>>) {
        let __mutex_guard_source_194 = self.mu.clone(); let __mutex_guard_194 = __mutex_guard_source_194.guard();
        if (*hit.borrow().as_ref().unwrap()) {
        { let __target = self.n.clone(); let __rhs = 10; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        drop(__mutex_guard_194);
    } else {
        { let __target = self.n.clone(); let __rhs = 20; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        drop(__mutex_guard_194);
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