use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Expr: std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool;
    fn pos(&self) -> i64;
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Self {
        Expr::__go_clone_box_expr(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Ident {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct StarExpr {
    pub x: Rc<RefCell<Option<Box<dyn Expr>>>>,
}

impl StarExpr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.x.clone();
        Self {
            x: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for StarExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct Field {
    pub r#type: Rc<RefCell<Option<Box<dyn Expr>>>>,
}

impl Field {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.r#type.clone();
        Self {
            r#type: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Ident {
    pub fn pos(&self) -> i64 {
        1
    }
}

impl Expr for Ident {
    fn pos(&self) -> i64 {
        Ident::pos(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Ident>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct IdentPtr(pub Rc<RefCell<Option<Ident>>>);

impl std::fmt::Display for IdentPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for IdentPtr {
    fn pos(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Ident::pos(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IdentPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl StarExpr {
    pub fn pos(&self) -> i64 {
        2
    }
}

impl Expr for StarExpr {
    fn pos(&self) -> i64 {
        StarExpr::pos(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StarExpr>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct StarExprPtr(pub Rc<RefCell<Option<StarExpr>>>);

impl std::fmt::Display for StarExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for StarExprPtr {
    fn pos(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        StarExpr::pos(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<StarExprPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn unwrap(fields: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Field>>>>>>>) -> Rc<RefCell<Option<String>>> {
    let mut t = (*(*fields.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).r#type.clone();
    {
        let (mut p, _) = ({
        let val = t.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Expr>::__go_as_any(any_val.as_ref()).downcast_ref::<StarExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<StarExpr>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<StarExpr>)), false)
        }
    });;
        if { let __nil_result = (*p.borrow()).is_some(); __nil_result } {
            { let __iface_handle = { let __field = (*p.borrow().as_ref().unwrap()).x.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.borrow(); (*__iface_guard).clone() }; *t.borrow_mut() = __iface_value; };;
        }
    }
    {
        let (mut id, _) = ({
        let val = t.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Expr>::__go_as_any(any_val.as_ref()).downcast_ref::<IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Ident>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Ident>)), false)
        }
    });;
        if { let __nil_result = (*id.borrow()).is_some(); __nil_result } {
            return Rc::new(RefCell::new(Some({ let __selector_holder = (*id.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        }
    }
    Rc::new(RefCell::new(Some("?".to_string())))
}

fn main() {
    let mut fields = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Field { r#type: Rc::new(RefCell::new(Some(Box::new(StarExprPtr(Rc::new(RefCell::new(Some(StarExpr { x: Rc::new(RefCell::new(Some(Box::new(IdentPtr(Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("hello".to_string()))), ..Default::default() }))).clone())) as Box<dyn Expr>))), ..Default::default() }))).clone())) as Box<dyn Expr>))), ..Default::default() })))])));
    println!("{}", format!("{}", (*unwrap(fields.clone()).borrow().as_ref().unwrap())));
}