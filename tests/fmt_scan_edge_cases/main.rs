#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
use go2rust_stdlib_stubs::*;
mod helper;
use helper::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Formatter};

#[derive(Debug)]
pub struct Celsius(pub Arc<Mutex<Option<f64>>>);

impl Clone for Celsius {
    fn clone(&self) -> Self {
        let __value = (*self.0.lock().unwrap()).clone();
        Celsius(Arc::new(Mutex::new(__value)))
    }
}

impl Default for Celsius {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Some(Default::default()))))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", go_float_string(*self.0.lock().unwrap().as_ref().unwrap() as f64, false))
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = Celsius;
    fn add(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = Celsius;
    fn add(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = Celsius;
    fn add(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Celsius {
    type Output = Celsius;
    fn sub(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = Celsius;
    fn sub(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = Celsius;
    fn sub(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Celsius {
    type Output = Celsius;
    fn mul(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<f64> for Celsius {
    type Output = Celsius;
    fn mul(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Celsius> for f64 {
    type Output = Celsius;
    fn mul(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Celsius {
    type Output = Celsius;
    fn div(self, other: Self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<f64> for Celsius {
    type Output = Celsius;
    fn div(self, other: f64) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Celsius> for f64 {
    type Output = Celsius;
    fn div(self, other: Celsius) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Celsius {
    type Output = Celsius;
    fn neg(self) -> Celsius {
        Celsius(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}


#[derive(Debug, Clone)]
pub struct errReader {
    pub s: Arc<Mutex<Option<String>>>,
    pub i: Arc<Mutex<Option<i64>>>,
}

impl errReader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            s: __go_clone_0_0,
            i: __go_clone_1_0,
        }
    }
}


impl Default for errReader {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            s: __go_default_0_0,
            i: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for errReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.i.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl errReader {
    pub fn read_rune(&mut self) -> (i32, i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        if {
            let __tmp_x = ((*self.i.clone().lock().unwrap().as_ref().unwrap()) as i64);
            let __tmp_y = (({ let __len = (*self.s.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64);
            __tmp_x < __tmp_y
        } {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*self.s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize] })));
        { let __target = self.i.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (
            (*Arc::new(Mutex::new(Some(({ let __v = (*c.lock().unwrap().as_ref().unwrap()); __v }) as i32))).lock().unwrap().as_ref().unwrap()),
            1,
            Arc::new(Mutex::new(None))
        );
    }
        (
            0,
            0,
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("boom".to_string())))))
        )
    }

    pub fn unread_rune(&mut self) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        { let __target = self.i.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        return Arc::new(Mutex::new(None));
    }

    pub fn read(&self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        (
            0,
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("unused".to_string())))))
        )
    }
}

#[derive(Clone)]
pub struct errReaderPtr(pub Arc<Mutex<Option<errReader>>>);

impl std::fmt::Display for errReaderPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl io::r#mod::Reader for errReaderPtr {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        errReader::read(__recv, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn io::r#mod::Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_reader(&self, other: &(dyn io::r#mod::Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<errReaderPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    ::io::__go_init_all();

    let mut f: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
    let (mut n, mut err) = { let mut __scan_src = go_scan_string_source("0x1.fp+2 -0x.8p1".to_string()); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Float(64, { let __scan_h = f.clone(); Box::new(move |__scan_v: f64| { *__scan_h.lock().unwrap() = Some(__scan_v as f64); }) }), GoScanTarget::Float(64, { let __scan_h = f.clone(); Box::new(move |__scan_v: f64| { *__scan_h.lock().unwrap() = Some(__scan_v as f64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) };
    println!("{} {} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), { let __text = go_float_string(({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text });
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_string_source("1p2147483648".to_string()); let __scan_format: String = "%b".to_string(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Float(64, { let __scan_h = f.clone(); Box::new(move |__scan_v: f64| { *__scan_h.lock().unwrap() = Some(__scan_v as f64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'f', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), { let __text = go_float_string(({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text });
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_string_source("1.5p3".to_string()); let __scan_format: String = "%g".to_string(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Float(64, { let __scan_h = f.clone(); Box::new(move |__scan_v: f64| { *__scan_h.lock().unwrap() = Some(__scan_v as f64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'f', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), { let __text = go_float_string(({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text });
    let mut a: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut b: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_string_source("7".to_string()); let __scan_format: String = "%100000000d%d".to_string(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = a.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) }), GoScanTarget::Int(64, { let __scan_h = b.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'f', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut u: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_string_source("18446744073709551616_".to_string()); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Uint(64, { let __scan_h = u.clone(); Box::new(move |__scan_v: u64| { *__scan_h.lock().unwrap() = Some(__scan_v as u64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut bs: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_string_source("\"\\xff\\101\u{e9}\"".to_string()); let __scan_format: String = "%q".to_string(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Bytes({ let __scan_h = bs.clone(); Box::new(move |__scan_v: std::vec::Vec<u8>| { *__scan_h.lock().unwrap() = Some((__scan_v).into()); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'f', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", format_slice(&bs)));
    let mut s: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_string_source("\"\\400\"".to_string()); let __scan_format: String = "%q".to_string(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Str({ let __scan_h = s.clone(); Box::new(move |__scan_v: String| { *__scan_h.lock().unwrap() = Some(__scan_v); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'f', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut r1: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut r2: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut r3: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut r4: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_string_source(String::from_utf8_lossy(&[0xf0u8, 0xc2u8, 0x41u8, 0x42u8]).into_owned().to_string()); let __scan_format: String = "%c%c%c%c".to_string(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(32, { let __scan_h = r1.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i32); }) }), GoScanTarget::Int(32, { let __scan_h = r2.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i32); }) }), GoScanTarget::Int(32, { let __scan_h = r3.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i32); }) }), GoScanTarget::Int(32, { let __scan_h = r4.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i32); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'f', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {} {} {} {} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*r1.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*r2.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32((({ let __v = (*r3.lock().unwrap().as_ref().unwrap()); __v })) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32((({ let __v = (*r4.lock().unwrap().as_ref().unwrap()); __v })) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())));
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i64>>>>>::from([("row".to_string(), Arc::new(Mutex::new(Some(vec![1i64, 2i64]))))]))));
    { let (__tmp_0, __tmp_1) = { let mut __scan_src = go_scan_string_source("9".to_string()); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = GoSliceElemPtr::new({ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"row".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(), (1) as usize); Box::new(move |__scan_v: i64| { *__scan_h.borrow_mut() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", format_slice(&{ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"row".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    { let (__tmp_0, __tmp_1) = { let __scan_reader1 = Arc::new(Mutex::new(Some(errReader { s: Arc::new(Mutex::new(Some("1 ".to_string()))), ..Default::default() }))); let mut __scan_src = GoScanRuneFns { read: Box::new(|| { let (__scan_r, _, __scan_e) = { let __recv = __scan_reader1.clone(); let __recv_ptr: *mut errReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut errReader }; let __result = unsafe { &mut *__recv_ptr }.read_rune(); __result }; let __scan_text = (*__scan_e.lock().unwrap()).as_ref().map(|__scan_x| __scan_x.to_string()); match __scan_text { None => Ok(char::from_u32(__scan_r as u32)), Some(__scan_t) if __scan_t == "EOF" => Ok(None), Some(__scan_t) => Err(__scan_t) } }), unread: Box::new(|| { let _ = { let __recv = __scan_reader1.clone(); let __recv_ptr: *mut errReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut errReader }; let __result = unsafe { &mut *__recv_ptr }.unread_rune(); __result }; }) }; let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = a.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) }), GoScanTarget::Int(64, { let __scan_h = b.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) }; n = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    { let (__tmp_0, __tmp_1) = read_pair(Arc::new(Mutex::new(Some("5".to_string())))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    { let __go_print_arg_1 = Arc::new(Mutex::new(Some(go_errors_is({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, { let __err_handle = { let __err_source = { let __field = io::EOF.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))); let __go_print_value = println!("{} {} {}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*__go_print_arg_1.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __left = err.clone(); let __right = io::EOF.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq })); __go_print_value };
    if { let __tmp_x = n; let __tmp_y = 0; __tmp_x < __tmp_y } {
        let mut c: Arc<Mutex<Option<Celsius>>> = Arc::new(Mutex::new(Some(Celsius(Arc::new(Mutex::new(Some(0.0)))))));
        { panic!("fmt.Sscan: an operand that is not a pointer to a bool, number, string or []byte is not supported; see docs/bridge_debt.md fmt-scan"); #[allow(unreachable_code)] (0 as i64, Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(String::new())))))) };
    }
}

impl GoValueClone for errReader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
