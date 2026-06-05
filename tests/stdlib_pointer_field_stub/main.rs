use go2rust_stdlib_stubs::*;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
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

pub fn pick(sel: Arc<Mutex<Option<go_ast::r#mod::SelectorExpr>>>) -> Arc<Mutex<Option<go_ast::r#mod::Ident>>> {
    (*sel.lock().unwrap().as_ref().unwrap()).sel.clone()
}

pub fn selector_name(sel: Arc<Mutex<Option<go_ast::r#mod::SelectorExpr>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some({ let __selector_holder = (*(*sel.lock().unwrap().as_ref().unwrap()).sel.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

pub fn has_selector_name(sel: Arc<Mutex<Option<go_ast::r#mod::SelectorExpr>>>) -> bool {
    return { let __tmp_x = { let __selector_holder = (*(*sel.lock().unwrap().as_ref().unwrap()).sel.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x != __tmp_y };
}

pub fn selector_name_map(sel: Arc<Mutex<Option<go_ast::r#mod::SelectorExpr>>>) -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<String>>>>>>> {
    let mut names = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::new())));
    { let __map_key = "selector".to_string(); let __map_value = (*(*sel.lock().unwrap().as_ref().unwrap()).sel.lock().unwrap().as_ref().unwrap()).name.clone(); (*names.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    return names.clone();
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    strings::__go_init_all();

    if false {
        println!("{}", format!("{}", format!("&{}", (*pick(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr { ..Default::default() })))).lock().unwrap().as_ref().unwrap()))));
        println!("{}", format!("{}", (*selector_name(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr { ..Default::default() })))).lock().unwrap().as_ref().unwrap())));
        println!("{}", format!("{}", has_selector_name(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr { ..Default::default() }))))));
        println!("{}", format!("{}", format_map(&selector_name_map(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr { ..Default::default() })))))));
    }
    println!("{}", format!("{}", "ok".to_string()));
}