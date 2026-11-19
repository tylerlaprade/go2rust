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
pub struct nodeSet(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>);

impl Display for nodeSet {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct graphNode {
    pub succ: Rc<RefCell<Option<nodeSet>>>,
}

impl graphNode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.succ.clone();
        Self {
            succ: __go_clone_0_0,
        }
    }
}


impl Default for graphNode {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(nodeSet::default())));
        Self {
            succ: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for graphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.succ.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    let mut n: Rc<RefCell<Option<graphNode>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{}", format!("{}", { let __named_map_holder = (*n.borrow().as_ref().unwrap()).succ.clone(); let __named_map_guard = __named_map_holder.borrow(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }));
}