#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}
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
        for (c, ch) in (line).clone().char_indices() {
        if { let __tmp_x = ch; let __tmp_y = '#'; __tmp_x == __tmp_y } {
        { let __rhs = { let __tmp_x = { let __tmp_x = r as i64; let __tmp_y = 10; __tmp_x * __tmp_y }; let __tmp_y = c as i64; __tmp_x + __tmp_y }; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
    } }
    let mut count = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = lines.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for line in __range_values.iter() {
        for (_, ch) in (line).clone().char_indices() {
        if { let __tmp_x = ch; let __tmp_y = '#'; __tmp_x != __tmp_y } {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        for (i, _) in (line).clone().char_indices() {
        let _ = &i;
    }
    } }
    let mut words = strings::fields(Arc::new(Mutex::new(Some("go to rust".to_string()))));
    let mut words = { let __slice_header = GoSliceHeader::go_slice_header(&words); Arc::new(Mutex::new(__slice_header)) };
    let mut initials: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = words.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for w in __range_values.iter() {
        for (_, ch) in (w).clone().char_indices() {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = initials.clone(); let __append_value = ch as i32; let mut __appended: Vec<i32> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *initials.lock().unwrap() = __slice_header; };
        break
    }
    } }
    println!("{} {} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*Arc::new(Mutex::new(Some((*initials.lock().unwrap().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>()))).lock().unwrap().as_ref().unwrap())));
    let mut s = Arc::new(Mutex::new(Some("abc".to_string())));
    for (s, _) in { let __range_string = (*s.lock().unwrap().as_ref().unwrap()).clone(); __range_string }.char_indices() {
        print!("{}{}", format!("{}", s), format!("{}", " ".to_string()));
    }
    println!();
    for mut line in vec!["ab".to_string(), "cd".to_string()].iter().cloned() {
        for (_, ch) in (line).clone().char_indices() {
        { let new_val = "".to_string(); line = new_val; };
        print!("{}", format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32(((ch as i32)) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())));
    }
        print!("{}{}{}", format!("{}", "[".to_string()), format!("{}", line), format!("{}", "]".to_string()));
    }
    println!();
    let mut counts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Arc<dyn Fn() -> i64 + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    for w in &vec!["xy".to_string(), "zzz".to_string()] {
        let w_closure_clone = w.clone(); { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = counts.clone(); let __append_value = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        let mut n = Arc::new(Mutex::new(Some(0i64)));
        for _ in (w_closure_clone).clone().chars() {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Arc<dyn Fn() -> i64 + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *counts.lock().unwrap() = __slice_header; };
    }
    { let __range_holder = counts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for f in __range_values.iter().cloned() {
        print!("{}{}", format!("{}", {
            let __f: Arc<dyn Fn() -> i64 + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
            (__f)(

            )
        }), format!("{}", " ".to_string()));
    } }
    println!();
}