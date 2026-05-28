use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Config {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Config {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct loader {
    pub config: Rc<RefCell<Option<Config>>>,
}

impl loader {
    pub fn __go_value_clone(&self) -> Self {
        Self { config: { let __guard = self.config.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for loader {
    fn default() -> Self {
        Self { config: Rc::new(RefCell::new(Some(Config::default()))) }
    }
}

impl std::fmt::Display for loader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.config.borrow().as_ref().unwrap()))
    }
}


impl loader {
}

pub fn r#use(cfg: Rc<RefCell<Option<Config>>>, patterns: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}{}", format!("{}{}", (*(*cfg.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), ":".to_string()), (*patterns.borrow().as_ref().unwrap())[(0) as usize].clone()))))
}

fn main() {
    let mut ld = Rc::new(RefCell::new(Some(loader { config: Rc::new(RefCell::new(Some(Config { name: Rc::new(RefCell::new(Some("cfg".to_string()))), ..Default::default() }))), ..Default::default() })));
    println!("{}", format!("{}", (*r#use((*ld.borrow().as_ref().unwrap()).config.clone(), Rc::new(RefCell::new(Some(vec!["pat".to_string()])))).borrow().as_ref().unwrap())));
}