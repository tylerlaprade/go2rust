#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


// An error that wraps others: fmt.Errorf with %w, or errors.Join. many is
// true when it wraps more than one, and then errors.Unwrap does not see it.
#[derive(Debug)]
struct GoWrapError {
    msg: String,
    wrapped: Vec<Arc<dyn StdError + Send + Sync>>,
    many: bool,
}

impl std::fmt::Display for GoWrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for GoWrapError {}

fn go_error_wrap(msg: String, wrapped: Vec<Option<Arc<dyn StdError + Send + Sync>>>) -> Arc<dyn StdError + Send + Sync> {
    let many = wrapped.len() > 1;
    Arc::new(GoWrapError { msg, wrapped: wrapped.into_iter().flatten().collect(), many }) as Arc<dyn StdError + Send + Sync>
}

fn go_errors_join(errs: Vec<Option<Arc<dyn StdError + Send + Sync>>>) -> Option<Arc<dyn StdError + Send + Sync>> {
    let errs: Vec<Arc<dyn StdError + Send + Sync>> = errs.into_iter().flatten().collect();
    if errs.is_empty() {
        return None;
    }
    let msg = errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
    Some(Arc::new(GoWrapError { msg, wrapped: errs, many: true }) as Arc<dyn StdError + Send + Sync>)
}

// err and every error it wraps, depth first, in the order errors.Is and
// errors.As visit them.
fn go_error_chain(err: Option<Arc<dyn StdError + Send + Sync>>) -> Vec<Arc<dyn StdError + Send + Sync>> {
    let mut out = Vec::new();
    let mut stack: Vec<Arc<dyn StdError + Send + Sync>> = err.into_iter().collect();
    while let Some(e) = stack.pop() {
        if let Some(w) = e.downcast_ref::<GoWrapError>() {
            stack.extend(w.wrapped.iter().rev().cloned());
        }
        if let Some(__v) = e.downcast_ref::<strconv::atoi::NumError>() { if let Some(__c) = (*__v.unwrap().lock().unwrap()).clone() { stack.push(__c); } }
        out.push(e);
    }
    out
}

fn go_errors_unwrap(err: Option<Arc<dyn StdError + Send + Sync>>) -> Option<Arc<dyn StdError + Send + Sync>> {
    let err = err?;
    if let Some(w) = err.downcast_ref::<GoWrapError>() {
        return if w.many { None } else { w.wrapped.first().cloned() };
    }
    let e = err;
    let mut stack: Vec<Arc<dyn StdError + Send + Sync>> = Vec::new();
        if let Some(__v) = e.downcast_ref::<strconv::atoi::NumError>() { if let Some(__c) = (*__v.unwrap().lock().unwrap()).clone() { stack.push(__c); } }
    stack.pop()
}

// Two errors are the same error when they point to the same value, as ==
// on two errors is written.
fn go_errors_is(err: Option<Arc<dyn StdError + Send + Sync>>, target: Option<Arc<dyn StdError + Send + Sync>>) -> bool {
    match (err, target) {
        (None, None) => true,
        (Some(err), Some(target)) => go_error_chain(Some(err)).iter().any(|e| std::ptr::addr_eq(&**e, &*target)),
        _ => false,
    }
}

