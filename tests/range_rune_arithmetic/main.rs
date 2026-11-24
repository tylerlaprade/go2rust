#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn atoi(s: Arc<Mutex<Option<String>>>) -> i64 {
    let mut n = Arc::new(Mutex::new(Some(0i64)));
    for (_, c) in { let __range_string = (*s.lock().unwrap().as_ref().unwrap()).clone(); __range_string }.char_indices() {
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(((((c as i32) - ('0' as i32)))) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    }
    return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn caesar(s: Arc<Mutex<Option<String>>>, k: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<String>>> {
    let mut b: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    for (_, c) in { let __range_string = (*s.lock().unwrap().as_ref().unwrap()).clone(); __range_string }.char_indices() {
        if { let __tmp_x = c; let __tmp_y = 'a'; __tmp_x >= __tmp_y } && { let __tmp_x = c; let __tmp_y = 'z'; __tmp_x <= __tmp_y } {
            (*b.lock().unwrap().as_mut().unwrap()).write_rune(Arc::new(Mutex::new(Some(('a' as i32) + ((c as i32) - ('a' as i32) + ((*Arc::new(Mutex::new(Some(({ let __v = (*k.lock().unwrap().as_ref().unwrap()); __v }) as i32))).lock().unwrap().as_ref().unwrap()))) % (26 as i32)))));
        } else if { let __tmp_x = c; let __tmp_y = 'A'; __tmp_x >= __tmp_y } && { let __tmp_x = c; let __tmp_y = 'Z'; __tmp_x <= __tmp_y } {
            (*b.lock().unwrap().as_mut().unwrap()).write_rune(Arc::new(Mutex::new(Some(('A' as i32) + ((c as i32) - ('A' as i32) + ((*Arc::new(Mutex::new(Some(({ let __v = (*k.lock().unwrap().as_ref().unwrap()); __v }) as i32))).lock().unwrap().as_ref().unwrap()))) % (26 as i32)))));
        } else {
            (*b.lock().unwrap().as_mut().unwrap()).write_rune(Arc::new(Mutex::new(Some(c as i32))));
        }
    }
    return (*b.lock().unwrap().as_ref().unwrap()).string();
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", { let __tmp_x = atoi(Arc::new(Mutex::new(Some("4096".to_string())))); let __tmp_y = 1; __tmp_x + __tmp_y }));
    println!("{} {}", format!("{}", (*caesar(Arc::new(Mutex::new(Some("Hello, World!".to_string()))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*caesar(Arc::new(Mutex::new(Some("xyz".to_string()))), Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap())));
    let mut counts = Arc::new(Mutex::new(Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])));
    for (_, c) in "banana".to_string().char_indices() {
        { let __idx = ((c as i32) - ('a' as i32)) as usize; let mut __seq_guard = counts.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
    }
    println!("{} {} {}", format!("{}", { let __seq = { let __seq_holder = counts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = counts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = counts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(13) as usize].clone() }));
    for (i, c) in "az".to_string().char_indices() {
        let mut up = Arc::new(Mutex::new(Some((c as i32) - ('a' as i32) + ('A' as i32))));
        let mut next = Arc::new(Mutex::new(Some((c as i32) + (1 as i32))));
        println!("{} {} {} {} {} {}", format!("{}", i), format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32((({ let __v = (*up.lock().unwrap().as_ref().unwrap()); __v })) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32((({ let __v = (*next.lock().unwrap().as_ref().unwrap()); __v })) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())), format!("{}", (c as i32) * (2 as i32)), format!("{}", (c as i32) | (0x20 as i32)), format!("{}", (c as i32) & ! (0x20 as i32)));
    }
}