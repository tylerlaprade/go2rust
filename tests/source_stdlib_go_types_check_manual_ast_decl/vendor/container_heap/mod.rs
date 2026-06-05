use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

/// The Interface type describes the requirements
/// for a type using the routines in this package.
/// Any type that implements it may be used as a
/// min-heap with the following invariants (established after
/// [Init] has been called or if the data is empty or sorted):
///
///	!h.Less(j, i) for 0 <= i < h.Len() and 2*i+1 <= j <= 2*i+2 and j < h.Len()
///
/// Note that [Push] and [Pop] in this interface are for package heap's
/// implementation to call. To add and remove things from the heap,
/// use [heap.Push] and [heap.Pop].
pub trait Interface: sort::r#mod::Interface + std::fmt::Display + Any {
    fn __go_clone_box_interface(&self) -> Box<dyn Interface + Send + Sync>;
    fn __go_eq_interface(&self, other: &(dyn Interface + Send + Sync)) -> bool;
    fn push(&self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>);
    fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>;
}

impl Clone for Box<dyn Interface + Send + Sync> {
    fn clone(&self) -> Self {
        Interface::__go_clone_box_interface(self.as_ref())
    }
}

/// Init establishes the heap invariants required by the other routines in this package.
/// Init is idempotent with respect to the heap invariants
/// and may be called whenever the heap invariants may have been invalidated.
/// The complexity is O(n) where n = h.Len().
pub fn init(h: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>) {
        // heapify
    let mut n = (*h.lock().unwrap().as_ref().unwrap()).len();
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = n; let __tmp_y = 2; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        down(h.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(n))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

/// Pop removes and returns the minimum element (according to Less) from the heap.
/// The complexity is O(log n) where n = h.Len().
/// Pop is equivalent to [Remove](h, 0).
pub fn pop(h: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*h.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 1; __tmp_x - __tmp_y })));
    (*h.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    down(h.clone(), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    (*h.lock().unwrap().as_mut().unwrap()).pop().clone()
}

/// Fix re-establishes the heap ordering after the element at index i has changed its value.
/// Changing the value of the element at index i and then calling Fix is equivalent to,
/// but less expensive than, calling [Remove](h, i) followed by a Push of the new value.
/// The complexity is O(log n) where n = h.Len().
pub fn fix(h: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, i: Arc<Mutex<Option<i32>>>) {
    if !down(h.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_value = (*h.lock().unwrap().as_ref().unwrap()).len(); __arg_value })))) {
        up(h.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

pub fn up(h: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, mut j: Arc<Mutex<Option<i32>>>) {
    loop {
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } || !(*h.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        break
    }
        (*h.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    }
}

pub fn down(h: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, i0: Arc<Mutex<Option<i32>>>, n: Arc<Mutex<Option<i32>>>) -> bool {
    let mut i = { let __owned = i0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut j1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*j1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } || { let __tmp_x = { let __v = (*j1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        break
    }
        let mut j = { let __owned = j1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        {
        let mut j2 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));;
        if { let __tmp_x = { let __v = (*j2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && (*h.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = j2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            { let new_val = j2.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };;
        }
    }
                // = 2*i + 2  // right child
        if !(*h.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        break
    }
        (*h.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = j.lock().unwrap().as_ref().unwrap().clone(); *i.lock().unwrap() = Some(new_val); };
    }
        // j1 < 0 after int overflow
        // left child
        // = 2*i + 2  // right child
    return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y };
}