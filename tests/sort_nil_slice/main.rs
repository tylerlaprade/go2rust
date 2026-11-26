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


/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>>>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.lock().unwrap().clone() }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));

fn go_install_panic_exit() {
    std::panic::set_hook(Box::new(|__info| {
        let __text = go_panic_payload_text(__info.payload());
        {
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
        eprintln!("panic: {}", __text);
        std::process::exit(2);
    }));
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text(__text.clone());
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_payload_text(&**__inner);
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_payload_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_payload_text(&**__inner);
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
    if let Some(__value) = __payload.downcast_ref::<f64>() {
        // Go prints a float in a panic as +1.500000e+000.
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if *__value > 0.0 { "+Inf" } else { "-Inf" });
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

pub(crate) const __SLICES_UNKNOWN_HINT: i64 = 0;
pub(crate) const __SLICES_INCREASING_HINT: i64 = 1;
pub(crate) const __SLICES_DECREASING_HINT: i64 = 2;


#[derive(Debug)]
pub struct __slices_xorshift(pub Arc<Mutex<Option<u64>>>);

impl Clone for __slices_xorshift {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        __slices_xorshift(Arc::new(Mutex::new(__value)))
    }
}

impl Default for __slices_xorshift {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for __slices_xorshift {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for __slices_xorshift {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for __slices_xorshift {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_xorshift {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for __slices_xorshift {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_xorshift> for u64 {
    fn eq(&self, other: &__slices_xorshift) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_xorshift> for u64 {
    fn partial_cmp(&self, other: &__slices_xorshift) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn add(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn sub(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn mul(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn div(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn rem(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitand(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitxor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_xorshift {
    type Output = __slices_xorshift;
    fn not(self) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_xorshift {}

impl Ord for __slices_xorshift {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug)]
pub struct __slices_sortedHint(pub Arc<Mutex<Option<i64>>>);

impl Clone for __slices_sortedHint {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        __slices_sortedHint(Arc::new(Mutex::new(__value)))
    }
}

impl Default for __slices_sortedHint {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for __slices_sortedHint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for __slices_sortedHint {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for __slices_sortedHint {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_sortedHint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for __slices_sortedHint {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_sortedHint> for i64 {
    fn eq(&self, other: &__slices_sortedHint) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_sortedHint> for i64 {
    fn partial_cmp(&self, other: &__slices_sortedHint) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn add(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn sub(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn mul(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn div(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn neg(self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn rem(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitand(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn not(self) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_sortedHint {}

impl Ord for __slices_sortedHint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl __slices_xorshift {
    pub fn next(&mut self) -> u64 {
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 13i32; __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 7i32; __tmp_x >> __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 17i32; __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap())
    }
}

pub fn compare_strings(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> i64 {
    std::panic::panic_any(Box::new("nil slice comparator called".to_string()) as Box<dyn Any + Send + Sync>);
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    sort::strings(names.clone());
    println!("{} {} {} {}", format!("{}", "strings".to_string()), format!("{}", ({ let __len = (*names.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*names.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*names.lock().unwrap()).is_none(); __nil_result }));
    let mut nums: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    sort::ints(nums.clone());
    println!("{} {} {} {}", format!("{}", "ints".to_string()), format!("{}", ({ let __len = (*nums.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*nums.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*nums.lock().unwrap()).is_none(); __nil_result }));
    let mut ordered: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    __slices__sort__slice_int__int(ordered.clone());
    println!("{} {} {} {}", format!("{}", "slices".to_string()), format!("{}", ({ let __len = (*ordered.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*ordered.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*ordered.lock().unwrap()).is_none(); __nil_result }));
    let mut words: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    __slices__sort_func__slice_string__string(words.clone(), Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<String>>>, __arg1: Arc<Mutex<Option<String>>>| -> i64 { compare_strings(__arg0, __arg1) }) as Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>))));
    println!("{} {} {} {}", format!("{}", "sortfunc".to_string()), format!("{}", ({ let __len = (*words.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*words.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*words.lock().unwrap()).is_none(); __nil_result }));
}

pub fn __slices__sort__slice_int__int(x: Arc<Mutex<Option<Vec<i64>>>>) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__int(
        x.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value })))
    );
}

pub fn __slices__sort_func__slice_string__string(x: Arc<Mutex<Option<Vec<String>>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) {
    let mut n = Arc::new(Mutex::new(Some(({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_cmp_func__string(
        x.clone(),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_value = math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value })))); __arg_value }))),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    );
}

pub fn __slices_pdqsort_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut limit: Arc<Mutex<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        __slices_heap_sort_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        __slices_break_patterns_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        __slices_reverse_range_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.lock().unwrap() = Some(new_val); };
    }
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        if __slices_partial_insertion_sort_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(pivot) as usize].clone() })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        let mut mid = __slices_partition_equal_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot)))
        );
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };
        let (mut leftLen, mut rightLen) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))); __tuple_value });
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(mid))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut limit: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        __slices_insertion_sort_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        return;
    }
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        __slices_heap_sort_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        return;
    }
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        __slices_break_patterns_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        if { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        __slices_reverse_range_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.lock().unwrap() = Some(new_val); };
    }
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*hint.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = __slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64)))); __tmp_x == __tmp_y } {
        if __slices_partial_insertion_sort_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ) {
        return;
    }
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(pivot) as usize].clone() })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        let mut mid = __slices_partition_equal_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(pivot))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };
        let (mut leftLen, mut rightLen) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))); __tuple_value });
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(mid))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        __slices_pdqsort_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn __slices_insertion_sort_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less__int(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_heap_sort_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0i64)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        __slices_sift_down_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_break_patterns_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(__slices_xorshift(Arc::new(Mutex::new(Some(({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*length.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = 4;
let __go_binary_3 = __go_binary_1 / __go_binary_2;
let __go_binary_4 = 2;
let __go_binary_5 = __go_binary_3 * __go_binary_4;
let __go_binary_6 = __go_binary_0 + __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 - __go_binary_7;
let mut idx = Arc::new(Mutex::new(Some(__go_binary_8)));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })) as i64)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_choose_pivot_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> (i64, Arc<Mutex<Option<__slices_sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<__slices_sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut swaps: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.lock().unwrap() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__int(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone()
        ); *j.lock().unwrap() = Some(new_val); };
    }
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_reverse_range_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_partial_insertion_sort_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !__cmp__less__int(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !__cmp__less__int(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __cmp__less__int(x: Arc<Mutex<Option<i64>>>, y: Arc<Mutex<Option<i64>>>) -> bool {
    (__cmp_is_na_n__int(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__int(Arc::new(Mutex::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }
}

pub fn __slices_partition_equal_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> i64 {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_partition_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = __cmp__less__int(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = !__cmp__less__int(
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
            );
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !__cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

pub fn __slices_insertion_sort_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = ({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_heap_sort_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0i64)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        __slices_sift_down_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        );
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_break_patterns_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(__slices_xorshift(Arc::new(Mutex::new(Some(({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __go_binary_0 = (*a.lock().unwrap().as_ref().unwrap());
let __go_binary_1 = (*length.lock().unwrap().as_ref().unwrap());
let __go_binary_2 = 4;
let __go_binary_3 = __go_binary_1 / __go_binary_2;
let __go_binary_4 = 2;
let __go_binary_5 = __go_binary_3 * __go_binary_4;
let __go_binary_6 = __go_binary_0 + __go_binary_5;
let __go_binary_7 = 1;
let __go_binary_8 = __go_binary_6 - __go_binary_7;
let mut idx = Arc::new(Mutex::new(Some(__go_binary_8)));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some((({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })) as i64)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_choose_pivot_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<__slices_sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<__slices_sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut swaps: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
        { let new_val = __slices_median_adjacent_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *i.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *j.lock().unwrap() = Some(new_val); };
        { let new_val = __slices_median_adjacent_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *k.lock().unwrap() = Some(new_val); };
    }
        { let new_val = __slices_median_cmp_func__string(
            data.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            swaps.clone(),
            Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
        ); *j.lock().unwrap() = Some(new_val); };
    }
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(__slices_sortedHint(Arc::new(Mutex::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_reverse_range_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_partial_insertion_sort_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partition_equal_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) -> i64 {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_partition_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, pivot: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) -> (i64, bool) {
    let mut newpivot: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*pivot.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = ({ let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))); __tuple_value }, { let __tuple_value = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))); __tuple_value });
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = ({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while {
        let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = !({
                let __tmp_x =
                    {
                        let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                        (__f)(
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                        )
                    };
                let __tmp_y = 0;
                __tmp_x < __tmp_y
            });
            __go_cond_1
        } else {
            false
        }
    } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

pub fn __slices_sift_down_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, lo: Arc<Mutex<Option<i64>>>, hi: Arc<Mutex<Option<i64>>>, first: Arc<Mutex<Option<i64>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = __cmp__less__int(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                );
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__int(
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
            Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
        ) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn __slices_next_power_of_two(length: Arc<Mutex<Option<i64>>>) -> u64 {
    return { let __shift_x = 1u64; let __shift_c = (math_bits::len(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __v = (*length.lock().unwrap().as_ref().unwrap()); __v }) as u64; __arg_value }))))) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } };
}

pub fn __slices_median_adjacent_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    __slices_median_ordered__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        swaps.clone()
    )
}

pub fn __slices_median_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut c: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone()
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __cmp_is_na_n__int(x: Arc<Mutex<Option<i64>>>) -> bool {
    return { let __bin_x = ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_x != __bin_x };
}

pub fn __slices_sift_down_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, lo: Arc<Mutex<Option<i64>>>, hi: Arc<Mutex<Option<i64>>>, first: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if {
            let __go_cond_0 = { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = ({
                    let __tmp_x =
                        {
                            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                            (__f)(
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                                Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                            )
                        };
                    let __tmp_y = 0;
                    __tmp_x < __tmp_y
                });
                __go_cond_1
            } else {
                false
            }
        } {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !({
            let __tmp_x =
                {
                    let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                    (__f)(
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value }))),
                        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() }; __arg_value })))
                    )
                };
            let __tmp_y = 0;
            __tmp_x < __tmp_y
        }) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize;
            let __tmp_0 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            let __tmp_1 = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn __slices_median_adjacent_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) -> i64 {
    __slices_median_cmp_func__string(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    )
}

pub fn __slices_median_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, mut a: Arc<Mutex<Option<i64>>>, mut b: Arc<Mutex<Option<i64>>>, mut c: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__string(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__string(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_cmp_func__string(
        data.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        swaps.clone(),
        Arc::new(Mutex::new(GoFuncValue::go_func_value(&(cmp))))
    ); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn __slices_order2_ordered__int(data: Arc<Mutex<Option<Vec<i64>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__int(
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
        Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
    ) {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn __slices_order2_cmp_func__string(data: Arc<Mutex<Option<Vec<String>>>>, a: Arc<Mutex<Option<i64>>>, b: Arc<Mutex<Option<i64>>>, swaps: Arc<Mutex<Option<i64>>>, cmp: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync>>>>) -> (i64, i64) {
    if {
        let __tmp_x =
            {
                let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> i64 + Send + Sync> = cmp.lock().unwrap().as_ref().unwrap().clone();
                (__f)(
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value }))),
                    Arc::new(Mutex::new(Some({ let __arg_value = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __arg_value })))
                )
            };
        let __tmp_y = 0;
        __tmp_x < __tmp_y
    } {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}