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
    let mut ages = Rc::new(RefCell::new(Some(HashMap::<String, Rc<RefCell<Option<i32>>>>::new())));
    (*ages.borrow_mut().as_mut().unwrap()).insert("Alice".to_string(), Rc::new(RefCell::new(Some(25))));
    (*ages.borrow_mut().as_mut().unwrap()).insert("Bob".to_string(), Rc::new(RefCell::new(Some(30))));
    (*ages.borrow_mut().as_mut().unwrap()).insert("Charlie".to_string(), Rc::new(RefCell::new(Some(35))));
    println!("{} {}", "Ages map:".to_string(), format_map(&ages));
    let mut colors = Rc::new(RefCell::new(Some(HashMap::<String, Rc<RefCell<Option<String>>>>::from([("red".to_string(), Rc::new(RefCell::new(Some("#FF0000".to_string())))), ("green".to_string(), Rc::new(RefCell::new(Some("#00FF00".to_string())))), ("blue".to_string(), Rc::new(RefCell::new(Some("#0000FF".to_string()))))]))));
    println!("{} {}", "Colors map:".to_string(), format_map(&colors));
    let (mut age, mut exists) = match (*ages.borrow().as_ref().unwrap()).get(&"Alice".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false)))) };
    if (*exists.borrow_mut().as_mut().unwrap()) {
        println!("{} {}", "Alice's age:".to_string(), (*age.borrow_mut().as_mut().unwrap()));
    }
    (*ages.borrow_mut().as_mut().unwrap()).remove(&"Bob".to_string());
    println!("{} {}", "After deleting Bob:".to_string(), format_map(&ages));
    println!("{}", "All colors:".to_string());
    let mut keys: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(Some(Default::default())));
    for (k, _) in (*colors.borrow().as_ref().unwrap()).clone() {
        {(*keys.borrow_mut().as_mut().unwrap()).push(k); keys.clone()};
    }
    (*keys.borrow_mut().as_mut().unwrap()).sort();
    for k in &(*keys.borrow_mut().as_mut().unwrap()) {
        println!("{} {} {}", k, "->".to_string(), (*(*colors.borrow().as_ref().unwrap()).get(k).unwrap().borrow().as_ref().unwrap()));
    }
}