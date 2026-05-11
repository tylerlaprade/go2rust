use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

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

#[derive(Debug, Clone, Default)]
pub struct holder {
    pub table: Arc<Mutex<Option<BTreeMap<i32, Arc<Mutex<Option<String>>>>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { table: self.table.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_map(&self.table))
    }
}


pub fn remove(h: Arc<Mutex<Option<holder>>>, key: Arc<Mutex<Option<i32>>>) {
    { let __map_handle = (*h.lock().unwrap().as_ref().unwrap()).table.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&{ let __v = (*key.lock().unwrap().as_ref().unwrap()).clone(); __v }); };
}

fn main() {
    std::thread::spawn(move || {
        ;
    });
    let mut h = Arc::new(Mutex::new(Some(holder { table: Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<String>>>>::from([(1, Arc::new(Mutex::new(Some("one".to_string()))))])))), ..Default::default() })));
    println!("{}", { let __map = { let __map_holder = (*h.lock().unwrap().as_ref().unwrap()).table.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&1).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) });
    remove(h.clone(), Arc::new(Mutex::new(Some(1))));
    let (_, mut ok) = { let __map = { let __map_holder = (*h.lock().unwrap().as_ref().unwrap()).table.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; match __map.get(&1) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false)))) } };
    println!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v });
}