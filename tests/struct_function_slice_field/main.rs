use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct queue {
    pub later: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>>>>,
}

impl queue {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.later.clone();
        Self {
            later: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for queue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.later.borrow(); match __guard.as_ref() { Some(__v) => format!("[{}]", std::iter::repeat("<func>").take(__v.len()).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    let mut q = Rc::new(RefCell::new(Some(queue { later: Rc::new(RefCell::new(Some(Vec::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>::new()))), ..Default::default() })));
    println!("{}", format!("{}", (*q.borrow().as_ref().unwrap())));
}