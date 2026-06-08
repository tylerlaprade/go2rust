use go2rust_stdlib_stubs::*;

use crate::{r#mod::{DirEntry, FileMode}};

use std::sync::{Arc, Mutex};

/// FormatDirEntry returns a formatted version of dir for human readability.
/// Implementations of [DirEntry] can call this from a String method.
/// The outputs for a directory named subdir and a file named hello.go are:
///
///	d subdir/
///	- hello.go
pub fn format_dir_entry(dir: Arc<Mutex<Option<Box<dyn DirEntry + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let mut name = (*dir.lock().unwrap().as_ref().unwrap()).name();
    let mut b = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __tmp_x = 5; let __tmp_y = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y }) as usize))));

        // The Type method does not return any permission bits,
        // so strip them from the string.
    let mut mode = crate::r#mod::FileMode::string(&(*(*dir.lock().unwrap().as_ref().unwrap()).r#type().lock().unwrap().as_ref().unwrap()));
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*mode.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = ((*mode.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 9; __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *mode.lock().unwrap() = __moved_val; };

    { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*mode.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
    { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((' ' as i32) as u8); __append_target.clone() }; b = new_val; };
    { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*name.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; b = new_val; };
    if (*dir.lock().unwrap().as_ref().unwrap()).is_dir() {
        { let new_val = { let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('/' as i32) as u8); __append_target.clone() }; b = new_val; };
    }
    return Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
}