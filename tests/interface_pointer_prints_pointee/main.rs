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

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}

fn go_install_panic_exit() {
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
    }
    go_panic_value_text(__payload)
}

fn go_panic_value_text(__payload: &dyn Any) -> String {
    // A boxed payload holds the Go value itself, so its string is printed
    // as it is.
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return __text.clone();
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return __text.to_string();
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_value_text(&**__inner);
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__err) = __payload.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>>() {
        return __err.lock().unwrap().as_ref().map(|__err| __err.to_string()).unwrap_or_else(|| String::from("nil"));
    }
    macro_rules! go_panic_display_text {
        ($($t:ty),*) => {
            $(if let Some(__value) = __payload.downcast_ref::<$t>() {
                return __value.to_string();
            })*
        };
    }
    go_panic_display_text!(i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool, char);
    // Go prints a float in a panic as +1.500000e+000; a float32 is widened
    // to float64 first, as Go's print does.
    let __float = __payload
        .downcast_ref::<f64>()
        .copied()
        .or_else(|| __payload.downcast_ref::<f32>().map(|__value| *__value as f64));
    if let Some(__value) = __float {
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if __value > 0.0 { "+Inf" } else { "-Inf" });
        }
        let __text = format!("{:+.6e}", __value);
        if let Some((__mantissa, __exponent)) = __text.split_once('e') {
            let __exponent: i32 = __exponent.parse().unwrap_or(0);
            return format!("{}e{}{:03}", __mantissa, if __exponent < 0 { '-' } else { '+' }, __exponent.abs());
        }
        return __text;
    }
    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}

fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
}

pub trait Named: std::fmt::Display + Any {
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Named + Send + Sync> {
    fn clone(&self) -> Self {
        Named::__go_clone_box_named(self.as_ref())
    }
}

pub trait Sized: std::fmt::Display + Any {
    fn __go_clone_box_sized(&self) -> Box<dyn Sized + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_sized(&self, other: &(dyn Sized + Send + Sync)) -> bool;
    fn size(&self) -> i64;
}

impl Clone for Box<dyn Sized + Send + Sync> {
    fn clone(&self) -> Self {
        Sized::__go_clone_box_sized(self.as_ref())
    }
}

pub trait Item: Named + Sized + std::fmt::Display + Any {
    fn __go_clone_box_item(&self) -> Box<dyn Item + Send + Sync>;
    fn __go_eq_item(&self, other: &(dyn Item + Send + Sync)) -> bool;
}

impl Clone for Box<dyn Item + Send + Sync> {
    fn clone(&self) -> Self {
        Item::__go_clone_box_item(self.as_ref())
    }
}

impl Named for Box<dyn Item + Send + Sync> {
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        Named::__go_as_any(&**self)
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        (**self).__go_eq_named(other)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        (**self).name()
    }
}

impl Sized for Box<dyn Item + Send + Sync> {
    fn __go_clone_box_sized(&self) -> Box<dyn Sized + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Sized + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        Named::__go_as_any(&**self)
    }
    fn __go_eq_sized(&self, other: &(dyn Sized + Send + Sync)) -> bool {
        (**self).__go_eq_sized(other)
    }
    fn size(&self) -> i64 {
        (**self).size()
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: Arc<Mutex<Option<String>>>,
    pub size: Arc<Mutex<Option<i64>>>,
}

impl File {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            size: __go_clone_1_0,
        }
    }
}


