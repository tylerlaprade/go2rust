#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


fn go_any_try_clone(value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Celsius>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    go_clone_registered_any(value)
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
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

fn __go_type_name(val: &(dyn Any + Send + Sync)) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Arc<Mutex<Option<Celsius>>>>() { return "*main.Celsius" }
    if val.is::<Arc<Mutex<Option<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>>>>() { return "*model.Label" }
    if val.is::<Celsius>() { return "main.Celsius" }
    if val.is::<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>() { return "model.Label" }
    if let Some(__go_name) = go_registered_any_type_name(val) { return __go_name }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct Celsius {
    pub deg: Arc<Mutex<Option<i64>>>,
}

impl Celsius {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.deg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            deg: __go_clone_0_0,
        }
    }
}


impl Default for Celsius {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            deg: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


impl Celsius {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{}C", (*self.deg.lock().unwrap().as_ref().unwrap())))))
    }
}

pub fn make_c() -> Arc<Mutex<Option<Celsius>>> {
    Arc::new(Mutex::new(Some(Celsius { deg: Arc::new(Mutex::new(Some(3))), ..Default::default() })))
}

fn main() {
    ::example_com_fmt_stringer_imported_types_goroutines_model::__go_init_all();

    let mut s: Arc<Mutex<Option<fmt_Stringer>>> = Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display((*make_c().lock().unwrap().as_ref().unwrap()).clone()))));
    println!("{}", format!("{}", (*(*s.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
    let mut v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some({ go_register_any_runtime_type::<Celsius>("main.Celsius", |__value| { let __typed = __value.downcast_ref::<Celsius>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<Celsius>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any + Send + Sync> }, None); Box::new(Celsius { deg: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Box<dyn Any + Send + Sync> })));
    println!("{}", format!("{}", format!("{}", ({
        let val = v.clone();
        let __stringer: Result<fmt_Stringer, String> = {
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { Ok(typed_val.clone()) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else { Err(format!("interface conversion: {} is not fmt.Stringer: missing method String", __go_type_name(&**any_val))) }
        } else {
            Err("interface conversion: interface is nil, not fmt.Stringer".to_string())
        }
        };
        match __stringer { Ok(__s) => __s, Err(__message) => panic!("{}", __message) }
    }))));
    print!("{}\n", { let __type_name = __go_type_name(&*(({
        let val = v.clone();
        let __stringer: Result<fmt_Stringer, String> = {
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { Ok(typed_val.clone()) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else { Err(format!("interface conversion: {} is not fmt.Stringer: missing method String", __go_type_name(&**any_val))) }
        } else {
            Err("interface conversion: interface is nil, not fmt.Stringer".to_string())
        }
        };
        match __stringer { Ok(__s) => __s, Err(__message) => panic!("{}", __message) }
    })).__go_value); __type_name });
    let (_, mut ok) = ({
        let val = Arc::new(Mutex::new(Some(Box::new(example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label { text: Arc::new(Mutex::new(Some("x".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>))).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { (Arc::new(Mutex::new(Some(typed_val.clone()))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else { (Arc::new(Mutex::new(None::<fmt_Stringer>)), false) }
        } else {
            (Arc::new(Mutex::new(None::<fmt_Stringer>)), false)
        }
    });
    let mut w: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label { text: Arc::new(Mutex::new(Some("y".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>)));
    let (mut st, mut ok2) = ({
        let val = w.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { (Arc::new(Mutex::new(Some(typed_val.clone()))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else { (Arc::new(Mutex::new(None::<fmt_Stringer>)), false) }
        } else {
            (Arc::new(Mutex::new(None::<fmt_Stringer>)), false)
        }
    });
    println!("{} {} {}", format!("{}", ok), format!("{}", ok2), format!("{}", format!("{}", (*st.lock().unwrap().as_ref().unwrap()))));
    let mut wg = WaitGroup::new();
    let mut labels = Arc::new(Mutex::new(Some(vec!["".to_string(); (2) as usize])));
    for (i, v) in vec![Box::new(example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label { text: Arc::new(Mutex::new(Some("a".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>, Box::new(Celsius { deg: Arc::new(Mutex::new(Some(4))), ..Default::default() }) as Box<dyn Any + Send + Sync>].iter().enumerate() {
        wg.add(1);
        let labels_thread = labels.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some(i as i64))); let __go_arg_1 = Arc::new(Mutex::new(Some(go_any_clone(v.as_ref())))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>, v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            {
        let (mut s, mut ok) = ({
        let val = v.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { (Arc::new(Mutex::new(Some(typed_val.clone()))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<Celsius>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<example_com_fmt_stringer_imported_types_goroutines_model::r#mod::Label>>>>() { (Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else { (Arc::new(Mutex::new(None::<fmt_Stringer>)), false) }
        } else {
            (Arc::new(Mutex::new(None::<fmt_Stringer>)), false)
        }
    });;
        if ok {
            { let __idx = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = (*(*s.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone(); (*labels_thread.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
        }
    };
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
        __closure(__go_arg_0, __go_arg_1);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    }
    wg.wait();
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = labels.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = labels.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
}

impl GoValueClone for Celsius {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
