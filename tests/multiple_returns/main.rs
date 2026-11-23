#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

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

/// Functions with multiple return values
pub fn divmod(a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> (i64, i64) {
    return (
        { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y },
        { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y }
    );
}

pub fn parse_number(s: Arc<Mutex<Option<String>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    let (mut num, mut err) = strconv::atoi(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (
            0,
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("failed to parse '{}': {}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }))))))
        );
    }
    (num, Arc::new(Mutex::new(None)))
}

pub fn get_name_age() -> (Arc<Mutex<Option<String>>>, i64) {
    (Arc::new(Mutex::new(Some("Alice".to_string()))), 30)
}

/// Named return values
pub fn calculate(a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> (i64, i64) {
    let mut sum: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut product: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *sum.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; *product.lock().unwrap() = Some(new_val); };
    return ((*sum.lock().unwrap().as_ref().unwrap()), (*product.lock().unwrap().as_ref().unwrap()));
}

pub fn process_data(data: Arc<Mutex<Option<Vec<i64>>>>) -> (i64, i64, i64) {
    let mut min: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut max: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut sum: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = (({ let __len = (*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (0, 0, 0);
    }
    { let new_val = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; *min.lock().unwrap() = Some(new_val); };
    { let new_val = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; *max.lock().unwrap() = Some(new_val); };
    { let new_val = 0; *sum.lock().unwrap() = Some(new_val); };
    { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for val in __range_values.iter().copied() {
        if { let __tmp_x = val; let __tmp_y = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = val; *min.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = val; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = val; *max.lock().unwrap() = Some(new_val); };
    }
        { let __rhs = val; let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return ((*min.lock().unwrap().as_ref().unwrap()), (*max.lock().unwrap().as_ref().unwrap()), (*sum.lock().unwrap().as_ref().unwrap()));
}

pub fn swap(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {
    return ({ let __owned = b.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
}

/// Function returning multiple values of different types
pub fn get_person_info() -> (Arc<Mutex<Option<String>>>, i64, f64, bool) {
    let mut name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut age: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut height: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut married: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    (Arc::new(Mutex::new(Some("Bob".to_string()))), 25, 5.9, false)
}

/// Function that can return early with different values
pub fn find_in_slice(slice: Arc<Mutex<Option<Vec<i64>>>>, target: Arc<Mutex<Option<i64>>>) -> (i64, bool) {
    let mut index: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    { let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, val) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = val; let __tmp_y = { let __v = (*target.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return (i as i64, true);
    }
    } }
    (-(1), false)
}

/// Multiple returns with error handling
pub fn safe_divide(a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    let mut result: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let mut err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } {
        return (
            0.0,
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("division by zero"))))))
        );
    }
    return (
        { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y },
        Arc::new(Mutex::new(None))
    );
}

fn main() {
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", "=== Basic multiple returns ===".to_string()));
    let (mut quotient, mut remainder) = divmod(Arc::new(Mutex::new(Some(17))), Arc::new(Mutex::new(Some(5))));
    print!("17 / 5 = {} remainder {}\n", format!("{}", quotient), format!("{}", remainder));
    let (mut name, mut age) = get_name_age();
    print!("Name: {}, Age: {}\n", format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", age));
    println!("{}", format!("{}", "\n=== Multiple returns with errors ===".to_string()));
    let (mut num, mut err) = parse_number(Arc::new(Mutex::new(Some("123".to_string()))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    } else {
        print!("Parsed number: {}\n", num);
    }
    { let (__tmp_0, __tmp_1) = parse_number(Arc::new(Mutex::new(Some("abc".to_string())))); num = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    } else {
        print!("Parsed number: {}\n", num);
    }
    println!("{}", format!("{}", "\n=== Named return values ===".to_string()));
    let (mut s, mut p) = calculate(Arc::new(Mutex::new(Some(6))), Arc::new(Mutex::new(Some(7))));
    print!("Sum: {}, Product: {}\n", format!("{}", s), format!("{}", p));
    let mut data = Arc::new(Mutex::new(Some(vec![3i64, 1i64, 4i64, 1i64, 5i64, 9i64, 2i64, 6i64])));
    let (mut min, mut max, mut sum) = process_data(data.clone());
    print!("Data: {}\n", format_slice(&data));
    print!("Min: {}, Max: {}, Sum: {}\n", format!("{}", min), format!("{}", max), format!("{}", sum));
    println!("{}", format!("{}", "\n=== Swapping values ===".to_string()));
    let (mut x, mut y) = (Arc::new(Mutex::new(Some("hello".to_string()))), Arc::new(Mutex::new(Some("world".to_string()))));
    print!("Before swap: x={}, y={}\n", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    { let (__tmp_0, __tmp_1) = swap(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_tmp_1; };
    print!("After swap: x={}, y={}\n", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", "\n=== Different types ===".to_string()));
    let (mut pName, mut pAge, mut pHeight, mut pMarried) = get_person_info();
    print!("Person: {}, {} years old, {} feet tall, married: {}\n", format!("{}", { let __v = (*pName.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", pAge), format!("{:.1}", pHeight), format!("{}", pMarried));
    println!("{}", format!("{}", "\n=== Finding in slice ===".to_string()));
    let mut numbers = Arc::new(Mutex::new(Some(vec![10i64, 20i64, 30i64, 40i64, 50i64])));
    let (mut index, mut found) = find_in_slice(numbers.clone(), Arc::new(Mutex::new(Some(30))));
    if found {
        print!("Found 30 at index {}\n", index);
    } else {
        println!("{}", format!("{}", "30 not found".to_string()));
    }
    { let (__tmp_0, __tmp_1) = find_in_slice(numbers.clone(), Arc::new(Mutex::new(Some(99)))); index = __tmp_0; found = __tmp_1; };
    if found {
        print!("Found 99 at index {}\n", index);
    } else {
        println!("{}", format!("{}", "99 not found".to_string()));
    }
    println!("{}", format!("{}", "\n=== Safe division ===".to_string()));
    let (mut result, __tmp_1) = safe_divide(Arc::new(Mutex::new(Some(10.0))), Arc::new(Mutex::new(Some(3.0)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    } else {
        print!("10.0 / 3.0 = {:.2}\n", result);
    }
    { let (__tmp_0, __tmp_1) = safe_divide(Arc::new(Mutex::new(Some(10.0))), Arc::new(Mutex::new(Some(0.0)))); result = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    } else {
        print!("Result: {:.2}\n", result);
    }
    println!("{}", format!("{}", "\n=== Ignoring return values ===".to_string()));
    let (_, mut remainder2) = divmod(Arc::new(Mutex::new(Some(23))), Arc::new(Mutex::new(Some(7))));
    print!("23 mod 7 = {} (quotient ignored)\n", remainder2);
    let (mut name2, _) = get_name_age();
    print!("Name only: {} (age ignored)\n", { let __v = (*name2.lock().unwrap().as_ref().unwrap()).clone(); __v });
    println!("{}", format!("{}", "\n=== Multiple assignment ===".to_string()));
    let (mut a, mut b, mut c) = (Arc::new(Mutex::new(Some(1i64))), Arc::new(Mutex::new(Some(2i64))), Arc::new(Mutex::new(Some(3i64))));
    print!("a={}, b={}, c={}\n", format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    {
        let __tmp_0 = ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
        let __tmp_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
        *a.lock().unwrap() = Some(__tmp_0);
        *b.lock().unwrap() = Some(__tmp_1);
    };
    print!("After swap: a={}, b={}\n", format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}