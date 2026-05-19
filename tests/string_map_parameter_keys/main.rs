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
pub struct cache {
    pub index: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<u64>>>>>>>,
}

impl cache {
    pub fn __go_value_clone(&self) -> Self {
        Self { index: self.index.clone() }
    }
}

impl std::fmt::Display for cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_map(&self.index))
    }
}


#[derive(Debug, Clone)]
pub struct position {
    pub filename: Rc<RefCell<Option<String>>>,
}

impl position {
    pub fn __go_value_clone(&self) -> Self {
        Self { filename: { let __guard = self.filename.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for position {
    fn default() -> Self {
        Self { filename: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.filename.borrow().as_ref().unwrap()))
    }
}


impl cache {
    pub fn off(&mut self, s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<u64>>> {
        let (mut off, mut ok) = match (*self.index.clone().borrow().as_ref().unwrap()).get(&(*s.borrow().as_ref().unwrap()).clone()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false)))) };
        if !(*ok.borrow().as_ref().unwrap()) {
        { let new_val = Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap()).len() as u64))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *off.borrow_mut() = __moved_val; };
        { let __map_key = (*s.borrow().as_ref().unwrap()).clone(); let __map_value = off.clone(); (*self.index.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
        return off.clone();
    }

    pub fn remember(&mut self, p: Rc<RefCell<Option<position>>>) -> Rc<RefCell<Option<u64>>> {
        let mut file = Rc::new(RefCell::new(Some({ let __selector_holder = (*p.borrow().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        return self.off(Rc::new(RefCell::new(Some((*file.borrow().as_ref().unwrap()).clone()))));
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(cache { index: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<u64>>>>::from([])))), ..Default::default() })));
    println!("{}", (*(*c.borrow_mut().as_mut().unwrap()).off(Rc::new(RefCell::new(Some("abc".to_string())))).borrow().as_ref().unwrap()));
    println!("{}", (*(*c.borrow_mut().as_mut().unwrap()).off(Rc::new(RefCell::new(Some("abc".to_string())))).borrow().as_ref().unwrap()));
    println!("{}", (*(*c.borrow_mut().as_mut().unwrap()).remember(Rc::new(RefCell::new(Some(position { filename: Rc::new(RefCell::new(Some("xyz".to_string()))), ..Default::default() })))).borrow().as_ref().unwrap()));
}