use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Expr: std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool;
    fn kind(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Self {
        Expr::__go_clone_box_expr(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct lit {
    pub v: Rc<RefCell<Option<String>>>,
}

impl lit {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.v.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl Default for lit {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            v: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.v.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct checker {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl checker {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for checker {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for checker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl lit {
    pub fn kind(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.v.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl Expr for lit {
    fn kind(&self) -> Rc<RefCell<Option<String>>> {
        lit::kind(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<lit>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct litPtr(pub Rc<RefCell<Option<lit>>>);

impl std::fmt::Display for litPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for litPtr {
    fn kind(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        lit::kind(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<litPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl checker {
    pub fn r#use(&mut self, args: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Expr>>>>>>>>) {
        { let __range_holder = args.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for a in __range_values.iter() {
        { let __target = self.n.clone(); let __rhs = (*(*a.borrow().as_ref().unwrap()).kind().borrow().as_ref().unwrap()).len() as i64; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    }

    pub fn run(&mut self, rhs: Rc<RefCell<Option<Box<dyn Expr>>>>) {
        self.r#use(Rc::new(RefCell::new(Some(vec![rhs.clone()]))));
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(checker { n: Rc::new(RefCell::new(Some(0))) })));
    (*c.borrow_mut().as_mut().unwrap()).run(Rc::new(RefCell::new(Some(Box::new(litPtr(Rc::new(RefCell::new(Some(lit { v: Rc::new(RefCell::new(Some("abc".to_string()))), ..Default::default() }))).clone())) as Box<dyn Expr>))));
    println!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));
}