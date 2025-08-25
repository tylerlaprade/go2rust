use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct base {
    num: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.num.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct container {
    base: Rc<RefCell<Option<base>>>,
    str: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.str.borrow().as_ref().unwrap()))
    }
}


impl base {
    pub fn describe(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(format!("base with num={}", (*self.num.borrow().as_ref().unwrap()))))))));
    }
}

impl container {
    pub fn describe(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.describe()
    }
}

fn main() {
    let mut co = Rc::new(RefCell::new(Some(container { base: Rc::new(RefCell::new(Some(base { num: Rc::new(RefCell::new(Some(1))) }))), str: Rc::new(RefCell::new(Some("some name".to_string()))) })));

    print!("co={{num: {}, str: {}}}\n", (*(*(*co.borrow().as_ref().unwrap()).base.borrow().as_ref().unwrap()).num.borrow().as_ref().unwrap()), (*(*co.borrow().as_ref().unwrap()).str.borrow().as_ref().unwrap()));
    println!("{} {}", "also num:".to_string(), (*(*(*(*co.borrow_mut().as_mut().unwrap()).base.borrow_mut().as_mut().unwrap()).base.borrow().as_ref().unwrap()).num.borrow().as_ref().unwrap()));
    println!("{} {}", "describe:".to_string(), (*(*co.borrow_mut().as_mut().unwrap()).describe().borrow().as_ref().unwrap()));

    type describer = Rc<RefCell<Option<Unknown>>>;

    let mut d: Rc<RefCell<Option<describer>>> = Rc::new(RefCell::new(Some((*co.borrow_mut().as_mut().unwrap()))));
    println!("{} {}", "describer:".to_string(), (*(*d.borrow_mut().as_mut().unwrap()).describe().borrow().as_ref().unwrap()));
}