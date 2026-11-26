#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>, __nil_text: &str) -> String
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
                    format!("{}:{}", go_display_value(k), __nil_text)
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

#[derive(Debug, Clone, Default)]
pub struct Store {
    pub data: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
    pub tags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<String>>>>>>>>,
}

impl Store {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = self.tags.clone();
        Self {
            data: __go_clone_0_0,
            tags: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.data));
        let __go_fmt_1 = format!("{}", "<map>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Store {
    pub fn data(&self) -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        return self.data.clone();
    }
}

pub fn keys(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut out = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..(({ let __len = (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = out.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *out.lock().unwrap() = __slice_header; };
    }
    sort::strings(out.clone());
    return out.clone();
}

fn main() {
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut st = Arc::new(Mutex::new(Some(Store { data: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))])))), tags: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<String>>>>>::from([])))), ..Default::default() })));
    let mut m = (*st.lock().unwrap().as_ref().unwrap()).data.clone();
    { let __map_key = "b".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    let mut d = { let __recv = st.clone(); let __recv_ptr: *const Store = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Store }; let __result = unsafe { &*__recv_ptr }.data(); __result };
    { let __map_key = "c".to_string(); let __map_value = Arc::new(Mutex::new(Some(3))); (*d.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    let mut v = (*st.lock().unwrap().as_ref().unwrap()).data.clone();
    { let __map_key = "d".to_string(); let __map_value = Arc::new(Mutex::new(Some(4))); (*v.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_handle = m.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&"a".to_string()); };
    println!("{} {} {} {}", format!("{}", format_slice(&keys({ let __field = (*st.lock().unwrap().as_ref().unwrap()).data.clone(); __field }))), format!("{}", ({ let __len = (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", ({ let __len = (*d.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", ({ let __len = (*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    let mut t = (*st.lock().unwrap().as_ref().unwrap()).tags.clone();
    { let __map_key = "x".to_string(); let __map_value = { let __slice = { let __map_holder = t.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&"x".to_string()).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = "first".to_string(); let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; (*t.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", format_map_nil(&(*st.lock().unwrap().as_ref().unwrap()).tags, "[]")));
    let mut s2 = Arc::new(Mutex::new(Some(Store { data: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))), ..Default::default() })));
    let mut m2 = (*s2.lock().unwrap().as_ref().unwrap()).data.clone();
    { let __map_key = "z".to_string(); let __map_value = Arc::new(Mutex::new(Some(1))); (*m2.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", format_slice(&keys({ let __field = (*s2.lock().unwrap().as_ref().unwrap()).data.clone(); __field }))));
    { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("new".to_string(), Arc::new(Mutex::new(Some(1i64))))])))); m = new_val; };
    println!("{} {}", format!("{}", format_slice(&keys({ let __field = (*st.lock().unwrap().as_ref().unwrap()).data.clone(); __field }))), format!("{}", format_slice(&keys(m.clone()))));
}

impl GoValueClone for Store {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
