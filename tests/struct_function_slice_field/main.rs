use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct queue {
    pub later: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>>>>,
}

impl queue {
    pub fn __go_value_clone(&self) -> Self {
        Self { later: self.later.clone() }
    }
}

impl std::fmt::Display for queue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.later.borrow(); match __guard.as_ref() { Some(__v) => format!("[{}]", std::iter::repeat("<func>").take(__v.len()).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } })
    }
}


fn main() {
    let mut q = Rc::new(RefCell::new(Some(queue { later: Rc::new(RefCell::new(Some(Vec::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>::new()))), ..Default::default() })));
    println!("{}", (*q.borrow().as_ref().unwrap()));
}