use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Label {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Label {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Label {
    pub fn valid(&self) -> bool {
        return (*self.name.borrow().as_ref().unwrap()).clone() != "".to_string();
    }

    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }

    pub fn echo(&self, other: Rc<RefCell<Option<Label>>>) -> Rc<RefCell<Option<String>>> {
        (*other.borrow().as_ref().unwrap()).name()
    }

    pub fn format(&self) -> Rc<RefCell<Option<String>>> {
        if !self.valid() {
        return Rc::new(RefCell::new(Some("nil".to_string())));
    }
        self.echo(Rc::new(RefCell::new(Some(self.clone()))))
    }
}

fn main() {
    println!("{}", format!("{}", (*Label { name: Rc::new(RefCell::new(Some("ready".to_string()))), ..Default::default() }.format().borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*Label { name: Rc::new(RefCell::new(Some(String::new()))) }.format().borrow().as_ref().unwrap())));
}