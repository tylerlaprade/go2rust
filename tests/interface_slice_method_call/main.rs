use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}


fn format_slice_wrapped_stringer<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        format_slice_wrapped_stringer_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}

fn format_slice_wrapped_stringer_values<T>(slice: &[Rc<RefCell<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.borrow();
        match inner.as_ref() {
            Some(value) => value.to_string(),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}

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
pub struct Lit {
    pub p: Rc<RefCell<Option<i32>>>,
}

impl Lit {
    pub fn __go_value_clone(&self) -> Self {
        Self { p: { let __guard = self.p.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Lit {
    fn default() -> Self {
        Self { p: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.p.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct Assign {
    pub lhs: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Expr>>>>>>>>,
}

impl Assign {
    pub fn __go_value_clone(&self) -> Self {
        Self { lhs: self.lhs.clone() }
    }
}

impl std::fmt::Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped_stringer(&self.lhs))
    }
}


impl Lit {
    pub fn pos(&self) -> i32 {
        return (*self.p.borrow().as_ref().unwrap());
    }
}

impl Expr for Lit {
    fn pos(&self) -> i32 {
        return (*self.p.borrow().as_ref().unwrap());
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Lit>() {
            self == __other
        } else {
            false
        }
    }
}

impl Assign {
    /// Mirrors ast.AssignStmt.Pos(): method body indexes the wrapped
    /// interface slice field and calls a trait method on the result.
    pub fn first_pos(&self) -> i32 {
        return { let __recv = (*self.lhs.borrow().as_ref().unwrap())[(0) as usize].clone(); let __result = (*__recv.borrow().as_ref().unwrap()).pos(); __result };
    }
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Assign { lhs: Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Lit { p: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() }) as Box<dyn Expr>))), Rc::new(RefCell::new(Some(Box::new(Lit { p: Rc::new(RefCell::new(Some(9 as i32))), ..Default::default() }) as Box<dyn Expr>)))]))), ..Default::default() })));
    println!("{}", format!("{}", (*a.borrow().as_ref().unwrap()).first_pos()));
}