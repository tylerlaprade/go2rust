use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

/// Delete removes the elements s[i:j] from s, returning the modified slice.
/// Delete panics if j > len(s) or s[i:j] is not a valid slice of s.
/// Delete is O(len(s)-i), so if many items must be deleted, it is better to
/// make a single call deleting them all together than to delete one at a time.
/// Delete zeroes the elements s[len(s)-(j-i):len(s)].
pub fn delete<S, E: Any + GoValueClone + Send + Sync + 'static>(mut s: Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>>, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<E>>>>>>> {
    let _ = Arc::new(Mutex::new(Some({
        let mut __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
        let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __max = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    })));

    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return s.clone();
    }

    let mut oldlen = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    { let __append_target = Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
        drop(__seq_guard);
        let __low = 0;
        let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __max = __source_cap;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some({
        let __seq_holder = s.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
        drop(__seq_guard);
        let __low = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
        let __high = __seq.len();
        let __max = __source_cap;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
    { let __clear_start = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; let __clear_end = ({ let __v = (*oldlen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __clear_holder = s.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = Default::default(); } } };
    s.clone()
}