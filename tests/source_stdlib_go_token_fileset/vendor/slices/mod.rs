use go2rust_stdlib_stubs::*;

use crate::iter::*;
use crate::sort::*;
use crate::zsortanyfunc::*;
use crate::zsortordered::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

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