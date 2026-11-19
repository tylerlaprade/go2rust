use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display};
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

fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())));
    { let __map_key = "k1".to_string(); let __map_value = Rc::new(RefCell::new(Some(7))); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "k2".to_string(); let __map_value = Rc::new(RefCell::new(Some(13))); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", "map:".to_string()), format!("{}", format_map(&m)));

    let mut v1 = Rc::new(RefCell::new(Some({ let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k1".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
    println!("{} {}", format!("{}", "v1:".to_string()), format!("{}", { let __v = (*v1.borrow().as_ref().unwrap()).clone(); __v }));

    { let __map_handle = m.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&"k2".to_string()); };
    println!("{} {}", format!("{}", "map:".to_string()), format!("{}", format_map(&m)));

    let (_, mut prs) = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"k2".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Rc::new(RefCell::new(Some(0))), false) } };
    println!("{} {}", format!("{}", "prs:".to_string()), format!("{}", prs));
}