impl Default for File {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            size: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.name.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.name.lock().unwrap().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.size.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.size.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{name:{} size:{}}}", __go_fmt_0, __go_fmt_1);
        }
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.size.lock().unwrap().as_ref().cloned() }; let __right = { other.size.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Dir {
    pub name: Arc<Mutex<Option<String>>>,
}

impl Dir {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Dir {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Problem {
    pub code: Arc<Mutex<Option<i64>>>,
}

impl Problem {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for Problem {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Problem {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.code.lock().unwrap().as_ref().cloned() }; let __right = { other.code.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Clone, Default)]
pub struct Holder {
    pub item: Arc<Mutex<Option<Box<dyn Item + Send + Sync>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.item.clone();
        Self {
            item: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.item.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.item.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{item:{}}}", __go_fmt_0);
        }
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug)]
pub struct Counter(pub Arc<Mutex<Option<i64>>>);

impl Clone for Counter {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Counter(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Counter {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Counter {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Counter> for i64 {
    fn eq(&self, other: &Counter) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Counter> for i64 {
    fn partial_cmp(&self, other: &Counter) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Counter {
    type Output = Counter;
    fn add(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Counter {
    type Output = Counter;
    fn add(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Counter> for i64 {
    type Output = Counter;
    fn add(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Counter {
    type Output = Counter;
    fn sub(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Counter {
    type Output = Counter;
    fn sub(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Counter> for i64 {
    type Output = Counter;
    fn sub(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Counter {
    type Output = Counter;
    fn mul(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Counter {
    type Output = Counter;
    fn mul(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Counter> for i64 {
    type Output = Counter;
    fn mul(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Counter {
    type Output = Counter;
    fn div(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Counter {
    type Output = Counter;
    fn div(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Counter> for i64 {
    type Output = Counter;
    fn div(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Counter {
    type Output = Counter;
    fn neg(self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Counter {
    type Output = Counter;
    fn rem(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Counter {
    type Output = Counter;
    fn rem(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Counter> for i64 {
    type Output = Counter;
    fn rem(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Counter {
    type Output = Counter;
    fn bitand(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Counter {
    type Output = Counter;
    fn bitand(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Counter> for i64 {
    type Output = Counter;
    fn bitand(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Counter {
    type Output = Counter;
    fn bitor(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Counter {
    type Output = Counter;
    fn bitor(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Counter> for i64 {
    type Output = Counter;
    fn bitor(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Counter {
    type Output = Counter;
    fn bitxor(self, other: Self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Counter {
    type Output = Counter;
    fn bitxor(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Counter> for i64 {
    type Output = Counter;
    fn bitxor(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Counter {
    type Output = Counter;
    fn not(self) -> Counter {
        Counter(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Counter {
    type Output = Counter;
    fn shl(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for Counter {
    type Output = Counter;
    fn shl(self, other: i32) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for Counter {
    type Output = Counter;
    fn shl(self, other: i8) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for Counter {
    type Output = Counter;
    fn shl(self, other: i16) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for Counter {
    type Output = Counter;
    fn shl(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for Counter {
    type Output = Counter;
    fn shl(self, other: u32) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for Counter {
    type Output = Counter;
    fn shl(self, other: u8) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for Counter {
    type Output = Counter;
    fn shl(self, other: u16) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for Counter {
    type Output = Counter;
    fn shl(self, other: u64) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for Counter {
    type Output = Counter;
    fn shl(self, other: usize) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for Counter {
    type Output = Counter;
    fn shr(self, other: Counter) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for Counter {
    type Output = Counter;
    fn shr(self, other: i32) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for Counter {
    type Output = Counter;
    fn shr(self, other: i8) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for Counter {
    type Output = Counter;
    fn shr(self, other: i16) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for Counter {
    type Output = Counter;
    fn shr(self, other: i64) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for Counter {
    type Output = Counter;
    fn shr(self, other: u32) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for Counter {
    type Output = Counter;
    fn shr(self, other: u8) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for Counter {
    type Output = Counter;
    fn shr(self, other: u16) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for Counter {
    type Output = Counter;
    fn shr(self, other: u64) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for Counter {
    type Output = Counter;
    fn shr(self, other: usize) -> Counter {
        Counter(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for Counter {}

impl Ord for Counter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl File {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
    }

    pub fn size(&self) -> i64 {
        return (*self.size.lock().unwrap().as_ref().unwrap());
    }
}

impl Item for File {
    fn __go_clone_box_item(&self) -> Box<dyn Item + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Item + Send + Sync>
    }
    fn __go_eq_item(&self, other: &(dyn Item + Send + Sync)) -> bool {
        if let Some(__other) = Named::__go_as_any(other).downcast_ref::<File>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct FilePtr(pub Arc<Mutex<Option<File>>>);

impl std::fmt::Display for FilePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Item for FilePtr {
    fn __go_clone_box_item(&self) -> Box<dyn Item + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Item + Send + Sync>
    }
    fn __go_eq_item(&self, other: &(dyn Item + Send + Sync)) -> bool {
        if let Some(__other) = Named::__go_as_any(other).downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Named for File {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        File::name(self)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<File>() {
            self == __other
        } else {
            false
        }
    }
}

impl Named for FilePtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_ptr: *const File = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const File };
        let __recv = unsafe { &*__recv_ptr };
        File::name(__recv)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Sized for File {
    fn size(&self) -> i64 {
        File::size(self)
    }
    fn __go_clone_box_sized(&self) -> Box<dyn Sized + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sized + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sized(&self, other: &(dyn Sized + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<File>() {
            self == __other
        } else {
            false
        }
    }
}

impl Sized for FilePtr {
    fn size(&self) -> i64 {
        let __recv_ptr: *const File = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const File };
        let __recv = unsafe { &*__recv_ptr };
        File::size(__recv)
    }
    fn __go_clone_box_sized(&self) -> Box<dyn Sized + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sized + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_sized(&self, other: &(dyn Sized + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Dir {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
    }

    pub fn size(&self) -> i64 {
        0
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "dir:".to_string(), (*self.name.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl Item for Dir {
    fn __go_clone_box_item(&self) -> Box<dyn Item + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Item + Send + Sync>
    }
    fn __go_eq_item(&self, other: &(dyn Item + Send + Sync)) -> bool {
        if let Some(__other) = Named::__go_as_any(other).downcast_ref::<Dir>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct DirPtr(pub Arc<Mutex<Option<Dir>>>);

impl std::fmt::Display for DirPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Item for DirPtr {
    fn __go_clone_box_item(&self) -> Box<dyn Item + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Item + Send + Sync>
    }
    fn __go_eq_item(&self, other: &(dyn Item + Send + Sync)) -> bool {
        if let Some(__other) = Named::__go_as_any(other).downcast_ref::<DirPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Named for Dir {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Dir::name(self)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Dir>() {
            self == __other
        } else {
            false
        }
    }
}

impl Named for DirPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_ptr: *const Dir = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Dir };
        let __recv = unsafe { &*__recv_ptr };
        Dir::name(__recv)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DirPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Sized for Dir {
    fn size(&self) -> i64 {
        Dir::size(self)
    }
    fn __go_clone_box_sized(&self) -> Box<dyn Sized + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sized + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sized(&self, other: &(dyn Sized + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Dir>() {
            self == __other
        } else {
            false
        }
    }
}

impl Sized for DirPtr {
    fn size(&self) -> i64 {
        let __recv_ptr: *const Dir = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Dir };
        let __recv = unsafe { &*__recv_ptr };
        Dir::size(__recv)
    }
    fn __go_clone_box_sized(&self) -> Box<dyn Sized + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sized + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_sized(&self, other: &(dyn Sized + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DirPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Problem {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("problem".to_string())))
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("problem {}", (*self.code.lock().unwrap().as_ref().unwrap())))))
    }
}

impl StdError for Problem {}


impl Named for Problem {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Problem::name(self)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Problem>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ProblemPtr(pub Arc<Mutex<Option<Problem>>>);

impl std::fmt::Display for ProblemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Named for ProblemPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_ptr: *const Problem = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Problem };
        let __recv = unsafe { &*__recv_ptr };
        Problem::name(__recv)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ProblemPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Counter {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("counter".to_string())))
    }
}

impl Named for Counter {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Counter::name(self)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Counter>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CounterPtr(pub Arc<Mutex<Option<Counter>>>);

impl std::fmt::Display for CounterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Named for CounterPtr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_ptr: *const Counter = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Counter };
        let __recv = unsafe { &*__recv_ptr };
        Counter::name(__recv)
    }
    fn __go_clone_box_named(&self) -> Box<dyn Named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn Named + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CounterPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn show(n: Arc<Mutex<Option<Box<dyn Named + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", format!("{}", match n.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() })))))
}

pub fn make_item() -> Arc<Mutex<Option<Box<dyn Item + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(FilePtr(Arc::new(Mutex::new(Some(File { name: Arc::new(Mutex::new(Some("made".to_string()))), size: Arc::new(Mutex::new(Some(3))), ..Default::default() }))).clone())) as Box<dyn Item + Send + Sync>)))
}

fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut items = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(FilePtr(Arc::new(Mutex::new(Some(File { name: Arc::new(Mutex::new(Some("a.txt".to_string()))), size: Arc::new(Mutex::new(Some(10))), ..Default::default() }))).clone())) as Box<dyn Item + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(DirPtr(Arc::new(Mutex::new(Some(Dir { name: Arc::new(Mutex::new(Some("src".to_string()))), ..Default::default() }))).clone())) as Box<dyn Item + Send + Sync>)))])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for it in __range_values.iter() {
        println!("{} {} {}", format!("{}", match it.lock().unwrap().as_ref() { Some(__iface) => Named::__go_print(__iface.as_ref(), false), None => "<nil>".to_string() }), format!("{}", (*(*it.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())), format!("{}", (*it.lock().unwrap().as_ref().unwrap()).size()));
        print!("{}|{}|{}\n", format!("{}", match it.lock().unwrap().as_ref() { Some(__iface) => Named::__go_print(__iface.as_ref(), false), None => "<nil>".to_string() }), format!("{:#}", match it.lock().unwrap().as_ref() { Some(__iface) => Named::__go_print(__iface.as_ref(), true), None => "<nil>".to_string() }), format!("{}", (*show({ let __inner: Option<Box<dyn Named + Send + Sync>> = (*it.lock().unwrap()).clone().map(|__v| -> Box<dyn Named + Send + Sync> { __v }); Arc::new(Mutex::new(__inner)) }).lock().unwrap().as_ref().unwrap())));
    } }
    let mut h = Arc::new(Mutex::new(Some(Holder { item: Arc::new(Mutex::new(Some(Box::new(FilePtr(Arc::new(Mutex::new(Some(File { name: Arc::new(Mutex::new(Some("held".to_string()))), size: Arc::new(Mutex::new(Some(1))), ..Default::default() }))).clone())) as Box<dyn Item + Send + Sync>))), ..Default::default() })));
    println!("{}", format!("{}", match (*h.lock().unwrap().as_ref().unwrap()).item.lock().unwrap().as_ref() { Some(__iface) => Named::__go_print(__iface.as_ref(), false), None => "<nil>".to_string() }));
    println!("{}", format!("{}", Named::__go_print((*(make_item()).lock().unwrap().as_ref().unwrap()).as_ref(), false)));
    let mut n: Arc<Mutex<Option<Box<dyn Named + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(ProblemPtr(Arc::new(Mutex::new(Some(Problem { code: Arc::new(Mutex::new(Some(7))), ..Default::default() }))).clone())) as Box<dyn Named + Send + Sync>)));
    println!("{}", format!("{}", match n.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() }));
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(format!("<{}>", match n.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() })))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __text = match Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("wrapped: {}", match n.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() })))))).lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut c: Arc<Mutex<Option<Counter>>> = Arc::new(Mutex::new(Some(Counter(Arc::new(Mutex::new(Some(4 as i64)))))));
    let mut cn: Arc<Mutex<Option<Box<dyn Named + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(CounterPtr(c.clone().clone())) as Box<dyn Named + Send + Sync>)));
    println!("{} {}", format!("{}", strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_value = format!("{}", format!("{}", match cn.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() })); __arg_value }))), Arc::new(Mutex::new(Some("0x".to_string()))))), format!("{}", (*(*cn.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())));
    let mut none: Arc<Mutex<Option<Box<dyn Item + Send + Sync>>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", match none.lock().unwrap().as_ref() { Some(__iface) => Named::__go_print(__iface.as_ref(), false), None => "<nil>".to_string() }));
    let mut err = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("plain".to_string())))));
    println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}

impl GoValueClone for File {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Dir {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Problem {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Holder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
