#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut out: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));let mut errb: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut cmd = exec::command("echo".to_string(), ("hello".to_string(), "world".to_string()));
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(out.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*cmd.lock().unwrap().as_mut().unwrap()).stdout.lock().unwrap() = __iface_value.map(|__io| go2rust_stdlib_stubs::GoStubWriter::__go_from_fn({ let __io = std::sync::Mutex::new(__io); move |__data: &[u8]| { let _ = __io.lock().unwrap().write(Arc::new(Mutex::new(Some(Vec::from(__data))))); } })); };
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(errb.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*cmd.lock().unwrap().as_mut().unwrap()).stderr.lock().unwrap() = __iface_value.map(|__io| go2rust_stdlib_stubs::GoStubWriter::__go_from_fn({ let __io = std::sync::Mutex::new(__io); move |__data: &[u8]| { let _ = __io.lock().unwrap().write(Arc::new(Mutex::new(Some(Vec::from(__data))))); } })); };
    let mut err = { let __recv = cmd.clone(); let __recv_ptr: *mut exec_Cmd = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut exec_Cmd }; let __result = unsafe { &mut *__recv_ptr }.run(); __result };
    println!("{} {} {}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*strings::trim_space((*out.lock().unwrap().as_ref().unwrap()).string()).lock().unwrap().as_ref().unwrap())), format!("{}", (*errb.lock().unwrap().as_ref().unwrap()).len()));
    let mut sb: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut c2 = exec::command("printf".to_string(), ("a\nb\n".to_string(),));
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(strings::builder::BuilderPtr(sb.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*c2.lock().unwrap().as_mut().unwrap()).stdout.lock().unwrap() = __iface_value.map(|__io| go2rust_stdlib_stubs::GoStubWriter::__go_from_fn({ let __io = std::sync::Mutex::new(__io); move |__data: &[u8]| { let _ = __io.lock().unwrap().write(Arc::new(Mutex::new(Some(Vec::from(__data))))); } })); };
    println!("{} {}", format!("{}", { let __text = match { let __recv = c2.clone(); let __recv_ptr: *mut exec_Cmd = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut exec_Cmd }; let __result = unsafe { &mut *__recv_ptr }.run(); __result }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", strings::count((*sb.lock().unwrap().as_ref().unwrap()).string(), Arc::new(Mutex::new(Some("\n".to_string()))))));
    let mut sorted: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut c3 = exec::command("sort".to_string(), ());
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(strings::reader::ReaderPtr(strings::new_reader(Arc::new(Mutex::new(Some("pear\napple\nfig\n".to_string())))).clone())) as Box<dyn io::r#mod::Reader + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*c3.lock().unwrap().as_mut().unwrap()).stdin.lock().unwrap() = __iface_value.map(|__io| go2rust_stdlib_stubs::GoStubReader::__go_from_fn({ let __io = std::sync::Mutex::new(__io); move || { let mut __all: std::vec::Vec<u8> = std::vec::Vec::new(); loop { let __buf = Arc::new(Mutex::new(Some(Vec::from(&[0u8; 4096][..])))); let (__n, __err) = __io.lock().unwrap().read(__buf.clone()); if __n > 0 { __all.extend_from_slice(&__buf.lock().unwrap().as_ref().unwrap().as_slice()[..__n as usize]); } if __err.lock().unwrap().is_some() { break; } } __all } })); };
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(sorted.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*c3.lock().unwrap().as_mut().unwrap()).stdout.lock().unwrap() = __iface_value.map(|__io| go2rust_stdlib_stubs::GoStubWriter::__go_from_fn({ let __io = std::sync::Mutex::new(__io); move |__data: &[u8]| { let _ = __io.lock().unwrap().write(Arc::new(Mutex::new(Some(Vec::from(__data))))); } })); };
    println!("{}", format!("{}", { let __text = match { let __recv = c3.clone(); let __recv_ptr: *mut exec_Cmd = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut exec_Cmd }; let __result = unsafe { &mut *__recv_ptr }.run(); __result }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{}", format!("{}", (*(*sorted.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
    let mut failed: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut c4 = exec::command("sh".to_string(), ("-c".to_string(), "echo oops >&2; exit 3".to_string()));
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(failed.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*c4.lock().unwrap().as_mut().unwrap()).stderr.lock().unwrap() = __iface_value.map(|__io| go2rust_stdlib_stubs::GoStubWriter::__go_from_fn({ let __io = std::sync::Mutex::new(__io); move |__data: &[u8]| { let _ = __io.lock().unwrap().write(Arc::new(Mutex::new(Some(Vec::from(__data))))); } })); };
    println!("{} {}", format!("{}", { let __text = match { let __recv = c4.clone(); let __recv_ptr: *mut exec_Cmd = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut exec_Cmd }; let __result = unsafe { &mut *__recv_ptr }.run(); __result }.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*strings::trim_space((*failed.lock().unwrap().as_ref().unwrap()).string()).lock().unwrap().as_ref().unwrap())));
}