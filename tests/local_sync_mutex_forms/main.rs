use std::cell::{RefCell};
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

// A readers-writer lock: any number of readers, or one writer. A clone is
// the same lock, as a *sync.RWMutex is.
#[derive(Clone, Debug, Default)]
pub struct GoRWMutex {
    // (a writer holds it, how many readers hold it), and the waiters.
    inner: std::sync::Arc<(std::sync::Mutex<(bool, usize)>, std::sync::Condvar)>,
}

impl GoRWMutex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        while held.0 || held.1 > 0 {
            held = ready.wait(held).unwrap();
        }
        held.0 = true;
    }

    pub fn unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if !held.0 {
            panic!("sync: Unlock of unlocked RWMutex");
        }
        held.0 = false;
        ready.notify_all();
    }

    pub fn r_lock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        while held.0 {
            held = ready.wait(held).unwrap();
        }
        held.1 += 1;
    }

    pub fn r_unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if held.1 == 0 {
            panic!("sync: RUnlock of unlocked RWMutex");
        }
        held.1 -= 1;
        if held.1 == 0 {
            ready.notify_all();
        }
    }
}

impl std::fmt::Display for GoRWMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RWMutex")
    }
}

pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static global: GoGlobal<GoMutex> = GoGlobal::new();


fn __go_init_globals() {
    *global.borrow_mut() = Some(Default::default());
}


pub fn bump(mu: GoMutex, n: Rc<RefCell<Option<i64>>>) {
    let __mutex_guard_source_374 = mu.clone(); let __mutex_guard_374 = __mutex_guard_source_374.guard();
    { let new_val = { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v } + 1; *n.borrow_mut() = Some(new_val); };
    drop(__mutex_guard_374);
}

pub fn read(rw: GoRWMutex, n: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    rw.r_lock();
    let rw_defer_captured = rw.clone(); __defer_stack.push(Box::new(move || {
        rw_defer_captured.r_unlock();
    }));
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v };
    }
}

pub fn locked(mu: GoMutex) -> i64 {
    let __mutex_guard_source_536 = mu.clone(); let __mutex_guard_536 = __mutex_guard_source_536.guard();
    // mu.Unlock() handled by RAII guard
    7
}

fn main() {
    __go_init_all();
    let mut n = Rc::new(RefCell::new(Some(0i64)));
    let mut a = GoMutex { ..Default::default() };
    let __mutex_guard_source_620 = a.clone(); let __mutex_guard_620 = __mutex_guard_source_620.guard();
    { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    drop(__mutex_guard_620);

    let mut b = GoMutex { ..Default::default() };
    let __mutex_guard_source_670 = b.clone(); let __mutex_guard_670 = __mutex_guard_source_670.guard();
    { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    drop(__mutex_guard_670);

    let mut c = GoMutex { ..Default::default() };
    bump(c.clone(), n.clone());

    let mut d = GoMutex::new();
    bump(d.clone(), n.clone());
    println!("{} {} {}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", locked(d.clone())), format!("{}", locked(c.clone())));

    let __mutex_guard_source_804 = (*global.borrow().as_ref().unwrap()).clone(); let __mutex_guard_804 = __mutex_guard_source_804.guard();
    { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    drop(__mutex_guard_804);

    let mut rw = GoRWMutex::new();
    rw.r_lock();
    rw.r_lock();
    println!("{}", format!("{}", read(rw.clone(), n.clone())));
    rw.r_unlock();
    rw.r_unlock();
    rw.lock();
    { let new_val = 10; *n.borrow_mut() = Some(new_val); };
    rw.unlock();
    println!("{}", format!("{}", read(rw.clone(), n.clone())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
