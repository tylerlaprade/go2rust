#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        go_float_string(*v, false)
    } else if let Some(v) = value.downcast_ref::<f32>() {
        go_float_string(*v as f64, true)
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
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

#[derive(Clone)]
pub struct Query {
    pub table: Arc<Mutex<Option<String>>>,
    pub wheres: Arc<Mutex<Option<Vec<String>>>>,
    pub order: Arc<Mutex<Option<String>>>,
    pub limit: Arc<Mutex<Option<i64>>>,
    pub params: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>>>>>,
}

impl Query {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.table.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.wheres.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.order.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.limit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = self.params.clone();
        Self {
            table: __go_clone_0_0,
            wheres: __go_clone_1_0,
            order: __go_clone_2_0,
            limit: __go_clone_3_0,
            params: __go_clone_4_0,
        }
    }
}


impl Default for Query {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(None));
        Self {
            table: __go_default_0_0,
            wheres: __go_default_1_0,
            order: __go_default_2_0,
            limit: __go_default_3_0,
            params: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub name: Arc<Mutex<Option<String>>>,
    pub age: Arc<Mutex<Option<i64>>>,
    pub roles: Arc<Mutex<Option<Vec<String>>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.roles.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
            roles: __go_clone_2_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
            roles: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


pub trait Writer: std::fmt::Display + Any {
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool;
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn Writer + Send + Sync> {
    fn clone(&self) -> Self {
        Writer::__go_clone_box_writer(self.as_ref())
    }
}

#[derive(Debug, Clone, Default)]
pub struct upperWriter {
    pub buf: Arc<Mutex<Option<Vec<u8>>>>,
}

impl upperWriter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            buf: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for upperWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.buf));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Query {
    pub fn r#where(&mut self, cond: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> Arc<Mutex<Option<Query>>> {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.wheres.clone(); let __append_value = ({ let __v = (*cond.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *self.wheres.lock().unwrap() = __slice_header; };
        if { let __nil_target = self.params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>>::from([])))); self.params = new_val; };
    }
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); std::mem::drop(__range_guard); let __range_values = 0..__range_len; for (i, a) in __range_values.map(|__range_index| { let __range_guard = __range_holder.lock().unwrap(); go_any_clone(__range_guard.as_ref().unwrap()[__range_index].as_ref()) }).enumerate() {
        { let __map_key = { let __map_key_holder = Arc::new(Mutex::new(Some(format!("p{}", { let __tmp_x = (({ let __len = ({ let __len_target = { let __field = self.params.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len }) as i64); let __tmp_y = (i as i64); __tmp_x + __tmp_y })))).clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); std::mem::drop(__map_key_guard); __cloned }; let __map_value = Arc::new(Mutex::new(Some(go_any_clone(a.as_ref())))); (*self.params.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    pub fn order_by(&mut self, col: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Query>>> {
        { let new_val = col.lock().unwrap().as_ref().unwrap().clone(); *self.order.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    pub fn limit(&mut self, n: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Query>>> {
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *self.limit.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut b: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*b.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("SELECT * FROM {}", (*self.table.lock().unwrap().as_ref().unwrap()))))));
        if { let __tmp_x = (({ let __len = ({ let __len_target = { let __field = self.wheres.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len }) as i64); let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}{}", " WHERE ".to_string(), (*strings::join({ let __field = self.wheres.clone(); __field }, Arc::new(Mutex::new(Some(" AND ".to_string())))).lock().unwrap().as_ref().unwrap()))))));
    }
        if { let __tmp_x = (*self.order.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}{}", " ORDER BY ".to_string(), (*self.order.clone().lock().unwrap().as_ref().unwrap()))))));
    }
        if { let __tmp_x = (*self.limit.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*b.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!(" LIMIT {}", (*self.limit.lock().unwrap().as_ref().unwrap()))))));
    }
        return (*b.lock().unwrap().as_ref().unwrap()).string();
    }
}

impl User {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{}({})", format!("{}", (*self.name.lock().unwrap().as_ref().unwrap())), format!("{}", (*self.age.lock().unwrap().as_ref().unwrap()))))))
    }
}

impl upperWriter {
    pub fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.buf.clone(); let __append_values: Vec<_> = { let __slice_holder = Arc::new(Mutex::new(Some((*strings::to_upper(Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*p.lock().unwrap().as_ref().unwrap()).clone())).unwrap())))).lock().unwrap().as_ref().unwrap()).as_bytes().to_go_slice()))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<u8> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); Arc::new(Mutex::new(Some(__appended))) })); *self.buf.lock().unwrap() = __slice_header; };
        (
            ({ let __len = (*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64,
            Arc::new(Mutex::new(None))
        )
    }
}

impl Writer for upperWriter {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        upperWriter::write(self, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<upperWriter>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct upperWriterPtr(pub Arc<Mutex<Option<upperWriter>>>);

impl std::fmt::Display for upperWriterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Writer for upperWriterPtr {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        upperWriter::write(__recv, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<upperWriterPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::Writer for upperWriterPtr {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        upperWriter::write(__recv, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn io::r#mod::Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn io::r#mod::Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<upperWriterPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn from(t: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Query>>> {
    Arc::new(Mutex::new(Some(Query { table: Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn emit(w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>, parts: Arc<Mutex<Option<Vec<String>>>>) -> i64 {
    let mut total = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = parts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for p in __range_values.iter() {
        let (mut n, _) = (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some((p).as_bytes().to_go_slice()))));
        { let __rhs = n; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

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
    let mut q = {
        let __recv = {
            let __recv = {
                let __recv = {
                    let __recv = from(Arc::new(Mutex::new(Some("users".to_string()))));
                    let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r#where(
                        Arc::new(Mutex::new(Some("age > ?".to_string()))),
                        Arc::new(Mutex::new(Some(vec![
                            Box::new(18i64) as Box<dyn Any + Send + Sync>,
                        ]))),
                    );
                    __result
                };
                let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r#where(
                    Arc::new(Mutex::new(Some("name LIKE ?".to_string()))),
                    Arc::new(Mutex::new(Some(vec![
                        Box::new("a%".to_string()) as Box<dyn Any + Send + Sync>,
                    ]))),
                );
                __result
            };
            let __result = (*__recv.lock().unwrap().as_mut().unwrap()).order_by(
                Arc::new(Mutex::new(Some("name".to_string()))),
            );
            __result
        };
        let __result = (*__recv.lock().unwrap().as_mut().unwrap()).limit(
            Arc::new(Mutex::new(Some(10))),
        );
        __result
    };
    println!("{}", format!("{}", { let __text = match q.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut keys = Arc::new(Mutex::new(Some(Vec::<String>::with_capacity(0))));
    for (k, _) in { let __range_holder = (*q.lock().unwrap().as_ref().unwrap()).params.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    { let __sort_target = keys.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for k in __range_values.iter() {
        print!("{}{}{}{}", format!("{}", k), format!("{}", "=".to_string()), format!("{}", { let __text = match ({ let __map = { let __map_holder = (*q.lock().unwrap().as_ref().unwrap()).params.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(k)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", " ".to_string()));
    } }
    println!();
    let mut users = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("bob".to_string()))), age: Arc::new(Mutex::new(Some(30))), roles: Arc::new(Mutex::new(None)), ..Default::default() }))), Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("al".to_string()))), age: Arc::new(Mutex::new(Some(25))), roles: Arc::new(Mutex::new(Some(vec!["admin".to_string()]))), ..Default::default() }))), Arc::new(Mutex::new(Some(User { name: Arc::new(Mutex::new(Some("cy".to_string()))), age: Arc::new(Mutex::new(Some(35))), roles: Arc::new(Mutex::new(Some(vec!["dev".to_string(), "ops".to_string()]))), ..Default::default() })))])));
    let users_closure_clone = users.clone(); { let __sort_target = users_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        return {
            let __tmp_x = (*(*{ let __seq = { let __seq_holder = users_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*{ let __seq = { let __seq_holder = users_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    println!("{} {} {}", format!("{}", { let __text = match { let __seq = { let __seq_holder = users.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*{
        let __recv = {
            let __seq = { let __seq_holder = users.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
            __seq[(1) as usize].clone()
        };
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string();
        __result
    }.lock().unwrap().as_ref().unwrap())), format!("{}", ({ let __len = (*users.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    let mut byRole = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<Arc<Mutex<Option<User>>>>>>>>::from([]))));
    { let __range_holder = users.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for u in __range_values.iter() {
        { let __range_holder = (*u.lock().unwrap().as_ref().unwrap()).roles.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for r in __range_values.iter() {
        { let __map_key = r.clone(); let __map_value = { let __slice = { let __map_holder = byRole.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(r).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = (*u).clone(); let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; (*byRole.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    } }
    { let __target = (*{ let __seq = { let __seq_holder = { let __map = { let __map_holder = byRole.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"admin".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).age.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{} {} {}", format!("{}", (*(*{ let __seq = { let __seq_holder = users.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap())), format!("{}", { let __text = match { let __seq = { let __seq_holder = { let __map = { let __map_holder = byRole.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"admin".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", ({ let __len = (*{ let __map = { let __map_holder = byRole.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"dev".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    let mut idx = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<User>>>>::from([]))));
    { let __range_holder = users.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for u in __range_values.iter() {
        { let __map_key = { let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let __map_value = u.clone(); (*idx.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = (*{ let __map = { let __map_holder = idx.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"cy".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).roles.clone(); let __append_value = "lead".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *(*{ let __map = { let __map_holder = idx.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"cy".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).roles.lock().unwrap() = __slice_header; };
    println!("{} {}", format!("{}", format_slice_values(&(*(*{ let __seq = { let __seq_holder = users.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }.lock().unwrap().as_ref().unwrap()).roles.lock().unwrap().as_ref().unwrap()).clone())), format!("{}", { let __text = match { let __map = { let __map_holder = idx.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"bob".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut w = Arc::new(Mutex::new(Some(upperWriter { buf: Default::default() })));
    let mut n = emit(Arc::new(Mutex::new(Some(Box::new(upperWriterPtr(w.clone())) as Box<dyn Writer + Send + Sync>))), Arc::new(Mutex::new(Some(vec!["hello ".to_string(), "world".to_string()]))));
    println!("{} {}", format!("{}", n), format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __slice_holder = (*w.lock().unwrap().as_ref().unwrap()).buf.clone(); let __slice_guard = __slice_holder.lock().unwrap(); (*__slice_guard.as_ref().unwrap()).clone() })).unwrap()))).lock().unwrap().as_ref().unwrap())));
    { let __s = format!(" {}!", 42); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from({ let __slice_holder = (*w.lock().unwrap().as_ref().unwrap()).buf.clone(); let __slice_guard = __slice_holder.lock().unwrap(); (*__slice_guard.as_ref().unwrap()).clone() })).unwrap()))).lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for Query {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for User {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for upperWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
