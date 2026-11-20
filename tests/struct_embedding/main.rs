use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait describer: std::fmt::Display + Any {
    fn __go_clone_box_describer(&self) -> Box<dyn describer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_describer(&self, other: &dyn describer) -> bool;
    fn describe(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn describer> {
    fn clone(&self) -> Self {
        describer::__go_clone_box_describer(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct base {
    pub num: Rc<RefCell<Option<i64>>>,
}

impl base {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.num.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            num: __go_clone_0_0,
        }
    }
}


impl Default for base {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            num: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.num.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct container {
    pub base: Rc<RefCell<Option<base>>>,
    pub str: Rc<RefCell<Option<String>>>,
}

impl container {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.str.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            str: __go_clone_1_0,
        }
    }
}


impl Default for container {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(base::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            base: __go_default_0_0,
            str: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.base.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.str.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl base {
    pub fn describe(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("base with num={}", (*self.num.borrow().as_ref().unwrap())))))
    }
}

impl describer for base {
    fn describe(&self) -> Rc<RefCell<Option<String>>> {
        base::describe(self)
    }
    fn __go_clone_box_describer(&self) -> Box<dyn describer> {
        Box::new(self.clone()) as Box<dyn describer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_describer(&self, other: &dyn describer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<base>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct basePtr(pub Rc<RefCell<Option<base>>>);

impl std::fmt::Display for basePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl describer for basePtr {
    fn describe(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        base::describe(__recv)
    }
    fn __go_clone_box_describer(&self) -> Box<dyn describer> {
        Box::new(self.clone()) as Box<dyn describer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_describer(&self, other: &dyn describer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<basePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
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

impl describer for container {
    fn describe(&self) -> Rc<RefCell<Option<String>>> {
        container::describe(self)
    }
    fn __go_clone_box_describer(&self) -> Box<dyn describer> {
        Box::new(self.clone()) as Box<dyn describer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_describer(&self, other: &dyn describer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<container>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct containerPtr(pub Rc<RefCell<Option<container>>>);

impl std::fmt::Display for containerPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl describer for containerPtr {
    fn describe(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        container::describe(__recv)
    }
    fn __go_clone_box_describer(&self) -> Box<dyn describer> {
        Box::new(self.clone()) as Box<dyn describer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_describer(&self, other: &dyn describer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<containerPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    let mut co = Rc::new(RefCell::new(Some(container { base: Rc::new(RefCell::new(Some(base { num: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), str: Rc::new(RefCell::new(Some("some name".to_string()))), ..Default::default() })));

    print!("co={{num: {}, str: {}}}\n", (*(*(*co.borrow().as_ref().unwrap()).base.borrow().as_ref().unwrap()).num.borrow().as_ref().unwrap()), (*(*co.borrow().as_ref().unwrap()).str.borrow().as_ref().unwrap()).clone());
    println!("{} {}", format!("{}", "also num:".to_string()), format!("{}", (*(*(*co.borrow().as_ref().unwrap()).base.borrow().as_ref().unwrap()).num.borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "describe:".to_string()), format!("{}", (*(*co.borrow().as_ref().unwrap()).describe().borrow().as_ref().unwrap())));

    

    let mut d = Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = co.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn describer>)));
    println!("{} {}", format!("{}", "describer:".to_string()), format!("{}", (*(*d.borrow().as_ref().unwrap()).describe().borrow().as_ref().unwrap())));
}