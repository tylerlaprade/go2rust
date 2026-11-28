#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
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

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
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

/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::rc::Rc<dyn Fn($($arg),*) -> R> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<std::rc::Rc<dyn Fn($($arg),*) -> R>> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::rc::Rc<std::cell::RefCell<Option<std::rc::Rc<dyn Fn($($arg),*) -> R>>>> {
            type Func = dyn Fn($($arg),*) -> R;
            fn go_func_value(&self) -> Option<std::rc::Rc<Self::Func>> { self.borrow().clone() }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));

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

pub(crate) const __SLICES_UNKNOWN_HINT: i64 = 0;
pub(crate) const __SLICES_INCREASING_HINT: i64 = 1;
pub(crate) const __SLICES_DECREASING_HINT: i64 = 2;


#[derive(Debug)]
pub struct __slices_xorshift(pub Rc<RefCell<Option<u64>>>);

impl Clone for __slices_xorshift {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        __slices_xorshift(Rc::new(RefCell::new(__value)))
    }
}

impl Default for __slices_xorshift {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for __slices_xorshift {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for __slices_xorshift {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for __slices_xorshift {
    fn eq(&self, other: &u64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_xorshift {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for __slices_xorshift {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_xorshift> for u64 {
    fn eq(&self, other: &__slices_xorshift) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_xorshift> for u64 {
    fn partial_cmp(&self, other: &__slices_xorshift) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn add(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn sub(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn mul(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn div(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn rem(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitand(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitxor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_xorshift {
    type Output = __slices_xorshift;
    fn not(self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_xorshift {}

impl Ord for __slices_xorshift {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug)]
pub struct __slices_sortedHint(pub Rc<RefCell<Option<i64>>>);

impl Clone for __slices_sortedHint {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        __slices_sortedHint(Rc::new(RefCell::new(__value)))
    }
}

impl Default for __slices_sortedHint {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for __slices_sortedHint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for __slices_sortedHint {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for __slices_sortedHint {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_sortedHint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for __slices_sortedHint {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_sortedHint> for i64 {
    fn eq(&self, other: &__slices_sortedHint) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_sortedHint> for i64 {
    fn partial_cmp(&self, other: &__slices_sortedHint) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn add(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn sub(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn mul(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn div(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn neg(self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn rem(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitand(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn not(self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_sortedHint {}

impl Ord for __slices_sortedHint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl __slices_xorshift {
    pub fn next(&mut self) -> u64 {
        { let __rhs = (*((*self).clone() << 13i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*((*self).clone() >> 7i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*((*self).clone() << 17i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as u64))).borrow().as_ref().unwrap())
    }
}

pub fn compare_length(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> i64 {
    if ((*a.borrow().as_ref().unwrap()).len() as i64) < ((*b.borrow().as_ref().unwrap()).len() as i64) {
        return -(1);
    }
    if ((*a.borrow().as_ref().unwrap()).len() as i64) > ((*b.borrow().as_ref().unwrap()).len() as i64) {
        return 1;
    }
    if (*a.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        return -(1);
    }
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return 1;
    }
    0
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();

    let mut words = Rc::new(RefCell::new(Some(vec!["pear".to_string(), "fig".to_string(), "apple".to_string(), "plum".to_string(), "date".to_string()])));
    __slices__sort_func__slice_string__string(words.clone(), Rc::new(RefCell::new(Some(Rc::new(move |__arg0: Rc<RefCell<Option<String>>>, __arg1: Rc<RefCell<Option<String>>>| -> i64 { compare_length(__arg0, __arg1) }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>))));
    println!("{}", format!("{}", format_slice(&words)));
    let mut numbers = Rc::new(RefCell::new(Some(vec![3i64, 1i64, 4i64, 2i64])));
    __slices__sort_func__slice_int__int(numbers.clone(), Rc::new(RefCell::new(Some(Rc::new(move |a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>| -> i64 {
        (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap())
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>))));
    println!("{}", format!("{}", format_slice(&numbers)));
}

pub fn __slices__sort_func__slice_string__string(x: Rc<RefCell<Option<Vec<String>>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_cmp_func__string(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
}

pub fn __slices__sort_func__slice_int__int(x: Rc<RefCell<Option<Vec<i64>>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_cmp_func__int(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()) as u64; __arg_value })))); __arg_value }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
}

pub fn __slices_pdqsort_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[(pivot) as usize].clone(); __arg_value })))) } < 0) {
        let mut mid = __slices_partition_equal_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[(pivot) as usize].clone(); __arg_value })))) } < 0) {
        let mut mid = __slices_partition_equal_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_insertion_sort_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_heap_sort_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_break_patterns_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_choose_pivot_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_reverse_range_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_partial_insertion_sort_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone(); __arg_value })))) } < 0) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partition_equal_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_equal_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_partition_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_sift_down_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_sift_down_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && ({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone(); __arg_value })))) } < 0) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_next_power_of_two(length: Rc<RefCell<Option<i64>>>) -> u64 {
    { let __shift_x = 1u64; let __shift_c = (math_bits::len(Rc::new(RefCell::new(Some({ let __arg_value = (*length.borrow().as_ref().unwrap()) as u64; __arg_value }))))) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }
}

pub fn __slices_median_adjacent_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) -> i64 {
    __slices_median_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) - 1; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + 1; __arg_value }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })))
}

pub fn __slices_median_adjacent_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> i64 {
    __slices_median_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) - 1; __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_value = (*a.borrow().as_ref().unwrap()) + 1; __arg_value }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value })))
}

pub fn __slices_median_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_median_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone(), Rc::new(RefCell::new({ let __func_value = (*cmp.borrow()).clone(); __func_value }))); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_order2_cmp_func__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64>>>>) -> (i64, i64) {
    if { let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0 {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}

pub fn __slices_order2_cmp_func__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>, cmp: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64>>>>) -> (i64, i64) {
    if { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> i64> = cmp.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone(); __arg_value }))), Rc::new(RefCell::new(Some({ let __arg_value = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone(); __arg_value })))) } < 0 {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}