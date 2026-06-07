use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
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
                    format!("{}:{}", k, val)
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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<time_Duration>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
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
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
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

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}

fn go_strconv_format_int(value: i64, base: i32) -> String {
    if base == 10 {
        return value.to_string();
    }
    if !(2..=36).contains(&base) {
        return value.to_string();
    }

    let negative = value < 0;
    let mut n = if negative {
        value.wrapping_neg() as u64
    } else {
        value as u64
    };
    let base = base as u64;
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut out = Vec::new();
    if n == 0 {
        out.push(b'0');
    }
    while n > 0 {
        out.push(digits[(n % base) as usize]);
        n /= base;
    }
    if negative {
        out.push(b'-');
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn go_strconv_format_float(value: f64, fmt: char, precision: i32) -> String {
    let precision = if precision < 0 { 6 } else { precision as usize };
    match fmt {
        'e' => format!("{:.*e}", precision, value),
        'E' => format!("{:.*E}", precision, value),
        'f' => format!("{:.*}", precision, value),
        'g' | 'G' => {
            if precision == 0 {
                format!("{:.0}", value)
            } else {
                format!("{:.*}", precision, value)
            }
        }
        _ => value.to_string(),
    }
}

static __GO_OS_ARGS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> =
    std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(Some(std::env::args().collect::<Vec<String>>()))));

fn go_os_args() -> std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>> {
    __GO_OS_ARGS.clone()
}

#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Arc<Mutex<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let raw = (*self.raw.lock().unwrap().as_ref().unwrap()).clone();
        let key = (*key.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Arc<Mutex<Option<String>>>,
    tag: Arc<Mutex<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Arc<Mutex<Option<String>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lock().unwrap().as_ref().unwrap())
    }
}

impl GoReflectType {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self.name.lock().unwrap().as_ref().unwrap()).clone())))
    }

    fn num_field(&self) -> i32 {
        self.fields.lock().unwrap().as_ref().unwrap().len() as i32
    }

    fn field(&self, index: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<GoReflectField>>> {
        let index = *index.lock().unwrap().as_ref().unwrap() as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }
}

type GoReflectBoolGetter = Box<dyn Fn() -> bool + Send + Sync>;
type GoReflectBoolSetter = Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync>;

#[derive(Clone)]
struct GoReflectValue {
    typ: Arc<Mutex<Option<GoReflectType>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectValue>>>>,
    bool_getter: Arc<Mutex<Option<GoReflectBoolGetter>>>,
    bool_setter: Arc<Mutex<Option<GoReflectBoolSetter>>>,
}

impl GoReflectValue {
    fn elem(&self) -> Arc<Mutex<Option<GoReflectValue>>> {
        Arc::new(Mutex::new(Some(self.clone())))
    }

    fn r#type(&self) -> Arc<Mutex<Option<GoReflectType>>> {
        self.typ.clone()
    }

    fn field(&self, index: i32) -> Arc<Mutex<Option<GoReflectValue>>> {
        let index = index as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }

    fn set_bool(&mut self, value: Arc<Mutex<Option<bool>>>) {
        let mut setter_guard = self.bool_setter.lock().unwrap();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        let getter_guard = self.bool_getter.lock().unwrap();
        let getter = getter_guard.as_ref().expect("reflect.Value.Bool requires a bool field");
        getter()
    }
}

fn go_reflect_tag_get(raw: &str, key: &str) -> String {
    let prefix = format!("{}:\"", key);
    let Some(start) = raw.find(&prefix) else {
        return String::new();
    };
    let rest = &raw[start + prefix.len()..];
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            break;
        } else {
            value.push(ch);
        }
    }
    value
}

pub const CONTINUE_ON_ERROR: i32 = 0;
pub const EXIT_ON_ERROR: i32 = 1;
pub const PANIC_ON_ERROR: i32 = 2;


/// -- bool Value
#[derive(Debug, Clone, Default)]
pub struct boolValue(pub Arc<Mutex<Option<bool>>>);

