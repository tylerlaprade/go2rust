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
pub struct Cache {
    pub mu: GoMutex,
    pub n: Rc<RefCell<Option<i32>>>,
}

impl Cache {
    pub fn __go_value_clone(&self) -> Self {
        Self { mu: self.mu.clone(), n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Cache {
    fn default() -> Self {
        Self { mu: GoMutex::new(), n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl Cache {
    pub fn set(&mut self, v: Rc<RefCell<Option<i32>>>) {
        { let new_val = v.borrow().as_ref().unwrap().clone(); *self.n.borrow_mut() = Some(new_val); };
    }

    pub fn update(&mut self) {
        let __mutex_guard_source_157 = self.mu.clone(); let __mutex_guard_157 = __mutex_guard_source_157.lock();
        // mu.Unlock() handled by RAII guard
        self.set(Rc::new(RefCell::new(Some(7))));
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(Cache { mu: GoMutex::new(), n: Rc::new(RefCell::new(Some(0))) })));
    (*c.borrow_mut().as_mut().unwrap()).update();
    println!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));
}