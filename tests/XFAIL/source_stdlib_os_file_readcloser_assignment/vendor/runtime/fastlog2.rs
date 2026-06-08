use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{fastlog2table::{FASTLOG_NUM_BITS, fastlog2Table}, float::{float64bits}};

use std::sync::{Arc, Mutex};

/// fastlog2 implements a fast approximation to the base 2 log of a
/// float64. This is used to compute a geometric distribution for heap
/// sampling, without introducing dependencies into package math. This
/// uses a very rough approximation using the float64 exponent and the
/// first 25 bits of the mantissa. The top 5 bits of the mantissa are
/// used to load limits from a table of constants and the rest are used
/// to scale linearly between them.
pub fn fastlog2(x: Arc<Mutex<Option<f64>>>) -> f64 {
    const fastlogScaleBits: i32 = 20;

    const fastlogScaleRatio: f64 = 1.0 / 1.048576e+06;


    let mut xBits = float64bits(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Extract the exponent from the IEEE float64, and index a constant
        // table with the first 10 bits from the mantissa.
    let mut xExp = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = xBits; let __tmp_y = 52; __tmp_x >> __tmp_y }); let __tmp_y = 0x7FF as u64; __tmp_x & __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1023 as i64; __tmp_x - __tmp_y })));
    let mut xManIndex = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = xBits; let __tmp_y = ({ let __tmp_x = 52; let __tmp_y = FASTLOG_NUM_BITS; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = ((1 as u64) << (FASTLOG_NUM_BITS as u64)) as u64; __tmp_x % __tmp_y })));
    let mut xManScale = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = xBits; let __tmp_y = ({ let __tmp_x = { let __tmp_x = 52; let __tmp_y = FASTLOG_NUM_BITS; __tmp_x - __tmp_y }; let __tmp_y = fastlogScaleBits; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = ((1 as u64) << (fastlogScaleBits as u64)) as u64; __tmp_x % __tmp_y })));

    let (mut low, mut high) = (Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = fastlog2Table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*xManIndex.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = fastlog2Table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*xManIndex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }) as usize].clone() }))));
    return { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*xExp.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*low.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*high.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*low.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*xManScale.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let __tmp_y = fastlogScaleRatio as f64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y };
}