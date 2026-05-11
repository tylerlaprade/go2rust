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
pub struct Package {
    pub i_d: Rc<RefCell<Option<String>>>,
    pub name: Rc<RefCell<Option<String>>>,
    pub imports: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Package>>>>>>>,
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.i_d.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()), format_map(&self.imports))
    }
}


impl Package {
    pub fn reset(&mut self, id: Rc<RefCell<Option<String>>>, name: Rc<RefCell<Option<String>>>) {
        { let new_val = Package { i_d: id.clone(), name: name.clone(), ..Default::default() }; *self = new_val; };
        { let new_val = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Package>>>>::new()))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *self.imports.borrow_mut() = __moved_val; };
        { let __map_key = "self".to_string(); let __map_value = Rc::new(RefCell::new(Some(Package { i_d: self.i_d.clone(), ..Default::default() }))); (*self.imports.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
}

fn main() {
    let mut pkg = Rc::new(RefCell::new(Some(Package { i_d: Rc::new(RefCell::new(Some("old".to_string()))), name: Rc::new(RefCell::new(Some("Old".to_string()))), imports: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Package>>>>::from([("dep".to_string(), Rc::new(RefCell::new(Some(Package { i_d: Rc::new(RefCell::new(Some("dep".to_string()))), ..Default::default() }))).clone())])))), ..Default::default() })));
    (*pkg.borrow_mut().as_mut().unwrap()).reset(Rc::new(RefCell::new(Some("new".to_string()))), Rc::new(RefCell::new(Some("New".to_string()))));
    println!("{}", (*(*pkg.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));
    println!("{}", (*(*pkg.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    println!("{}", (*(*(*(*pkg.borrow().as_ref().unwrap()).imports.borrow().as_ref().unwrap()).get(&"self".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));
}