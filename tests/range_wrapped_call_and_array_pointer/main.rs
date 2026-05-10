use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
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

#[derive(Debug, Clone, Default)]
pub struct bucket {
    pub values: Rc<RefCell<Option<[i32; 3]>>>,
}

impl std::fmt::Display for bucket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.values))
    }
}


impl bucket {
    pub fn sum(&self) -> Rc<RefCell<Option<i32>>> {
        let mut total = Rc::new(RefCell::new(Some(0)));
        { let __range_holder = self.values.clone().clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for value in __range_values.iter().copied() {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + value); };
    } }
        return total.clone();
    }
}

pub fn names() -> Rc<RefCell<Option<Vec<String>>>> {

    return Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()])));
}

fn main() {
    { let __range_holder = names().clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for name in __range_values.iter() {
        println!("{}", name);
    } }
    let mut b = Rc::new(RefCell::new(Some(bucket { values: Rc::new(RefCell::new(Some([2, 3, 5]))), ..Default::default() })));
    println!("{}", (*(*b.borrow_mut().as_mut().unwrap()).sum().borrow().as_ref().unwrap()));

    let mut groups = Rc::new(RefCell::new(Some([vec!["go".to_string()], vec!["rust".to_string(), "zig".to_string()]])));
    let mut total = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = groups.clone().clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for group in __range_values.iter() {
        for name in group.iter() {
        if (*name).clone() != "" {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    } }
    println!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v });
}