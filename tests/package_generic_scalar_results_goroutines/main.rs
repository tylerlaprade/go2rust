#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display};
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
                    format!("{}:{}", go_display_value(k), go_display_value(val))
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
        let formatted: Vec<String> = s.as_ref().iter().map(|v| go_display_value(v)).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| go_display_value(v)).collect();
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

fn go_float_string(__value: f64, __single: bool) -> String {
    // The names are prefixed: this helper is at the crate root, where a Go
    // package variable of the same name would be a static it cannot shadow.
    if __value.is_nan() {
        return "NaN".to_string();
    }
    if __value.is_infinite() {
        return if __value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let __sign = if __value.is_sign_negative() { "-" } else { "" };
    let __shortest = if __single { format!("{:e}", __value.abs() as f32) } else { format!("{:e}", __value.abs()) };
    let (__mantissa, __exponent_text) = __shortest.split_once('e').unwrap();
    let __digits: String = __mantissa.chars().filter(|__c| __c.is_ascii_digit()).collect();
    let __exponent: i32 = __exponent_text.parse().unwrap();
    if __exponent < -4 || __exponent >= 6 {
        let mut __text = String::from(__sign);
        __text.push_str(&__digits[..1]);
        if __digits.len() > 1 {
            __text.push('.');
            __text.push_str(&__digits[1..]);
        }
        __text.push('e');
        __text.push(if __exponent < 0 { '-' } else { '+' });
        __text.push_str(&format!("{:02}", __exponent.abs()));
        return __text;
    }
    if __exponent < 0 {
        return format!("{}0.{}{}", __sign, "0".repeat((-__exponent - 1) as usize), __digits);
    }
    let __whole = (__exponent + 1) as usize;
    if __digits.len() <= __whole {
        return format!("{}{}{}", __sign, __digits, "0".repeat(__whole - __digits.len()));
    }
    format!("{}{}.{}", __sign, &__digits[..__whole], &__digits[__whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(__value: &T) -> String {
    let __name = std::any::type_name::<T>();
    if __name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(__value as *const T as *const f64) }, false);
    }
    if __name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(__value as *const T as *const f32) } as f64, true);
    }
    __value.to_string()
}

pub fn double(n: Arc<Mutex<Option<i64>>>) -> i64 {
    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y };
}

