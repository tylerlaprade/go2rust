use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoScanError,
    GoScanRuneFns,
    GoScanRunes,
    GoScanTarget,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    GoWrapError,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_error_chain,
    go_error_wrap,
    go_errors_is,
    go_errors_join,
    go_errors_unwrap,
    go_float_string,
    go_fmt_scan,
    go_scan_stdin_source,
    go_scan_string_source,
};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// readPair scans two ints; its file does not import io.
pub fn read_pair(s: Arc<Mutex<Option<String>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    let mut x: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut y: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let (_, mut err) = { let mut __scan_src = go_scan_string_source((*s.lock().unwrap().as_ref().unwrap()).clone()); let __scan_format: String = String::new(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = x.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) }), GoScanTarget::Int(64, { let __scan_h = y.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 's', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) };
    return (
        { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y },
        { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }
    );
}