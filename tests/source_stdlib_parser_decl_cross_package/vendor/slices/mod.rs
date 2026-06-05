use go2rust_stdlib_stubs::*;

use crate::iter::*;
use crate::sort::*;
use crate::zsortanyfunc::*;
use crate::zsortordered::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

/// Insert inserts the values v... into s at index i,
/// returning the modified slice.
/// The elements at s[i:] are shifted up to make room.
/// In the returned slice r, r[i] == v[0],
/// and, if i < len(s), r[i+len(v)] == value originally at r[i].
/// Insert panics if i > len(s).
/// This function is O(len(s) + len(v)).
pub fn insert<S, E: Any + GoValueClone + Send + Sync + 'static>(mut s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, i: Arc<Mutex<Option<i32>>>, v: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>> {
    let _ = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })));

    let mut m = Arc::new(Mutex::new(Some((*v.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return s.clone();
    }
    let mut n = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __append_target = s.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = v.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
    }
    if { let __tmp_x = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*s.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); __tmp_x > __tmp_y } {
                // Use append rather than make so that we bump the size of
                // the slice up to the next storage class.
                // This is what Grow does but we don't call Grow because
                // that might copy the values twice.
        let mut s2 = { let __append_target = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some(vec![Default::default(); ({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
        { let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = (*s2.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*s2.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let _dst_start = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let _dst_len = (*s2.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*s2.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        return s2.clone();
    }
        // Use append rather than make so that we bump the size of
        // the slice up to the next storage class.
        // This is what Grow does but we don't call Grow because
        // that might copy the values twice.
    { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __seq[..__high].to_vec() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };

        // before:
        // s: aaaaaaaabbbbccccccccdddd
        //            ^   ^       ^   ^
        //            i  i+m      n  n+m
        // after:
        // s: aaaaaaaavvvvbbbbcccccccc
        //            ^   ^       ^   ^
        //            i  i+m      n  n+m
        //
        // a are the values that don't move in s.
        // v are the values copied in from v.
        // b and c are the values from s that are shifted up in index.
        // d are the values that get overwritten, never to be seen again.
    if !overlaps::<E>(v.clone(), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __seq[__low..].to_vec() })))) {
                // Easy case - v does not overlap either the c or d regions.
                // (It might be in some of a or b, or elsewhere entirely.)
                // The data we copy up doesn't write to v at all, so just do it.
        { let _dst_start = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let _dst_len = (*s.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*s.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
                // Now we have
                // s: aaaaaaaabbbbbbbbcccccccc
                //            ^   ^       ^   ^
                //            i  i+m      n  n+m
                // Note the b values are duplicated.
        { let _dst_start = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = (*s.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*s.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
                // Now we have
                // s: aaaaaaaavvvvbbbbcccccccc
                //            ^   ^       ^   ^
                //            i  i+m      n  n+m
                // That's the result we want.
        return s.clone();
    }

        // Easy case - v does not overlap either the c or d regions.
        // (It might be in some of a or b, or elsewhere entirely.)
        // The data we copy up doesn't write to v at all, so just do it.
        // Now we have
        // s: aaaaaaaabbbbbbbbcccccccc
        //            ^   ^       ^   ^
        //            i  i+m      n  n+m
        // Note the b values are duplicated.
        // Now we have
        // s: aaaaaaaavvvvbbbbcccccccc
        //            ^   ^       ^   ^
        //            i  i+m      n  n+m
        // That's the result we want.
        // The hard case - v overlaps c or d. We can't just shift up
        // the data because we'd move or clobber the values we're trying
        // to insert.
        // So instead, write v on top of d, then rotate.
    { let _dst_start = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = (*s.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = { let __copy_src_holder = v.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*s.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };

        // Now we have
        // s: aaaaaaaabbbbccccccccvvvv
        //            ^   ^       ^   ^
        //            i  i+m      n  n+m
    rotate_right::<E>(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))), Arc::new(Mutex::new(Some({ let __arg_holder = m.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Now we have
        // s: aaaaaaaavvvvbbbbcccccccc
        //            ^   ^       ^   ^
        //            i  i+m      n  n+m
        // That's the result we want.
    s.clone()
}

/// Delete removes the elements s[i:j] from s, returning the modified slice.
/// Delete panics if j > len(s) or s[i:j] is not a valid slice of s.
/// Delete is O(len(s)-i), so if many items must be deleted, it is better to
/// make a single call deleting them all together than to delete one at a time.
/// Delete zeroes the elements s[len(s)-(j-i):len(s)].
pub fn delete<S, E: Any + GoValueClone + Send + Sync + 'static>(mut s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>> {
    let _ = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __max = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; let _slice = &__seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]; let mut _v = Vec::with_capacity(((__max) - (({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)) as usize); _v.extend_from_slice(_slice); _v })));

    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return s.clone();
    }

    let mut oldlen = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    { let __append_target = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
    unimplemented!("clear requires map or slice type");
    s.clone()
}

/// rotateLeft rotates s left by r spaces.
/// s_final[i] = s_orig[i+r], wrapping around.
pub fn rotate_left<E: Any + GoValueClone + Send + Sync + 'static>(s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, r: Arc<Mutex<Option<i32>>>) {
    reverse::<Vec<Arc<Mutex<Option<E>>>>, E>(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))));
    reverse::<Vec<Arc<Mutex<Option<E>>>>, E>(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))));
    reverse::<Vec<Arc<Mutex<Option<E>>>>, E>(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

pub fn rotate_right<E: Any + GoValueClone + Send + Sync + 'static>(s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, r: Arc<Mutex<Option<i32>>>) {
    rotate_left::<E>(s.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y }))));
}

/// overlaps reports whether the memory ranges a[:len(a)] and b[:len(b)] overlap.
pub fn overlaps<E: Any + GoValueClone + Send + Sync + 'static>(a: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, b: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>) -> bool {
    if { let __tmp_x = ((*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
    let mut elemSize = Arc::new(Mutex::new(Some(std::mem::size_of::<E>())));
    if { let __tmp_x = { let __v = (*elemSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return false;
    }

        // TODO: use a runtime/unsafe facility once one becomes available. See issue 12445.
        // Also see crypto/internal/fips140/alias/alias.go:AnyOverlap
    return { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __v = (*elemSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __tmp_x = ((*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __v = (*elemSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x + __tmp_y }; __tmp_x <= __tmp_y };
}

/// Reverse reverses the elements of the slice in place.
pub fn reverse<S, E: Any + GoValueClone + Send + Sync + 'static>(s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>) {
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }))));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __tmp_0 = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_1 = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_0; (*s.lock().unwrap().as_mut().unwrap())[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_1; };
        { let __tmp_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_1 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; *i.lock().unwrap() = Some(__tmp_0); *j.lock().unwrap() = Some(__tmp_1); };
    }
}