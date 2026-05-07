use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct person {
    pub name: Rc<RefCell<Option<String>>>,
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
    return Rc::new(RefCell::new(Some("missing".to_string())));
}

fn main() {
    println!("{}", (*asserted_name(Rc::new(RefCell::new(Some(person { name: Rc::new(RefCell::new(Some("Ada".to_string()))), ..Default::default() })))).borrow().as_ref().unwrap()));
}