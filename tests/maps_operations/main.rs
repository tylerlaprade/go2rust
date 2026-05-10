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
    println!("{} {}", "map:".to_string(), format_map(&m));

    let mut v1 = Rc::new(RefCell::new(Some((*m.borrow().as_ref().unwrap()).get(&"k1".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0))));
    println!("{} {}", "v1:".to_string(), { let __v = (*v1.borrow().as_ref().unwrap()).clone(); __v });

    { let __map_handle = m.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&"k2".to_string()); };
    println!("{} {}", "map:".to_string(), format_map(&m));

    let (_, mut prs) = match (*m.borrow().as_ref().unwrap()).get(&"k2".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false)))) };
    println!("{} {}", "prs:".to_string(), { let __v = (*prs.borrow().as_ref().unwrap()).clone(); __v });
}