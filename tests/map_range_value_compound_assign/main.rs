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
    V: Display + Clone,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display + Clone,
{
    // The entries are copied out first, in key order, so a value's String
    // method that reads this map does not wait on the lock printing holds.
    let items: Option<Vec<(K, Arc<Mutex<Option<V>>>)>> = { let guard = map.lock().unwrap(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()) };
    if let Some(items) = items {
        let formatted: Vec<String> = items
            .iter()
            .map(|(k, v)| {
                // The value is copied out too, so its String method may
                // read its own entry.
                let value = { let v_guard = v.lock().unwrap(); v_guard.clone() };
                if let Some(ref val) = value {
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

#[derive(Debug)]
pub struct score(pub Arc<Mutex<Option<i64>>>);

impl Clone for score {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        score(Arc::new(Mutex::new(__value)))
    }
}

impl Default for score {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for score {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for score {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for score {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for score {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<score> for i64 {
    fn eq(&self, other: &score) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<score> for i64 {
    fn partial_cmp(&self, other: &score) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for score {
    type Output = score;
    fn add(self, other: Self) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for score {
    type Output = score;
    fn add(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<score> for i64 {
    type Output = score;
    fn add(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for score {
    type Output = score;
    fn sub(self, other: Self) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for score {
    type Output = score;
    fn sub(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<score> for i64 {
    type Output = score;
    fn sub(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for score {
    type Output = score;
    fn mul(self, other: Self) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for score {
    type Output = score;
    fn mul(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<score> for i64 {
    type Output = score;
    fn mul(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for score {
    type Output = score;
    fn div(self, other: Self) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for score {
    type Output = score;
    fn div(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<score> for i64 {
    type Output = score;
    fn div(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for score {
    type Output = score;
    fn neg(self) -> score {
        score(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for score {
    type Output = score;
    fn rem(self, other: Self) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for score {
    type Output = score;
    fn rem(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<score> for i64 {
    type Output = score;
    fn rem(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for score {
    type Output = score;
    fn bitand(self, other: Self) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for score {
    type Output = score;
    fn bitand(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<score> for i64 {
    type Output = score;
    fn bitand(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for score {
    type Output = score;
    fn bitor(self, other: Self) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for score {
    type Output = score;
    fn bitor(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<score> for i64 {
    type Output = score;
    fn bitor(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for score {
    type Output = score;
    fn bitxor(self, other: Self) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for score {
    type Output = score;
    fn bitxor(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<score> for i64 {
    type Output = score;
    fn bitxor(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for score {
    type Output = score;
    fn not(self) -> score {
        score(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for score {
    type Output = score;
    fn shl(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for score {
    type Output = score;
    fn shl(self, other: i32) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for score {
    type Output = score;
    fn shl(self, other: i8) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for score {
    type Output = score;
    fn shl(self, other: i16) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for score {
    type Output = score;
    fn shl(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for score {
    type Output = score;
    fn shl(self, other: u32) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for score {
    type Output = score;
    fn shl(self, other: u8) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for score {
    type Output = score;
    fn shl(self, other: u16) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for score {
    type Output = score;
    fn shl(self, other: u64) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for score {
    type Output = score;
    fn shl(self, other: usize) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for score {
    type Output = score;
    fn shr(self, other: score) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (*other.0.lock().unwrap().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for score {
    type Output = score;
    fn shr(self, other: i32) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for score {
    type Output = score;
    fn shr(self, other: i8) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for score {
    type Output = score;
    fn shr(self, other: i16) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for score {
    type Output = score;
    fn shr(self, other: i64) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for score {
    type Output = score;
    fn shr(self, other: u32) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for score {
    type Output = score;
    fn shr(self, other: u8) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for score {
    type Output = score;
    fn shr(self, other: u16) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for score {
    type Output = score;
    fn shr(self, other: u64) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for score {
    type Output = score;
    fn shr(self, other: usize) -> score {
        score(Arc::new(Mutex::new(Some({ let __shift_x = *self.0.lock().unwrap().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for score {}

impl Ord for score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct tally {
    pub n: Arc<Mutex<Option<i64>>>,
}

impl tally {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for tally {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for tally {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut counts = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(2i64)))), ("b".to_string(), Arc::new(Mutex::new(Some(5i64))))]))));
    let mut n = Arc::new(Mutex::new(Some(0i64)));
    for (_, v) in { let __range_holder = counts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __rhs = (*v.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    let mut text = Arc::new(Mutex::new(Some("".to_string())));
    let mut words = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<String>>>>::from([(1i64, Arc::new(Mutex::new(Some("x".to_string())))), (2i64, Arc::new(Mutex::new(Some("y".to_string()))))]))));
    let mut keys = Arc::new(Mutex::new(Some(Vec::<i64>::new())));
    for (k, _) in { let __range_holder = words.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k; let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    sort::ints(keys.clone());
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for k in __range_values.iter().copied() {
        { (*text.lock().unwrap().as_mut().unwrap()).push_str(&{ let __map = { let __map_holder = words.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&k)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }); };
    } }
    for (_, w) in { let __range_holder = words.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        if { let __tmp_x = (*w.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "y".to_string(); __tmp_x == __tmp_y } {
        { (*text.lock().unwrap().as_mut().unwrap()).push_str(&(*w.lock().unwrap().as_mut().unwrap())); };
    }
    }
    let mut f = Arc::new(Mutex::new(Some(1.0)));
    for (_, v) in { let __range_holder = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<f64>>>>::from([("h".to_string(), Arc::new(Mutex::new(Some(0.5))))])))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __rhs = (*v.lock().unwrap().as_ref().unwrap()); let mut guard = f.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let __rhs = (*v.lock().unwrap().as_ref().unwrap()); let mut guard = f.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
    let mut s: Arc<Mutex<Option<score>>> = Arc::new(Mutex::new(Some(score(Arc::new(Mutex::new(Some(0)))))));
    for (_, v) in { let __range_holder = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<score>>>>::from([("s".to_string(), Arc::new(Mutex::new(Some(score(Arc::new(Mutex::new(Some(4 as i64))))))))])))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __rhs = (*((*v.lock().unwrap().as_mut().unwrap())).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + __rhs); };
    }
    let mut t = Arc::new(Mutex::new(Some(tally { n: Arc::new(Mutex::new(Some(0))) })));
    for (_, v) in { let __range_holder = counts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).n.clone(); let __rhs = (*v.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    let mut total = Arc::new(Mutex::new(Some(0i64)));
    for (_, v) in { let __range_holder = counts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        let mut total_closure_clone = total.clone(); let v_closure_clone = v.clone(); { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || {
        let mut total_closure_clone = total_closure_clone.clone();
        { let __rhs = (*v_closure_clone.lock().unwrap().as_ref().unwrap()); let mut guard = total_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Arc<dyn Fn() -> () + Send + Sync>))); let __f: Arc<dyn Fn() -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
    }
    println!("{} {} {} {} {} {} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*text.lock().unwrap().as_ref().unwrap()).clone(); __v }), { let __text = go_float_string(({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text }, format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*t.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_map(&counts)));
}

impl GoValueClone for tally {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
