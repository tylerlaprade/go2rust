use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{metrics::{metricFloat64Histogram, metricValue, timeHistBuckets}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const TIME_HIST_MIN_BUCKET_BITS: i32 = 9;
pub(crate) const TIME_HIST_MAX_BUCKET_BITS: i32 = 48;
pub(crate) const TIME_HIST_SUB_BUCKET_BITS: i32 = 2;
pub(crate) const TIME_HIST_NUM_SUB_BUCKETS: i32 = 1 << TIME_HIST_SUB_BUCKET_BITS;
pub(crate) const TIME_HIST_NUM_BUCKETS: i32 = TIME_HIST_MAX_BUCKET_BITS - TIME_HIST_MIN_BUCKET_BITS + 1;
pub(crate) const TIME_HIST_TOTAL_BUCKETS: i32 = TIME_HIST_NUM_BUCKETS * TIME_HIST_NUM_SUB_BUCKETS + 2;


pub(crate) const F_INF: i64 = 0x7FF0000000000000;
pub(crate) const F_NEG_INF: u64 = 0xFFF0000000000000;


/// timeHistogram represents a distribution of durations in
/// nanoseconds.
///
/// The accuracy and range of the histogram is defined by the
/// timeHistSubBucketBits and timeHistNumBuckets constants.
///
/// It is an HDR histogram with exponentially-distributed
/// buckets and linearly distributed sub-buckets.
///
/// The histogram is safe for concurrent reads and writes.
#[derive(Clone)]
pub struct timeHistogram {
    pub counts: Arc<Mutex<Option<[internal_runtime_atomic::types::Uint64; 160]>>>,
    pub underflow: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub overflow: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
}

impl timeHistogram {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.counts.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.underflow.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.overflow.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            counts: __go_clone_0_0,
            underflow: __go_clone_1_0,
            overflow: __go_clone_2_0,
        }
    }
}


impl Default for timeHistogram {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            counts: __go_default_0_0,
            underflow: __go_default_1_0,
            overflow: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for timeHistogram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.counts));
        let __go_fmt_1 = format!("{}", (*self.underflow.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.overflow.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl timeHistogram {
    /// record adds the given duration to the distribution.
    ///
    /// Disallow preemptions and stack growths because this function
    /// may run in sensitive locations.
    ///
    ///go:nosplit
    pub fn record(&self, duration: Arc<Mutex<Option<i64>>>) {
                // If the duration is negative, capture that in underflow.
        if { let __tmp_x = { let __v = (*duration.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        (*self.underflow.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i64))));
        return;
    }
                // bucketBit is the target bit for the bucket which is usually the
                // highest 1 bit, but if we're less than the minimum, is the highest
                // 1 bit of the minimum (which will be zero in the duration).
                //
                // bucket is the bucket index, which is the bucketBit minus the
                // highest bit of the minimum, plus one to leave room for the catch-all
                // bucket for samples lower than the minimum.
        let mut bucketBit: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut bucket: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        {
        let mut l = internal_runtime_sys::len64(Arc::new(Mutex::new(Some((*duration.lock().unwrap().as_ref().unwrap()) as u64))));;
        if { let __tmp_x = l; let __tmp_y = 9; __tmp_x < __tmp_y } {
            { let new_val = TIME_HIST_MIN_BUCKET_BITS as u64; *bucketBit.lock().unwrap() = Some(new_val); };;
            { let new_val = 0 as u64; *bucket.lock().unwrap() = Some(new_val); };;
        } else {
            { let new_val = Arc::new(Mutex::new(Some(l as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *bucketBit.lock().unwrap() = __moved_val; };;
            { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*bucketBit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TIME_HIST_MIN_BUCKET_BITS as u64; __tmp_x - __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }; *bucket.lock().unwrap() = Some(new_val); };;
        }
    }
                // bucketBit - timeHistMinBucketBits
                // If the bucket we computed is greater than the number of buckets,
                // count that in overflow.
        if { let __tmp_x = { let __v = (*bucket.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TIME_HIST_NUM_BUCKETS as u64; __tmp_x >= __tmp_y } {
        (*self.overflow.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i64))));
        return;
    }
                // The sub-bucket index is just next timeHistSubBucketBits after the bucketBit.
        let mut subBucket = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*duration.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __tmp_x = { let __v = (*bucketBit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }; let __tmp_y = TIME_HIST_SUB_BUCKET_BITS as u64; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = TIME_HIST_NUM_SUB_BUCKETS as u64; __tmp_x % __tmp_y })));
        {
            let mut __recv = {
                let __seq = { let __seq_holder = self.counts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*bucket.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TIME_HIST_NUM_SUB_BUCKETS as u64; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*subBucket.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone()
            };
            let __result = __recv.add(
                Arc::new(Mutex::new(Some(1 as i64))),
            );
            __result
        };
    }

    /// write dumps the histogram to the passed metricValue as a float64 histogram.
    pub fn write(&self, out: Arc<Mutex<Option<metricValue>>>) {
        let mut hist: GoPtr<crate::metrics::metricFloat64Histogram> = { let __recv = out.clone(); let __recv_ptr: *mut crate::metrics::metricValue = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::metrics::metricValue }; let __result = unsafe { &mut *__recv_ptr }.float64_hist_or_init(timeHistBuckets.clone()); __result };
                // The bottom-most bucket, containing negative values, is tracked
                // separately as underflow, so fill that in manually and then iterate
                // over the rest.
        (*{ let __ptr_value = hist.with_mut(|__ptr_value| __ptr_value.counts.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*self.underflow.lock().unwrap().as_mut().unwrap()).load();
        for i in 0..(({ let __range_holder = self.counts.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*{ let __ptr_value = hist.with_mut(|__ptr_value| __ptr_value.counts.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] = {
            let mut __recv = {
                let __seq = { let __seq_holder = self.counts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[(i) as usize].clone()
            };
            let __result = __recv.load();
            __result
        };
    }
        (*{ let __ptr_value = hist.with_mut(|__ptr_value| __ptr_value.counts.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = hist.with_mut(|__ptr_value| __ptr_value.counts.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = (*self.overflow.lock().unwrap().as_mut().unwrap()).load();
    }
}

impl GoValueClone for timeHistogram {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
