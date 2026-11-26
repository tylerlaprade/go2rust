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

pub trait localDecl: std::fmt::Display + Any {
    fn __go_clone_box_local_decl(&self) -> Box<dyn localDecl + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_local_decl(&self, other: &(dyn localDecl + Send + Sync)) -> bool;
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>;
}

impl Clone for Box<dyn localDecl + Send + Sync> {
    fn clone(&self) -> Self {
        localDecl::__go_clone_box_local_decl(self.as_ref())
    }
}

#[derive(Clone, Default)]
pub struct localVarDecl {
    pub spec: Arc<Mutex<Option<go_ast::r#mod::ValueSpec>>>,
}

impl localVarDecl {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.spec.clone();
        Self {
            spec: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for localVarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.spec.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for localVarDecl {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left_some = self.spec.lock().unwrap().is_some(); let __right_some = other.spec.lock().unwrap().is_some(); (!__left_some && !__right_some) || (__left_some && __right_some && Arc::ptr_eq(&self.spec, &other.spec)) }
        )
    }
}


impl localVarDecl {
    pub fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(self.spec.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))
    }
}

impl localDecl for localVarDecl {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        localVarDecl::node(self)
    }
    fn __go_clone_box_local_decl(&self) -> Box<dyn localDecl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn localDecl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_local_decl(&self, other: &(dyn localDecl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<localVarDecl>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct localVarDeclPtr(pub Arc<Mutex<Option<localVarDecl>>>);

impl std::fmt::Display for localVarDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl localDecl for localVarDeclPtr {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        localVarDecl::node(__recv)
    }
    fn __go_clone_box_local_decl(&self) -> Box<dyn localDecl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn localDecl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_local_decl(&self, other: &(dyn localDecl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<localVarDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn local_walk_decl(mut d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>, f: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>) -> () + Send + Sync>>>>) {
    let mut d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>> = Arc::new(Mutex::new(d.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Decl::__go_clone_box_decl(__v.as_ref()))));
    {
    let _ts_subject = d.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    std::mem::drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Decl + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).is_some() {
        let d = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).unwrap().0.clone();
        { let __range_holder = (*d.lock().unwrap().as_ref().unwrap()).specs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for mut s in __range_values.iter().cloned() {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    std::mem::drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Spec + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ValueSpecPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ValueSpecPtr>()).unwrap().0.clone();
        if {
            let __tmp_x = { let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned };
            let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i64))));
            __tmp_x == __tmp_y
        } {
        {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>) -> () + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(Box::new(localVarDecl { spec: s.clone(), ..Default::default() }) as Box<dyn localDecl + Send + Sync>)))
            )
        };
    };
    }
    }
    } };
    }
    }
}

pub fn local_decl_stmt(d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>) {
    local_walk_decl(
        d.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |d: Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>| {
        {
    let _ts_subject = d.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    std::mem::drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn localDecl + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<localVarDecl>()).is_some() {
        println!("{}", format!("{}", "local var".to_string()));;
    } else {
        println!("{}", format!("{}", "local unknown".to_string()));;
    }
    }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>) -> () + Send + Sync>)))
    );
}

fn main() {
    go_install_panic_exit();
    ::bytes::__go_init_all();
    ::cmp::__go_init_all();
    ::container_heap::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_constant::__go_init_all();
    ::go_parser::__go_init_all();
    ::go_scanner::__go_init_all();
    ::go_token::__go_init_all();
    ::go_types::__go_init_all();
    ::go_version::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_bisect::__go_init_all();
    ::internal_buildcfg::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_goexperiment::__go_init_all();
    ::internal_gover::__go_init_all();
    ::internal_goversion::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_sync::__go_init_all();
    ::internal_types_errors::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_big::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::slices::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut fset = go_token::new_file_set();
    let (mut file, mut err) = go_parser::parse_file(fset.clone(), Arc::new(Mutex::new(Some("input.go".to_string()))), Arc::new(Mutex::new(Some(Box::new("package main\nvar x int\n".to_string()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(go_parser::interface::Mode(Arc::new(Mutex::new(Some(0 as u64))))))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "parse failed".to_string()));
        return;
    }
    {
    let _ts_subject = { let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    std::mem::drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Decl + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).is_some() {
        println!("{}", format!("{}", "gen".to_string()));;
    } else {
        println!("{}", format!("{}", "other".to_string()));;
    }
    }
    local_decl_stmt({ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone());
    let (mut pkg, __tmp_1) = {
        let __recv = Arc::new(Mutex::new(Some(go_types::api::Config::default())));
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).check(
            Arc::new(Mutex::new(Some("main".to_string()))),
            fset.clone(),
            Arc::new(Mutex::new(Some(vec![file.clone()]))),
            Arc::new(Mutex::new(None)),
        );
        __result
    }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    println!("{} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*{ let __recv = pkg.clone(); let __recv_ptr: *const go_types::package::Package = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::package::Package }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for localVarDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
