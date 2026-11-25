#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut lines = Arc::new(Mutex::new(Some(vec!["a#".to_string(), "#b".to_string(), "\u{e9}#".to_string()])));
    let mut n = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = lines.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (r, line) in __range_values.iter().enumerate() {
        for (c, ch) in line.char_indices() {
        if { let __tmp_x = ch; let __tmp_y = '#'; __tmp_x == __tmp_y } {
        { let __rhs = { let __tmp_x = { let __tmp_x = r as i64; let __tmp_y = 10; __tmp_x * __tmp_y }; let __tmp_y = c as i64; __tmp_x + __tmp_y }; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
    } }
    let mut count = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = lines.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for line in __range_values.iter() {
        for (_, ch) in line.char_indices() {
        if { let __tmp_x = ch; let __tmp_y = '#'; __tmp_x != __tmp_y } {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        for (i, _) in line.char_indices() {
        let _ = &i;
    }
    } }
    let mut words = strings::fields(Arc::new(Mutex::new(Some("go to rust".to_string()))));
    let mut words = { let __slice_header = GoSliceHeader::go_slice_header(&words); Arc::new(Mutex::new(__slice_header)) };
    let mut initials: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = words.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for w in __range_values.iter() {
        for (_, ch) in w.char_indices() {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = initials.clone(); let __append_value = ch as i32; let mut __appended: Vec<i32> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *initials.lock().unwrap() = __slice_header; };
        break
    }
    } }
    println!("{} {} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*Arc::new(Mutex::new(Some((*initials.lock().unwrap().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>()))).lock().unwrap().as_ref().unwrap())));
}