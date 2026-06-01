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
    let mut dst = Rc::new(RefCell::new(Some(vec![0; (6) as usize])));
    let mut src1 = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    let mut src2 = Rc::new(RefCell::new(Some(vec![4, 5, 6])));

    let mut i = Rc::new(RefCell::new(Some(0)));
    { let __rhs = (*{ let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize; let _dst_len = (*dst.borrow().as_ref().unwrap()).len() - _dst_start; let _src = { let __copy_src_holder = src1.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*dst.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) }.borrow().as_ref().unwrap()); let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __rhs = (*{ let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize; let _dst_len = (*dst.borrow().as_ref().unwrap()).len() - _dst_start; let _src = { let __copy_src_holder = src2.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*dst.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) }.borrow().as_ref().unwrap()); let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

    println!("{} {}", format!("{}", { let __v = (*i.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&dst)));
}