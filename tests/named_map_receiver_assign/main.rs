use std::cell::{RefCell};
use std::collections::BTreeMap;
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

#[derive(Debug, Clone, Default)]
pub struct Tally(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Vec<i32>>>>>>>>);


impl Tally {
    pub fn add(&self, name: Rc<RefCell<Option<String>>>, value: Rc<RefCell<Option<i32>>>) {
        { let __map_key = (*name.borrow().as_ref().unwrap()).clone(); let __map_value = { let __slice = { let __map_holder = self.0.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&(*name.borrow().as_ref().unwrap()).clone()).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; (*__slice.borrow_mut()).get_or_insert_with(Vec::new).push((*value.borrow().as_ref().unwrap()).clone()); __slice.clone() }; (*self.0.clone().borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    pub fn replace(&self, old: Rc<RefCell<Option<String>>>, new: Rc<RefCell<Option<String>>>) {
        {
        let (mut list, mut ok) = match (*self.0.borrow().as_ref().unwrap()).get(&(*old.borrow().as_ref().unwrap()).clone()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Default::default(), Rc::new(RefCell::new(Some(false)))) };;
        if (*ok.borrow().as_ref().unwrap()) {
            { let __map_handle = self.0.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&(*old.borrow().as_ref().unwrap()).clone()); };;
            { let __map_key = (*new.borrow().as_ref().unwrap()).clone(); let __map_value = { let __slice = { let __map_holder = self.0.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&(*new.borrow().as_ref().unwrap()).clone()).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; (*__slice.borrow_mut()).get_or_insert_with(Vec::new).extend({ let __slice_holder = list.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __slice.clone() }; (*self.0.clone().borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }
}

fn main() {
    let mut t = Rc::new(RefCell::new(Some(Tally(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i32>>>>>::new())))))));
    (*t.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(1))));
    (*t.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(2))));
    (*t.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some("b".to_string()))), Rc::new(RefCell::new(Some(3))));
    (*t.borrow().as_ref().unwrap()).replace(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some("c".to_string()))));

    let mut keys = Rc::new(RefCell::new(Some(Vec::with_capacity(({ let __map_holder = { let __named_map = (*t.borrow().as_ref().unwrap()).0.clone(); __named_map }; let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize))));
    for (k, _) in { let __range_holder = { let __named_map = (*t.borrow().as_ref().unwrap()).0.clone(); __named_map }; let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(k.clone()); __append_target.clone() }; keys = new_val; };
    }
    { let mut __sort_guard = keys.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for k in __range_values.iter() {
        println!("{} {}", format!("{}", k), format!("{}", format_slice(&(*{ let __named_map = (*t.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow().as_ref().unwrap()).get(k).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()))));
    } }
}