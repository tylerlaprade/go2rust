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
        // Create and initialize map
    let mut ages = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())));
    { let __map_key = "Alice".to_string(); let __map_value = Rc::new(RefCell::new(Some(25))); (*ages.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "Bob".to_string(); let __map_value = Rc::new(RefCell::new(Some(30))); (*ages.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "Charlie".to_string(); let __map_value = Rc::new(RefCell::new(Some(35))); (*ages.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    println!("{} {}", "Ages map:".to_string(), format_map(&ages));

        // Map literal
    let mut colors = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("red".to_string(), Rc::new(RefCell::new(Some("#FF0000".to_string())))), ("green".to_string(), Rc::new(RefCell::new(Some("#00FF00".to_string())))), ("blue".to_string(), Rc::new(RefCell::new(Some("#0000FF".to_string()))))]))));

    println!("{} {}", "Colors map:".to_string(), format_map(&colors));

        // Check if key exists
    let (mut age, mut exists) = match (*ages.borrow().as_ref().unwrap()).get(&"Alice".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false)))) };
    if (*exists.borrow().as_ref().unwrap()) {
        println!("{} {}", "Alice's age:".to_string(), { let __v = (*age.borrow().as_ref().unwrap()).clone(); __v });
    }

        // Delete from map
    { let __map_handle = ages.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&"Bob".to_string()); };
    println!("{} {}", "After deleting Bob:".to_string(), format_map(&ages));

        // Iterate over map in sorted order for deterministic output
    println!("{}", "All colors:".to_string());

        // Collect all keys into a slice
    let mut keys: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    for (k, _) in (*colors.borrow().as_ref().unwrap()).clone() {
        {(*keys.borrow_mut()).get_or_insert_with(Vec::new).push(k.clone()); keys.clone()};
    }

        // Sort the keys
    (*keys.borrow_mut().as_mut().unwrap()).sort();

        // Print in sorted order
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for k in __range_values.iter() {
        println!("{} {} {}", k, "->".to_string(), (*colors.borrow().as_ref().unwrap()).get(k).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()));
    } }
}