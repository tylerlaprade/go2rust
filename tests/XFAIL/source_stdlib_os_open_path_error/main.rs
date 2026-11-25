#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
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
        if let Some(__v) = e.downcast_ref::<io_fs::r#mod::PathError>() { if let Some(__c) = (*__v.unwrap().lock().unwrap()).clone() { stack.push(__c); } }
        if let Some(__v) = e.downcast_ref::<fs_PathError>() { if let Some(__c) = (*__v.err.lock().unwrap()).clone() { stack.push(__c); } }
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
        if let Some(__v) = e.downcast_ref::<io_fs::r#mod::PathError>() { if let Some(__c) = (*__v.unwrap().lock().unwrap()).clone() { stack.push(__c); } }
        if let Some(__v) = e.downcast_ref::<fs_PathError>() { if let Some(__c) = (*__v.err.lock().unwrap()).clone() { stack.push(__c); } }
    stack.pop()
}

// A filesystem sentinel of the stub crate (fs.ErrNotExist, ErrExist,
// ErrPermission, and os's names for them): an I/O error of its kind with no
// OS code and Go's text, or none.
fn go_native_error_is_sentinel(e: &std::io::Error) -> bool {
    if e.raw_os_error().is_some() {
        return false;
    }
    let text = match e.kind() {
        std::io::ErrorKind::NotFound => "file does not exist",
        std::io::ErrorKind::AlreadyExists => "file already exists",
        std::io::ErrorKind::PermissionDenied => "permission denied",
        _ => return false,
    };
    e.get_ref().map_or(true, |inner| inner.to_string() == text)
}

// Go compares syscall.Errno values by their numeric code. Its Is method
// additionally recognizes the filesystem sentinel of its kind when it is
// the target.
fn go_native_error_is(err: &Arc<dyn StdError + Send + Sync>, target: &Arc<dyn StdError + Send + Sync>) -> bool {
    let (Some(err), Some(target)) = (
        err.downcast_ref::<std::io::Error>(),
        target.downcast_ref::<std::io::Error>(),
    ) else { return false; };
    if let Some(target_code) = target.raw_os_error() {
        return err.raw_os_error() == Some(target_code);
    }
    go_native_error_is_sentinel(target) && err.kind() == target.kind()
        && (err.raw_os_error().is_some() || go_native_error_is_sentinel(err))
}

// Two errors are the same error when they point to the same value, as ==
// on two errors is written.
fn go_errors_is(err: Option<Arc<dyn StdError + Send + Sync>>, target: Option<Arc<dyn StdError + Send + Sync>>) -> bool {
    match (err, target) {
        (None, None) => true,
        (Some(err), Some(target)) => go_error_chain(Some(err)).iter().any(|e| std::ptr::addr_eq(&**e, &*target) || go_native_error_is(e, &target)),
        _ => false,
    }
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::io::__go_init_all();
    ::io_fs::__go_init_all();
    ::unicode_utf8::__go_init_all();

    const path: &'static str = "/definitely/missing/go2rust";

    let (_, mut err) = os::open("/definitely/missing/go2rust".to_string());
    let mut target: Arc<Mutex<Option<io_fs::r#mod::PathError>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some({ let mut __as_found = None; for __as_err in go_error_chain({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }) { if let Some(__as_value) = __as_err.downcast_ref::<io_fs::r#mod::PathError>() { __as_found = Some(__as_value.clone()); break; } } match __as_found { Some(__as_value) => { let new_val = Arc::new(Mutex::new(Some(__as_value))); target = new_val; true } None => false } }))).lock().unwrap().as_ref().unwrap())));
    if { let __nil_result = (*target.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", (*{ let __field = (*target.lock().unwrap().as_ref().unwrap()).op.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", { let __tmp_x = { let __selector_holder = (*target.lock().unwrap().as_ref().unwrap()).path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let __tmp_y = "/definitely/missing/go2rust".to_string(); __tmp_x == __tmp_y }));
        println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(go_errors_is({ let __err_handle = Arc::new(Mutex::new(go_errors_unwrap({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }))); let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, { let __err_handle = { let __err_source = { let __field = io_fs::ErrNotExist.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))).lock().unwrap().as_ref().unwrap())));
    }
}