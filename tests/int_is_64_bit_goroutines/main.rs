#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}

fn __go_type_name(val: &dyn Any) -> &'static str {
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
    std::any::type_name_of_val(val)
}

pub(crate) static memo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<i64>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *memo.lock().unwrap() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<i64, Arc<Mutex<Option<i64>>>>::new();
        *memo.lock().unwrap() = Some(__go_map);
    }
}


pub fn fib(n: Arc<Mutex<Option<i64>>>) -> i64 {
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    {
        let (mut v, mut ok) = { let __map = { let __map_holder = memo.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v })) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };;
        }
    }
    let mut v = Arc::new(Mutex::new(Some({ let __tmp_x = fib(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))); let __tmp_y = fib(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y })))); __tmp_x + __tmp_y })));
    { let __map_key = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*memo.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn factorial(n: Arc<Mutex<Option<i64>>>) -> i64 {
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x <= __tmp_y } {
        return 1;
    }
    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = factorial(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))); __tmp_x * __tmp_y };
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    __go_init_all();
    let mut started = GoChannel::<bool>::new();
    let started_thread = started.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        started_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    started.recv().unwrap_or_default();
    println!("{} {}", format!("{}", fib(Arc::new(Mutex::new(Some(90))))), format!("{}", factorial(Arc::new(Mutex::new(Some(20))))));
    let mut big = Arc::new(Mutex::new(Some(1099511627776i64)));
    println!("{} {} {} {}", format!("{}", { let __v = (*big.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = { let __v = (*big.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x * __tmp_y }), format!("{}", 9223372036854775807i64), format!("{}", i64::MIN));
    let mut total: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100000; __tmp_x < __tmp_y } {
        { let __rhs = { let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i }; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let (mut n, _) = strconv::atoi(Arc::new(Mutex::new(Some("9000000000".to_string()))));
    println!("{} {}", format!("{}", { let __tmp_x = n; let __tmp_y = 1; __tmp_x + __tmp_y }), format!("{}", (*strconv::itoa(Arc::new(Mutex::new(Some({ let __tmp_x = n; let __tmp_y = 2; __tmp_x * __tmp_y })))).lock().unwrap().as_ref().unwrap())));
    let mut wrapped = Arc::new(Mutex::new(Some(9223372036854775807i64)));
    { let mut guard = wrapped.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{}", format!("{}", { let __tmp_x = { let __v = (*wrapped.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i64::MIN; __tmp_x == __tmp_y }));
    let mut x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(7i64) as Box<dyn Any + Send + Sync>)));
    let (_, mut isInt) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i64>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(0))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), false)
        }
    });
    print!("{} {} {}\n", format!("{}", { let __type_name = match x.lock().unwrap().as_ref() { Some(__any_value) => __go_type_name(&**__any_value), None => "<nil>" }; __type_name }), format!("{}", isInt), format!("{}", 5000000000i64));
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(('A' as i32))));
    print!("{} {} {}\n", format!("{}", "int32"), format!("{}", { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", char::from_u32(({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as i64 as u32).unwrap_or('\u{FFFD}')));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
