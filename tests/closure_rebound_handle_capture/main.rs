use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait expr: std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn expr>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_expr(&self, other: &dyn expr) -> bool;
    fn expr_node(&self);
}

impl Clone for Box<dyn expr> {
    fn clone(&self) -> Self {
        expr::__go_clone_box_expr(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ident {
    pub name: Rc<RefCell<Option<String>>>,
}

impl ident {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for ident {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl ident {
    pub fn expr_node(&self) {
    }
}

impl expr for ident {
    fn expr_node(&self) {
        ident::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn expr> {
        Box::new(self.clone()) as Box<dyn expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ident>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct identPtr(pub Rc<RefCell<Option<ident>>>);

impl std::fmt::Display for identPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl expr for identPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        ident::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn expr> {
        Box::new(self.clone()) as Box<dyn expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<identPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn grouped() -> i32 {
    let mut names: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    let mut typ: Rc<RefCell<Option<Box<dyn expr>>>> = Rc::new(RefCell::new(None));
    let mut source: Rc<RefCell<Option<Box<dyn expr>>>> = Rc::new(RefCell::new(None));
    let mut names_closure_clone = names.clone(); let typ_closure_clone = typ.clone(); let mut add = Rc::new(RefCell::new(Some(Box::new(move || -> i32 {
        if { let __nil_result = (*typ_closure_clone.borrow()).is_none(); __nil_result } {
        return -(1);
    }
        let mut n = Rc::new(RefCell::new(Some((*names_closure_clone.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        *names_closure_clone.borrow_mut() = None;
        return (*n.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> i32>)));

    { let new_val = { let __append_target = names.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(vec!["a".to_string(), "b".to_string()]); __append_target.clone() }; names = new_val; };
    { let __iface_handle = Rc::new(RefCell::new(Some(Box::new(identPtr(Rc::new(RefCell::new(Some(ident { name: Rc::new(RefCell::new(Some("int".to_string()))), ..Default::default() }))).clone())) as Box<dyn expr>))); let __iface_value = { let __iface_guard = __iface_handle.borrow(); (*__iface_guard).clone() }; *source.borrow_mut() = __iface_value; };
    { let __iface_handle = source.clone(); let __iface_value = { let __iface_guard = __iface_handle.borrow(); (*__iface_guard).clone() }; *typ.borrow_mut() = __iface_value; };
    return { let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = add.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
}

fn main() {
    println!("{}", format!("{}", grouped()));
}