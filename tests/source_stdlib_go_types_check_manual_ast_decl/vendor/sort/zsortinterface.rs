use go2rust_stdlib_stubs::*;

use crate::search::*;
use crate::slice::*;
use crate::r#mod::*;
use crate::zsortfunc::*;

use std::sync::{Arc, Mutex};

/// insertionSort sorts data[a:b] using insertion sort.
pub fn insertion_sort(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = i.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } && (*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))) {
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

/// siftDown implements the heap property on data[lo:hi].
/// first is an offset into the array where the root of the heap lies.
pub fn sift_down(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, lo: Arc<Mutex<Option<i32>>>, hi: Arc<Mutex<Option<i32>>>, first: Arc<Mutex<Option<i32>>>) {
    let mut root = { let __owned = lo.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    loop {
        let mut child = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && (*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })))) {
        { let mut guard = child.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !(*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))) {
        return;
    }
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*root.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*child.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
        { let new_val = child.lock().unwrap().as_ref().unwrap().clone(); *root.lock().unwrap() = Some(new_val); };
    }
}

pub fn heap_sort(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) {
    let mut first = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut lo = Arc::new(Mutex::new(Some(0)));
    let mut hi = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));

        // Build heap with greatest element at top.
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = 2; __tmp_x / __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        sift_down(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

        // Pop elements, largest first, into end of data.
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
        sift_down(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

/// pdqsort sorts data[a:b].
/// The algorithm based on pattern-defeating quicksort(pdqsort), but without the optimizations from BlockQuicksort.
/// pdqsort paper: https://arxiv.org/pdf/2106.05123.pdf
/// C++ implementation: https://github.com/orlp/pdqsort
/// Rust implementation: https://docs.rs/pdqsort/latest/pdqsort/
/// limit is the number of allowed bad (very unbalanced) pivots before falling back to heapsort.
pub fn pdqsort(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, mut a: Arc<Mutex<Option<i32>>>, mut b: Arc<Mutex<Option<i32>>>, mut limit: Arc<Mutex<Option<i32>>>) {
    const maxInsertion: i32 = 12;


    let mut wasBalanced = Arc::new(Mutex::new(Some(true)));let mut wasPartitioned = Arc::new(Mutex::new(Some(true)));

        // whether the last partitioning was reasonably balanced
        // whether the slice was already partitioned
    loop {
        let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));

        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12; __tmp_x <= __tmp_y } {
        insertion_sort(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }

                // Fall back to heapsort if too many bad choices were made.
        if { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        heap_sort(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }

                // If the last partitioning was imbalanced, we need to breaking patterns.
        if !{ let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        break_patterns(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

        let (mut pivot, mut hint) = choose_pivot(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*hint.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::r#mod::sortedHint(Arc::new(Mutex::new(Some(DECREASING_HINT as i32)))); __tmp_x == __tmp_y } {
        reverse_range(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // The chosen pivot was pivot-a elements after the start of the array.
                // After reversing it is pivot-a elements before the end of the array.
                // The idea came from Rust's implementation.
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = pivot; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; pivot = new_val; };
        { let new_val = crate::r#mod::sortedHint(Arc::new(Mutex::new(Some(INCREASING_HINT as i32)))); *hint.lock().unwrap() = Some(new_val); };
    }

                // The chosen pivot was pivot-a elements after the start of the array.
                // After reversing it is pivot-a elements before the end of the array.
                // The idea came from Rust's implementation.
                // The slice is likely already sorted.
        if { let __v = (*wasBalanced.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*wasPartitioned.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*hint.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::r#mod::sortedHint(Arc::new(Mutex::new(Some(INCREASING_HINT as i32)))); __tmp_x == __tmp_y } {
        if partial_insertion_sort(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }

                // Probably the slice contains many duplicate elements, partition the slice into
                // elements equal to and elements greater than the pivot.
        if { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && !(*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some(pivot)))) {
        let mut mid = partition_equal(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(pivot))));
        { let new_val = mid; *a.lock().unwrap() = Some(new_val); };
        continue
    }

        let (mut mid, mut alreadyPartitioned) = partition(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(pivot))));
        { let new_val = alreadyPartitioned; *wasPartitioned.lock().unwrap() = Some(new_val); };

        let (mut leftLen, mut rightLen) = (Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mid; __tmp_x - __tmp_y }))));
        let mut balanceThreshold = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*leftLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        pdqsort(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(mid))), Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }; *a.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*rightLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*balanceThreshold.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }; *wasBalanced.lock().unwrap() = Some(new_val); };
        pdqsort(data.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = mid; let __tmp_y = 1; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid; *b.lock().unwrap() = Some(new_val); };
    }
    }
}

