#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: GoSliceView<T>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: std::vec::Vec<String> = s.go_slice_view().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T, C>(slice: &C) -> String
where
    C: GoSliceView<T> + ?Sized,
    T: Display,
{
    let __view = slice.go_slice_view();
    let formatted: std::vec::Vec<String> = __view.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: GoSliceView<Arc<Mutex<Option<T>>>>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: std::vec::Vec<String> = s.go_slice_view().iter().map(|v| {
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

fn main() {
    ::internal_filepathlite::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::path_filepath::__go_init_all();

    let mut link = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xfe as u8, ('L' as u8)]))).lock().unwrap().as_ref().unwrap()).clone())))));
    let mut target = Arc::new(Mutex::new(Some(__go_string_from_bytes(std::vec::Vec::from((*Arc::new(Mutex::new(Some(vec![0xfa as u8, ('T' as u8)]))).lock().unwrap().as_ref().unwrap()).clone())))));

    let (mut info, mut lstatErr) = os::lstat({ let __arg_holder = link.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    let mut isLink = Arc::new(Mutex::new(Some({
        let __go_cond_0 = { let __nil_result = (*lstatErr.lock().unwrap()).is_none(); __nil_result };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __tmp_x = (*(*info.lock().unwrap().as_ref().unwrap()).mode().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = os::MODE_SYMLINK; __tmp_x & __tmp_y };
                let __tmp_y = fs_FileMode(0 as u32);
                __tmp_x != __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    })));
    let (mut resolved, mut evalErr) = path_filepath::eval_symlinks(Arc::new(Mutex::new(Some({ let __arg_holder = link.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    println!("{} {} {}", format!("{}", { let __nil_result = (*lstatErr.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __v = (*isLink.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_result = (*evalErr.lock().unwrap()).is_none(); __nil_result }));
    println!("{}", format!("{}", { let __left = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*resolved.lock().unwrap().as_ref().unwrap()).clone(); __v })); let __right = __go_string_from_bytes(__go_string_bytes(&{ let __v = (*target.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_string_cmp(&__left, &__right) == std::cmp::Ordering::Equal }));
    println!("{}", format!("{}", format_slice(&Arc::new(Mutex::new(Some(Vec::from(__go_string_bytes(&({ let __v = (*resolved.lock().unwrap().as_ref().unwrap()).clone(); __v })))))))));
}