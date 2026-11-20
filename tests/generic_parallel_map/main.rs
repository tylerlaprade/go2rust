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

#[derive(Clone)]
pub struct SafeCounter__string {
    pub mu: GoMutex,
    pub m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>>,
}

impl SafeCounter__string {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.mu.clone();
        let __go_clone_1_0 = self.m.clone();
        Self {
            mu: __go_clone_0_0,
            m: __go_clone_1_0,
        }
    }
}


impl Default for SafeCounter__string {
    fn default() -> Self {
        let __go_default_0_0 = GoMutex::new();
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            mu: __go_default_0_0,
            m: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for SafeCounter__string {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.m));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl SafeCounter__string {
    pub fn inc(&mut self, k: Arc<Mutex<Option<String>>>) {
        let __mutex_guard_source_285 = self.mu.clone(); let __mutex_guard_285 = __mutex_guard_source_285.guard();
        // mu.Unlock() handled by RAII guard
        { let __map_holder = self.m.clone(); let __key = (*k.lock().unwrap().as_ref().unwrap()).clone(); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
    }

    pub fn value(&self, k: Arc<Mutex<Option<String>>>) -> i32 {
        let __mutex_guard_source_374 = self.mu.clone(); let __mutex_guard_374 = __mutex_guard_source_374.guard();
        // mu.Unlock() handled by RAII guard
        { let __map = { let __map_holder = self.m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*k.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }
    }
}

pub fn parallel_map__int__int(xs: Arc<Mutex<Option<Vec<i32>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<i32>>>> {
    let mut out = Arc::new(Mutex::new(Some(vec![0; ((*xs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    let mut wg = WaitGroup::new();
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().copied().enumerate() {
        wg.add(1);
        let f_thread = f.clone(); let out_thread = out.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some(i as i32))); let __go_arg_1 = Arc::new(Mutex::new(Some(x))); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i32>>>, x: Arc<Mutex<Option<i32>>>| {
            let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = {
                let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = {
                    let mut __f_guard = f_thread.lock().unwrap();
                    __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>
                };
                let __f = unsafe { &mut *__f_ptr };
                (*__f)(
                    x.clone()
                )
            }; (*out_thread.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        };
        __closure(__go_arg_0, __go_arg_1);
    });
    } }
    wg.wait();
    return out.clone();
}

pub fn parallel_map__string__int(xs: Arc<Mutex<Option<Vec<String>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> i32 + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<i32>>>> {
    let mut out = Arc::new(Mutex::new(Some(vec![0; ((*xs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    let mut wg = WaitGroup::new();
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        wg.add(1);
        let f_thread = f.clone(); let out_thread = out.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some(i as i32))); let __go_arg_1 = Arc::new(Mutex::new(Some((*x).clone()))); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i32>>>, x: Arc<Mutex<Option<String>>>| {
            let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = {
                let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> i32 + Send + Sync> = {
                    let mut __f_guard = f_thread.lock().unwrap();
                    __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> i32 + Send + Sync>
                };
                let __f = unsafe { &mut *__f_ptr };
                (*__f)(
                    Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
                )
            }; (*out_thread.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        };
        __closure(__go_arg_0, __go_arg_1);
    });
    } }
    wg.wait();
    return out.clone();
}

fn main() {
    let mut c = Arc::new(Mutex::new(Some(SafeCounter__string { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::new()))), ..Default::default() })));
    let mut wg = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        wg.add(1);
        let c_thread = c.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i32>>>| {
            let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        (*c_thread.lock().unwrap().as_mut().unwrap()).inc(Arc::new(Mutex::new(Some("even".to_string()))));
    } else {
        (*c_thread.lock().unwrap().as_mut().unwrap()).inc(Arc::new(Mutex::new(Some("odd".to_string()))));
    };
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        };
        __closure(__go_arg_0);
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    wg.wait();
    println!("{} {}", format!("{}", (*c.lock().unwrap().as_ref().unwrap()).value(Arc::new(Mutex::new(Some("even".to_string()))))), format!("{}", (*c.lock().unwrap().as_ref().unwrap()).value(Arc::new(Mutex::new(Some("odd".to_string()))))));
    let mut sq = parallel_map__int__int(
        Arc::new(Mutex::new(Some(vec![1, 2, 3, 4]))),
        Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<i32>>>| -> i32 {
        return { let __bin_n = (*n.lock().unwrap().as_ref().unwrap()).clone(); __bin_n * __bin_n };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>)))
    );
    println!("{}", format!("{}", format_slice(&sq)));
    let mut lens = parallel_map__string__int(
        Arc::new(Mutex::new(Some(vec!["a".to_string(), "bb".to_string(), "ccc".to_string()]))),
        Arc::new(Mutex::new(Some(Box::new(move |s: Arc<Mutex<Option<String>>>| -> i32 {
        (*s.lock().unwrap().as_ref().unwrap()).len() as i32
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> i32 + Send + Sync>)))
    );
    println!("{}", format!("{}", format_slice(&lens)));
}