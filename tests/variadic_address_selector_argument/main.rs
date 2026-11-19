use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


thread_local! {
    static GO_EMBEDDED_OWNER_REGISTRY: RefCell<std::collections::HashMap<usize, Box<dyn Any>>> = RefCell::new(std::collections::HashMap::new());
}

fn go_register_embedded_owner<T: 'static>(embedded_key: usize, owner: Rc<RefCell<Option<T>>>) {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(embedded_key, Box::new(owner));
    });
}

fn go_lookup_embedded_owner<T: 'static>(embedded_key: usize, target: &str) -> Rc<RefCell<Option<T>>> {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
        owner
            .downcast_ref::<Rc<RefCell<Option<T>>>>()
            .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
            .clone()
    })
}

#[derive(Debug, Clone)]
pub struct Config {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct loader {
    pub config: Rc<RefCell<Option<Config>>>,
}

impl loader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.config.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            config: __go_clone_0_0,
        }
    }
}


impl Default for loader {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Config::default())));
        Self {
            config: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for loader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.config.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl loader {
}

pub fn r#use(cfg: Rc<RefCell<Option<Config>>>, patterns: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<String>>> {
    return Rc::new(RefCell::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*(*cfg.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
        __s.push_str(&format!("{}", ":".to_string()));
        __s.push_str(&format!("{}", (*patterns.borrow().as_ref().unwrap())[(0) as usize].clone()));
        __s
    })));
}

fn main() {
    let mut ld = {
        let __owner = Rc::new(RefCell::new(Some(loader { config: Rc::new(RefCell::new(Some(Config { name: Rc::new(RefCell::new(Some("cfg".to_string()))), ..Default::default() }))), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.borrow(); __owner_guard.as_ref().unwrap().config.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.borrow(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    println!("{}", format!("{}", (*r#use((*ld.borrow().as_ref().unwrap()).config.clone(), Rc::new(RefCell::new(Some(vec!["pat".to_string()])))).borrow().as_ref().unwrap())));
}