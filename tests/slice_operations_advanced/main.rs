use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn main() {
        // Slice with capacity
    let mut s = Rc::new(RefCell::new(Some({ let mut v = Vec::with_capacity((10) as usize); v.resize((3) as usize, 0); v })));
    print!("len={} cap={} {}\n", (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0), (*s.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0), format_slice(&s));

        // Append beyond capacity
    { let new_val = { let __append_target = s.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(vec![1, 2, 3, 4, 5, 6, 7, 8]); __append_target.clone() }; s = new_val; };
    print!("len={} cap={} {}\n", (*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0), (*s.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0), format_slice(&s));

        // Three-index slice
    let mut s2 = Rc::new(RefCell::new(Some({ let mut __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = (2) as usize; let __high = (5) as usize; let __max = (7) as usize; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    print!("s2: len={} cap={} {}\n", (*s2.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0), (*s2.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0), format_slice(&s2));

        // Copy
    let mut s3 = Rc::new(RefCell::new(Some(vec![0; (3) as usize])));
    let mut n = {
        let _src = { let __copy_src_holder = s.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
        let _n = std::cmp::min((*s3.borrow().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*s3.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone();
        }
        Rc::new(RefCell::new(Some(_n as i32)))
    };
    print!("Copied {} elements: {}\n", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }, format_slice(&s3));

        // Nil slice vs empty slice
    let mut s4: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(None));
    let mut s5 = Rc::new(RefCell::new(Some(Vec::<i32>::new())));
    print!("s4==nil: {}, s5==nil: {}\n", { let __nil_result = (*s4.borrow()).is_none(); __nil_result }, { let __nil_result = (*s5.borrow()).is_none(); __nil_result });
}