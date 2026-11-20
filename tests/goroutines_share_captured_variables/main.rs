use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
}

fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    __text
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}

struct WaitGroup {
    count: std::sync::Arc<(std::sync::Mutex<i32>, std::sync::Condvar)>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            count: std::sync::Arc::new((std::sync::Mutex::new(0), std::sync::Condvar::new())),
        }
    }

    fn add(&self, n: i32) {
        let (lock, _) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count += n;
    }

    fn done(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count -= 1;
        if *count <= 0 {
            cvar.notify_all();
        }
    }

    fn wait(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        WaitGroup {
            count: self.count.clone(),
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaitGroup")
    }
}

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

#[derive(Clone)]
pub struct tally {
    pub mu: GoRWMutex,
    pub count: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>,
}

impl tally {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.mu.clone();
        let __go_clone_1_0 = self.count.clone();
        Self {
            mu: __go_clone_0_0,
            count: __go_clone_1_0,
        }
    }
}


impl Default for tally {
    fn default() -> Self {
        let __go_default_0_0 = GoRWMutex::new();
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            mu: __go_default_0_0,
            count: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for tally {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.count));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl tally {
    pub fn add(&mut self, key: Arc<Mutex<Option<String>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.mu.lock();
            let mut t_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        t_defer_captured.mu.unlock();
    }));
            { let __map_holder = self.count.clone(); let __key = (*key.lock().unwrap().as_ref().unwrap()).clone(); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                std::panic::set_hook(__go_previous_panic_hook);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                        go_store_panic_payload(__go_defer_panic);
                    }
                }
                std::panic::set_hook(__go_previous_panic_hook);
                go_resume_unrecovered_panic();
                ()
            }
        }
    }

    pub fn get(&self, key: Arc<Mutex<Option<String>>>) -> i32 {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.mu.r_lock();
            let mut t_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        t_defer_captured.mu.r_unlock();
    }));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return { let __map = { let __map_holder = self.count.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*key.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) };
    }
        }));
        match __go_panic_result {
            Ok(__go_value) => {
                std::panic::set_hook(__go_previous_panic_hook);
                __go_value
            }
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                        go_store_panic_payload(__go_defer_panic);
                    }
                }
                std::panic::set_hook(__go_previous_panic_hook);
                go_resume_unrecovered_panic();
                0 as i32
            }
        }
    }
}

pub fn square(i: Arc<Mutex<Option<i32>>>, results: Arc<Mutex<Option<Vec<i32>>>>, mu: GoMutex, total: Arc<Mutex<Option<i32>>>, wg: WaitGroup) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let wg_defer_captured = wg.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));
        { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = { let __bin_i = (*i.lock().unwrap().as_ref().unwrap()).clone(); __bin_i * __bin_i }; (*results.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        let __mutex_guard_source_663 = mu.clone(); let __mutex_guard_663 = __mutex_guard_source_663.guard();
        { let __rhs = (*i.lock().unwrap().as_ref().unwrap()); let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        drop(__mutex_guard_663);

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            std::panic::set_hook(__go_previous_panic_hook);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                    go_store_panic_payload(__go_defer_panic);
                }
            }
            std::panic::set_hook(__go_previous_panic_hook);
            go_resume_unrecovered_panic();
            ()
        }
    }
}

fn main() {
    let mut total = Arc::new(Mutex::new(Some(0)));
    let mut results = Arc::new(Mutex::new(Some(vec![0; (4) as usize])));
    let mut counts = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([]))));
    let mut mu = GoMutex::new();
    let mut wg = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        wg.add(1);
        let counts_thread = counts.clone(); let i_thread = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); let mu_thread = mu.clone(); let results_thread = results.clone(); let mut total_thread = total.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
        mu_thread.lock();;
        { let __rhs = 10; let mut guard = total_thread.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };;
        { let __map_holder = counts_thread.clone(); let __key = "n".to_string(); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); };
        mu_thread.unlock();;
        { let __idx = ({ let __v = (*i_thread.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = { let __tmp_x = { let __v = (*i_thread.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x + __tmp_y }; (*results_thread.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
        while let Some(f) = __defer_stack.pop() {
            f();
        };
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // Each iteration has its own i.
    wg.wait();
    println!("{} {} {}", format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map = { let __map_holder = counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"n".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", format_slice(&results)));

        // A plain function as the goroutine: its arguments are evaluated when
        // the go statement runs, and the slice and the pointers are shared.
    let mut sum = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        wg.add(1);
        let i_thread = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); let mu_thread = mu.clone(); let results_thread = results.clone(); let sum_thread = sum.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        square(
            Arc::new(Mutex::new(Some({ let __arg_holder = i_thread.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            results_thread.clone(),
            mu_thread.clone(),
            sum_thread.clone(),
            wg_thread.clone()
        );
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    wg.wait();
    println!("{} {}", format!("{}", { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&results)));

        // Other ways to make a mutex.
    let mut other = GoMutex { ..Default::default() };
    let mut another = GoMutex { ..Default::default() };
    let mut ptr = GoMutex { ..Default::default() };
    let mut hits = Arc::new(Mutex::new(Some(0)));
    for m in &vec![other.clone(), another.clone(), ptr.clone()] {
        wg.add(1);
        let mut hits_thread = hits.clone(); let mu_thread = mu.clone(); let wg_thread = wg.clone(); let __go_arg_0 = (*m).clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let mut __closure = move |lock: GoMutex| {
            let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            lock.lock();;
            lock.unlock();;
            mu_thread.lock();;
            { let mut guard = hits_thread.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
            mu_thread.unlock();;
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        };
        __closure(__go_arg_0);
    });
    }
    wg.wait();
    println!("{}", format!("{}", { let __v = (*hits.lock().unwrap().as_ref().unwrap()).clone(); __v }));

        // A readers-writer lock inside a struct, and a local one.
    let mut t = Arc::new(Mutex::new(Some(tally { count: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([])))), ..Default::default() })));
    let mut rw = GoRWMutex::new();
    let mut seen = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        wg.add(1);
        let rw_thread = rw.clone(); let mut seen_thread = seen.clone(); let t_thread = t.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
        { let __recv = t_thread.clone(); let __recv_ptr: *mut tally = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut tally }; let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some("k".to_string())))); __result };;
        rw_thread.lock();;
        { let mut guard = seen_thread.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        rw_thread.unlock();;
        while let Some(f) = __defer_stack.pop() {
            f();
        };
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    wg.wait();
    rw.r_lock();
    println!("{} {}", format!("{}", { let __recv = t.clone(); let __recv_ptr: *const tally = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const tally }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some("k".to_string())))); __result }), format!("{}", { let __v = (*seen.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    rw.r_unlock();
}