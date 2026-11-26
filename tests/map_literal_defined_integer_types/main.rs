#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
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

pub(crate) const NAMED: i64 = 3;


pub(crate) const OFFSET: i64 = 2;


#[derive(Debug)]
pub struct Index(pub Rc<RefCell<Option<i64>>>);

impl Clone for Index {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Index(Rc::new(RefCell::new(__value)))
    }
}

impl Default for Index {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Index {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Index {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Index> for i64 {
    fn eq(&self, other: &Index) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Index> for i64 {
    fn partial_cmp(&self, other: &Index) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Index {
    type Output = Index;
    fn add(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Index {
    type Output = Index;
    fn add(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Index> for i64 {
    type Output = Index;
    fn add(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Index {
    type Output = Index;
    fn sub(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Index {
    type Output = Index;
    fn sub(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Index> for i64 {
    type Output = Index;
    fn sub(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Index {
    type Output = Index;
    fn mul(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Index {
    type Output = Index;
    fn mul(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Index> for i64 {
    type Output = Index;
    fn mul(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Index {
    type Output = Index;
    fn div(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Index {
    type Output = Index;
    fn div(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Index> for i64 {
    type Output = Index;
    fn div(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Index {
    type Output = Index;
    fn neg(self) -> Index {
        Index(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Index {
    type Output = Index;
    fn rem(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Index {
    type Output = Index;
    fn rem(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Index> for i64 {
    type Output = Index;
    fn rem(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Index {
    type Output = Index;
    fn bitand(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Index {
    type Output = Index;
    fn bitand(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Index> for i64 {
    type Output = Index;
    fn bitand(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Index {
    type Output = Index;
    fn bitor(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Index {
    type Output = Index;
    fn bitor(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Index> for i64 {
    type Output = Index;
    fn bitor(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Index {
    type Output = Index;
    fn bitxor(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Index {
    type Output = Index;
    fn bitxor(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Index> for i64 {
    type Output = Index;
    fn bitxor(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Index {
    type Output = Index;
    fn not(self) -> Index {
        Index(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Index {
    type Output = Index;
    fn shl(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for Index {
    type Output = Index;
    fn shl(self, other: i32) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for Index {
    type Output = Index;
    fn shl(self, other: i8) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for Index {
    type Output = Index;
    fn shl(self, other: i16) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for Index {
    type Output = Index;
    fn shl(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for Index {
    type Output = Index;
    fn shl(self, other: u32) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for Index {
    type Output = Index;
    fn shl(self, other: u8) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for Index {
    type Output = Index;
    fn shl(self, other: u16) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for Index {
    type Output = Index;
    fn shl(self, other: u64) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for Index {
    type Output = Index;
    fn shl(self, other: usize) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for Index {
    type Output = Index;
    fn shr(self, other: Index) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for Index {
    type Output = Index;
    fn shr(self, other: i32) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for Index {
    type Output = Index;
    fn shr(self, other: i8) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for Index {
    type Output = Index;
    fn shr(self, other: i16) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for Index {
    type Output = Index;
    fn shr(self, other: i64) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for Index {
    type Output = Index;
    fn shr(self, other: u32) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for Index {
    type Output = Index;
    fn shr(self, other: u8) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for Index {
    type Output = Index;
    fn shr(self, other: u16) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for Index {
    type Output = Index;
    fn shr(self, other: u64) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for Index {
    type Output = Index;
    fn shr(self, other: usize) -> Index {
        Index(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for Index {}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Indexes(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Index>>>>>>>);

impl Display for Indexes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


fn main() {
    go_install_panic_exit();
    ::example_com_mapliteral_dep::__go_init_all();

    let mut local = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Index>>>>::from([("literal".to_string(), Rc::new(RefCell::new(Some(Index(Rc::new(RefCell::new(Some(2 as i64)))))))), ("named".to_string(), Rc::new(RefCell::new(Some(Index(Rc::new(RefCell::new(Some(NAMED as i64))))))))]))));
    println!("{} {}", format!("{}", { let __map_holder = local.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"literal".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Index(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = local.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"named".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Index(Rc::new(RefCell::new(Some(0))))) }));
    let mut indexes = Rc::new(RefCell::new(Some(Indexes(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Index>>>>::from([("computed".to_string(), Rc::new(RefCell::new(Some(Index(Rc::new(RefCell::new(Some((2 + 3) as i64))))))))]))))))));
    println!("{}", format!("{}", { let __map_holder = { let __named_map = (*indexes.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"computed".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Index(Rc::new(RefCell::new(Some(0))))) }));
    let mut levels = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<example_com_mapliteral_dep::r#mod::Level>>>>::from([("literal".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(4 as i64)))))))), ("named".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(example_com_mapliteral_dep::HIGH as i64)))))))), ("computed".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some((1 + 2) as i64)))))))), ("local".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(OFFSET as i64)))))))), ("imported".to_string(), Rc::new(RefCell::new(Some(example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(example_com_mapliteral_dep::OFFSET as i64))))))))]))));
    println!("{} {} {} {} {}", format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"literal".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"named".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"computed".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"local".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }), format!("{}", { let __map_holder = levels.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"imported".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| example_com_mapliteral_dep::r#mod::Level(Rc::new(RefCell::new(Some(0))))) }));
}