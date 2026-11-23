#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::path_filepath::__go_init_all();

    let (mut entries, mut err) = os::read_dir("data".to_string());
    let mut entries = { let __slice_header = GoSliceHeader::go_slice_header(&entries); Rc::new(RefCell::new(__slice_header)) };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{}", format!("{}", "read error".to_string()));
        return;
    }
    { let __range_holder = entries.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for entry in __range_values.iter() {
        let mut joined = path_filepath::join(Rc::new(RefCell::new(Some(vec!["data".to_string(), (*entry.name().borrow().as_ref().unwrap()).clone()]))));
        println!("{} {} {} {}", format!("{}", "entry".to_string()), format!("{}", { let __v = (*joined.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*path_filepath::base(Rc::new(RefCell::new(Some({ let __arg_holder = joined.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap())), format!("{}", entry.is_dir()));
    } }
    let (mut info, __tmp_1) = os::stat("data/nested".to_string()); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1;;
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{}", format!("{}", "stat error".to_string()));
        return;
    }
    println!("{} {}", format!("{}", "nested".to_string()), format!("{}", (*info.borrow().as_ref().unwrap()).is_dir()));
}