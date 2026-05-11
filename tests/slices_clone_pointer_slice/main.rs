use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct pkg {
    pub path: Rc<RefCell<Option<String>>>,
}

impl pkg {
    pub fn __go_value_clone(&self) -> Self {
        Self { path: { let __guard = self.path.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for pkg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.path.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut pkgs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(pkg { path: Rc::new(RefCell::new(Some("root".to_string()))), ..Default::default() }))), Rc::new(RefCell::new(Some(pkg { path: Rc::new(RefCell::new(Some("dep".to_string()))), ..Default::default() })))])));
    let mut list = Rc::new(RefCell::new(Some((*Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = pkgs.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() }))).borrow().as_ref().unwrap()).clone())));
    println!("{}", (*list.borrow().as_ref().unwrap()).len());
}