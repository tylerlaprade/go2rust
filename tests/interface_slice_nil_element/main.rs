use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Decl: std::fmt::Display + Any {
    fn __go_clone_box_decl(&self) -> Box<dyn Decl>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_decl(&self, other: &dyn Decl) -> bool;
    fn decl_name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Decl> {
    fn clone(&self) -> Self {
        self.__go_clone_box_decl()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDecl {
    pub name: Rc<RefCell<Option<String>>>,
}

impl FuncDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for FuncDecl {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for FuncDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl FuncDecl {
    pub fn decl_name(&self) -> Rc<RefCell<Option<String>>> {
        self.name.clone()
    }
}

impl Decl for FuncDecl {
    fn decl_name(&self) -> Rc<RefCell<Option<String>>> {
        self.name.clone()
    }
    fn __go_clone_box_decl(&self) -> Box<dyn Decl> {
        Box::new(self.clone()) as Box<dyn Decl>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &dyn Decl) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncDecl>() {
            self == __other
        } else {
            false
        }
    }
}

fn main() {
    let mut decls: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Decl>>>>>>>> = Rc::new(RefCell::new(Some(vec![Default::default(); (3) as usize])));
    (*decls.borrow_mut().as_mut().unwrap())[(0) as usize] = Rc::new(RefCell::new(Some(Box::new(FuncDecl { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() }) as Box<dyn Decl>)));
    (*decls.borrow_mut().as_mut().unwrap())[(2) as usize] = Rc::new(RefCell::new(Some(Box::new(FuncDecl { name: Rc::new(RefCell::new(Some("c".to_string()))), ..Default::default() }) as Box<dyn Decl>)));

        // Read with nil check pattern.
    { let __range_holder = decls.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, d) in __range_values.iter().enumerate() {
        if (*d.borrow()).is_some() {
        println!("{} {}", format!("{}", i), format!("{}", (*(*d.borrow().as_ref().unwrap()).decl_name().borrow().as_ref().unwrap())));
    } else {
        println!("{} {}", format!("{}", i), format!("{}", "<nil>".to_string()));
    }
    } }

        // Reassign nil.
    (*decls.borrow_mut().as_mut().unwrap())[(0) as usize] = Rc::new(RefCell::new(None));
    { let __range_holder = decls.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, d) in __range_values.iter().enumerate() {
        if (*d.borrow()).is_none() {
        println!("{} {}", format!("{}", i), format!("{}", "<nil>".to_string()));
    } else {
        println!("{} {}", format!("{}", i), format!("{}", (*(*d.borrow().as_ref().unwrap()).decl_name().borrow().as_ref().unwrap())));
    }
    } }
}