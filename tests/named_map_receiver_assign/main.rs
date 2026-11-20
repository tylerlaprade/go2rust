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
pub struct Tally(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Vec<i64>>>>>>>>);


impl Tally {
    pub fn add(&self, name: Rc<RefCell<Option<String>>>, value: Rc<RefCell<Option<i64>>>) {
        let mut __self = self.clone();
        { let __map_key = (*name.borrow().as_ref().unwrap()).clone(); let __map_value = { let __slice = { let __map_holder = __self.0.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&(*name.borrow().as_ref().unwrap()).clone()).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; let __append_value = (*value.borrow().as_ref().unwrap()).clone(); (*__slice.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __slice.clone() }; (*__self.0.clone().borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    pub fn replace(&self, old: Rc<RefCell<Option<String>>>, new: Rc<RefCell<Option<String>>>) {
        let mut __self = self.clone();
        {
        let (mut list, mut ok) = { let __map_holder = __self.0.clone().clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&(*old.borrow().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if ok {
            { let __map_handle = __self.0.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&(*old.borrow().as_ref().unwrap()).clone()); };;
            { let __map_key = (*new.borrow().as_ref().unwrap()).clone(); let __map_value = { let __slice = { let __map_holder = __self.0.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&(*new.borrow().as_ref().unwrap()).clone()).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; let __append_values: Vec<_> = { let __slice_holder = list.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); (*__slice.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __slice.clone() }; (*__self.0.clone().borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }
}

fn main() {
    let mut t = Rc::new(RefCell::new(Some(Tally(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i64>>>>>::new())))))));
    (*t.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(1))));
    (*t.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(2))));
    (*t.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some("b".to_string()))), Rc::new(RefCell::new(Some(3))));
    (*t.borrow().as_ref().unwrap()).replace(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some("c".to_string()))));

    let mut keys = Rc::new(RefCell::new(Some(Vec::<String>::with_capacity(({ let __named_map_holder = t.clone(); let __named_map_guard = __named_map_holder.borrow(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }) as usize))));
    for (k, _) in { let __range_holder = { let __named_map_holder = t.clone(); let __named_map_guard = __named_map_holder.borrow(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); let __append_value = k.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; keys = new_val; };
    }
    { let __sort_target = keys.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for k in __range_values.iter() {
        println!("{} {}", format!("{}", k), format!("{}", format_slice(&{ let __map_holder = { let __named_map = (*t.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(k)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    } }
}