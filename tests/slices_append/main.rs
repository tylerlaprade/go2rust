use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T>(slice: &Rc<RefCell<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn main() {
    let mut s: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(Some(Default::default())));
    {(*s.borrow_mut().as_mut().unwrap()).push("a".to_string()); s.clone()};
    {(*s.borrow_mut().as_mut().unwrap()).extend(vec!["b".to_string(), "c".to_string()]); s.clone()};
    println!("{} {}", "slice:".to_string(), format_slice(&s));

    let mut c = Rc::new(RefCell::new(Some(vec!["".to_string(); (*s.borrow().as_ref().unwrap()).len()])));
    { let _src = ((*s.borrow().as_ref().unwrap())).clone(); let _n = std::cmp::min(((*c.borrow().as_ref().unwrap())).len(), _src.len()); for _i in 0.._n { (*c.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    println!("{} {}", "copy:".to_string(), format_slice(&c));

    let mut l = Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap())[1 as usize..3 as usize].to_vec())));
    println!("{} {}", "slice[1:3]:".to_string(), format_slice(&l));
}