#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};
use std::thread;


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
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}

thread_local! {
    static __GO_PANIC_HOOK_SILENCE: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

fn go_silence_panic_hook() -> usize {
    static __GO_PANIC_HOOK: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_HOOK.call_once(|| {
        let __previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |__info| {
            if __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.get()) == 0 {
                __previous(__info);
            }
        }));
    });
    __GO_PANIC_HOOK_SILENCE.with(|__depth| {
        let __previous = __depth.get();
        __depth.set(__previous + 1);
        __previous
    })
}

fn go_restore_panic_hook(__previous: usize) {
    __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.set(__previous));
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}

struct WaitGroup {
    count: std::sync::Arc<(std::sync::Mutex<i64>, std::sync::Condvar)>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            count: std::sync::Arc::new((std::sync::Mutex::new(0), std::sync::Condvar::new())),
        }
    }

    fn add(&self, n: i64) {
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
    is_nil: bool,
}

struct GoMutexGuard {
    mutex: GoMutex,
    active: bool,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
            is_nil: false,
        }
    }

    fn nil() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
            is_nil: true,
        }
    }

    fn go_ptr_eq(left: &Self, right: &Self) -> bool {
        if left.is_nil || right.is_nil {
            return left.is_nil && right.is_nil;
        }
        std::sync::Arc::ptr_eq(&left.inner, &right.inner)
    }

    fn lock(&self) {
        if self.is_nil { panic!("nil *sync.Mutex"); }
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        while *locked {
            locked = ready.wait(locked).unwrap();
        }
        *locked = true;
    }

    fn unlock(&self) {
        if self.is_nil { panic!("nil *sync.Mutex"); }
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
            is_nil: self.is_nil,
        }
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
    }
}

pub fn report(w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>, name: Arc<Mutex<Option<String>>>, n: Arc<Mutex<Option<i64>>>) {
    { let __s = format!("{}: {}\n", format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v })); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
}

fn main() {
    ::bufio::__go_init_all();
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut w = bufio::new_writer(Arc::new(Mutex::new(Some(Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))));
        let w_defer_captured = w.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = w_defer_captured.clone(); let __recv_ptr: *mut bufio::r#mod::Writer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::r#mod::Writer }; let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
    }));
        println!("{}", format!("{}", "before the buffer".to_string()));
        { let __s = format!("{}\n", format!("{}", "buffered line".to_string())); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __s = format!("buffered {}\n", 2); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        println!("{} {}", format!("{}", "buffered bytes:".to_string()), format!("{}", { let __recv = w.clone(); let __recv_ptr: *const bufio::r#mod::Writer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const bufio::r#mod::Writer }; let __result = unsafe { &*__recv_ptr }.buffered(); __result }));
        report(Arc::new(Mutex::new(Some(Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))), Arc::new(Mutex::new(Some("direct".to_string()))), Arc::new(Mutex::new(Some(1))));
        let mut out = os::Stdout().clone();
        { let __s = format!("{}\n", format!("{}", "through a variable".to_string())); (*out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        io::write_string(Arc::new(Mutex::new(Some(Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))), Arc::new(Mutex::new(Some("io.WriteString\n".to_string()))));
        { let __std_stream = (*os::Stdout().lock().unwrap().as_ref().unwrap()).clone(); __std_stream }.write_string(Arc::new(Mutex::new(Some("WriteString\n".to_string()))));
        let (mut n, mut err) = { let __std_stream = (*os::Stdout().lock().unwrap().as_ref().unwrap()).clone(); __std_stream }.write(Arc::new(Mutex::new(Some(("Write\n".to_string()).as_bytes().to_go_slice()))));
        println!("{} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        let mut e = bufio::new_writer(Arc::new(Mutex::new(Some(Box::new((*os::Stderr().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))));
        { let __s = format!("{}\n", format!("{}", "to stderr".to_string())); (*e.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __recv = e.clone(); let __recv_ptr: *mut bufio::r#mod::Writer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::r#mod::Writer }; let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
        report(Arc::new(Mutex::new(Some(Box::new((*os::Stderr().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))), Arc::new(Mutex::new(Some("stderr".to_string()))), Arc::new(Mutex::new(Some(3))));
        let mut mu = GoMutex::new();
        let mut wg = WaitGroup::new();
        let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        wg.add(1);
        let mu_thread = mu.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            let __mutex_guard_source_677 = mu_thread.clone(); let __mutex_guard_677 = __mutex_guard_source_677.guard();;
            report(
                Arc::new(Mutex::new(Some(Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))),
                Arc::new(Mutex::new(Some("worker".to_string()))),
                Arc::new(Mutex::new(Some({ let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i })))
            );;
            std::mem::drop(__mutex_guard_677);;
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            }));
            match __go_panic_result {
                Ok(__go_value) => {
                    go_restore_panic_hook(__go_previous_panic_hook);
                    __go_value
                }
                Err(__go_panic_payload) => {
                    go_store_panic_payload(__go_panic_payload);
                    while let Some(f) = __defer_stack.pop() {
                        if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                            go_store_panic_payload(__go_defer_panic);
                        }
                    }
                    go_restore_panic_hook(__go_previous_panic_hook);
                    go_resume_unrecovered_panic();
                    ()
                }
            }

        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        wg.wait();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            go_restore_panic_hook(__go_previous_panic_hook);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                    go_store_panic_payload(__go_defer_panic);
                }
            }
            go_restore_panic_hook(__go_previous_panic_hook);
            go_resume_unrecovered_panic();
            ()
        }
    }
}