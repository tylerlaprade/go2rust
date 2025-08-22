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
        // Slice with capacity
    let mut s = Rc::new(RefCell::new(Some(vec![0; 3])));
    print!("len={} cap={} {}\n", (*s.borrow().as_ref().unwrap()).len(), (*s.borrow_mut().as_mut().unwrap()).capacity(), format_slice(&s));

        // Append beyond capacity
    {(*s.borrow_mut().as_mut().unwrap()).extend(vec![1, 2, 3, 4, 5, 6, 7, 8]); s.clone()};
    print!("len={} cap={} {}\n", (*s.borrow().as_ref().unwrap()).len(), (*s.borrow_mut().as_mut().unwrap()).capacity(), format_slice(&s));

        // Three-index slice
    let mut s2 = Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap())[2 as usize..5 as usize].to_vec())));
    print!("s2: len={} cap={} {}\n", (*s2.borrow().as_ref().unwrap()).len(), (*s2.borrow_mut().as_mut().unwrap()).capacity(), format_slice(&s2));

        // Copy
    let mut s3 = Rc::new(RefCell::new(Some(vec![0; 3])));
    let mut n = copy(s3.clone(), s.clone());
    print!("Copied {} elements: {}\n", (*n.borrow_mut().as_mut().unwrap()), format_slice(&s3));

        // Nil slice vs empty slice
    let mut s4: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut s5 = Rc::new(RefCell::new(Some(vec![])));
    print!("s4==nil: {}, s5==nil: {}\n", (*s4.borrow()).is_none(), (*s5.borrow()).is_none());
}