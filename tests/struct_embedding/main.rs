use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait describer: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn describer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn describer) -> bool;
    fn describe(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn describer> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct base {
    pub num: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.num.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct container {
    pub base: Rc<RefCell<Option<base>>>,
    pub str: Rc<RefCell<Option<String>>>,
}


impl Default for container {
    fn default() -> Self {
        Self { base: Rc::new(RefCell::new(Some(base::default()))), str: Default::default() }
    }
}

impl std::fmt::Display for container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.borrow().as_ref().unwrap()), (*self.str.borrow().as_ref().unwrap()))
    }
}


impl base {
    pub fn describe(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("base with num={}", (*self.num.borrow().as_ref().unwrap())))));
    }
}

impl describer for base {
    fn describe(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("base with num={}", (*self.num.borrow().as_ref().unwrap())))));
    }
    fn __go_clone_box(&self) -> Box<dyn describer> {
        Box::new(self.clone()) as Box<dyn describer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn describer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<base>() {
            self == __other
        } else {
            false
        }
    }
}

impl container {
    pub fn describe(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.describe()
    }
}

fn main() {
    let mut co = Rc::new(RefCell::new(Some(container { base: Rc::new(RefCell::new(Some(base { num: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), str: Rc::new(RefCell::new(Some("some name".to_string()))), ..Default::default() })));

    print!("co={{num: {}, str: {}}}\n", (*(*(*co.borrow().as_ref().unwrap()).base.borrow().as_ref().unwrap()).num.borrow().as_ref().unwrap()), (*(*co.borrow().as_ref().unwrap()).str.borrow().as_ref().unwrap()));
    println!("{} {}", "also num:".to_string(), (*(*(*co.borrow().as_ref().unwrap()).base.borrow().as_ref().unwrap()).num.borrow().as_ref().unwrap()));
    println!("{} {}", "describe:".to_string(), (*(*co.borrow().as_ref().unwrap()).describe().borrow().as_ref().unwrap()));

    

    let mut d = co.clone();
    println!("{} {}", "describer:".to_string(), (*(*d.borrow().as_ref().unwrap()).describe().borrow().as_ref().unwrap()));
}