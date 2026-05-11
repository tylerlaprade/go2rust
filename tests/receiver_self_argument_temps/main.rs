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
pub struct r#box {
    pub items: Rc<RefCell<Option<Vec<i32>>>>,
}

impl std::fmt::Display for r#box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.items))
    }
}


impl r#box {
    pub fn print(&self, n: Rc<RefCell<Option<i32>>>) {
        println!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v });
    }

    pub fn add(&mut self, x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {
        { let new_val = { let __append_target = self.items.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*x.borrow().as_ref().unwrap()).clone()); __append_target.clone() }; self.items = new_val; };
        return Rc::new(RefCell::new(Some((*self.items.borrow().as_ref().unwrap()).len() as i32)));
    }

    pub fn flush(&mut self) {
        { let __method_arg0 = Rc::new(RefCell::new(Some((*self.items.borrow().as_ref().unwrap()).len() as i32))); self.print(__method_arg0) };
        { let __method_arg0 = { let __method_arg0 = Rc::new(RefCell::new(Some((*self.items.borrow().as_ref().unwrap()).len() as i32))); self.add(__method_arg0) }; self.print(__method_arg0) };
    }
}

fn main() {
    let mut b = Rc::new(RefCell::new(Some(r#box { items: Rc::new(RefCell::new(Some(vec![1, 2, 3]))), ..Default::default() })));
    (*b.borrow_mut().as_mut().unwrap()).flush();
    println!("{}", (*(*b.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).len());
}