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
pub struct Free {
    pub seen: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>,
}

impl std::fmt::Display for Free {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_map(&self.seen))
    }
}


impl Free {
    pub fn has(&mut self, key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<bool>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

        if (*self.seen.borrow()).is_none() {
        { let new_val = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::new()))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *self.seen.borrow_mut() = __moved_val; };
    }
        let mut f_defer_captured = self.clone(); let key_defer_captured = key.clone(); let res_defer_captured = res.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        (*f_defer_captured.seen.borrow_mut().as_mut().unwrap()).insert((*key_defer_captured.borrow().as_ref().unwrap()).clone(), res_defer_captured.clone());
    }) as Box<dyn Fn() -> ()>))); let __f_guard = __f_holder.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
    }));
        { let new_val = true; *res.borrow_mut() = Some(new_val); };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res
    }
    }
}

fn main() {
    let mut f: Rc<RefCell<Option<Free>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut has = (*f.borrow_mut().as_mut().unwrap()).has(Rc::new(RefCell::new(Some("x".to_string()))));
    let mut seen = Rc::new(RefCell::new(Some((*(*f.borrow().as_ref().unwrap()).seen.borrow().as_ref().unwrap()).get(&"x".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false))));
    println!("{} {}", { let __v = (*has.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*seen.borrow().as_ref().unwrap()).clone(); __v });
}