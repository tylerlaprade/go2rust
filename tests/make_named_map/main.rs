use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Counts(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>);

impl Display for Counts {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


fn main() {
    let mut m = Rc::new(RefCell::new(Some(Counts(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())))))));
    { let __map_key = "a".to_string(); let __map_value = Rc::new(RefCell::new(Some(1))); (*{ let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "b".to_string(); let __map_value = Rc::new(RefCell::new(Some(2))); (*{ let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "c".to_string(); let __map_value = Rc::new(RefCell::new(Some(3))); (*{ let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut keys = Rc::new(RefCell::new(Some(Vec::<String>::with_capacity(({ let __named_map_holder = m.clone(); let __named_map_guard = __named_map_holder.borrow(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }) as usize))));
    for (k, _) in { let __range_holder = { let __named_map_holder = m.clone(); let __named_map_guard = __named_map_holder.borrow(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(k.clone()); __append_target.clone() }; keys = new_val; };
    }
    { let __sort_target = keys.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for k in __range_values.iter() {
        println!("{} {}", format!("{}", k), format!("{}", { let __map_holder = { let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(k)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    } }
}