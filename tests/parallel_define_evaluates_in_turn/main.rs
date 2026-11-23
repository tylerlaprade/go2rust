#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
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

fn go_float_string(value: f64, single: bool) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let shortest = if single { format!("{:e}", value.abs() as f32) } else { format!("{:e}", value.abs()) };
    let (mantissa, exponent) = shortest.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let exponent: i32 = exponent.parse().unwrap();
    if exponent < -4 || exponent >= 6 {
        let mut text = String::from(sign);
        text.push_str(&digits[..1]);
        if digits.len() > 1 {
            text.push('.');
            text.push_str(&digits[1..]);
        }
        text.push('e');
        text.push(if exponent < 0 { '-' } else { '+' });
        text.push_str(&format!("{:02}", exponent.abs()));
        return text;
    }
    if exponent < 0 {
        return format!("{}0.{}{}", sign, "0".repeat((-exponent - 1) as usize), digits);
    }
    let whole = (exponent + 1) as usize;
    if digits.len() <= whole {
        return format!("{}{}{}", sign, digits, "0".repeat(whole - digits.len()));
    }
    format!("{}{}.{}", sign, &digits[..whole], &digits[whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(value: &T) -> String {
    let name = std::any::type_name::<T>();
    if name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(value as *const T as *const f64) }, false);
    }
    if name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(value as *const T as *const f32) } as f64, true);
    }
    value.to_string()
}

#[derive(Debug, Clone, Default)]
pub struct Stack(pub Arc<Mutex<Option<Vec<i64>>>>);

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct counter {
    pub n: Arc<Mutex<Option<i64>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Matrix(pub Arc<Mutex<Option<Vec<Vec<f64>>>>>);


impl Stack {
    pub fn push(&mut self, v: Arc<Mutex<Option<i64>>>) {
        { let new_val = { let __base = self.0.clone(); let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v })); Arc::new(Mutex::new(Some(Stack(Arc::new(Mutex::new(Some(__values))))))) }; *self = new_val.lock().unwrap().take().unwrap_or_default(); };
    }

    pub fn pop(&mut self) -> i64 {
        let mut old = Arc::new(Mutex::new(Some((*self).clone())));
        let mut v = Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() })));
        { let new_val = Stack(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; __seq.reslice3(__low, __high, __max) })))); *self = new_val; };
        return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}

impl counter {
    pub fn next(&mut self) -> i64 {
        { let __target = self.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*self.n.lock().unwrap().as_ref().unwrap());
    }
}

impl Matrix {
    pub fn same(&self, o: Arc<Mutex<Option<Matrix>>>) -> bool {
        return { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*o.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); __tmp_x == __tmp_y };
    }
}

pub fn eval_r_p_n(expr: Arc<Mutex<Option<String>>>) -> i64 {
    let mut st: Arc<Mutex<Option<Stack>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = Arc::new(Mutex::new(Some({ let __s = (*expr.lock().unwrap().as_ref().unwrap()).clone(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        { let _switch_val = (*t).clone();
    if _switch_val == ("+".to_string()) || _switch_val == ("*".to_string()) {
            let (mut b, mut a) = ({ let __tuple_value = (*st.lock().unwrap().as_mut().unwrap()).pop(); __tuple_value }, { let __tuple_value = (*st.lock().unwrap().as_mut().unwrap()).pop(); __tuple_value });
            if { let __tmp_x = (*t).clone(); let __tmp_y = "+".to_string(); __tmp_x == __tmp_y } {
        (*st.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some({ let __tmp_x = a; let __tmp_y = b; __tmp_x + __tmp_y }))));
    } else {
        (*st.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some({ let __tmp_x = a; let __tmp_y = b; __tmp_x * __tmp_y }))));
    }
        } else {
            let (mut n, _) = strconv::atoi(Arc::new(Mutex::new(Some((*t).clone()))));
            (*st.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some(n))));
        }
    }
    } }
    return (*st.lock().unwrap().as_mut().unwrap()).pop();
}

fn main() {
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", eval_r_p_n(Arc::new(Mutex::new(Some("3 4 + 2 *".to_string()))))));
    let mut c = Arc::new(Mutex::new(Some(counter { n: Arc::new(Mutex::new(Some(0))) })));
    let (mut x, mut y, mut z) = ({ let __tuple_value = { let __recv = c.clone(); let __recv_ptr: *mut counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut counter }; let __result = unsafe { &mut *__recv_ptr }.next(); __result }; __tuple_value }, { let __tuple_value = { let __recv = c.clone(); let __recv_ptr: *mut counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut counter }; let __result = unsafe { &mut *__recv_ptr }.next(); __result }; __tuple_value }, { let __tuple_value = { let __recv = c.clone(); let __recv_ptr: *mut counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut counter }; let __result = unsafe { &mut *__recv_ptr }.next(); __result }; __tuple_value });
    println!("{} {} {}", format!("{}", x), format!("{}", y), format!("{}", z));
    let mut v = Arc::new(Mutex::new(Some(7i64)));
    let (mut p, mut q) = ({ let __tuple_value = { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value }, { let __tuple_value = { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value });
    println!("{} {}", format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut byLen = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<Vec<String>>>>>::from([]))));
    { let __range_holder = Arc::new(Mutex::new(Some({ let __s = "go is fun to use".to_string(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for w in __range_values.iter() {
        { let __map_key = w.len() as i64; let __map_value = { let __slice = { let __map_holder = byLen.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&(w.len() as i64)).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = (*w).clone(); let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; (*byLen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    println!("{} {}", format!("{}", format_slice(&{ let __map = { let __map_holder = byLen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&2)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&{ let __map = { let __map_holder = byLen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&3)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    println!("{}", format!("{}", {
        let __recv = Matrix(Arc::new(Mutex::new(Some(vec![vec![1.0]]))));
        let __result = __recv.same(
            Arc::new(Mutex::new(Some(Matrix(Arc::new(Mutex::new(Some(vec![vec![2.0]]))))))),
        );
        __result
    }));
}

impl GoValueClone for counter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