fn main() {
    ::example_com_packagegenericscalarresultsgoroutines_dep::__go_init_all();

    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send("started".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
    let mut x = example_com_packagegenericscalarresultsgoroutines_dep::pick::<i64>(Arc::new(Mutex::new(Some(7))));
    let mut y = Arc::new(Mutex::new(Some({ let __tmp_x = (*example_com_packagegenericscalarresultsgoroutines_dep::pick::<i64>(Arc::new(Mutex::new(Some(8)))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut z: Arc<Mutex<Option<i64>>> = example_com_packagegenericscalarresultsgoroutines_dep::pick::<i64>(Arc::new(Mutex::new(Some(10))));
    println!("{} {} {} {} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }), format!("{}", (*example_com_packagegenericscalarresultsgoroutines_dep::pick::<i64>(Arc::new(Mutex::new(Some(9)))).lock().unwrap().as_ref().unwrap())));
    let mut f = Arc::new(Mutex::new(Some({ let __tmp_x = (*example_com_packagegenericscalarresultsgoroutines_dep::pick::<f64>(Arc::new(Mutex::new(Some(1.5)))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2.0; __tmp_x * __tmp_y })));
    let mut ok = Arc::new(Mutex::new(Some((*example_com_packagegenericscalarresultsgoroutines_dep::pick::<bool>(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref().unwrap()) && !((*example_com_packagegenericscalarresultsgoroutines_dep::pick::<bool>(Arc::new(Mutex::new(Some(false)))).lock().unwrap().as_ref().unwrap())))));
    println!("{} {} {} {}", { let __text = go_float_string(({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text }, format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", double(example_com_packagegenericscalarresultsgoroutines_dep::pick::<i64>(Arc::new(Mutex::new(Some(4)))))), format!("{}", { let __tmp_x = example_com_packagegenericscalarresultsgoroutines_dep::size::<String>(Arc::new(Mutex::new(Some("s".to_string())))); let __tmp_y = 1; __tmp_x + __tmp_y }));
    let (__tmp_0, __tmp_1) = example_com_packagegenericscalarresultsgoroutines_dep::pair::<i64, f64>(Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(2.5)))); let mut a = { let __tmp_holder = __tmp_0.clone(); let __tmp_guard = __tmp_holder.lock().unwrap(); (*__tmp_guard.as_ref().unwrap()).clone() }; let mut b = { let __tmp_holder = __tmp_1.clone(); let __tmp_guard = __tmp_holder.lock().unwrap(); (*__tmp_guard.as_ref().unwrap()).clone() };;
    println!("{} {}", format!("{}", { let __tmp_x = a; let __tmp_y = 1; __tmp_x + __tmp_y }), { let __text = go_float_string(({ let __tmp_x = b; let __tmp_y = 2.0; __tmp_x * __tmp_y }) as f64, false); __text });
    { let (__tmp_0, __tmp_1) = example_com_packagegenericscalarresultsgoroutines_dep::pair::<i64, f64>(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(0.5)))); a = { let __tmp_holder = __tmp_0.clone(); let __tmp_guard = __tmp_holder.lock().unwrap(); (*__tmp_guard.as_ref().unwrap()).clone() }; b = { let __tmp_holder = __tmp_1.clone(); let __tmp_guard = __tmp_holder.lock().unwrap(); (*__tmp_guard.as_ref().unwrap()).clone() }; };
    println!("{} {}", format!("{}", a), { let __text = go_float_string((b) as f64, false); __text });
    { let new_val = example_com_packagegenericscalarresultsgoroutines_dep::pick::<i64>(Arc::new(Mutex::new(Some(11)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z.lock().unwrap() = __moved_val; };
    { let __rhs = (*example_com_packagegenericscalarresultsgoroutines_dep::max::<i64>(2, 12).lock().unwrap().as_ref().unwrap()); let mut guard = z.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {}", format!("{}", { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }), { let __text = go_float_string(((*example_com_packagegenericscalarresultsgoroutines_dep::max::<f64>(1.5, -(2.0)).lock().unwrap().as_ref().unwrap())) as f64, false); __text }, format!("{}", example_com_packagegenericscalarresultsgoroutines_dep::larger::<i64>(3, 2)));
    if example_com_packagegenericscalarresultsgoroutines_dep::larger::<f64>(2.5, 1.0) && { let __tmp_x = (*example_com_packagegenericscalarresultsgoroutines_dep::max::<i64>(3, 4).lock().unwrap().as_ref().unwrap()); let __tmp_y = 4; __tmp_x == __tmp_y } {
        println!("{}", format!("{}", "compared".to_string()));
    }
    let mut nums = Arc::new(Mutex::new(Some(vec![{ let __v = example_com_packagegenericscalarresultsgoroutines_dep::pick::<i64>(Arc::new(Mutex::new(Some(1)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }, { let __v = example_com_packagegenericscalarresultsgoroutines_dep::max::<i64>(2, 1); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }])));
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<f64>>>>::from([("k".to_string(), example_com_packagegenericscalarresultsgoroutines_dep::max::<f64>(0.5, 0.25))]))));
    println!("{} {} {}", format!("{}", format_slice(&nums)), format!("{}", format_map(&m)), format!("{}", { let __tmp_x = (({ let __len = (*nums.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = ((*example_com_packagegenericscalarresultsgoroutines_dep::pick::<i64>(Arc::new(Mutex::new(Some(0)))).lock().unwrap().as_ref().unwrap()) as i64); __tmp_x + __tmp_y }));
}