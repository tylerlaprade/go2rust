#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

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

#[derive(Debug, Clone)]
pub struct rec {
    pub buf: Arc<Mutex<Option<[u8; 8]>>>,
    pub n: Arc<Mutex<Option<i64>>>,
}

impl rec {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            buf: __go_clone_0_0,
            n: __go_clone_1_0,
        }
    }
}


impl Default for rec {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 8, _>(|_| 0))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            buf: __go_default_0_0,
            n: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for rec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.buf));
        let __go_fmt_1 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl rec {
    pub fn put(&mut self, s: Arc<Mutex<Option<String>>>) {
        { let new_val = {
            let _dst_start = (*self.n.clone().lock().unwrap().as_ref().unwrap()) as usize;
            let _dst_len = (*self.buf.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*s.lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*self.buf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.n.lock().unwrap() = __moved_val; };
    }
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    done.recv().unwrap_or_default();
    let mut arr: Arc<Mutex<Option<[i64; 5]>>> = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 5, _>(|_| 0))));
    let mut s = Arc::new(Mutex::new(Some({ let __array_cell = arr.clone(); GoSlice::from_array_arc_cell(&__array_cell, 0, (0) as usize, 5) })));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = s.clone(); let __append_values = vec![1, 2]; let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); Arc::new(Mutex::new(Some(__appended))) })); *s.lock().unwrap() = __slice_header; };
    println!("{} {} {}", format!("{}", format_slice(&arr)), format!("{}", ({ let __len = (*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*s.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)));
    let mut t = Arc::new(Mutex::new(Some({ let __array_cell = arr.clone(); GoSlice::from_array_arc_cell(&__array_cell, (3) as usize, 5, 5) })));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = t.clone(); let __append_values = vec![9, 9, 9]; let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); Arc::new(Mutex::new(Some(__appended))) })); *t.lock().unwrap() = __slice_header; };
    { let __idx = (0) as usize; let __rhs = 7; (*t.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", format_slice(&arr)), format!("{}", format_slice(&t)));
    let mut r: Arc<Mutex<Option<rec>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*r.lock().unwrap().as_mut().unwrap()).put(Arc::new(Mutex::new(Some("hey".to_string()))));
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*Arc::new(Mutex::new(Some({ let __array_cell = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); GoSlice::from_array_arc_cell(&__array_cell, 0, ((*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize, 8) }))).lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __seq = { let __seq_holder = (*r.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    let mut buf: Arc<Mutex<Option<[u8; 4]>>> = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 4, _>(|_| 0))));
    let mut rd = strings::new_reader(Arc::new(Mutex::new(Some("abcdef".to_string()))));
    let (mut n, _) = { let __recv = rd.clone(); let __recv_ptr: *mut strings::reader::Reader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut strings::reader::Reader }; let __result = unsafe { &mut *__recv_ptr }.read(Arc::new(Mutex::new(Some({ let __array_cell = buf.clone(); GoSlice::from_array_arc_cell(&__array_cell, 0, 4, 4) })))); __result };
    println!("{} {}", format!("{}", n), format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*Arc::new(Mutex::new(Some({ let __array_cell = buf.clone(); GoSlice::from_array_arc_cell(&__array_cell, 0, (n) as usize, 4) }))).lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap())));
    let mut sum = Arc::new(Mutex::new(Some(0i64)));
    for v in { let __array_cell = arr.clone(); GoSlice::from_array_arc_cell(&__array_cell, 0, 5, 5) }.iter().copied() {
        { let __rhs = v; let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    println!("{}", format!("{}", { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    { let new_val = Arc::new(Mutex::new(Some([0, 0, 0, 0, 0]))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *arr.lock().unwrap() = __moved_val; };
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));
    { let __idx = (1) as usize; let __rhs = 42; (*s.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{}", format!("{}", { let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    let mut rows = Arc::new(Mutex::new(Some(vec![[1i64, 2i64, 3i64]])));
    let mut q = Arc::new(Mutex::new(Some({
        let mut __seq = { let __seq = { let __seq_holder = rows.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let __low = 0;
        let __high = (2) as usize;
        let __max = __seq.len();
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    })));
    println!("{}", format!("{}", format_slice(&q)));
    let mut p = arr.clone();
    let mut ps = Arc::new(Mutex::new(Some({ let __array_cell = p.clone(); GoSlice::from_array_arc_cell(&__array_cell, (1) as usize, (3) as usize, 5) })));
    { let __idx = (0) as usize; let __rhs = 5; (*ps.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", { let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }), format!("{}", ({ let __len = (*ps.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*ps.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)));
}

impl GoValueClone for rec {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
