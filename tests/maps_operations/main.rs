use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::HashMap;
use std::fmt::{Display};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<HashMap<K, Rc<RefCell<Option<V>>>>>>>) -> String 
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
    let mut m = Rc::new(RefCell::new(Some(HashMap::<String, Rc<RefCell<Option<i32>>>>::new())));
    (*m.borrow_mut().as_mut().unwrap()).insert("k1".to_string(), Rc::new(RefCell::new(Some(7))));
    (*m.borrow_mut().as_mut().unwrap()).insert("k2".to_string(), Rc::new(RefCell::new(Some(13))));
    println!("{} {}", "map:".to_string(), format_map(&m));

    let mut v1 = Rc::new(RefCell::new(Some((*(*m.borrow().as_ref().unwrap()).get(&"k1".to_string()).unwrap().borrow().as_ref().unwrap()))));
    println!("{} {}", "v1:".to_string(), (*v1.borrow_mut().as_mut().unwrap()));

    (*m.borrow_mut().as_mut().unwrap()).remove(&"k2".to_string());
    println!("{} {}", "map:".to_string(), format_map(&m));

    let (_, mut prs) = match (*m.borrow().as_ref().unwrap()).get(&"k2".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false)))) };
    println!("{} {}", "prs:".to_string(), (*prs.borrow_mut().as_mut().unwrap()));
}