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
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::text_tabwriter::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut w = text_tabwriter::new_writer(Arc::new(Mutex::new(Some({ let __arg_value = Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>; __arg_value }))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(8))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some((' ' as u8) as u8))), Arc::new(Mutex::new(Some(0 as u64))));
    { let __s = format!("{}\n", format!("{}", "NAME\tAGE\tCITY".to_string())); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("{}\n", format!("{}", "Ann\t30\tParis".to_string())); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("{}\n", format!("{}", "Bartholomew\t5\tRome".to_string())); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __recv = w.clone(); let __recv_ptr: *mut text_tabwriter::r#mod::Writer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut text_tabwriter::r#mod::Writer }; let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
    let mut tw = text_tabwriter::new_writer(Arc::new(Mutex::new(Some({ let __arg_value = Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>; __arg_value }))), Arc::new(Mutex::new(Some(6))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(('.' as u8) as u8))), Arc::new(Mutex::new(Some(({ let __tmp_x = text_tabwriter::ALIGN_RIGHT as u64; let __tmp_y = text_tabwriter::DEBUG as u64; __tmp_x | __tmp_y }) as u64))));
    { let __s = format!("a\tbbb\tc\n"); (*tw.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("aaaa\tb\tccccc\n"); (*tw.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __recv = tw.clone(); let __recv_ptr: *mut text_tabwriter::r#mod::Writer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut text_tabwriter::r#mod::Writer }; let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
    let mut sb: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut t = text_tabwriter::new_writer(Arc::new(Mutex::new(Some(Box::new(strings::builder::BuilderPtr(sb.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some((' ' as u8) as u8))), Arc::new(Mutex::new(Some(text_tabwriter::TAB_INDENT as u64))));
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        { let __s = format!("{}\t{}\t{}\n", format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("x".to_string()))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x * __tmp_y })))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i }), format!("{}", { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y })); (*t.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __recv = t.clone(); let __recv_ptr: *mut text_tabwriter::r#mod::Writer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut text_tabwriter::r#mod::Writer }; let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
    print!("{}", format!("{}", (*(*sb.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", strings::count((*sb.lock().unwrap().as_ref().unwrap()).string(), Arc::new(Mutex::new(Some("\n".to_string()))))));
}