use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Runner {
}

impl Runner {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for Runner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Runner {
    pub fn run_piped(&self) -> Rc<RefCell<Option<String>>> {
        self.run_piped_1()
    }

    pub fn run_piped_1(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("private method".to_string())))
    }
}

fn main() {
    let mut r: Rc<RefCell<Option<Runner>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{}", format!("{}", (*(*r.borrow().as_ref().unwrap()).run_piped().borrow().as_ref().unwrap())));
}