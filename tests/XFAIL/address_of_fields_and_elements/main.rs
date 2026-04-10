use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
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

#[derive(Debug, Clone, Default)]
struct Point {
    x: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.x.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut p = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(10))), ..Default::default() })));
    let mut px = (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()).clone();
    { let new_val = 20; *px.borrow_mut() = Some(new_val); };

    let mut nums = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    let mut first = (*nums.borrow().as_ref().unwrap())[0 as usize].clone().clone();
    { let new_val = 9; *first.borrow_mut() = Some(new_val); };

    println!("{} {}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()), format_slice(&nums));
}