fn main() {
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    for s in &vec!["42".to_string(), "-7".to_string(), "+3".to_string(), "0x1f".to_string(), "abc".to_string(), "99999999999999999999".to_string(), "".to_string()] {
        let (mut n, mut err) = strconv::atoi(Arc::new(Mutex::new(Some((*s).clone()))));
        println!("{} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    for tc in &vec![
        AnonymousStruct1 { s: Arc::new(Mutex::new(Some("ff".to_string()))), base: Arc::new(Mutex::new(Some(16))), bits: Arc::new(Mutex::new(Some(64))), ..Default::default() },
        AnonymousStruct1 { s: Arc::new(Mutex::new(Some("0x_ff".to_string()))), base: Arc::new(Mutex::new(Some(0))), bits: Arc::new(Mutex::new(Some(64))), ..Default::default() },
        AnonymousStruct1 { s: Arc::new(Mutex::new(Some("-128".to_string()))), base: Arc::new(Mutex::new(Some(10))), bits: Arc::new(Mutex::new(Some(8))), ..Default::default() },
        AnonymousStruct1 { s: Arc::new(Mutex::new(Some("128".to_string()))), base: Arc::new(Mutex::new(Some(10))), bits: Arc::new(Mutex::new(Some(8))), ..Default::default() },
        AnonymousStruct1 { s: Arc::new(Mutex::new(Some("777".to_string()))), base: Arc::new(Mutex::new(Some(8))), bits: Arc::new(Mutex::new(Some(16))), ..Default::default() },
        AnonymousStruct1 { s: Arc::new(Mutex::new(Some("1010".to_string()))), base: Arc::new(Mutex::new(Some(2))), bits: Arc::new(Mutex::new(Some(64))), ..Default::default() },
        AnonymousStruct1 { s: Arc::new(Mutex::new(Some("z".to_string()))), base: Arc::new(Mutex::new(Some(36))), bits: Arc::new(Mutex::new(Some(64))), ..Default::default() },
    ] {
        let tc = &tc.__go_value_clone();
        let (mut n, mut err) = strconv::parse_int(Arc::new(Mutex::new(Some({ let __selector_holder = tc.s.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = tc.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = tc.bits.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        println!("{} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    let (mut u, mut err) = strconv::parse_uint(Arc::new(Mutex::new(Some("18446744073709551615".to_string()))), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(64))));
    println!("{} {}", format!("{}", u), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let (__tmp_0, __tmp_1) = strconv::parse_uint(Arc::new(Mutex::new(Some("-1".to_string()))), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(64)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    for s in &vec!["3.14159".to_string(), "1e10".to_string(), "-0.0".to_string(), "inf".to_string(), "NaN".to_string(), "1e400".to_string(), "0x1p-2".to_string(), "12abc".to_string(), ".5".to_string()] {
        let (mut f, mut err) = strconv::parse_float(Arc::new(Mutex::new(Some((*s).clone()))), Arc::new(Mutex::new(Some(64))));
        println!("{} {}", format!("{}", f), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    let (mut f32, __tmp_1) = strconv::parse_float(Arc::new(Mutex::new(Some("3.4e39".to_string()))), Arc::new(Mutex::new(Some(32)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    println!("{} {}", format!("{}", f32), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    for b in &vec!["true".to_string(), "F".to_string(), "1".to_string(), "yes".to_string()] {
        let (mut v, mut err) = strconv::parse_bool(Arc::new(Mutex::new(Some((*b).clone()))));
        println!("{} {}", format!("{}", v), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    println!("{} {} {} {}", format!("{}", (*strconv::itoa(Arc::new(Mutex::new(Some(-12345)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::format_int(Arc::new(Mutex::new(Some(255 as i64))), Arc::new(Mutex::new(Some(16)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::format_int(Arc::new(Mutex::new(Some(-255 as i64))), Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::format_uint(Arc::new(Mutex::new(Some(((1 as u64) << (63 as u64)) as u64))), Arc::new(Mutex::new(Some(36)))).lock().unwrap().as_ref().unwrap())));
    for v in vec![
        0.0,
        1.0,
        -(2.5),
        3.14159265358979,
        1e21,
        1e-7,
        123456789.125,
        0.3,
    ].iter().copied() {
        println!("{} {} {} {} {}", format!("{}", (*strconv::format_float(Arc::new(Mutex::new(Some(v.clone()))), Arc::new(Mutex::new(Some(('f' as u8) as u8))), Arc::new(Mutex::new(Some(-1))), Arc::new(Mutex::new(Some(64)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::format_float(Arc::new(Mutex::new(Some(v.clone()))), Arc::new(Mutex::new(Some(('e' as u8) as u8))), Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(64)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::format_float(Arc::new(Mutex::new(Some(v.clone()))), Arc::new(Mutex::new(Some(('g' as u8) as u8))), Arc::new(Mutex::new(Some(-1))), Arc::new(Mutex::new(Some(64)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::format_float(Arc::new(Mutex::new(Some(v.clone()))), Arc::new(Mutex::new(Some(('f' as u8) as u8))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(64)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::format_float(Arc::new(Mutex::new(Some(v.clone()))), Arc::new(Mutex::new(Some(('g' as u8) as u8))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(32)))).lock().unwrap().as_ref().unwrap())));
    }
    println!("{} {} {} {} {}", format!("{}", (*strconv::format_bool(Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::quote(Arc::new(Mutex::new(Some("hi\n\"there\"\t\u{263a}\u{0}".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::quote_to_a_s_c_i_i(Arc::new(Mutex::new(Some("\u{263a} \u{e9}".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::quote_rune(Arc::new(Mutex::new(Some(('\u{263a}' as i32))))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strconv::quote_rune_to_a_s_c_i_i(Arc::new(Mutex::new(Some(('\n' as i32))))).lock().unwrap().as_ref().unwrap())));
    let mut buf = Arc::new(Mutex::new(Some(("n=".to_string()).as_bytes().to_go_slice())));
    { let __slice_header = GoSliceHeader::go_slice_header(&(strconv::append_int(buf.clone(), Arc::new(Mutex::new(Some(-42 as i64))), Arc::new(Mutex::new(Some(10)))))); *buf.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = buf.clone(); let __append_value = (' ' as u8) as u8; let mut __appended: Vec<u8> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *buf.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&(strconv::append_float(buf.clone(), Arc::new(Mutex::new(Some(2.5))), Arc::new(Mutex::new(Some(('f' as u8) as u8))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(64)))))); *buf.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = buf.clone(); let __append_value = (' ' as u8) as u8; let mut __appended: Vec<u8> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *buf.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&(strconv::append_quote(buf.clone(), Arc::new(Mutex::new(Some("q".to_string())))))); *buf.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&(strconv::append_bool(buf.clone(), Arc::new(Mutex::new(Some(false)))))); *buf.lock().unwrap() = __slice_header; };
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*buf.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap())));
    { let (__tmp_0, __tmp_1) = strconv::atoi(Arc::new(Mutex::new(Some("12x".to_string())))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    let mut ne: Arc<Mutex<Option<strconv::atoi::NumError>>> = Arc::new(Mutex::new(None));
    if (*Arc::new(Mutex::new(Some({ let mut __as_found = None; for __as_err in go_error_chain({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }) { if let Some(__as_value) = __as_err.downcast_ref::<strconv::atoi::NumError>() { __as_found = Some(__as_value.clone()); break; } } match __as_found { Some(__as_value) => { let new_val = Arc::new(Mutex::new(Some(__as_value))); ne = new_val; true } None => false } }))).lock().unwrap().as_ref().unwrap()) {
        println!("{} {} {} {} {} {} {}", format!("{}", "func".to_string()), format!("{}", (*{ let __field = (*ne.lock().unwrap().as_ref().unwrap()).func.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", "num".to_string()), format!("{}", (*{ let __field = (*ne.lock().unwrap().as_ref().unwrap()).num.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", "err".to_string()), format!("{}", { let __text = match (*ne.lock().unwrap().as_ref().unwrap()).err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*Arc::new(Mutex::new(Some(go_errors_is({ let __err_handle = { let __err_source = { let __field = (*ne.lock().unwrap().as_ref().unwrap()).err.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, { let __err_handle = { let __err_source = { let __field = strconv::ErrSyntax.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))).lock().unwrap().as_ref().unwrap())));
    }
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(go_errors_is({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, { let __err_handle = { let __err_source = { let __field = strconv::ErrSyntax.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))).lock().unwrap().as_ref().unwrap())));
    { let (__tmp_0, __tmp_1) = strconv::parse_int(Arc::new(Mutex::new(Some("99999999999999999999".to_string()))), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(64)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(go_errors_is({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, { let __err_handle = { let __err_source = { let __field = strconv::ErrRange.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {} {}", format!("{}", strconv::is_print(Arc::new(Mutex::new(Some(('a' as i32)))))), format!("{}", strconv::is_print(Arc::new(Mutex::new(Some(('\u{7}' as i32)))))), format!("{}", strconv::can_backquote(Arc::new(Mutex::new(Some("a`b".to_string()))))), format!("{}", strconv::INT_SIZE));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub s: Arc<Mutex<Option<String>>>,
    pub base: Arc<Mutex<Option<i64>>>,
    pub bits: Arc<Mutex<Option<i64>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.bits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            s: __go_clone_0_0,
            base: __go_clone_1_0,
            bits: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            s: __go_default_0_0,
            base: __go_default_1_0,
            bits: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.base.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.bits.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}
