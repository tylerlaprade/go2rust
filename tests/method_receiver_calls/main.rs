use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Label {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl Label {
    pub fn valid(&self) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*self.name.borrow().as_ref().unwrap()).clone() != "")));
    }

    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }

    pub fn echo(&self, other: Rc<RefCell<Option<Label>>>) -> Rc<RefCell<Option<String>>> {
        return (*other.borrow().as_ref().unwrap()).name();
    }

    pub fn format(&self) -> Rc<RefCell<Option<String>>> {
        if !(*self.valid().borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some("nil".to_string())));
    }
        return self.echo(Rc::new(RefCell::new(Some(self.clone()))));
    }
}

fn main() {
    println!("{}", (*Label { name: Rc::new(RefCell::new(Some("ready".to_string()))), ..Default::default() }.format().borrow().as_ref().unwrap()));
    println!("{}", (*Label { name: Rc::new(RefCell::new(Some(String::new()))) }.format().borrow().as_ref().unwrap()));
}