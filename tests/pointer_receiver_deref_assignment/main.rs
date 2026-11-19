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
    pub name: Rc<RefCell<Option<String>>>,
    pub imports: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Package>>>>>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = self.imports.clone();
        Self {
            i_d: __go_clone_0_0,
            name: __go_clone_1_0,
            imports: __go_clone_2_0,
        }
    }
}


impl Default for Package {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        Self {
            i_d: __go_default_0_0,
            name: __go_default_1_0,
            imports: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.i_d.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_map(&self.imports));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl Package {
    pub fn reset(&mut self, id: Rc<RefCell<Option<String>>>, name: Rc<RefCell<Option<String>>>) {
        { let new_val = Package { i_d: Rc::new(RefCell::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), name: Rc::new(RefCell::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }; *self = new_val; };
        { let new_val = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Package>>>>::new()))); self.imports = new_val; };
        { let __map_key = "self".to_string(); let __map_value = Rc::new(RefCell::new(Some(Package { i_d: Rc::new(RefCell::new(Some({ let __selector_holder = self.i_d.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))); (*self.imports.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
}

fn main() {
    let mut pkg = Rc::new(RefCell::new(Some(Package { i_d: Rc::new(RefCell::new(Some("old".to_string()))), name: Rc::new(RefCell::new(Some("Old".to_string()))), imports: Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Package>>>>::from([("dep".to_string(), Rc::new(RefCell::new(Some(Package { i_d: Rc::new(RefCell::new(Some("dep".to_string()))), ..Default::default() }))).clone())])))), ..Default::default() })));
    (*pkg.borrow_mut().as_mut().unwrap()).reset(Rc::new(RefCell::new(Some("new".to_string()))), Rc::new(RefCell::new(Some("New".to_string()))));
    println!("{}", format!("{}", (*(*pkg.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*(*pkg.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*(*(*(*pkg.borrow().as_ref().unwrap()).imports.borrow().as_ref().unwrap()).clone().get(&"self".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()).clone()));
}