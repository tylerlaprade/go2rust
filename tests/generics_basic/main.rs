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
pub struct List__int {
    pub head: Rc<RefCell<Option<element__int>>>,
    pub tail: Rc<RefCell<Option<element__int>>>,
}

impl List__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.head.clone();
        let __go_clone_0_1 = self.tail.clone();
        Self {
            head: __go_clone_0_0,
            tail: __go_clone_0_1,
        }
    }
}

impl std::fmt::Display for List__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.head.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.tail.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct element__int {
    pub next: Rc<RefCell<Option<element__int>>>,
    pub val: Rc<RefCell<Option<i32>>>,
}

impl element__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.next.clone();
        let __go_clone_1_0 = { let __guard = self.val.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            next: __go_clone_0_0,
            val: __go_clone_1_0,
        }
    }
}


impl Default for element__int {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            next: __go_default_0_0,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for element__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.next.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.val.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl List__int {
    pub fn push(&mut self, v: Rc<RefCell<Option<i32>>>) {
        if { let __nil_target = self.tail.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result } {
        { let new_val = Rc::new(RefCell::new(Some(element__int { val: Rc::new(RefCell::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(); self.head = new_val; };
        { let new_val = self.head.clone(); self.tail = new_val; };
    } else {
        { let new_val = Rc::new(RefCell::new(Some(element__int { val: Rc::new(RefCell::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(); (*self.tail.borrow_mut().as_mut().unwrap()).next = new_val; };
        { let new_val = (*self.tail.borrow().as_ref().unwrap()).next.clone(); self.tail = new_val; };
    }
    }
}

pub fn map_keys__int__string(m: Rc<RefCell<Option<BTreeMap<i32, Rc<RefCell<Option<String>>>>>>>) -> Rc<RefCell<Option<Vec<i32>>>> {
    let mut r = Rc::new(RefCell::new(Some(Vec::<i32>::with_capacity(((*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = r.clone(); let __append_value = k; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; r = new_val; };
    }
    return r.clone();
}

fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<String>>>>::from([(1, Rc::new(RefCell::new(Some("2".to_string())))), (2, Rc::new(RefCell::new(Some("4".to_string())))), (4, Rc::new(RefCell::new(Some("8".to_string()))))]))));
    let mut keys = map_keys__int__string(m.clone());
    { let __sort_target = keys.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{} {}", format!("{}", "keys:".to_string()), format!("{}", format_slice(&keys)));

    let mut lst = Rc::new(RefCell::new(Some(List__int { head: Default::default(), tail: Default::default() })));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(10))));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(13))));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(23))));
}