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
        // Create a slice
    let mut slice = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {}", format!("{}", "Original slice:".to_string()), format!("{}", format_slice(&slice)));

        // Append to slice
    { let new_val = { let __append_target = slice.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(vec![6, 7]); __append_target.clone() }; slice = new_val; };
    println!("{} {}", format!("{}", "After append:".to_string()), format!("{}", format_slice(&slice)));

        // Slice operations
    let mut subSlice = Rc::new(RefCell::new(Some({ let __seq_holder = slice.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = (4) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    println!("{} {}", format!("{}", "Sub-slice [1:4]:".to_string()), format!("{}", format_slice(&subSlice)));

        // Length and capacity
    println!("{} {}", format!("{}", "Length:".to_string()), format!("{}", (*slice.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    println!("{} {}", format!("{}", "Capacity:".to_string()), format!("{}", (*slice.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)));

        // Make slice
    let mut made = Rc::new(RefCell::new(Some({ let mut v = Vec::with_capacity((5) as usize); v.resize((3) as usize, 0); v })));
    (*made.borrow_mut().as_mut().unwrap())[(0) as usize] = 10;
    (*made.borrow_mut().as_mut().unwrap())[(1) as usize] = 20;
    (*made.borrow_mut().as_mut().unwrap())[(2) as usize] = 30;
    let mut idx = Rc::new(RefCell::new(Some(1)));
    (*made.borrow_mut().as_mut().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize] = 25;
    println!("{} {}", format!("{}", "Made slice:".to_string()), format!("{}", format_slice(&made)));
}