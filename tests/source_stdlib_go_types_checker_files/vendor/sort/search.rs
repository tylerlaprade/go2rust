use go2rust_stdlib_stubs::*;

use crate::slice::*;
use crate::r#mod::*;
use crate::zsortfunc::*;
use crate::zsortinterface::*;

use std::sync::{Arc, Mutex};

/// Search uses binary search to find and return the smallest index i
/// in [0, n) at which f(i) is true, assuming that on the range [0, n),
/// f(i) == true implies f(i+1) == true. That is, Search requires that
/// f is false for some (possibly empty) prefix of the input range [0, n)
/// and then true for the (possibly empty) remainder; Search returns
/// the first true index. If there is no such index, Search returns n.
/// (Note that the "not found" return value is not -1 as in, for instance,
/// strings.Index.)
/// Search calls f(i) only for i in the range [0, n).
///
/// A common use of Search is to find the index i for a value x in
/// a sorted, indexable data structure such as an array or slice.
/// In this case, the argument f, typically a closure, captures the value
/// to be searched for, and how the data structure is indexed and
/// ordered.
///
/// For instance, given a slice data sorted in ascending order,
/// the call Search(len(data), func(i int) bool { return data[i] >= 23 })
/// returns the smallest index i such that data[i] >= 23. If the caller
/// wants to find whether 23 is in the slice, it must test data[i] == 23
/// separately.
///
/// Searching data sorted in descending order would use the <=
/// operator instead of the >= operator.
///
/// To complete the example above, the following code tries to find the value
/// x in an integer slice data sorted in ascending order:
///
///	x := 23
///	i := sort.Search(len(data), func(i int) bool { return data[i] >= x })
///	if i < len(data) && data[i] == x {
///		// x is present at data[i]
///	} else {
///		// x is not present in data,
///		// but i is the index where it would be inserted.
///	}
///
/// As a more whimsical example, this program guesses your number:
///
///	func GuessingGame() {
///		var s string
///		fmt.Printf("Pick an integer from 0 to 100.\n")
///		answer := sort.Search(100, func(i int) bool {
///			fmt.Printf("Is your number <= %d? ", i)
///			fmt.Scanf("%s", &s)
///			return s != "" && s[0] == 'y'
///		})
///		fmt.Printf("Your number is %d.\n", answer)
///	}
pub fn search(n: Arc<Mutex<Option<i32>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> i32 {
        // Define f(-1) == false and f(n) == true.
        // Invariant: f(i-1) == false, f(j) == true.
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0))), { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut h = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y }) as i32)));

                // i ≤ h < j
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(h.clone()) } {
        { let new_val = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *i.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = h.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    }
    }

        // avoid overflow when computing h
        // i ≤ h < j
        // preserves f(i-1) == false
        // preserves f(j) == true
        // i == j, f(i-1) == false, and f(j) (= f(i)) == true  =>  answer is i.
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}