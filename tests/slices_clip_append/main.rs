use go2rust_stdlib_stubs::*;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

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

fn main() {
    ::cmp::__go_init_all();
    ::iter::__go_init_all();
    ::math_bits::__go_init_all();
    ::slices::__go_init_all();

    let mut env = Arc::new(Mutex::new(Some({ let mut v = Vec::with_capacity((2) as usize); v.resize((1) as usize, "".to_string()); v })));
    { let __idx = (0) as usize; let __rhs = "A=1".to_string(); (*env.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    let mut combined = { let __append_target = { let __result = slices::clip::<Vec<String>, String>({ let __slice_holder = env.clone(); { let __slice_guard = __slice_holder.lock().unwrap(); Arc::new(Mutex::new(__slice_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| Arc::new(Mutex::new(Some(__elem)))).collect::<Vec<_>>()))) } }); let __result_guard = __result.lock().unwrap(); Arc::new(Mutex::new(__result_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| (*__elem.lock().unwrap().as_ref().unwrap()).clone()).collect::<Vec<_>>()))) }.clone(); let __append_value = "PWD=/tmp".to_string(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() };
    { let __idx = (0) as usize; let __rhs = "B=2".to_string(); (*combined.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{}", format!("{}", format_slice(&env)));
    println!("{}", format!("{}", format_slice(&combined)));
}