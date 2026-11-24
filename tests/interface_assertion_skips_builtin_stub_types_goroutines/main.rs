#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Temp>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

fn go_any_try_clone(value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Point>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Temp>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

struct GoPointerError<T> {
    handle: Arc<Mutex<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Arc<Mutex<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.lock().unwrap();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + Send + Sync + 'static> std::error::Error for GoPointerError<T> {}

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

pub trait Stringer: std::fmt::Display + Any {
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Stringer + Send + Sync> {
    fn clone(&self) -> Self {
        Stringer::__go_clone_box_stringer(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.x.lock().unwrap().as_ref().cloned() }; let __right = { other.x.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.y.lock().unwrap().as_ref().cloned() }; let __right = { other.y.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug)]
pub struct Temp(pub Arc<Mutex<Option<f64>>>);

impl Clone for Temp {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Temp(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Temp {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Temp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Temp {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Temp {
    fn eq(&self, other: &f64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Temp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Temp {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Temp> for f64 {
    fn eq(&self, other: &Temp) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Temp> for f64 {
    fn partial_cmp(&self, other: &Temp) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Temp {
    type Output = Temp;
    fn add(self, other: Self) -> Temp {
        Temp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Temp {
    type Output = Temp;
    fn add(self, other: f64) -> Temp {
        Temp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Temp> for f64 {
    type Output = Temp;
    fn add(self, other: Temp) -> Temp {
        Temp(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Temp {
    type Output = Temp;
    fn sub(self, other: Self) -> Temp {
        Temp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Temp {
    type Output = Temp;
    fn sub(self, other: f64) -> Temp {
        Temp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Temp> for f64 {
    type Output = Temp;
    fn sub(self, other: Temp) -> Temp {
        Temp(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Temp {
    type Output = Temp;
    fn mul(self, other: Self) -> Temp {
        Temp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Temp {
    type Output = Temp;
    fn mul(self, other: f64) -> Temp {
        Temp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Temp> for f64 {
    type Output = Temp;
    fn mul(self, other: Temp) -> Temp {
        Temp(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Temp {
    type Output = Temp;
    fn div(self, other: Self) -> Temp {
        Temp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Temp {
    type Output = Temp;
    fn div(self, other: f64) -> Temp {
        Temp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Temp> for f64 {
    type Output = Temp;
    fn div(self, other: Temp) -> Temp {
        Temp(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Temp {
    type Output = Temp;
    fn neg(self) -> Temp {
        Temp(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}


impl Point {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("({},{})", format!("{}", (*self.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()))))))
    }
}

impl Stringer for Point {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Point::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Point>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct PointPtr(pub Arc<Mutex<Option<Point>>>);

impl std::fmt::Display for PointPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{}", __v), None => write!(f, "<nil>") }
    }
}

impl Stringer for PointPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Point::string(__recv)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PointPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Temp {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{:.1}C", (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap())))))
    }
}

impl Stringer for Temp {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Temp::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Temp>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TempPtr(pub Arc<Mutex<Option<Temp>>>);

impl std::fmt::Display for TempPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Stringer for TempPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Temp::string(__recv)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TempPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stringer for strings::builder::Builder {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        self.string()
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn Stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn Stringer + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<strings::builder::Builder>() {
            false
        } else {
            false
        }
    }
}

pub fn describe(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Point>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Temp>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<strings::builder::Builder>()).is_some() {
        let x: Arc<Mutex<Option<Box<dyn Stringer + Send + Sync>>>> = if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Point>()) {
            Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Stringer + Send + Sync>)))
        } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Temp>()) {
            Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Stringer + Send + Sync>)))
        } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<strings::builder::Builder>()) {
            Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Stringer + Send + Sync>)))
        } else {
            panic!("type switch interface case condition matched no concrete implementor")
        };
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("{}{}", "stringer ".to_string(), (*(*x.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<fmt_wrapError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<fmt_wrapErrors>>>>()).is_some() {
        let x = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<fmt_wrapError>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<fmt_wrapErrors>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else { unreachable!("type switch error case matched no error payload") };
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("{}{}", "error ".to_string(), (*Arc::new(Mutex::new(Some(format!("{}", x.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())))));;
    } else {
        let x = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("other {}", { let __text = match x.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }))));;
    }
    }
    unreachable!()
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
    let mut sb: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*sb.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("built".to_string()))));
    println!("{}", format!("{}", (*(*sb.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
    for v in &vec![
        Box::new("plain".to_string()) as Box<dyn Any + Send + Sync>,
        Box::new(Point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Box<dyn Any + Send + Sync>,
        Box::new(Temp(Arc::new(Mutex::new(Some(21.5 as f64))))) as Box<dyn Any + Send + Sync>,
        { let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("bad {}", 3)))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any + Send + Sync>, Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } },
        Box::new(42i64) as Box<dyn Any + Send + Sync>,
    ] {
        println!("{}", format!("{}", (*describe(Arc::new(Mutex::new(Some(go_any_clone(v.as_ref()))))).lock().unwrap().as_ref().unwrap())));
    }
    let mut s: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("text".to_string()) as Box<dyn Any + Send + Sync>)));
    {
        let (mut st, mut ok) = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Point>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Stringer + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Temp>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Stringer + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<strings::builder::Builder>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn Stringer + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn Stringer + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn Stringer + Send + Sync>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "wrongly a stringer".to_string()), format!("{}", (*(*st.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));;
        } else {
            println!("{}", format!("{}", "string is not a Stringer".to_string()));;
        }
    }
}

impl GoValueClone for Point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
