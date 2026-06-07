use go2rust_stdlib_stubs::*;

use crate::iter::*;
use crate::sort::*;
use crate::zsortanyfunc::*;
use crate::zsortordered::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

/// EqualFunc reports whether two slices are equal using an equality
/// function on each pair of elements. If the lengths are different,
/// EqualFunc returns false. Otherwise, the elements are compared in
/// increasing index order, and the comparison stops at the first index
/// for which eq returns false.
pub fn equal_func<S1, S2, E1: Any + GoValueClone + Send + Sync + 'static, E2: Any + GoValueClone + Send + Sync + 'static>(s1: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E1>>>>>>>, s2: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E2>>>>>>>, eq: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<E1>>>, Arc<Mutex<Option<E2>>>) -> bool + Send + Sync>>>>) -> bool {
    if { let __tmp_x = ((*s1.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*s2.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        return false;
    }
    { let __range_holder = s1.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, v1) in __range_values.iter().enumerate() {
        let mut v2 = { let __seq = { let __seq_holder = s2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() };
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<E1>>>, Arc<Mutex<Option<E2>>>) -> bool + Send + Sync> = { let mut __f_guard = eq.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<E1>>>, Arc<Mutex<Option<E2>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*v1).clone(), v2.clone()) } {
        return false;
    }
    } }
    true
}

/// Index returns the index of the first occurrence of v in s,
/// or -1 if not present.
pub fn index<S, E: Any + GoComparable + GoValueClone + Send + Sync + 'static>(s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, v: Arc<Mutex<Option<E>>>) -> i32 {
    for i in 0..(({ let __range_holder = s.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __left = v.clone(); let __right = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq } {
        return i as i32;
    }
    }
    -(1)
}

/// Contains reports whether v is present in s.
pub fn contains<S, E: Any + GoComparable + GoValueClone + Send + Sync + 'static>(s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, v: Arc<Mutex<Option<E>>>) -> bool {
    return { let __tmp_x = index::<S, E>(s.clone(), v.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y };
}

/// Delete removes the elements s[i:j] from s, returning the modified slice.
/// Delete panics if j > len(s) or s[i:j] is not a valid slice of s.
/// Delete is O(len(s)-i), so if many items must be deleted, it is better to
/// make a single call deleting them all together than to delete one at a time.
/// Delete zeroes the elements s[len(s)-(j-i):len(s)].
pub fn delete<S, E: Any + GoValueClone + Send + Sync + 'static>(mut s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>> {
    let _ = Arc::new(Mutex::new(Some({ let mut __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));

    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return s.clone();
    }

    let mut oldlen = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    { let __append_target = Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
    unimplemented!("clear requires map or slice type");
    s.clone()
}