use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub name: Rc<RefCell<Option<String>>>,
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

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.owner.borrow().as_ref().unwrap()))
    }
}


impl Node {
    pub fn link(&self) -> Rc<RefCell<Option<Link>>> {
        return Rc::new(RefCell::new(Some(Link { owner: Rc::new(RefCell::new(Some(self.clone()))), ..Default::default() })));
    }
}

impl Link {
    pub fn owner_name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*(*self.owner.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone())));
    }
}

fn main() {
    let mut node = Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("root".to_string()))), ..Default::default() })));
    println!("{}", (*{ let __recv = (*node.borrow_mut().as_mut().unwrap()).link(); let __result = (*__recv.borrow().as_ref().unwrap()).owner_name(); __result }.borrow().as_ref().unwrap()));
}