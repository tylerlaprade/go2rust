use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Type;

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}


impl types_Type {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct cache {
}

impl std::fmt::Display for cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl cache {
    pub fn r#use(&self, T: Rc<RefCell<Option<types_Type>>>) {
        let _ = T;
    }
}

pub fn exercise(T: Rc<RefCell<Option<types_Type>>>, c: Rc<RefCell<Option<cache>>>) {
    (*c.borrow_mut().as_mut().unwrap()).r#use(T.clone());
    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<types_Type, Rc<RefCell<Option<i32>>>>::from([((*T.borrow().as_ref().unwrap()).clone(), Rc::new(RefCell::new(Some(1))))]))));
    println!("{}", (*seen.borrow().as_ref().unwrap()).get(&(*T.borrow().as_ref().unwrap()).clone()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0));
}

fn main() {
    if false {
        exercise(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(Some(cache {  }))));
    }
    println!("{}", "ok".to_string());
}