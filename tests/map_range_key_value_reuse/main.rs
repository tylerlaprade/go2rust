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

#[derive(Debug, Clone)]
pub struct Package {
    pub i_d: Rc<RefCell<Option<String>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { i_d: { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Package {
    fn default() -> Self {
        Self { i_d: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.i_d.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct LoaderPackage {
    pub imports: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Package>>>>>>>,
}

impl LoaderPackage {
    pub fn __go_value_clone(&self) -> Self {
        Self { imports: self.imports.clone() }
    }
}

impl std::fmt::Display for LoaderPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_map(&self.imports))
    }
}


fn main() {
    let mut ids = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::from([("C".to_string(), Rc::new(RefCell::new(Some(true)))), ("pkg".to_string(), Rc::new(RefCell::new(Some(true))))]))));
    let mut pkg = Rc::new(RefCell::new(Some(LoaderPackage { imports: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Package>>>>::new()))), ..Default::default() })));

    for (id, _) in { let __range_holder = ids.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        if id.clone() == "C" {
        continue
    }
        { let __map_key = id.clone(); let __map_value = Rc::new(RefCell::new(Some(Package { i_d: Rc::new(RefCell::new(Some(id.clone()))), ..Default::default() }))); (*(*pkg.borrow().as_ref().unwrap()).imports.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    println!("{}", format!("{}", (*(*pkg.borrow().as_ref().unwrap()).imports.borrow().as_ref().unwrap()).len()));
    println!("{}", format!("{}", (*(*(*(*pkg.borrow().as_ref().unwrap()).imports.borrow().as_ref().unwrap()).clone().get(&"pkg".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap())));
}