use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Expr: std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool;
    fn expr_node(&self);
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Self {
        self.__go_clone_box_expr()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChanType {
    pub dir: Rc<RefCell<Option<i32>>>,
}

impl ChanType {
    pub fn __go_value_clone(&self) -> Self {
        Self { dir: { let __guard = self.dir.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for ChanType {
    fn default() -> Self {
        Self { dir: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for ChanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.dir.borrow().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ChanType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Dir") {
            out.dir = <Rc<RefCell<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
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

impl GoJsonDecode for Ident {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Rc<RefCell<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct Field {
    pub value: Rc<RefCell<Option<Box<dyn Expr>>>>,
}

impl Field {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Field {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl ChanType {
    pub fn expr_node(&self) {
    }
}

impl Expr for ChanType {
    fn expr_node(&self) {
        self.expr_node()
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ChanType>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ident {
    pub fn expr_node(&self) {
    }
}

impl Expr for Ident {
    fn expr_node(&self) {
        self.expr_node()
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