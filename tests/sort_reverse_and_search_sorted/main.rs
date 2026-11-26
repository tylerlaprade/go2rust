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
pub struct byLen(pub Arc<Mutex<Option<Vec<String>>>>);

impl Display for byLen {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct point {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct byX(pub Arc<Mutex<Option<Vec<point>>>>);

impl Display for byX {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


impl byLen {
    pub fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }

    pub fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = ({ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.len() as i64);
            let __tmp_y = ({ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.len() as i64);
            __tmp_x < __tmp_y
        };
    }

    pub fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let mut __self = self.clone();
        {
            let __go_lhs_sequence_0 = __self.0.clone().clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = __self.0.clone().clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            let __tmp_1 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
    }
}

impl sort::r#mod::Interface for byLen {
    fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = ({ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.len() as i64);
            let __tmp_y = ({ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.len() as i64);
            __tmp_x < __tmp_y
        };
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let mut __self = self.clone();
        {
            let __go_lhs_sequence_0 = __self.0.clone().clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = __self.0.clone().clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            let __tmp_1 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<byLen>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct byLenPtr(pub Arc<Mutex<Option<byLen>>>);

impl std::fmt::Display for byLenPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl sort::r#mod::Interface for byLenPtr {
    fn len(&self) -> i64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byLen::len(__recv)
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byLen::less(__recv, i, j)
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byLen::swap(__recv, i, j)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<byLenPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl byX {
    pub fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }

    /// Less reads two slice elements in one expression; the method body is
    /// a single tail expression that starts with a block.
    pub fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = (*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.x.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.x.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        };
    }

    pub fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let mut __self = self.clone();
        {
            let __go_lhs_sequence_0 = __self.0.clone().clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = __self.0.clone().clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            let __tmp_1 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
    }
}

impl sort::r#mod::Interface for byX {
    fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }
    /// Less reads two slice elements in one expression; the method body is
    /// a single tail expression that starts with a block.
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = (*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.x.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.x.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        };
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let mut __self = self.clone();
        {
            let __go_lhs_sequence_0 = __self.0.clone().clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = __self.0.clone().clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            let __tmp_1 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<byX>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct byXPtr(pub Arc<Mutex<Option<byX>>>);

impl std::fmt::Display for byXPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl sort::r#mod::Interface for byXPtr {
    fn len(&self) -> i64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byX::len(__recv)
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byX::less(__recv, i, j)
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byX::swap(__recv, i, j)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<byXPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn first_at_least(values: Arc<Mutex<Option<Vec<i64>>>>, limit: Arc<Mutex<Option<i64>>>) -> i64 {
    let mut values = Arc::new(Mutex::new((*values.lock().unwrap()).clone()));
    let limit_closure_clone = limit.clone(); let values_closure_clone = values.clone(); return sort::search(Arc::new(Mutex::new(Some({ let __arg_value = ({ let __len = (*values_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64; __arg_value }))), Arc::new(Mutex::new(Some({ let values_closure_clone_closure_clone = values_closure_clone.clone(); Arc::new(move |i: Arc<Mutex<Option<i64>>>| -> bool {
        return {
            let __tmp_x = { let __seq = { let __seq_holder = values_closure_clone_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = { let __v = (*limit_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v };
            __tmp_x >= __tmp_y
        };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync> }))));
}

fn main() {
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut words = Arc::new(Mutex::new(Some(vec!["banana".to_string(), "kiwi".to_string(), "apple".to_string(), "fig".to_string()])));
    sort::sort(Arc::new(Mutex::new(Some(Box::new(byLen(words.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>))));
    println!("{} {}", format!("{}", format_slice(&words)), format!("{}", sort::is_sorted(Arc::new(Mutex::new(Some(Box::new(byLen(words.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>))))));
    sort::stable(sort::reverse(Arc::new(Mutex::new(Some(Box::new(byLen(words.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>)))).clone());
    println!("{} {} {}", format!("{}", format_slice(&words)), format!("{}", sort::is_sorted(Arc::new(Mutex::new(Some(Box::new(byLen(words.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>))))), format!("{}", sort::is_sorted(sort::reverse(Arc::new(Mutex::new(Some(Box::new(byLen(words.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>)))).clone())));
    let mut points = Arc::new(Mutex::new(Some(vec![
        point { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() },
        point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() },
        point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(3))), ..Default::default() },
        point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(0))), ..Default::default() },
    ])));
    sort::stable(Arc::new(Mutex::new(Some(Box::new(byX(points.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>))));
    println!("{}", format!("{}", format_slice(&points)));
    sort::sort(sort::reverse(Arc::new(Mutex::new(Some(Box::new(byX(points.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>)))).clone());
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = points.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = points.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    let mut ints = Arc::new(Mutex::new(Some(vec![1i64, 3i64, 5i64, 7i64])));
    println!("{} {} {} {}", format!("{}", sort::search_ints(ints.clone(), Arc::new(Mutex::new(Some(5))))), format!("{}", sort::search_ints(ints.clone(), Arc::new(Mutex::new(Some(4))))), format!("{}", sort::search_ints(ints.clone(), Arc::new(Mutex::new(Some(9))))), format!("{}", sort::search_ints(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(1))))));
    let mut floats = Arc::new(Mutex::new(Some(vec![0.5, 1.5, 2.5])));
    println!("{} {}", format!("{}", sort::search_float64s(floats.clone(), Arc::new(Mutex::new(Some(1.5))))), format!("{}", sort::search_float64s(floats.clone(), Arc::new(Mutex::new(Some(3.0))))));
    let mut names = Arc::new(Mutex::new(Some(vec!["ada".to_string(), "bob".to_string(), "eve".to_string()])));
    println!("{} {}", format!("{}", sort::search_strings(names.clone(), Arc::new(Mutex::new(Some("bob".to_string()))))), format!("{}", sort::search_strings(names.clone(), Arc::new(Mutex::new(Some("carl".to_string()))))));
    println!("{} {}", format!("{}", sort::search(Arc::new(Mutex::new(Some(100))), Arc::new(Mutex::new(Some(Arc::new(move |i: Arc<Mutex<Option<i64>>>| -> bool {
        return { let __tmp_x = { let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i }; let __tmp_y = 50; __tmp_x >= __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>))))), format!("{}", first_at_least(ints.clone(), Arc::new(Mutex::new(Some(6))))));
}

impl GoValueClone for point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
