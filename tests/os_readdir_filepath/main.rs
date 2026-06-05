use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    internal_filepathlite::__go_init_all();
    internal_stringslite::__go_init_all();
    path_filepath::__go_init_all();

    let (mut entries, mut err) = os::read_dir("data".to_string());
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "read error".to_string()));
        return;
    }
    { let __range_holder = entries.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for entry in __range_values.iter() {
        let mut joined = path_filepath::join(Arc::new(Mutex::new(Some(vec!["data".to_string(), (*entry.name().lock().unwrap().as_ref().unwrap()).clone()]))));
        println!("{} {} {} {}", format!("{}", "entry".to_string()), format!("{}", { let __v = (*joined.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*path_filepath::base(Arc::new(Mutex::new(Some({ let __arg_holder = joined.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", entry.is_dir()));
    } }
    let (mut info, __tmp_1) = os::stat("data/nested".to_string()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "stat error".to_string()));
        return;
    }
    println!("{} {}", format!("{}", "nested".to_string()), format!("{}", (*info.lock().unwrap().as_ref().unwrap()).is_dir()));
}