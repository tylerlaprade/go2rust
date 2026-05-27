use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Expr: std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool;
    fn pos(&self) -> i32;
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Self {
        self.__go_clone_box_expr()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Ident {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct StarExpr {
    pub x: Rc<RefCell<Option<Box<dyn Expr>>>>,
}

impl StarExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone() }
    }
}

impl std::fmt::Display for StarExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.x.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct Field {
    pub r#type: Rc<RefCell<Option<Box<dyn Expr>>>>,
}

impl Field {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: self.r#type.clone() }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.r#type.borrow().as_ref().unwrap()))
    }
}


impl Ident {
    pub fn pos(&self) -> i32 {
        return 1 as i32;
    }
}

impl Expr for Ident {
    fn pos(&self) -> i32 {
        return 1 as i32;
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

impl StarExpr {
    pub fn pos(&self) -> i32 {
        return 2 as i32;
    }
}

impl Expr for StarExpr {
    fn pos(&self) -> i32 {
        return 2 as i32;
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

pub fn unwrap(fields: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Field>>>>>>>) -> Rc<RefCell<Option<String>>> {

    let mut t = Rc::new(RefCell::new(Some((*(*(*fields.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).r#type.borrow().as_ref().unwrap()).clone())));
    {
        let (mut p, _) = ({
        let val = t.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.__go_as_any().downcast_ref::<StarExpr>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(None::<StarExpr>)), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(None::<StarExpr>)), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*p.borrow()).is_some() {
            t = (*p.borrow().as_ref().unwrap()).x.clone();;
        }
    }
    {
        let (mut id, _) = ({
        let val = t.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.__go_as_any().downcast_ref::<Ident>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(None::<Ident>)), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(None::<Ident>)), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*id.borrow()).is_some() {
            return Rc::new(RefCell::new(Some({ let __selector_holder = (*id.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        }
    }
    return Rc::new(RefCell::new(Some("?".to_string())));
}

fn main() {
    let mut fields = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Field { r#type: Rc::new(RefCell::new(Some(Box::new(StarExpr { x: Rc::new(RefCell::new(Some(Box::new(Ident { name: Rc::new(RefCell::new(Some("hello".to_string()))), ..Default::default() }) as Box<dyn Expr>))), ..Default::default() }) as Box<dyn Expr>))), ..Default::default() })))])));
    println!("{}", format!("{}", (*unwrap(fields.clone()).borrow().as_ref().unwrap())));
}