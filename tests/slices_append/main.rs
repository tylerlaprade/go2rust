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
    let mut s: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    { let new_val = { let __append_target = s.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push("a".to_string()); __append_target.clone() }; s = new_val; };
    { let new_val = { let __append_target = s.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(vec!["b".to_string(), "c".to_string()]); __append_target.clone() }; s = new_val; };
    println!("{} {}", format!("{}", "slice:".to_string()), format!("{}", format_slice(&s)));

    let mut c = Rc::new(RefCell::new(Some(vec!["".to_string(); ((*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    {
        let _src = { let __copy_src_holder = s.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
        let _n = std::cmp::min((*c.borrow().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*c.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone();
        }
        Rc::new(RefCell::new(Some(_n as i32)))
    };
    println!("{} {}", format!("{}", "copy:".to_string()), format!("{}", format_slice(&c)));

    let mut l = Rc::new(RefCell::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = (3) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    println!("{} {}", format!("{}", "slice[1:3]:".to_string()), format!("{}", format_slice(&l)));
}