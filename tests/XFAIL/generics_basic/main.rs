use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
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
pub struct List {
    pub head: Rc<RefCell<Option</* ERROR: Unsupported instantiated generic type */ Rc<RefCell<Option<()>>>>>>,
    pub tail: Rc<RefCell<Option</* ERROR: Unsupported instantiated generic type */ Rc<RefCell<Option<()>>>>>>,
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.head.borrow().as_ref().unwrap()), (*self.tail.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct element {
    pub next: Rc<RefCell<Option</* ERROR: Unsupported instantiated generic type */ Rc<RefCell<Option<()>>>>>>,
    pub val: Rc<RefCell<Option<T>>>,
}

impl std::fmt::Display for element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.next.borrow().as_ref().unwrap()), (*self.val.borrow().as_ref().unwrap()))
    }
}


impl Unknown {
    pub fn push(&mut self, v: Rc<RefCell<Option<T>>>) {
        if (*self.tail.borrow()).is_none() {
        { let new_val = Rc::new(RefCell::new(Some())).clone(); self.head = new_val; };
        { let new_val = self.head.clone(); self.tail = new_val; };
    } else {
        { let new_val = Rc::new(RefCell::new(Some())).clone(); (*self.tail.borrow_mut().as_mut().unwrap()).next = new_val; };
        { let new_val = (*self.tail.borrow().as_ref().unwrap()).next.clone(); self.tail = new_val; };
    }
    }
}

pub fn map_keys(m: Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> Rc<RefCell<Option<Vec<K>>>> {

    let mut r = Rc::new(RefCell::new(Some(Vec::with_capacity(((*m.borrow().as_ref().unwrap()).len()) as usize))));
    for (k, _) in (*m.borrow().as_ref().unwrap()).clone() {
        {(*r.borrow_mut()).get_or_insert_with(Vec::new).push(k); r.clone()};
    }
    return r.clone();
}

fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<String>>>>::from([(1, Rc::new(RefCell::new(Some("2".to_string())))), (2, Rc::new(RefCell::new(Some("4".to_string())))), (4, Rc::new(RefCell::new(Some("8".to_string()))))]))));
    let mut keys = map_keys(m.clone());
    (*keys.borrow_mut().as_mut().unwrap()).sort();
    println!("{} {}", "keys:".to_string(), format_slice(&keys));

    let mut lst = ;
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(10))));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(13))));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(23))));
}