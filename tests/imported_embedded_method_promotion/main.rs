use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct Reader {
    pub decoder: Rc<RefCell<Option<example_com_importedembed_base::Decoder>>>,
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.decoder.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()))
    }
}


impl Reader {
    pub fn add(&self, _arg0: Rc<RefCell<Option<i32>>>) {
        let embedded = self.decoder.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.add(_arg0)
    }

    pub fn label(&self, _arg0: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let embedded = self.decoder.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.label(_arg0)
    }

    pub fn snapshot(&self) -> Rc<RefCell<Option<i32>>> {
        let embedded = self.decoder.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.snapshot()
    }
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(Reader { decoder: Rc::new(RefCell::new(Some(example_com_importedembed_base::Decoder { value: Rc::new(RefCell::new(Some(3))), ..Default::default() }))), name: Rc::new(RefCell::new(Some("reader".to_string()))), ..Default::default() })));
    (*r.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(4))));
    println!("{}", (*(*r.borrow_mut().as_mut().unwrap()).label(Rc::new(RefCell::new(Some("reader".to_string())))).borrow().as_ref().unwrap()));
    println!("{}", (*(*r.borrow().as_ref().unwrap()).snapshot().borrow().as_ref().unwrap()));
}