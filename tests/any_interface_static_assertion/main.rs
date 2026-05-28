use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct person {
    pub name: Rc<RefCell<Option<String>>>,
}

impl person {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for person {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl person {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

pub fn asserted_name(p: Rc<RefCell<Option<person>>>) -> Rc<RefCell<Option<String>>> {
    let (mut named, mut ok) = ({
        let __asserted = p.clone();
        (__asserted.clone(), Rc::new(RefCell::new(Some(true))))
    });
    if (*ok.borrow().as_ref().unwrap()) {
        return (*named.borrow().as_ref().unwrap()).name();
    }
    Rc::new(RefCell::new(Some("missing".to_string())))
}

fn main() {
    println!("{}", format!("{}", (*asserted_name(Rc::new(RefCell::new(Some(person { name: Rc::new(RefCell::new(Some("Ada".to_string()))), ..Default::default() })))).borrow().as_ref().unwrap())));
}