/// partition does one quicksort partition.
/// Let p = data[pivot]
/// Moves elements in data[a:b] around, so that data[i]<p and data[j]>=p for i<newpivot and j>newpivot.
/// On return, data[newpivot] = p
pub fn partition(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>, pivot: Arc<Mutex<Option<i32>>>) -> (i32, bool) {
    let mut newpivot: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyPartitioned: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pivot.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut i, mut j) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))));

    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && (*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && !(*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
    (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }

    loop {
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && (*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && !(*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, false);
}

/// partitionEqual partitions data[a:b] into elements equal to data[pivot] followed by elements greater than data[pivot].
/// It assumed that data[a:b] does not contain elements smaller than the data[pivot].
pub fn partition_equal(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>, pivot: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut newpivot: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pivot.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut i, mut j) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))));

    loop {
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && !(*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && (*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        break
    }
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// partialInsertionSort partially sorts a slice, returns true if the slice is sorted at the end.
pub fn partial_insertion_sort(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> bool {
    const maxSteps: i32 = 5;
const shortestShifting: i32 = 50;

        // maximum number of adjacent out-of-order pairs that will get shifted
        // don't shift any elements on short arrays
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } {
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && !(*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))) {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return true;
    }

        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return false;
    }

        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))));

                // Shift the smaller one to the left.
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        if !(*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))) {
        break
    }
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }

                // Shift the greater one to the right.
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !(*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))) {
        break
    }
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // Shift the smaller one to the left.
        // Shift the greater one to the right.
    false
}

/// breakPatterns scatters some elements around in an attempt to break some patterns
/// that might cause imbalanced partitions in quicksort.
pub fn break_patterns(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) {
    let mut length = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        let mut random = Arc::new(Mutex::new(Some(crate::r#mod::xorshift(Arc::new(Mutex::new(Some((*length.lock().unwrap().as_ref().unwrap()) as u64)))))));
        let mut modulus = next_power_of_two(Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }); let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; __tmp_x <= __tmp_y } {
        let mut other = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*random.lock().unwrap().as_mut().unwrap()).next() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = modulus; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }) as i32)));
        if { let __tmp_x = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let __rhs = (*length.lock().unwrap().as_ref().unwrap()); let mut guard = other.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = idx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
        { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

/// choosePivot chooses a pivot in data[a:b].
///
/// [0,8): chooses a static pivot.
/// [8,shortestNinther): uses the simple median-of-three method.
/// [shortestNinther,∞): uses the Tukey ninther method.
pub fn choose_pivot(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<crate::r#mod::sortedHint>>>) {
    let mut pivot: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut hint: Arc<Mutex<Option<sortedHint>>> = Arc::new(Mutex::new(Some(Default::default())));

    const shortestNinther: i32 = 50;
const maxSwaps: i32 = 4 * 3;


    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));

    let mut swaps: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 1; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 2; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x / __tmp_y }; let __tmp_y = 3; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));

    if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 50; __tmp_x >= __tmp_y } {
                // Tukey ninther method, the idea came from Rust's implementation.
        { let new_val = median_adjacent(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.lock().unwrap() = Some(new_val); };
        { let new_val = median_adjacent(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.lock().unwrap() = Some(new_val); };
        { let new_val = median_adjacent(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.lock().unwrap() = Some(new_val); };
    }
                // Tukey ninther method, the idea came from Rust's implementation.
                // Find the median among i, j, k and stores it into j.
        { let new_val = median(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.lock().unwrap() = Some(new_val); };
    }

        // Tukey ninther method, the idea came from Rust's implementation.
        // Find the median among i, j, k and stores it into j.
    { let _switch_val = { let __v = (*swaps.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(crate::r#mod::sortedHint(Arc::new(Mutex::new(Some(INCREASING_HINT as i32))))))));
        } else if _switch_val == (12) {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(crate::r#mod::sortedHint(Arc::new(Mutex::new(Some(DECREASING_HINT as i32))))))));
        } else {
            return ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(crate::r#mod::sortedHint(Arc::new(Mutex::new(Some(UNKNOWN_HINT as i32))))))));
        }
    }
}

/// order2 returns x,y where data[x] <= data[y], where x,y=a,b or x,y=b,a.
pub fn order2(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>, swaps: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
    if (*data.lock().unwrap().as_ref().unwrap()).less(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let mut guard = swaps.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    return ({ let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

/// median returns x where data[x] is the median of data[a],data[b],data[c], where x is a, b, or c.
pub fn median(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, mut a: Arc<Mutex<Option<i32>>>, mut b: Arc<Mutex<Option<i32>>>, mut c: Arc<Mutex<Option<i32>>>, swaps: Arc<Mutex<Option<i32>>>) -> i32 {
    { let (__tmp_0, __tmp_1) = order2(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = order2(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *b.lock().unwrap() = Some(__tmp_0); *c.lock().unwrap() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = order2(data.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.lock().unwrap() = Some(__tmp_0); *b.lock().unwrap() = Some(__tmp_1); };
    return { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// medianAdjacent finds the median of data[a - 1], data[a], data[a + 1] and stores the index into a.
pub fn median_adjacent(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, swaps: Arc<Mutex<Option<i32>>>) -> i32 {
    median(data.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), swaps.clone())
}

pub fn reverse_range(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) {
    let mut i = { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*data.lock().unwrap().as_ref().unwrap()).swap(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}