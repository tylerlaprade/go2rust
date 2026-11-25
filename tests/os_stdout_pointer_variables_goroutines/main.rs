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
use std::fmt::{Display, Formatter};
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

#[derive(Clone, Default)]
pub struct Logger {
    pub out: Arc<Mutex<Option<os_File>>>,
    pub w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>,
}

impl Logger {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.out.clone();
        let __go_clone_1_0 = self.w.clone();
        Self {
            out: __go_clone_0_0,
            w: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.out.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn pick(verbose: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<os_File>>> {
    if { let __v = (*verbose.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return os::Stdout().clone();
    }
    os::Stderr().clone()
}

fn main() {
    ::io::__go_init_all();

    let mut out: Arc<Mutex<Option<os_File>>> = os::Stdout().clone();
    { let __s = format!("{}\n", format!("{}", "to stdout".to_string())); (*out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    println!("{} {} {} {}", format!("{}", {
        let __left = os::Stdout().clone();
        let __right = os::Stdout().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stdout().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        !__eq
    }));
    let mut l = Arc::new(Mutex::new(Some(Logger {
        out: pick(Arc::new(Mutex::new(Some(true)))).clone(),
        w: Arc::new(Mutex::new(Some(Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))),
        ..Default::default()
    })));
    { let __s = format!("logger {}\n", 1); (*(*l.lock().unwrap().as_ref().unwrap()).out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("{}\n", format!("{}", "writer".to_string())); (*(*l.lock().unwrap().as_ref().unwrap()).w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    println!("{} {} {}", format!("{}", {
        let __left = (*l.lock().unwrap().as_ref().unwrap()).out.clone();
        let __right = os::Stdout().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = pick(Arc::new(Mutex::new(Some(false))));
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = pick(Arc::new(Mutex::new(Some(false))));
        let __right = out.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));
    let mut errOut: Arc<Mutex<Option<os_File>>> = Arc::new(Mutex::new(None));
    { let new_val = os::Stderr().clone(); errOut = new_val; };
    println!("{} {}", format!("{}", {
        let __left = errOut.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", { let __nil_result = (*errOut.lock().unwrap()).is_some(); __nil_result }));
    let mut files = Arc::new(Mutex::new(Some(vec![{ let __field = os::Stdout().clone(); __field }, { let __field = os::Stderr().clone(); __field }])));
    { let __range_holder = files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        println!("{} {} {}", format!("{}", i), format!("{}", {
            let __left = f.clone();
            let __right = os::Stdout().clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            __eq
        }), format!("{}", (*{ let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));
    } }
    let mut wg = WaitGroup::new();
    let mut same = Arc::new(Mutex::new(Some(vec![false; (2) as usize])));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        wg.add(1);
        let out_thread = out.clone(); let same_thread = same.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            let mut mine: Arc<Mutex<Option<os_File>>> = os::Stdout().clone();;
            { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = {
                let __left = mine.clone();
                let __right = out_thread.clone();
                let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
                let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
                __eq
            }; (*same_thread.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
            { let __s = format!("{} {}\n", format!("{}", "worker".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })); (*mine.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };;
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
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = same.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = same.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
}

impl GoValueClone for Logger {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
