use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Node {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Node {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Link {
    pub owner: Rc<RefCell<Option<Node>>>,
}

impl Link {
    pub fn __go_value_clone(&self) -> Self {
        Self { owner: self.owner.clone() }
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.owner.borrow().as_ref().unwrap()))
    }
}


impl Node {
    pub fn link(&self) -> Rc<RefCell<Option<Link>>> {
        Rc::new(RefCell::new(Some(Link { owner: Rc::new(RefCell::new(Some(self.clone()))), ..Default::default() })))
    }
}

impl Link {
    pub fn owner_name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = (*self.owner.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

fn main() {
    let mut node = Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("root".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*{ let __recv = (*node.borrow().as_ref().unwrap()).link(); let __result = (*__recv.borrow().as_ref().unwrap()).owner_name(); __result }.borrow().as_ref().unwrap())));
}