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
pub struct Cache {
    pub mu: GoMutex,
    pub n: Rc<RefCell<Option<i64>>>,
}

impl Cache {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.mu.clone();
        let __go_clone_1_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            mu: __go_clone_0_0,
            n: __go_clone_1_0,
        }
    }
}


impl Default for Cache {
    fn default() -> Self {
        let __go_default_0_0 = GoMutex::new();
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            mu: __go_default_0_0,
            n: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Cache {
    pub fn set(&mut self, v: Rc<RefCell<Option<i64>>>) {
        { let new_val = v.borrow().as_ref().unwrap().clone(); *self.n.borrow_mut() = Some(new_val); };
    }

    pub fn update(&mut self) {
        let __mutex_guard_source_156 = self.mu.clone(); let __mutex_guard_156 = __mutex_guard_source_156.guard();
        // mu.Unlock() handled by RAII guard
        self.set(Rc::new(RefCell::new(Some(7))));
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(Cache { mu: GoMutex::new(), n: Rc::new(RefCell::new(Some(0))) })));
    (*c.borrow_mut().as_mut().unwrap()).update();
    println!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));
}