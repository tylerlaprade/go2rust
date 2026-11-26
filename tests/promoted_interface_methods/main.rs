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

pub trait Animal: std::fmt::Display + Any {
    fn __go_clone_box_animal(&self) -> Box<dyn Animal + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_animal(&self, other: &(dyn Animal + Send + Sync)) -> bool;
    fn sound(&self) -> Arc<Mutex<Option<String>>>;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Animal + Send + Sync> {
    fn clone(&self) -> Self {
        Animal::__go_clone_box_animal(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Dog {
    pub name: Arc<Mutex<Option<String>>>,
}

impl Dog {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Dog {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for Dog {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


/// Loud overrides Sound and keeps the embedded Animal's Name.
#[derive(Clone, Default)]
pub struct Loud {
    pub animal: Arc<Mutex<Option<Box<dyn Animal + Send + Sync>>>>,
}

impl Loud {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.animal.clone();
        Self {
            animal: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Loud {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.animal.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub trait Store: std::fmt::Display + Any {
    fn __go_clone_box_store(&self) -> Box<dyn Store + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_store(&self, other: &(dyn Store + Send + Sync)) -> bool;
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool);
    fn put(&mut self, key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>);
    fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>>;
}

impl Clone for Box<dyn Store + Send + Sync> {
    fn clone(&self) -> Self {
        Store::__go_clone_box_store(self.as_ref())
    }
}

#[derive(Debug, Clone, Default)]
pub struct mapStore {
    pub m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<String>>>>>>>,
}

impl mapStore {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.m.clone();
        Self {
            m: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for mapStore {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.m));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// logged overrides Put and keeps Get and the unexported keys.
#[derive(Clone, Default)]
pub struct logged {
    pub store: Arc<Mutex<Option<Box<dyn Store + Send + Sync>>>>,
    pub log: Arc<Mutex<Option<Vec<String>>>>,
}

impl logged {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.store.clone();
        let __go_clone_1_0 = { let __guard = self.log.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            store: __go_clone_0_0,
            log: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for logged {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.store.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.log));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// upper gets Put from logged and keys from the Store inside logged.
#[derive(Clone, Default)]
pub struct upper {
    pub logged: Arc<Mutex<Option<logged>>>,
}

impl upper {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.logged.clone();
        Self {
            logged: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for upper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.logged.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Dog {
    pub fn sound(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("woof".to_string())))
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
    }
}

impl Animal for Dog {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Dog::name(self)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        Dog::sound(self)
    }
    fn __go_clone_box_animal(&self) -> Box<dyn Animal + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Animal + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_animal(&self, other: &(dyn Animal + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Dog>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct DogPtr(pub Arc<Mutex<Option<Dog>>>);

impl std::fmt::Display for DogPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Animal for DogPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Dog::name(__recv)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Dog::sound(__recv)
    }
    fn __go_clone_box_animal(&self) -> Box<dyn Animal + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Animal + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_animal(&self, other: &(dyn Animal + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DogPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Loud {
    pub fn sound(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", (*strings::to_upper((*self.animal.lock().unwrap().as_ref().unwrap()).sound()).lock().unwrap().as_ref().unwrap()), "!".to_string()))));
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        let embedded = self.animal.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }
}

impl Animal for Loud {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Loud::name(self)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        Loud::sound(self)
    }
    fn __go_clone_box_animal(&self) -> Box<dyn Animal + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Animal + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_animal(&self, other: &(dyn Animal + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Loud>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct LoudPtr(pub Arc<Mutex<Option<Loud>>>);

impl std::fmt::Display for LoudPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Animal for LoudPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Loud::name(__recv)
    }
    fn sound(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Loud::sound(__recv)
    }
    fn __go_clone_box_animal(&self) -> Box<dyn Animal + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Animal + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_animal(&self, other: &(dyn Animal + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LoudPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl mapStore {
    pub fn get(&self, k: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        let (mut v, mut ok) = { let __map = { let __map_holder = self.m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(String::new()))), false) } };
        return ({ let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, ok);
    }

    pub fn put(&mut self, k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<String>>>) {
        { let __map_key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*self.m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    pub fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut ks: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        for (k, _) in { let __range_holder = self.m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = ks.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *ks.lock().unwrap() = __slice_header; };
    }
        sort::strings(ks.clone());
        return ks.clone();
    }
}

impl Store for mapStore {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        mapStore::get(self, key)
    }
    fn put(&mut self, key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) {
        mapStore::put(self, key, value)
    }
    fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        mapStore::keys(self)
    }
    fn __go_clone_box_store(&self) -> Box<dyn Store + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Store + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_store(&self, other: &(dyn Store + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<mapStore>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct mapStorePtr(pub Arc<Mutex<Option<mapStore>>>);

impl std::fmt::Display for mapStorePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Store for mapStorePtr {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        mapStore::get(__recv, key)
    }
    fn put(&mut self, key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        mapStore::put(__recv, key, value)
    }
    fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        mapStore::keys(__recv)
    }
    fn __go_clone_box_store(&self) -> Box<dyn Store + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Store + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_store(&self, other: &(dyn Store + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<mapStorePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl logged {
    pub fn put(&mut self, k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<String>>>) {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.log.clone(); let __append_value = format!("{}{}", "put ".to_string(), { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *self.log.lock().unwrap() = __slice_header; };
        (*self.store.lock().unwrap().as_mut().unwrap()).put(Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    pub fn get(&self, _arg0: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        let embedded = self.store.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.get(_arg0)
    }

    pub fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let embedded = self.store.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.keys()
    }
}

impl Store for logged {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        logged::get(self, key)
    }
    fn put(&mut self, key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) {
        logged::put(self, key, value)
    }
    fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        logged::keys(self)
    }
    fn __go_clone_box_store(&self) -> Box<dyn Store + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Store + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_store(&self, other: &(dyn Store + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<logged>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct loggedPtr(pub Arc<Mutex<Option<logged>>>);

impl std::fmt::Display for loggedPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Store for loggedPtr {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        logged::get(__recv, key)
    }
    fn put(&mut self, key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        logged::put(__recv, key, value)
    }
    fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        logged::keys(__recv)
    }
    fn __go_clone_box_store(&self) -> Box<dyn Store + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Store + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_store(&self, other: &(dyn Store + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<loggedPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl upper {
    pub fn get(&self, k: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        let (mut v, mut ok) = (*self.logged.lock().unwrap().as_ref().unwrap()).get(Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return (
            strings::to_upper(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))),
            ok
        );
    }

    pub fn put(&mut self, k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<String>>>) {
        // Forward to embedded type's method
        let embedded = self.logged.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.put(k, v)
    }

    pub fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let embedded = self.logged.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.keys()
    }
}

impl Store for upper {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        upper::get(self, key)
    }
    fn put(&mut self, key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) {
        upper::put(self, key, value)
    }
    fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        upper::keys(self)
    }
    fn __go_clone_box_store(&self) -> Box<dyn Store + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Store + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_store(&self, other: &(dyn Store + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<upper>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct upperPtr(pub Arc<Mutex<Option<upper>>>);

impl std::fmt::Display for upperPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Store for upperPtr {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        upper::get(__recv, key)
    }
    fn put(&mut self, key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        upper::put(__recv, key, value)
    }
    fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        upper::keys(__recv)
    }
    fn __go_clone_box_store(&self) -> Box<dyn Store + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Store + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_store(&self, other: &(dyn Store + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<upperPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn dump(s: Arc<Mutex<Option<Box<dyn Store + Send + Sync>>>>) {
    { let __range_holder = (*s.lock().unwrap().as_ref().unwrap()).keys().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for k in __range_values.iter() {
        let (mut v, _) = (*s.lock().unwrap().as_ref().unwrap()).get(Arc::new(Mutex::new(Some((*k).clone()))));
        print!("{}{}{}{}", format!("{}", k), format!("{}", "=".to_string()), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()));
    } }
    println!();
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut a: Arc<Mutex<Option<Box<dyn Animal + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(Dog { name: Arc::new(Mutex::new(Some("rex".to_string()))), ..Default::default() }) as Box<dyn Animal + Send + Sync>)));
    let mut l = Arc::new(Mutex::new(Some(Loud { animal: { let __iface_value = a.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }, ..Default::default() })));
    println!("{} {}", format!("{}", (*(*l.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())), format!("{}", (*(*l.lock().unwrap().as_ref().unwrap()).sound().lock().unwrap().as_ref().unwrap())));
    let mut b: Arc<Mutex<Option<Box<dyn Animal + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Animal + Send + Sync>)));
    println!("{} {}", format!("{}", (*(*b.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())), format!("{}", (*(*b.lock().unwrap().as_ref().unwrap()).sound().lock().unwrap().as_ref().unwrap())));
    let mut base = Arc::new(Mutex::new(Some(mapStore { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([])))), ..Default::default() })));
    let mut lg = {
        let __owner = Arc::new(Mutex::new(Some(logged { store: Arc::new(Mutex::new(Some(Box::new(mapStorePtr(base.clone())) as Box<dyn Store + Send + Sync>))), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().store.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __recv = lg.clone(); let __recv_ptr: *mut logged = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut logged }; let __result = unsafe { &mut *__recv_ptr }.put(Arc::new(Mutex::new(Some("a".to_string()))), Arc::new(Mutex::new(Some("x".to_string())))); __result };
    { let __recv = lg.clone(); let __recv_ptr: *mut logged = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut logged }; let __result = unsafe { &mut *__recv_ptr }.put(Arc::new(Mutex::new(Some("b".to_string()))), Arc::new(Mutex::new(Some("y".to_string())))); __result };
    let (mut v, mut ok) = { let __recv = lg.clone(); let __recv_ptr: *const logged = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const logged }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some("a".to_string())))); __result };
    { let __go_print_arg_1 = { let __recv = lg.clone(); let __recv_ptr: *const logged = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const logged }; let __result = unsafe { &*__recv_ptr }.keys(); __result };
let mut __go_print_arg_1 = { let __slice_header = GoSliceHeader::go_slice_header(&__go_print_arg_1); Arc::new(Mutex::new(__slice_header)) }; let __go_print_value = println!("{} {} {} {}", format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ok), format!("{}", format_slice_values(&(*{ let __field = (*lg.lock().unwrap().as_ref().unwrap()).log.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())), format!("{}", format_slice(&__go_print_arg_1))); __go_print_value };
    dump(Arc::new(Mutex::new(Some(Box::new(loggedPtr(lg.clone())) as Box<dyn Store + Send + Sync>))));
    let mut u = Arc::new(Mutex::new(Some(upper { logged: lg.clone(), ..Default::default() })));
    (*u.lock().unwrap().as_mut().unwrap()).put(Arc::new(Mutex::new(Some("c".to_string()))), Arc::new(Mutex::new(Some("z".to_string()))));
    dump(Arc::new(Mutex::new(Some({ let __arg_value = Box::new({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Store + Send + Sync>; __arg_value }))));
    { let __go_print_arg_2 = Arc::new(Mutex::new(Some(({ let __len = (*(*u.lock().unwrap().as_ref().unwrap()).keys().lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_value = println!("{} {}", format!("{}", format_slice_values(&(*(*(*u.lock().unwrap().as_ref().unwrap()).logged.lock().unwrap().as_ref().unwrap()).log.lock().unwrap().as_ref().unwrap()).clone())), format!("{}", { let __v = (*__go_print_arg_2.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let mut s: Arc<Mutex<Option<Box<dyn Store + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Store + Send + Sync>)));
    { let (__tmp_0, __tmp_1) = (*s.lock().unwrap().as_ref().unwrap()).get(Arc::new(Mutex::new(Some("nope".to_string())))); ok = __tmp_1; };
    println!("{}", format!("{}", ok));
}

impl GoValueClone for Dog {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Loud {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mapStore {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for logged {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for upper {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
