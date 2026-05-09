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

#[derive(Debug, Clone, Default)]
pub struct ast_Ident {
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for ast_Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Ident>")
    }
}


impl ast_Ident {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SelectorExpr {
    pub sel: Rc<RefCell<Option<ast_Ident>>>,
}

impl std::fmt::Display for ast_SelectorExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SelectorExpr>")
    }
}


impl ast_SelectorExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub fn pick(sel: Rc<RefCell<Option<ast_SelectorExpr>>>) -> Rc<RefCell<Option<ast_Ident>>> {

    return (*sel.borrow().as_ref().unwrap()).sel.clone();
}

pub fn selector_name(sel: Rc<RefCell<Option<ast_SelectorExpr>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some((*(*(*sel.borrow().as_ref().unwrap()).sel.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone())));
}

pub fn has_selector_name(sel: Rc<RefCell<Option<ast_SelectorExpr>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some((*(*(*sel.borrow().as_ref().unwrap()).sel.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone() != "_")));
}

pub fn selector_name_map(sel: Rc<RefCell<Option<ast_SelectorExpr>>>) -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>> {

    let mut names = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::new())));
    { let __map_key = "selector".to_string(); let __map_value = Rc::new(RefCell::new(Some((*(*(*sel.borrow().as_ref().unwrap()).sel.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()))); (*names.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    return names.clone();
}

fn main() {
    if false {
        println!("{}", format!("&{}", (*pick(Rc::new(RefCell::new(Some(ast_SelectorExpr { ..Default::default() })))).borrow().as_ref().unwrap())));
        println!("{}", (*selector_name(Rc::new(RefCell::new(Some(ast_SelectorExpr { ..Default::default() })))).borrow().as_ref().unwrap()));
        println!("{}", (*has_selector_name(Rc::new(RefCell::new(Some(ast_SelectorExpr { ..Default::default() })))).borrow().as_ref().unwrap()));
        println!("{}", format_map(&selector_name_map(Rc::new(RefCell::new(Some(ast_SelectorExpr { ..Default::default() }))))));
    }
    println!("{}", "ok".to_string());
}