impl Display for boolValue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for boolValue {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


/// optional interface to indicate boolean flags that can be
/// supplied without "=value" text
pub trait boolFlag: Value + std::fmt::Display + Any {
    fn __go_clone_box_bool_flag(&self) -> Box<dyn boolFlag + Send + Sync>;
    fn __go_eq_bool_flag(&self, other: &(dyn boolFlag + Send + Sync)) -> bool;
    fn is_bool_flag(&self) -> bool;
}

impl Clone for Box<dyn boolFlag + Send + Sync> {
    fn clone(&self) -> Self {
        boolFlag::__go_clone_box_bool_flag(self.as_ref())
    }
}

impl Value for Box<dyn boolFlag + Send + Sync> {
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        (**self).__go_eq_value(other)
    }
    fn set(&mut self, _arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        (**self).set(_arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        (**self).string()
    }
}

/// -- int Value
#[derive(Debug, Clone, Default)]
pub struct intValue(pub Arc<Mutex<Option<i32>>>);

impl Display for intValue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for intValue {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for intValue {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for intValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for intValue {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<intValue> for i32 {
    fn eq(&self, other: &intValue) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<intValue> for i32 {
    fn partial_cmp(&self, other: &intValue) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for intValue {
    type Output = intValue;
    fn add(self, other: Self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for intValue {
    type Output = intValue;
    fn add(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<intValue> for i32 {
    type Output = intValue;
    fn add(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for intValue {
    type Output = intValue;
    fn sub(self, other: Self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for intValue {
    type Output = intValue;
    fn sub(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<intValue> for i32 {
    type Output = intValue;
    fn sub(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for intValue {
    type Output = intValue;
    fn mul(self, other: Self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for intValue {
    type Output = intValue;
    fn mul(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<intValue> for i32 {
    type Output = intValue;
    fn mul(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for intValue {
    type Output = intValue;
    fn div(self, other: Self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for intValue {
    type Output = intValue;
    fn div(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<intValue> for i32 {
    type Output = intValue;
    fn div(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for intValue {
    type Output = intValue;
    fn neg(self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for intValue {
    type Output = intValue;
    fn rem(self, other: Self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for intValue {
    type Output = intValue;
    fn rem(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<intValue> for i32 {
    type Output = intValue;
    fn rem(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for intValue {
    type Output = intValue;
    fn bitand(self, other: Self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for intValue {
    type Output = intValue;
    fn bitand(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<intValue> for i32 {
    type Output = intValue;
    fn bitand(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for intValue {
    type Output = intValue;
    fn bitor(self, other: Self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for intValue {
    type Output = intValue;
    fn bitor(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<intValue> for i32 {
    type Output = intValue;
    fn bitor(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for intValue {
    type Output = intValue;
    fn bitxor(self, other: Self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for intValue {
    type Output = intValue;
    fn bitxor(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<intValue> for i32 {
    type Output = intValue;
    fn bitxor(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for intValue {
    type Output = intValue;
    fn not(self) -> intValue {
        intValue(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for intValue {
    type Output = intValue;
    fn shl(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for intValue {
    type Output = intValue;
    fn shl(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for intValue {
    type Output = intValue;
    fn shl(self, other: i8) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for intValue {
    type Output = intValue;
    fn shl(self, other: i16) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for intValue {
    type Output = intValue;
    fn shl(self, other: i64) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for intValue {
    type Output = intValue;
    fn shl(self, other: u32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for intValue {
    type Output = intValue;
    fn shl(self, other: u8) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for intValue {
    type Output = intValue;
    fn shl(self, other: u16) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for intValue {
    type Output = intValue;
    fn shl(self, other: u64) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for intValue {
    type Output = intValue;
    fn shl(self, other: usize) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for intValue {
    type Output = intValue;
    fn shr(self, other: intValue) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for intValue {
    type Output = intValue;
    fn shr(self, other: i32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for intValue {
    type Output = intValue;
    fn shr(self, other: i8) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for intValue {
    type Output = intValue;
    fn shr(self, other: i16) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for intValue {
    type Output = intValue;
    fn shr(self, other: i64) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for intValue {
    type Output = intValue;
    fn shr(self, other: u32) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for intValue {
    type Output = intValue;
    fn shr(self, other: u8) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for intValue {
    type Output = intValue;
    fn shr(self, other: u16) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for intValue {
    type Output = intValue;
    fn shr(self, other: u64) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for intValue {
    type Output = intValue;
    fn shr(self, other: usize) -> intValue {
        intValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for intValue {}

impl Ord for intValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// -- int64 Value
#[derive(Debug, Clone, Default)]
pub struct int64Value(pub Arc<Mutex<Option<i64>>>);

impl Display for int64Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for int64Value {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for int64Value {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for int64Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for int64Value {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<int64Value> for i64 {
    fn eq(&self, other: &int64Value) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<int64Value> for i64 {
    fn partial_cmp(&self, other: &int64Value) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for int64Value {
    type Output = int64Value;
    fn add(self, other: Self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for int64Value {
    type Output = int64Value;
    fn add(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<int64Value> for i64 {
    type Output = int64Value;
    fn add(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for int64Value {
    type Output = int64Value;
    fn sub(self, other: Self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for int64Value {
    type Output = int64Value;
    fn sub(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<int64Value> for i64 {
    type Output = int64Value;
    fn sub(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for int64Value {
    type Output = int64Value;
    fn mul(self, other: Self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for int64Value {
    type Output = int64Value;
    fn mul(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<int64Value> for i64 {
    type Output = int64Value;
    fn mul(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for int64Value {
    type Output = int64Value;
    fn div(self, other: Self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for int64Value {
    type Output = int64Value;
    fn div(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<int64Value> for i64 {
    type Output = int64Value;
    fn div(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for int64Value {
    type Output = int64Value;
    fn neg(self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for int64Value {
    type Output = int64Value;
    fn rem(self, other: Self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for int64Value {
    type Output = int64Value;
    fn rem(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<int64Value> for i64 {
    type Output = int64Value;
    fn rem(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for int64Value {
    type Output = int64Value;
    fn bitand(self, other: Self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for int64Value {
    type Output = int64Value;
    fn bitand(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<int64Value> for i64 {
    type Output = int64Value;
    fn bitand(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for int64Value {
    type Output = int64Value;
    fn bitor(self, other: Self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for int64Value {
    type Output = int64Value;
    fn bitor(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<int64Value> for i64 {
    type Output = int64Value;
    fn bitor(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for int64Value {
    type Output = int64Value;
    fn bitxor(self, other: Self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for int64Value {
    type Output = int64Value;
    fn bitxor(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<int64Value> for i64 {
    type Output = int64Value;
    fn bitxor(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for int64Value {
    type Output = int64Value;
    fn not(self) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for int64Value {
    type Output = int64Value;
    fn shl(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for int64Value {
    type Output = int64Value;
    fn shl(self, other: i32) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for int64Value {
    type Output = int64Value;
    fn shl(self, other: i8) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for int64Value {
    type Output = int64Value;
    fn shl(self, other: i16) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for int64Value {
    type Output = int64Value;
    fn shl(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for int64Value {
    type Output = int64Value;
    fn shl(self, other: u32) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for int64Value {
    type Output = int64Value;
    fn shl(self, other: u8) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for int64Value {
    type Output = int64Value;
    fn shl(self, other: u16) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for int64Value {
    type Output = int64Value;
    fn shl(self, other: u64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for int64Value {
    type Output = int64Value;
    fn shl(self, other: usize) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for int64Value {
    type Output = int64Value;
    fn shr(self, other: int64Value) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for int64Value {
    type Output = int64Value;
    fn shr(self, other: i32) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for int64Value {
    type Output = int64Value;
    fn shr(self, other: i8) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for int64Value {
    type Output = int64Value;
    fn shr(self, other: i16) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for int64Value {
    type Output = int64Value;
    fn shr(self, other: i64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for int64Value {
    type Output = int64Value;
    fn shr(self, other: u32) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for int64Value {
    type Output = int64Value;
    fn shr(self, other: u8) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for int64Value {
    type Output = int64Value;
    fn shr(self, other: u16) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for int64Value {
    type Output = int64Value;
    fn shr(self, other: u64) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for int64Value {
    type Output = int64Value;
    fn shr(self, other: usize) -> int64Value {
        int64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for int64Value {}

impl Ord for int64Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// -- uint Value
#[derive(Debug, Clone, Default)]
pub struct uintValue(pub Arc<Mutex<Option<u64>>>);

impl Display for uintValue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for uintValue {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for uintValue {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for uintValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for uintValue {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<uintValue> for u64 {
    fn eq(&self, other: &uintValue) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<uintValue> for u64 {
    fn partial_cmp(&self, other: &uintValue) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for uintValue {
    type Output = uintValue;
    fn add(self, other: Self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for uintValue {
    type Output = uintValue;
    fn add(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<uintValue> for u64 {
    type Output = uintValue;
    fn add(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for uintValue {
    type Output = uintValue;
    fn sub(self, other: Self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for uintValue {
    type Output = uintValue;
    fn sub(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<uintValue> for u64 {
    type Output = uintValue;
    fn sub(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for uintValue {
    type Output = uintValue;
    fn mul(self, other: Self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for uintValue {
    type Output = uintValue;
    fn mul(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<uintValue> for u64 {
    type Output = uintValue;
    fn mul(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for uintValue {
    type Output = uintValue;
    fn div(self, other: Self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for uintValue {
    type Output = uintValue;
    fn div(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<uintValue> for u64 {
    type Output = uintValue;
    fn div(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for uintValue {
    type Output = uintValue;
    fn rem(self, other: Self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for uintValue {
    type Output = uintValue;
    fn rem(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<uintValue> for u64 {
    type Output = uintValue;
    fn rem(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for uintValue {
    type Output = uintValue;
    fn bitand(self, other: Self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for uintValue {
    type Output = uintValue;
    fn bitand(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<uintValue> for u64 {
    type Output = uintValue;
    fn bitand(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for uintValue {
    type Output = uintValue;
    fn bitor(self, other: Self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for uintValue {
    type Output = uintValue;
    fn bitor(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<uintValue> for u64 {
    type Output = uintValue;
    fn bitor(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for uintValue {
    type Output = uintValue;
    fn bitxor(self, other: Self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for uintValue {
    type Output = uintValue;
    fn bitxor(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<uintValue> for u64 {
    type Output = uintValue;
    fn bitxor(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for uintValue {
    type Output = uintValue;
    fn not(self) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for uintValue {
    type Output = uintValue;
    fn shl(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for uintValue {
    type Output = uintValue;
    fn shl(self, other: i32) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for uintValue {
    type Output = uintValue;
    fn shl(self, other: i8) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for uintValue {
    type Output = uintValue;
    fn shl(self, other: i16) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for uintValue {
    type Output = uintValue;
    fn shl(self, other: i64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for uintValue {
    type Output = uintValue;
    fn shl(self, other: u32) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for uintValue {
    type Output = uintValue;
    fn shl(self, other: u8) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for uintValue {
    type Output = uintValue;
    fn shl(self, other: u16) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for uintValue {
    type Output = uintValue;
    fn shl(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for uintValue {
    type Output = uintValue;
    fn shl(self, other: usize) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for uintValue {
    type Output = uintValue;
    fn shr(self, other: uintValue) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for uintValue {
    type Output = uintValue;
    fn shr(self, other: i32) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for uintValue {
    type Output = uintValue;
    fn shr(self, other: i8) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for uintValue {
    type Output = uintValue;
    fn shr(self, other: i16) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for uintValue {
    type Output = uintValue;
    fn shr(self, other: i64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for uintValue {
    type Output = uintValue;
    fn shr(self, other: u32) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for uintValue {
    type Output = uintValue;
    fn shr(self, other: u8) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for uintValue {
    type Output = uintValue;
    fn shr(self, other: u16) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for uintValue {
    type Output = uintValue;
    fn shr(self, other: u64) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for uintValue {
    type Output = uintValue;
    fn shr(self, other: usize) -> uintValue {
        uintValue(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for uintValue {}

impl Ord for uintValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// -- uint64 Value
#[derive(Debug, Clone, Default)]
pub struct uint64Value(pub Arc<Mutex<Option<u64>>>);

impl Display for uint64Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for uint64Value {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for uint64Value {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for uint64Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for uint64Value {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<uint64Value> for u64 {
    fn eq(&self, other: &uint64Value) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<uint64Value> for u64 {
    fn partial_cmp(&self, other: &uint64Value) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for uint64Value {
    type Output = uint64Value;
    fn add(self, other: Self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for uint64Value {
    type Output = uint64Value;
    fn add(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<uint64Value> for u64 {
    type Output = uint64Value;
    fn add(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for uint64Value {
    type Output = uint64Value;
    fn sub(self, other: Self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for uint64Value {
    type Output = uint64Value;
    fn sub(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<uint64Value> for u64 {
    type Output = uint64Value;
    fn sub(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for uint64Value {
    type Output = uint64Value;
    fn mul(self, other: Self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for uint64Value {
    type Output = uint64Value;
    fn mul(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<uint64Value> for u64 {
    type Output = uint64Value;
    fn mul(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for uint64Value {
    type Output = uint64Value;
    fn div(self, other: Self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for uint64Value {
    type Output = uint64Value;
    fn div(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<uint64Value> for u64 {
    type Output = uint64Value;
    fn div(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for uint64Value {
    type Output = uint64Value;
    fn rem(self, other: Self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for uint64Value {
    type Output = uint64Value;
    fn rem(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<uint64Value> for u64 {
    type Output = uint64Value;
    fn rem(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for uint64Value {
    type Output = uint64Value;
    fn bitand(self, other: Self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for uint64Value {
    type Output = uint64Value;
    fn bitand(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<uint64Value> for u64 {
    type Output = uint64Value;
    fn bitand(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for uint64Value {
    type Output = uint64Value;
    fn bitor(self, other: Self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for uint64Value {
    type Output = uint64Value;
    fn bitor(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<uint64Value> for u64 {
    type Output = uint64Value;
    fn bitor(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for uint64Value {
    type Output = uint64Value;
    fn bitxor(self, other: Self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for uint64Value {
    type Output = uint64Value;
    fn bitxor(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<uint64Value> for u64 {
    type Output = uint64Value;
    fn bitxor(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for uint64Value {
    type Output = uint64Value;
    fn not(self) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: i32) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: i8) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: i16) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: i64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: u32) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: u8) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: u16) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for uint64Value {
    type Output = uint64Value;
    fn shl(self, other: usize) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: uint64Value) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: i32) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: i8) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: i16) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: i64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: u32) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: u8) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: u16) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: u64) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for uint64Value {
    type Output = uint64Value;
    fn shr(self, other: usize) -> uint64Value {
        uint64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for uint64Value {}

impl Ord for uint64Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// -- string Value
#[derive(Debug, Clone, Default)]
pub struct stringValue(pub Arc<Mutex<Option<String>>>);

impl Display for stringValue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for stringValue {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


/// -- float64 Value
#[derive(Debug, Clone, Default)]
pub struct float64Value(pub Arc<Mutex<Option<f64>>>);

impl Display for float64Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for float64Value {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for float64Value {
    fn eq(&self, other: &f64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for float64Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for float64Value {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<float64Value> for f64 {
    fn eq(&self, other: &float64Value) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<float64Value> for f64 {
    fn partial_cmp(&self, other: &float64Value) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for float64Value {
    type Output = float64Value;
    fn add(self, other: Self) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for float64Value {
    type Output = float64Value;
    fn add(self, other: f64) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<float64Value> for f64 {
    type Output = float64Value;
    fn add(self, other: float64Value) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for float64Value {
    type Output = float64Value;
    fn sub(self, other: Self) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for float64Value {
    type Output = float64Value;
    fn sub(self, other: f64) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<float64Value> for f64 {
    type Output = float64Value;
    fn sub(self, other: float64Value) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for float64Value {
    type Output = float64Value;
    fn mul(self, other: Self) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for float64Value {
    type Output = float64Value;
    fn mul(self, other: f64) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<float64Value> for f64 {
    type Output = float64Value;
    fn mul(self, other: float64Value) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for float64Value {
    type Output = float64Value;
    fn div(self, other: Self) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for float64Value {
    type Output = float64Value;
    fn div(self, other: f64) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<float64Value> for f64 {
    type Output = float64Value;
    fn div(self, other: float64Value) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for float64Value {
    type Output = float64Value;
    fn neg(self) -> float64Value {
        float64Value(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}


/// -- time.Duration Value
#[derive(Debug, Clone, Default)]
pub struct durationValue(pub Arc<Mutex<Option<time_Duration>>>);

impl Display for durationValue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


/// -- encoding.TextUnmarshaler Value
#[derive(Clone, Default)]
pub struct textValue {
    pub p: Arc<Mutex<Option<encoding_TextUnmarshaler>>>,
}

impl textValue {
    pub fn __go_value_clone(&self) -> Self {
        Self { p: self.p.clone() }
    }
}

impl std::fmt::Display for textValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for textValue {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// -- func Value
pub type funcValue = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>;

#[derive(Clone)]
pub struct funcValueAsValue(pub funcValue);

impl std::fmt::Display for funcValueAsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<funcValueAsValue>")
    }
}

impl Value for funcValueAsValue {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        funcValueMethods::set(&self.0, __arg0.clone())
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        funcValueMethods::string(&self.0)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone())
    }
    fn __go_as_any(&self) -> &dyn std::any::Any {
        &self.0
    }
    fn __go_eq_value(&self, _other: &(dyn Value + Send + Sync)) -> bool {
        false
    }
}


/// -- boolFunc Value
pub type boolFuncValue = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>;

#[derive(Clone)]
pub struct boolFuncValueAsValue(pub boolFuncValue);

impl std::fmt::Display for boolFuncValueAsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<boolFuncValueAsValue>")
    }
}

impl Value for boolFuncValueAsValue {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        boolFuncValueMethods::set(&self.0, __arg0.clone())
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        boolFuncValueMethods::string(&self.0)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone())
    }
    fn __go_as_any(&self) -> &dyn std::any::Any {
        &self.0
    }
    fn __go_eq_value(&self, _other: &(dyn Value + Send + Sync)) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct boolFuncValueAsboolFlag(pub boolFuncValue);

impl std::fmt::Display for boolFuncValueAsboolFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<boolFuncValueAsboolFlag>")
    }
}

impl Value for boolFuncValueAsboolFlag {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        boolFuncValueMethods::set(&self.0, __arg0.clone())
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        boolFuncValueMethods::string(&self.0)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone())
    }
    fn __go_as_any(&self) -> &dyn std::any::Any {
        &self.0
    }
    fn __go_eq_value(&self, _other: &(dyn Value + Send + Sync)) -> bool {
        false
    }
}
impl boolFlag for boolFuncValueAsboolFlag {
    fn is_bool_flag(&self) -> bool {
        boolFuncValueMethods::is_bool_flag(&self.0)
    }
    fn __go_clone_box_bool_flag(&self) -> Box<dyn boolFlag + Send + Sync> {
        Box::new(self.clone())
    }
    fn __go_eq_bool_flag(&self, _other: &(dyn boolFlag + Send + Sync)) -> bool {
        false
    }
}


/// Value is the interface to the dynamic value stored in a flag.
/// (The default value is represented as a string.)
///
/// If a Value has an IsBoolFlag() bool method returning true,
/// the command-line parser makes -name equivalent to -name=true
/// rather than using the next command-line argument.
///
/// Set is called once, in command line order, for each flag present.
/// The flag package may call the [String] method with a zero-valued receiver,
/// such as a nil pointer.
pub trait Value: std::fmt::Display + Any {
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
}

impl Clone for Box<dyn Value + Send + Sync> {
    fn clone(&self) -> Self {
        Value::__go_clone_box_value(self.as_ref())
    }
}

/// ErrorHandling defines how [FlagSet.Parse] behaves if the parse fails.
#[derive(Debug, Clone, Default)]
pub struct ErrorHandling(pub Arc<Mutex<Option<i32>>>);

impl Display for ErrorHandling {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for ErrorHandling {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for ErrorHandling {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ErrorHandling {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for ErrorHandling {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ErrorHandling> for i32 {
    fn eq(&self, other: &ErrorHandling) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ErrorHandling> for i32 {
    fn partial_cmp(&self, other: &ErrorHandling) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ErrorHandling {
    type Output = ErrorHandling;
    fn add(self, other: Self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn add(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ErrorHandling> for i32 {
    type Output = ErrorHandling;
    fn add(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ErrorHandling {
    type Output = ErrorHandling;
    fn sub(self, other: Self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn sub(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ErrorHandling> for i32 {
    type Output = ErrorHandling;
    fn sub(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ErrorHandling {
    type Output = ErrorHandling;
    fn mul(self, other: Self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn mul(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ErrorHandling> for i32 {
    type Output = ErrorHandling;
    fn mul(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ErrorHandling {
    type Output = ErrorHandling;
    fn div(self, other: Self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn div(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ErrorHandling> for i32 {
    type Output = ErrorHandling;
    fn div(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for ErrorHandling {
    type Output = ErrorHandling;
    fn neg(self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ErrorHandling {
    type Output = ErrorHandling;
    fn rem(self, other: Self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn rem(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ErrorHandling> for i32 {
    type Output = ErrorHandling;
    fn rem(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ErrorHandling {
    type Output = ErrorHandling;
    fn bitand(self, other: Self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn bitand(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ErrorHandling> for i32 {
    type Output = ErrorHandling;
    fn bitand(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ErrorHandling {
    type Output = ErrorHandling;
    fn bitor(self, other: Self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn bitor(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ErrorHandling> for i32 {
    type Output = ErrorHandling;
    fn bitor(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ErrorHandling {
    type Output = ErrorHandling;
    fn bitxor(self, other: Self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn bitxor(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ErrorHandling> for i32 {
    type Output = ErrorHandling;
    fn bitxor(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ErrorHandling {
    type Output = ErrorHandling;
    fn not(self) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: i8) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: i16) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: i64) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: u32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: u8) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: u16) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: u64) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ErrorHandling {
    type Output = ErrorHandling;
    fn shl(self, other: usize) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: ErrorHandling) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: i32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: i8) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: i16) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: i64) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: u32) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: u8) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: u16) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: u64) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ErrorHandling {
    type Output = ErrorHandling;
    fn shr(self, other: usize) -> ErrorHandling {
        ErrorHandling(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ErrorHandling {}

impl Ord for ErrorHandling {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A FlagSet represents a set of defined flags. The zero value of a FlagSet
/// has no name and has [ContinueOnError] error handling.
///
/// [Flag] names must be unique within a FlagSet. An attempt to define a flag whose
/// name is already in use will cause a panic.
#[derive(Clone)]
pub struct FlagSet {
    pub usage: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub parsed: Arc<Mutex<Option<bool>>>,
    pub actual: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Flag>>>>>>>,
    pub formal: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Flag>>>>>>>,
    pub args: Arc<Mutex<Option<Vec<String>>>>,
    pub error_handling: Arc<Mutex<Option<ErrorHandling>>>,
    pub output: Arc<Mutex<Option<io_Writer>>>,
    pub undef: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<String>>>>>>>,
}

impl FlagSet {
    pub fn __go_value_clone(&self) -> Self {
        Self { usage: self.usage.clone(), name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, parsed: { let __guard = self.parsed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, actual: self.actual.clone(), formal: self.formal.clone(), args: self.args.clone(), error_handling: { let __guard = self.error_handling.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, output: self.output.clone(), undef: self.undef.clone() }
    }
}


impl Default for FlagSet {
    fn default() -> Self {
        Self { usage: Arc::new(Mutex::new(None)), name: Arc::new(Mutex::new(Some(String::new()))), parsed: Arc::new(Mutex::new(Some(false))), actual: Arc::new(Mutex::new(None)), formal: Arc::new(Mutex::new(None)), args: Arc::new(Mutex::new(None)), error_handling: Arc::new(Mutex::new(Some(ErrorHandling(Arc::new(Mutex::new(Some(0))))))), output: Arc::new(Mutex::new(None)), undef: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for FlagSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", "<func>", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.parsed.lock().unwrap().as_ref().unwrap()), format_map(&self.actual), format_map(&self.formal), format_slice(&self.args), (*self.error_handling.lock().unwrap().as_ref().unwrap()), (*self.output.lock().unwrap().as_ref().unwrap()), format_map(&self.undef))
    }
}

impl GoJsonDecode for FlagSet {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Flag represents the state of a flag.
#[derive(Clone)]
pub struct Flag {
    pub name: Arc<Mutex<Option<String>>>,
    pub usage: Arc<Mutex<Option<String>>>,
    pub value: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>,
    pub def_value: Arc<Mutex<Option<String>>>,
}

impl Flag {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, usage: { let __guard = self.usage.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, value: self.value.clone(), def_value: { let __guard = self.def_value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Flag {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), usage: Arc::new(Mutex::new(Some(String::new()))), value: Arc::new(Mutex::new(None)), def_value: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.usage.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()), (*self.def_value.lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Flag {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for Flag {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Usage") {
            out.usage = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("DefValue") {
            out.def_value = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub static ErrHelp: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errParse: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errRange: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Usage: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static CommandLine: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<FlagSet>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrHelp.lock().unwrap() = None;
    *errParse.lock().unwrap() = None;
    *errRange.lock().unwrap() = None;
    *CommandLine.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("flag: help requested".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrHelp.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("parse error".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errParse.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("value out of range".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errRange.lock().unwrap() = new_val; }
    *Usage.lock().unwrap() = Some(Box::new(move || {
        { let __s = format!("Usage of {}:\n", { let __seq = { let __seq_holder = go_os_args().clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }); let __n = __s.len() as i32; (*{ let __recv_holder = (*CommandLine.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.output(); __result }.lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };
        print_defaults();
    }) as Box<dyn FnMut() -> () + Send + Sync>);
}


impl boolValue {
    pub fn set(&mut self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut v, mut err) = { let __parse_bool_input = (*s.lock().unwrap().as_ref().unwrap()).clone(); match __parse_bool_input.as_str() { "1" | "t" | "T" | "TRUE" | "true" | "True" => (true, Arc::new(Mutex::new(None))), "0" | "f" | "F" | "FALSE" | "false" | "False" => (false, Arc::new(Mutex::new(None))), _ => (false, Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("strconv.ParseBool: parsing \"{}\": invalid syntax", __parse_bool_input)))))) } };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = errParse.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        { let new_val = boolValue(Arc::new(Mutex::new(Some(v)))); *self = new_val; };
        return err.clone();
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new({ let __v = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        strconv::format_bool(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))))
    }

    pub fn is_bool_flag(&self) -> bool {
        true
    }
}

impl Value for boolValue {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        boolValue::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        boolValue::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boolValue>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct boolValuePtr(pub Arc<Mutex<Option<boolValue>>>);

impl std::fmt::Display for boolValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for boolValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        boolValue::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        boolValue::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boolValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl boolFlag for boolValue {
    fn is_bool_flag(&self) -> bool {
        boolValue::is_bool_flag(self)
    }
    fn __go_clone_box_bool_flag(&self) -> Box<dyn boolFlag + Send + Sync> {
        Box::new(self.clone()) as Box<dyn boolFlag + Send + Sync>
    }
    fn __go_eq_bool_flag(&self, other: &(dyn boolFlag + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boolValue>() {
            self == __other
        } else {
            false
        }
    }
}

impl boolFlag for boolValuePtr {
    fn is_bool_flag(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        boolValue::is_bool_flag(__recv)
    }
    fn __go_clone_box_bool_flag(&self) -> Box<dyn boolFlag + Send + Sync> {
        Box::new(self.clone()) as Box<dyn boolFlag + Send + Sync>
    }
    fn __go_eq_bool_flag(&self, other: &(dyn boolFlag + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boolValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl intValue {
    pub fn set(&mut self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut v, mut err) = strconv::parse_int({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, 0, strconv::INT_SIZE);
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = num_error(err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        { let new_val = intValue(Arc::new(Mutex::new(Some(v as i32)))); *self = new_val; };
        return err.clone();
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new({ let __v = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) as i32).to_string())))
    }
}

impl Value for intValue {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        intValue::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        intValue::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<intValue>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct intValuePtr(pub Arc<Mutex<Option<intValue>>>);

impl std::fmt::Display for intValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for intValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        intValue::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        intValue::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<intValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl int64Value {
    pub fn set(&mut self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut v, mut err) = strconv::parse_int({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, 0, 64);
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = num_error(err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        { let new_val = int64Value(Arc::new(Mutex::new(Some(v as i64)))); *self = new_val; };
        return err.clone();
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new({ let __v = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i64))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(go_strconv_format_int((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()) as i64, 10 as i32))))
    }
}

impl Value for int64Value {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        int64Value::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        int64Value::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<int64Value>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct int64ValuePtr(pub Arc<Mutex<Option<int64Value>>>);

impl std::fmt::Display for int64ValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for int64ValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        int64Value::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        int64Value::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<int64ValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl uintValue {
    pub fn set(&mut self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut v, mut err) = strconv::parse_uint({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, 0, strconv::INT_SIZE);
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = num_error(err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        { let new_val = uintValue(Arc::new(Mutex::new(Some(v as u64)))); *self = new_val; };
        return err.clone();
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new({ let __v = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        strconv::format_uint(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))), 10)
    }
}

impl Value for uintValue {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        uintValue::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        uintValue::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<uintValue>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct uintValuePtr(pub Arc<Mutex<Option<uintValue>>>);

impl std::fmt::Display for uintValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for uintValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        uintValue::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        uintValue::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<uintValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl uint64Value {
    pub fn set(&mut self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut v, mut err) = strconv::parse_uint({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, 0, 64);
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = num_error(err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        { let new_val = uint64Value(Arc::new(Mutex::new(Some(v as u64)))); *self = new_val; };
        return err.clone();
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new({ let __v = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        strconv::format_uint(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))), 10)
    }
}

impl Value for uint64Value {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        uint64Value::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        uint64Value::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<uint64Value>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct uint64ValuePtr(pub Arc<Mutex<Option<uint64Value>>>);

impl std::fmt::Display for uint64ValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for uint64ValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        uint64Value::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        uint64Value::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<uint64ValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl stringValue {
    pub fn set(&mut self, val: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        { let new_val = stringValue(Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()).clone())))); *self = new_val; };
        return Arc::new(Mutex::new(None));
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new({ let __v = Arc::new(Mutex::new(Some((*self).clone().to_string()))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self).clone().to_string())))
    }
}

impl Value for stringValue {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        stringValue::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        stringValue::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<stringValue>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct stringValuePtr(pub Arc<Mutex<Option<stringValue>>>);

impl std::fmt::Display for stringValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for stringValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        stringValue::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        stringValue::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<stringValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl float64Value {
    pub fn set(&mut self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut v, mut err) = strconv::parse_float({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, 64);
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = num_error(err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        { let new_val = float64Value(Arc::new(Mutex::new(Some(v as f64)))); *self = new_val; };
        return err.clone();
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new({ let __v = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as f64))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(go_strconv_format_float((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()) as f64, char::from_u32((('g' as i32)) as u32).unwrap_or('f'), -(1) as i32))))
    }
}

impl Value for float64Value {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        float64Value::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        float64Value::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<float64Value>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct float64ValuePtr(pub Arc<Mutex<Option<float64Value>>>);

impl std::fmt::Display for float64ValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for float64ValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        float64Value::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        float64Value::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<float64ValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl durationValue {
    pub fn set(&mut self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut v, mut err) = time::parse_duration({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = errParse.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        { let new_val = durationValue(Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone())))); *self = new_val; };
        return err.clone();
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new((*self.0.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        time_Duration::string(&(*Arc::new(Mutex::new(Some(time_Duration::default()))).lock().unwrap().as_ref().unwrap()))
    }
}

impl Value for durationValue {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        durationValue::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        durationValue::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<durationValue>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct durationValuePtr(pub Arc<Mutex<Option<durationValue>>>);

impl std::fmt::Display for durationValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for durationValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        durationValue::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        durationValue::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<durationValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl textValue {
    pub fn set(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        (*self.p.lock().unwrap().as_ref().unwrap()).unmarshal_text(Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_vec()))))
    }

    pub fn get(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        return Arc::new(Mutex::new(Some(Box::new({ let __selector_holder = self.p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>)));
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        {
        let (mut m, mut ok) = ({
        let val = self.p.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<encoding_TextMarshaler>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<encoding_TextMarshaler>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<encoding_TextMarshaler>)), false)
        }
    });;
        if ok {
            {
        let (mut b, mut err) = (*m.lock().unwrap().as_ref().unwrap()).marshal_text();;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            return Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));;
        }
    };
        }
    }
        Arc::new(Mutex::new(Some("".to_string())))
    }
}

impl Value for textValue {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        textValue::set(self, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        textValue::string(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<textValue>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct textValuePtr(pub Arc<Mutex<Option<textValue>>>);

impl std::fmt::Display for textValuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for textValuePtr {
    fn set(&mut self, __arg0: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        textValue::set(__recv, __arg0)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        textValue::string(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<textValuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub trait funcValueMethods {
    fn set(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
}

impl funcValueMethods for funcValue {
    fn set(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = self.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) }
    }

    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("".to_string())))
    }
}

pub trait boolFuncValueMethods {
    fn set(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
    fn is_bool_flag(&self) -> bool;
}

impl boolFuncValueMethods for boolFuncValue {
    fn set(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = self.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) }
    }

    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("".to_string())))
    }

    fn is_bool_flag(&self) -> bool {
        true
    }
}

impl FlagSet {
    /// Output returns the destination for usage and error messages. [os.Stderr] is returned if
    /// output was not set or was set to nil.
    pub fn output(&self) -> Arc<Mutex<Option<io_Writer>>> {
        if { let __nil_target = self.output.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return { let __arg = os::Stderr(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<io_Writer> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) };
    }
        return self.output.clone();
    }

    /// Name returns the name of the flag set.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    /// ErrorHandling returns the error handling behavior of the flag set.
    pub fn error_handling(&self) -> Arc<Mutex<Option<ErrorHandling>>> {
        return self.error_handling.clone();
    }

    /// SetOutput sets the destination for usage and error messages.
    /// If output is nil, [os.Stderr] is used.
    pub fn set_output(&mut self, output: Arc<Mutex<Option<io_Writer>>>) {
        { let new_val = output.lock().unwrap().as_ref().unwrap().clone(); *self.output.lock().unwrap() = Some(new_val); };
    }

    /// VisitAll visits the flags in lexicographical order, calling fn for each.
    /// It visits all flags, even those not set.
    pub fn visit_all(&self, r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Flag>>>) -> () + Send + Sync>>>>) {
        { let __range_holder = sort_flags({ let __field = self.formal.clone(); __field }).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for flag in __range_values.iter() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Flag>>>) -> () + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Flag>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*flag).clone()) };
    } }
    }

    /// Visit visits the flags in lexicographical order, calling fn for each.
    /// It visits only those flags that have been set.
    pub fn visit(&self, r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Flag>>>) -> () + Send + Sync>>>>) {
        { let __range_holder = sort_flags({ let __field = self.actual.clone(); __field }).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for flag in __range_values.iter() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Flag>>>) -> () + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Flag>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*flag).clone()) };
    } }
    }

    /// Lookup returns the [Flag] structure of the named flag, returning nil if none exists.
    pub fn lookup(&self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Flag>>> {
        { let __map = { let __map_holder = self.formal.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }
    }

    /// Set sets the value of the named flag.
    pub fn set(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.set_1(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn set_1(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let (mut flag, mut ok) = { let __map = { let __map_holder = self.formal.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        if !ok {
                // Remember that a flag that isn't defined is being set.
                // We return an error in this case, but in addition if
                // subsequently that flag is defined, we want to panic
                // at the definition point.
                // This is a problem which occurs if both the definition
                // and the Set call are in init code and for whatever
                // reason the init code changes evaluation order.
                // See issue 57411.
        let (_, mut file, mut line, mut ok) = runtime::caller(2);
        if !ok {
        { let new_val = "?".to_string(); *file.lock().unwrap() = Some(new_val); };
        { let new_val = 0; line = new_val; };
    }
        if { let __nil_target = self.undef.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([])))); self.undef = new_val; };
    }
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(format!("{}:{}", { let __v = (*file.lock().unwrap().as_ref().unwrap()).clone(); __v }, line)))); (*self.undef.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("no such flag -{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
    }
                // Remember that a flag that isn't defined is being set.
                // We return an error in this case, but in addition if
                // subsequently that flag is defined, we want to panic
                // at the definition point.
                // This is a problem which occurs if both the definition
                // and the Set call are in init code and for whatever
                // reason the init code changes evaluation order.
                // See issue 57411.
        let mut err = (*(*flag.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_mut().unwrap()).set(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return err.clone();
    }
        if { let __nil_target = self.actual.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Flag>>>>::new()))); self.actual = new_val; };
    }
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = flag.clone(); (*self.actual.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return Arc::new(Mutex::new(None));
    }

    /// PrintDefaults prints, to standard error unless configured otherwise, the
    /// default values of all defined command-line flags in the set. See the
    /// documentation for the global function PrintDefaults for more information.
    pub fn print_defaults(&self) {
        let mut isZeroValueErrs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        let mut f_closure_clone = (*self).clone(); let mut isZeroValueErrs_closure_clone = isZeroValueErrs.clone(); { let mut __recv = f_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut f_closure_clone_closure_clone = f_closure_clone.clone(); Box::new(move |flag: Arc<Mutex<Option<Flag>>>| {
        let mut b: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*b.clone().lock().unwrap().as_mut().unwrap()).push_str(&format!("  -{}", (*{ let __field = (*flag.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
        let (mut name, mut usage) = unquote_usage(flag.clone());
        if { let __tmp_x = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*b.lock().unwrap().as_mut().unwrap()).push_str(" ");
        (*b.lock().unwrap().as_mut().unwrap()).push_str(&(*name.lock().unwrap().as_ref().unwrap()).clone());
    }
        if { let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).len() as i32; let __tmp_y = 4; __tmp_x <= __tmp_y } {
        (*b.lock().unwrap().as_mut().unwrap()).push_str("\t");
    } else {
        (*b.lock().unwrap().as_mut().unwrap()).push_str("\n    \t");
    }
        (*b.lock().unwrap().as_mut().unwrap()).push_str(&(*Arc::new(Mutex::new(Some({ let __s = (*usage.lock().unwrap().as_ref().unwrap()).clone(); let __old = "\n".to_string(); let __new = "\n    \t".to_string(); __s.replace(&__old, &__new) }))).lock().unwrap().as_ref().unwrap()).clone());
        {
        let (mut isZero, mut err) = is_zero_value(flag.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*flag.lock().unwrap().as_ref().unwrap()).def_value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            { let __append_target = isZeroValueErrs_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(err.clone()); __append_target.clone() };;
        } else if !isZero {
        {
        let (_, mut ok) = ({
        let val = (*flag.lock().unwrap().as_ref().unwrap()).value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<stringValuePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<stringValue>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<stringValue>)), false)
        }
    });;
        if ok {
            (*b.clone().lock().unwrap().as_mut().unwrap()).push_str(&format!(" (default {:?})", (*{ let __field = (*flag.lock().unwrap().as_ref().unwrap()).def_value.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));;
        } else {
            (*b.clone().lock().unwrap().as_mut().unwrap()).push_str(&format!(" (default {})", (*{ let __field = (*flag.lock().unwrap().as_ref().unwrap()).def_value.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));;
        }
    }
    }
    }
        fmt::fprint(f_closure_clone_closure_clone.output(), (Arc::new(Mutex::new(Some({ let __builder = b.clone(); let __guard = __builder.lock().unwrap(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value }))), "\n".to_string()));
    }) as Box<dyn FnMut(Arc<Mutex<Option<Flag>>>) -> () + Send + Sync> }))); __recv.visit_all(__method_arg0) };
                // Two spaces before -; see next two comments.
                // Boolean flags of one ASCII letter are so common we
                // treat them specially, putting their usage on the same line.
                // space, space, '-', 'x'.
                // Four spaces before the tab triggers good alignment
                // for both 4- and 8-space tab stops.
                // Print the default value only if it differs to the zero value
                // for this flag type.
                // put quotes on the value
                // If calling String on any zero flag.Values triggered a panic, print
                // the messages after the full set of defaults so that the programmer
                // knows to fix the panic.
        {
        let mut errs = Arc::new(Mutex::new(Some({ let __v = (*isZeroValueErrs.lock().unwrap().as_ref().unwrap()).clone(); __v })));;
        if { let __tmp_x = ((*errs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
            println!();;
            { let __range_holder = errs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for err in __range_values.iter().cloned() {
        println!("{}", format!("{}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))));
    } };
        }
    }
    }

    /// defaultUsage is the default function to print a usage message.
    pub fn default_usage(&self) {
        if { let __tmp_x = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let __s = format!("Usage:\n"); let __n = __s.len() as i32; (*self.output().lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };
    } else {
        { let __s = format!("Usage of {}:\n", (*self.name.lock().unwrap().as_ref().unwrap())); let __n = __s.len() as i32; (*self.output().lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };
    }
        self.print_defaults();
    }

    /// NFlag returns the number of flags that have been set.
    pub fn n_flag(&self) -> i32 {
        ({ let __len_target = { let __field = self.actual.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Arg returns the i'th argument. Arg(0) is the first remaining argument
    /// after flags have been processed. Arg returns an empty string if the
    /// requested element does not exist.
    pub fn arg(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.args.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })))
    }

    /// NArg is the number of arguments remaining after flags have been processed.
    pub fn n_arg(&self) -> i32 {
        ({ let __len_target = { let __field = self.args.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Args returns the non-flag arguments.
    pub fn args(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        return self.args.clone();
    }

    /// BoolVar defines a bool flag with specified name, default value, and usage string.
    /// The argument p points to a bool variable in which to store the value of the flag.
    pub fn bool_var(&mut self, p: Arc<Mutex<Option<bool>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<bool>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(boolValuePtr(new_bool_value(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()).clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Bool defines a bool flag with specified name, default value, and usage string.
    /// The return value is the address of a bool variable that stores the value of the flag.
    pub fn bool(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<bool>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<bool>>> {
        let mut p = Arc::new(Mutex::new(Some(bool::default())));
        self.bool_var(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return p.clone();
    }

    /// IntVar defines an int flag with specified name, default value, and usage string.
    /// The argument p points to an int variable in which to store the value of the flag.
    pub fn int_var(&mut self, p: Arc<Mutex<Option<i32>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<i32>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(intValuePtr(new_int_value(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()).clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Int defines an int flag with specified name, default value, and usage string.
    /// The return value is the address of an int variable that stores the value of the flag.
    pub fn int(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<i32>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<i32>>> {
        let mut p = Arc::new(Mutex::new(Some(i32::default())));
        self.int_var(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return p.clone();
    }

    /// Int64Var defines an int64 flag with specified name, default value, and usage string.
    /// The argument p points to an int64 variable in which to store the value of the flag.
    pub fn int64_var(&mut self, p: Arc<Mutex<Option<i64>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<i64>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(int64ValuePtr(new_int64_value(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()).clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Int64 defines an int64 flag with specified name, default value, and usage string.
    /// The return value is the address of an int64 variable that stores the value of the flag.
    pub fn int64(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<i64>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<i64>>> {
        let mut p = Arc::new(Mutex::new(Some(i64::default())));
        self.int64_var(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return p.clone();
    }

    /// UintVar defines a uint flag with specified name, default value, and usage string.
    /// The argument p points to a uint variable in which to store the value of the flag.
    pub fn uint_var(&mut self, p: Arc<Mutex<Option<u64>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<u64>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(uintValuePtr(new_uint_value(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()).clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Uint defines a uint flag with specified name, default value, and usage string.
    /// The return value is the address of a uint variable that stores the value of the flag.
    pub fn uint(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<u64>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<u64>>> {
        let mut p = Arc::new(Mutex::new(Some(u64::default())));
        self.uint_var(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return p.clone();
    }

    /// Uint64Var defines a uint64 flag with specified name, default value, and usage string.
    /// The argument p points to a uint64 variable in which to store the value of the flag.
    pub fn uint64_var(&mut self, p: Arc<Mutex<Option<u64>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<u64>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(uint64ValuePtr(new_uint64_value(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()).clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Uint64 defines a uint64 flag with specified name, default value, and usage string.
    /// The return value is the address of a uint64 variable that stores the value of the flag.
    pub fn uint64(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<u64>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<u64>>> {
        let mut p = Arc::new(Mutex::new(Some(u64::default())));
        self.uint64_var(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return p.clone();
    }

    /// StringVar defines a string flag with specified name, default value, and usage string.
    /// The argument p points to a string variable in which to store the value of the flag.
    pub fn string_var(&mut self, p: Arc<Mutex<Option<String>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(stringValuePtr(new_string_value(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()).clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// String defines a string flag with specified name, default value, and usage string.
    /// The return value is the address of a string variable that stores the value of the flag.
    pub fn string(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let mut p = Arc::new(Mutex::new(Some(String::default())));
        self.string_var(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return p.clone();
    }

    /// Float64Var defines a float64 flag with specified name, default value, and usage string.
    /// The argument p points to a float64 variable in which to store the value of the flag.
    pub fn float64_var(&mut self, p: Arc<Mutex<Option<f64>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<f64>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(float64ValuePtr(new_float64_value(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()).clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Float64 defines a float64 flag with specified name, default value, and usage string.
    /// The return value is the address of a float64 variable that stores the value of the flag.
    pub fn float64(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<f64>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<f64>>> {
        let mut p = Arc::new(Mutex::new(Some(f64::default())));
        self.float64_var(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return p.clone();
    }

    /// DurationVar defines a time.Duration flag with specified name, default value, and usage string.
    /// The argument p points to a time.Duration variable in which to store the value of the flag.
    /// The flag accepts a value acceptable to time.ParseDuration.
    pub fn duration_var(&mut self, p: Arc<Mutex<Option<time_Duration>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<time_Duration>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(durationValuePtr(new_duration_value(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()).clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Duration defines a time.Duration flag with specified name, default value, and usage string.
    /// The return value is the address of a time.Duration variable that stores the value of the flag.
    /// The flag accepts a value acceptable to time.ParseDuration.
    pub fn duration(&mut self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<time_Duration>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<time_Duration>>> {
        let mut p = Arc::new(Mutex::new(Some(time_Duration::default())));
        self.duration_var(p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return p.clone();
    }

    /// TextVar defines a flag with a specified name, default value, and usage string.
    /// The argument p must be a pointer to a variable that will hold the value
    /// of the flag, and p must implement encoding.TextUnmarshaler.
    /// If the flag is used, the flag value will be passed to p's UnmarshalText method.
    /// The type of the default value must be the same as the type of p.
    pub fn text_var(&mut self, p: Arc<Mutex<Option<encoding_TextUnmarshaler>>>, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<encoding_TextMarshaler>>>, usage: Arc<Mutex<Option<String>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new((*new_text_value(value.clone(), p.clone()).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Func defines a flag with the specified name and usage string.
    /// Each time the flag is seen, fn is called with the value of the flag.
    /// If fn returns a non-nil error, it will be treated as a flag value parsing error.
    pub fn func(&mut self, name: Arc<Mutex<Option<String>>>, usage: Arc<Mutex<Option<String>>>, r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(funcValueAsValue(r#fn.clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// BoolFunc defines a flag with the specified name and usage string without requiring values.
    /// Each time the flag is seen, fn is called with the value of the flag.
    /// If fn returns a non-nil error, it will be treated as a flag value parsing error.
    pub fn bool_func(&mut self, name: Arc<Mutex<Option<String>>>, usage: Arc<Mutex<Option<String>>>, r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>) {
        self.var(Arc::new(Mutex::new(Some(Box::new(boolFuncValueAsValue(r#fn.clone())) as Box<dyn Value + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Var defines a flag with the specified name and usage string. The type and
    /// value of the flag are represented by the first argument, of type [Value], which
    /// typically holds a user-defined implementation of [Value]. For instance, the
    /// caller could create a flag that turns a comma-separated string into a slice
    /// of strings by giving the slice the methods of [Value]; in particular, [Set] would
    /// decompose the comma-separated string into the slice.
    pub fn var(&mut self, value: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, name: Arc<Mutex<Option<String>>>, usage: Arc<Mutex<Option<String>>>) {
                // Flag must not begin "-" or contain "=".
        if (*Arc::new(Mutex::new(Some({ let __s = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "-".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        std::panic::panic_any(Box::new({ let __v = self.sprintf(Arc::new(Mutex::new(Some("flag %q begins with -".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    } else if (*Arc::new(Mutex::new(Some({ let __s = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "=".to_string(); __s.contains(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        std::panic::panic_any(Box::new({ let __v = self.sprintf(Arc::new(Mutex::new(Some("flag %q contains =".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
                // Remember the default value as a string; it won't change.
        let mut flag = Arc::new(Mutex::new(Some(Flag { name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), usage: Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), value: value.clone(), def_value: (*value.lock().unwrap().as_ref().unwrap()).string(), ..Default::default() })));
        let (_, mut alreadythere) = { let __map = { let __map_holder = self.formal.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        if alreadythere {
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __tmp_x = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = self.sprintf(Arc::new(Mutex::new(Some("flag redefined: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *msg.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = { let __method_arg0 = Arc::new(Mutex::new(Some("%s flag redefined: %s".to_string()))); self.sprintf(__method_arg0, Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *msg.lock().unwrap() = __moved_val; };
    }
        std::panic::panic_any(Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>);
    }
                // Happens only if flags are declared with identical names
        {
        let mut pos = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = self.undef.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) })));;
        if { let __tmp_x = (*pos.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("flag {} set at {} before being defined", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
        }
    }
        if { let __nil_target = self.formal.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Flag>>>>::new()))); self.formal = new_val; };
    }
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = flag.clone(); (*self.formal.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    /// sprintf formats the message, prints it to output, and returns it.
    pub fn sprintf(&self, format: Arc<Mutex<Option<String>>>, a: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> Arc<Mutex<Option<String>>> {
        let mut msg = Arc::new(Mutex::new(Some(format!("{}", (*format.lock().unwrap().as_ref().unwrap()).clone()))));
        println!("{}", format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        return { let __owned = msg.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// failf prints to standard error a formatted error and usage message and
    /// returns the error.
    pub fn failf(&self, format: Arc<Mutex<Option<String>>>, a: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut msg = self.sprintf(Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), a.clone());
        self.usage();
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from((*msg.lock().unwrap().as_ref().unwrap()).clone()))));
    }

    /// usage calls the Usage method for the flag set if one is specified,
    /// or the appropriate default usage function otherwise.
    pub fn usage(&self) {
        if { let __nil_target = self.usage.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        self.default_usage();
    } else {
        { let __f_holder = self.usage.clone(); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
    }

    /// parseOne parses one flag. It reports whether a flag was seen.
    pub fn parse_one(&mut self) -> (bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if { let __tmp_x = (({ let __len_target = { let __field = self.args.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (false, Arc::new(Mutex::new(None)));
    }
        let mut s = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x < __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as i32) as u8; __tmp_x != __tmp_y } {
        return (false, Arc::new(Mutex::new(None)));
    }
        let mut numMinuses = Arc::new(Mutex::new(Some(1)));
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] }; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } {
        { let mut guard = numMinuses.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.args = new_val; };
        return (false, Arc::new(Mutex::new(None)));
    }
    }
                // "--" terminates the flags
        let mut name = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*numMinuses.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() })));
        if { let __tmp_x = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('=' as i32) as u8; __tmp_x == __tmp_y } {
        return (false, self.failf(Arc::new(Mutex::new(Some("bad flag syntax: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))));
    }
                // it's a flag. does it have an argument?
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.args = new_val; };
        let mut hasValue = Arc::new(Mutex::new(Some(false)));
        let mut value = Arc::new(Mutex::new(Some("".to_string())));
        let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('=' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
        { let new_val = true; *hasValue.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_val; };
        break
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        let (mut flag, mut ok) = { let __map = { let __map_holder = self.formal.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        if !ok {
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "help".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "h".to_string(); __tmp_x == __tmp_y } {
        self.usage();
        return (false, ErrHelp.clone());
    }
        return (false, self.failf(Arc::new(Mutex::new(Some("flag provided but not defined: -%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))));
    }
                // special case for nice help message.
        {
        let (mut fv, mut ok) = ({
        let val = (*flag.lock().unwrap().as_ref().unwrap()).value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<boolFuncValue>() {
                (Arc::new(Mutex::new(Some(Box::new(boolFuncValueAsboolFlag(typed_val.clone())) as Box<dyn boolFlag + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<boolValuePtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn boolFlag + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn boolFlag + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn boolFlag + Send + Sync>>)), false)
        }
    });;
        if ok && (*fv.lock().unwrap().as_ref().unwrap()).is_bool_flag() {
            if { let __v = (*hasValue.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        let mut err = (*fv.lock().unwrap().as_mut().unwrap()).set(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (false, self.failf(Arc::new(Mutex::new(Some("invalid boolean value %q for -%s: %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fs_PathError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_LinkError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_SyscallError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_errSymlink>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<reflect_ValueError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_PanicNilError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_boundsError>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<strconv_NumError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<syscall_Errno>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<time_ParseError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<time_fileSizeError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } }])))));;
        }
    }
    } else {
        {
        let mut err = (*fv.lock().unwrap().as_mut().unwrap()).set(Arc::new(Mutex::new(Some("true".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (false, self.failf(Arc::new(Mutex::new(Some("invalid boolean flag %s: %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fs_PathError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_LinkError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_SyscallError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_errSymlink>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<reflect_ValueError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_PanicNilError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_boundsError>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<strconv_NumError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<syscall_Errno>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<time_ParseError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<time_fileSizeError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } }])))));;
        }
    }
    };
        } else {
            if !{ let __v = (*hasValue.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (({ let __len_target = { let __field = self.args.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = true; *hasValue.lock().unwrap() = Some(new_val); };
        { let __tmp_0 = { let __seq = { let __seq_holder = self.args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_1 = Arc::new(Mutex::new(Some({ let __seq_holder = self.args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); *value.lock().unwrap() = Some(__tmp_0); *self.args.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    };
            if !{ let __v = (*hasValue.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (false, self.failf(Arc::new(Mutex::new(Some("flag needs an argument: -%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))));
    };
            {
        let mut err = (*(*flag.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_mut().unwrap()).set(Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (false, self.failf(Arc::new(Mutex::new(Some("invalid value %q for flag -%s: %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fs_PathError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_LinkError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_SyscallError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_errSymlink>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<reflect_ValueError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_PanicNilError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_boundsError>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<strconv_NumError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<syscall_Errno>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<time_ParseError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<time_fileSizeError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } }])))));;
        }
    };
        }
    }
                // It must have a value, which might be the next argument.
                // value is the next arg
        if { let __nil_target = self.actual.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Flag>>>>::new()))); self.actual = new_val; };
    }
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = flag.clone(); (*self.actual.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        (true, Arc::new(Mutex::new(None)))
    }

    /// Parse parses flag definitions from the argument list, which should not
    /// include the command name. Must be called after all flags in the [FlagSet]
    /// are defined and before flags are accessed by the program.
    /// The return value will be [ErrHelp] if -help or -h were set but not defined.
    pub fn parse(&mut self, arguments: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        { let new_val = true; *self.parsed.lock().unwrap() = Some(new_val); };
        { let new_val = arguments.clone(); self.args = new_val; };
        loop {
        let (mut seen, mut err) = self.parse_one();
        if seen {
        continue
    }
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        break
    }
        { let _switch_val = { let __selector_holder = self.error_handling.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (ErrorHandling(Arc::new(Mutex::new(Some(CONTINUE_ON_ERROR as i32))))) {
            return err.clone();
        } else if _switch_val == (ErrorHandling(Arc::new(Mutex::new(Some(EXIT_ON_ERROR as i32))))) {
            if { let __left = err.clone(); let __right = ErrHelp.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        os::exit(0);
    }
            os::exit(2);
        } else if _switch_val == (ErrorHandling(Arc::new(Mutex::new(Some(PANIC_ON_ERROR as i32))))) {
            std::panic::panic_any({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fs_PathError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_LinkError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_SyscallError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os_errSymlink>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<reflect_ValueError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_PanicNilError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_boundsError>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<strconv_NumError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<syscall_Errno>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<time_ParseError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<time_fileSizeError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
        }
    }
    }
        return Arc::new(Mutex::new(None));
    }

    /// Parsed reports whether f.Parse has been called.
    pub fn parsed(&self) -> bool {
        return (*self.parsed.lock().unwrap().as_ref().unwrap());
    }

    /// Init sets the name and error handling property for a flag set.
    /// By default, the zero [FlagSet] uses an empty name and the
    /// [ContinueOnError] error handling policy.
    pub fn init(&mut self, name: Arc<Mutex<Option<String>>>, errorHandling: Arc<Mutex<Option<ErrorHandling>>>) {
        { let new_val = name.lock().unwrap().as_ref().unwrap().clone(); *self.name.lock().unwrap() = Some(new_val); };
        { let new_val = errorHandling.lock().unwrap().as_ref().unwrap().clone(); *self.error_handling.lock().unwrap() = Some(new_val); };
    }
}

pub fn num_error(err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let (mut ne, mut ok) = ({
        let val = err.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<strconv_NumError>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<strconv_NumError>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<strconv_NumError>)), false)
        }
    });
    if !ok {
        return err.clone();
    }
    if { let __left = (*ne.lock().unwrap().as_ref().unwrap()).err.clone(); let __right = strconv::ErrSyntax().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        return errParse.clone();
    }
    if { let __left = (*ne.lock().unwrap().as_ref().unwrap()).err.clone(); let __right = strconv::ErrRange().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        return errRange.clone();
    }
    err.clone()
}

pub fn new_bool_value(val: Arc<Mutex<Option<bool>>>, p: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<boolValue>>> {
    { let new_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; *p.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(boolValue::default())))
}

pub fn new_int_value(val: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<intValue>>> {
    { let new_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; *p.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(intValue::default())))
}

pub fn new_int64_value(val: Arc<Mutex<Option<i64>>>, p: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<int64Value>>> {
    { let new_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; *p.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(int64Value::default())))
}

pub fn new_uint_value(val: Arc<Mutex<Option<u64>>>, p: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<uintValue>>> {
    { let new_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; *p.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(uintValue::default())))
}

pub fn new_uint64_value(val: Arc<Mutex<Option<u64>>>, p: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<uint64Value>>> {
    { let new_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; *p.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(uint64Value::default())))
}

pub fn new_string_value(val: Arc<Mutex<Option<String>>>, p: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<stringValue>>> {
    { let new_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; *p.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(stringValue::default())))
}

pub fn new_float64_value(val: Arc<Mutex<Option<f64>>>, p: Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<float64Value>>> {
    { let new_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; *p.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(float64Value::default())))
}

pub fn new_duration_value(val: Arc<Mutex<Option<time_Duration>>>, p: Arc<Mutex<Option<time_Duration>>>) -> Arc<Mutex<Option<durationValue>>> {
    { let new_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; *p.lock().unwrap() = Some(new_val); };
    Arc::new(Mutex::new(Some(durationValue::default())))
}

pub fn new_text_value(val: Arc<Mutex<Option<encoding_TextMarshaler>>>, p: Arc<Mutex<Option<encoding_TextUnmarshaler>>>) -> Arc<Mutex<Option<textValue>>> {
    let mut ptrVal = unimplemented!("reflect.ValueOf requires statically known pointer-to-struct type");
    if { let __tmp_x = (*(*ptrVal.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = reflect::PTR; __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("variable value type must be a pointer".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let mut defVal = unimplemented!("reflect.ValueOf requires statically known pointer-to-struct type");
    if { let __tmp_x = (*(*defVal.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = reflect::PTR; __tmp_x == __tmp_y } {
        { let new_val = (*defVal.lock().unwrap().as_ref().unwrap()).elem(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *defVal.lock().unwrap() = __moved_val; };
    }
    if { let __tmp_x = (*(*defVal.lock().unwrap().as_ref().unwrap()).r#type().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = (*ptrVal.lock().unwrap().as_ref().unwrap()).r#type(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).elem(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("default type does not match variable type: {} != {}", format!("{}", (*((*defVal.lock().unwrap().as_ref().unwrap()).r#type()).lock().unwrap().as_ref().unwrap())), format!("{}", (*({ let __recv = (*ptrVal.lock().unwrap().as_ref().unwrap()).r#type(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).elem(); __result }).lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
    { let __recv = (*ptrVal.lock().unwrap().as_ref().unwrap()).elem(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).set(defVal.clone()); __result };
    Arc::new(Mutex::new(Some(textValue { p: p.clone(), ..Default::default() })))
}

/// sortFlags returns the flags as a slice in lexicographical sorted order.
pub fn sort_flags(flags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Flag>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Flag>>>>>>> {
    let mut result: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Flag>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ((*flags.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    let mut i = Arc::new(Mutex::new(Some(0)));
    for (_, f) in { let __range_holder = flags.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        (*result.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = f.clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __cmp_holder = Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<Flag>>>, b: Arc<Mutex<Option<Flag>>>| -> i32 {
        (*Arc::new(Mutex::new(Some({ let __a = (*(*a.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone(); let __b = (*(*b.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone(); match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 } }))).lock().unwrap().as_ref().unwrap())
    }) as Box<dyn FnMut(Arc<Mutex<Option<Flag>>>, Arc<Mutex<Option<Flag>>>) -> i32 + Send + Sync>))); let mut __sort_guard = result.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = { let mut __cmp_guard = __cmp_holder.lock().unwrap(); let __cmp_fn = __cmp_guard.as_mut().unwrap(); (*__cmp_fn)(__a.clone(), __b.clone()) }; let __ord = __cmp.cmp(&0); __ord }); } };
    return result.clone();
}

/// isZeroValue determines whether the string represents the zero
/// value for a flag.
pub fn is_zero_value(flag: Arc<Mutex<Option<Flag>>>, value: Arc<Mutex<Option<String>>>) -> (bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Build a zero value of the flag's Value type, and see if the
                // result of calling its String method equals the value passed in.
                // This works unless the Value type is itself an interface type.
        let mut typ = Arc::new(Mutex::new(Some(GoReflectType { name: Arc::new(Mutex::new(Some("flag.Value".to_string()))), fields: Arc::new(Mutex::new(Some(vec![]))) })));
        let mut z: Arc<Mutex<Option<reflect_Value>>> = Arc::new(Mutex::new(Some(Default::default())));
        if { let __tmp_x = (*(*typ.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = reflect::POINTER; __tmp_x == __tmp_y } {
        { let new_val = reflect::new((*typ.lock().unwrap().as_ref().unwrap()).elem()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = reflect::zero(typ.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *z.lock().unwrap() = __moved_val; };
    }

                // Catch panics calling the String method, which shouldn't prevent the
                // usage message from being printed, but that we should report to the
                // user so that they know to fix their code.
        let mut err_defer_captured = err.clone(); let flag_defer_captured = flag.clone(); let mut typ_defer_captured = typ.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        {
        let mut e = go_recover();;
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
            if { let __tmp_x = (*(*typ_defer_captured.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = reflect::POINTER; __tmp_x == __tmp_y } {
        { let new_val = (*typ_defer_captured.lock().unwrap().as_ref().unwrap()).elem(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *typ_defer_captured.lock().unwrap() = __moved_val; };
    };
            { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("panic calling String method on zero {} for flag {}: {}", format!("{}", (*typ.lock().unwrap().as_ref().unwrap())), (*{ let __field = (*flag_defer_captured.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone(), format_any(e.lock().unwrap().as_ref().unwrap().as_ref())))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err_defer_captured.lock().unwrap() = new_val; };;
        }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        {
        { let new_val = { let __tmp_x = (*value.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = ({
        let val = (*z.lock().unwrap().as_ref().unwrap()).interface().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<boolFuncValue>() {
                Arc::new(Mutex::new(Some(Box::new(boolFuncValueAsValue(typed_val.clone())) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<boolValuePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<durationValuePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<float64ValuePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<funcValue>() {
                Arc::new(Mutex::new(Some(Box::new(funcValueAsValue(typed_val.clone())) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<int64ValuePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<intValuePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<stringValuePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<textValue>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<uint64ValuePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else if let Some(typed_val) = any_val.downcast_ref::<uintValuePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Value + Send + Sync>)))
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y }; *ok.lock().unwrap() = Some(new_val); };;
        *err.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*ok.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            ((*ok.lock().unwrap().as_ref().unwrap()), err.clone())
        }
    }
}

/// UnquoteUsage extracts a back-quoted name from the usage
/// string for a flag and returns it and the un-quoted usage.
/// Given "a `name` to show" it returns ("name", "a name to show").
/// If there are no back quotes, the name is an educated guess of the
/// type of the flag's value, or the empty string if the flag is boolean.
pub fn unquote_usage(flag: Arc<Mutex<Option<Flag>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {
    let mut name: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut usage: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

        // Look for a back-quoted name, but avoid the strings package.
    { let new_val = { let __selector_holder = (*flag.lock().unwrap().as_ref().unwrap()).usage.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *usage.lock().unwrap() = Some(new_val); };
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*usage.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*usage.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('`' as i32) as u8; __tmp_x == __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*usage.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*usage.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('`' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*usage.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_val; };
        { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some({ let __s = &((*usage.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some({ let __s = &((*usage.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()))); __s }; *usage.lock().unwrap() = Some(new_val); };
        return ({ let __owned = name.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __owned = usage.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        break
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Only one back quote; use type name.
        // No explicit name, so use type if we can find one.
    { let new_val = "value".to_string(); *name.lock().unwrap() = Some(new_val); };
    {
    let _ts_subject = (*flag.lock().unwrap().as_ref().unwrap()).value.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<boolFuncValue>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<boolValuePtr>()).is_some() {
        let fv: Arc<Mutex<Option<Box<dyn boolFlag + Send + Sync>>>> = unimplemented!("type info required: type switch on interface case with 2 concrete implementors needs a synthesized trait object");
        if (*fv.lock().unwrap().as_ref().unwrap()).is_bool_flag() {
        { let new_val = "".to_string(); *name.lock().unwrap() = Some(new_val); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<durationValuePtr>()).is_some() {
        let fv = _ts_val.and_then(|__v| __v.downcast_ref::<durationValuePtr>()).unwrap().0.clone();
        { let new_val = "duration".to_string(); *name.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<float64ValuePtr>()).is_some() {
        let fv = _ts_val.and_then(|__v| __v.downcast_ref::<float64ValuePtr>()).unwrap().0.clone();
        { let new_val = "float".to_string(); *name.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intValuePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<int64ValuePtr>()).is_some() {
        let fv = _ts_subject.clone();
        { let new_val = "int".to_string(); *name.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<stringValuePtr>()).is_some() {
        let fv = _ts_val.and_then(|__v| __v.downcast_ref::<stringValuePtr>()).unwrap().0.clone();
        { let new_val = "string".to_string(); *name.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<uintValuePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<uint64ValuePtr>()).is_some() {
        let fv = _ts_subject.clone();
        { let new_val = "uint".to_string(); *name.lock().unwrap() = Some(new_val); };;
    }
    }
    (name.clone(), usage.clone())
}

/// PrintDefaults prints, to standard error unless configured otherwise,
/// a usage message showing the default settings of all defined
/// command-line flags.
/// For an integer valued flag x, the default output has the form
///
///	-x int
///		usage-message-for-x (default 7)
///
/// The usage message will appear on a separate line for anything but
/// a bool flag with a one-byte name. For bool flags, the type is
/// omitted and if the flag name is one byte the usage message appears
/// on the same line. The parenthetical default is omitted if the
/// default is the zero value for the type. The listed type, here int,
/// can be changed by placing a back-quoted name in the flag's usage
/// string; the first such item in the message is taken to be a parameter
/// name to show in the message and the back quotes are stripped from
/// the message when displayed. For instance, given
///
///	flag.String("I", "", "search `directory` for include files")
///
/// the output will be
///
///	-I directory
///		search directory for include files.
///
/// To change the destination for flag messages, call [CommandLine].SetOutput.
pub fn print_defaults() {
    { let __recv_holder = (*CommandLine.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.print_defaults(); __result };
}

/// String defines a string flag with specified name, default value, and usage string.
/// The return value is the address of a string variable that stores the value of the flag.
pub fn string(name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<String>>>, usage: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    { let __recv_holder = (*CommandLine.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).string(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = usage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }
}

/// Parse parses the command-line flags from [os.Args][1:]. Must be called
/// after all flags are defined and before flags are accessed by the program.
pub fn parse() {
        // Ignore errors; CommandLine is set for ExitOnError.
    { let __recv_holder = (*CommandLine.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).parse(Arc::new(Mutex::new(Some({ let __seq_holder = go_os_args().clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __result };
}

fn __go_init_0() {
        // It's possible for execl to hand us an empty os.Args.
    if { let __tmp_x = (({ let __len_target = { let __field = go_os_args().clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = new_flag_set(Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(ErrorHandling(Arc::new(Mutex::new(Some(EXIT_ON_ERROR as i32)))))))).clone(); *CommandLine.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = new_flag_set(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = go_os_args().clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))), Arc::new(Mutex::new(Some(ErrorHandling(Arc::new(Mutex::new(Some(EXIT_ON_ERROR as i32)))))))).clone(); *CommandLine.lock().unwrap() = Some(new_val); };
    }

        // Override generic FlagSet default Usage with call to global Usage.
        // Note: This is not CommandLine.Usage = Usage,
        // because we want any eventual call to use any updated value of Usage,
        // not the value it has when this line is run.
    { let new_val = Box::new(move || { command_line_usage() }) as Box<dyn FnMut() -> () + Send + Sync>; *(*(*CommandLine.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).usage.lock().unwrap() = Some(new_val); };
}

pub fn command_line_usage() {
    { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = Usage.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
}

/// NewFlagSet returns a new, empty flag set with the specified name and
/// error handling property. If the name is not empty, it will be printed
/// in the default usage message and in error messages.
pub fn new_flag_set(name: Arc<Mutex<Option<String>>>, errorHandling: Arc<Mutex<Option<ErrorHandling>>>) -> Arc<Mutex<Option<FlagSet>>> {
    let mut f = Arc::new(Mutex::new(Some(FlagSet { name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), error_handling: Arc::new(Mutex::new(Some({ let __arg_holder = errorHandling.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    { let new_val = Arc::new(Mutex::new(Some({ let __recv = f.clone(); Box::new(move || { (*__recv.lock().unwrap().as_mut().unwrap()).default_usage() }) as Box<dyn FnMut() -> () + Send + Sync> }))); (*f.lock().unwrap().as_mut().unwrap()).usage = new_val; };
    return f.clone();
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for textValue {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for FlagSet {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Flag {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
