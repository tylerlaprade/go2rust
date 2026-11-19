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
        Decl::__go_clone_box_decl(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDecl {
    pub name: Rc<RefCell<Option<String>>>,
}

impl FuncDecl {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for FuncDecl {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for FuncDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl FuncDecl {
    pub fn decl_name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Decl for FuncDecl {
    fn decl_name(&self) -> Rc<RefCell<Option<String>>> {
        FuncDecl::decl_name(self)
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

#[derive(Clone)]
pub struct FuncDeclPtr(pub Rc<RefCell<Option<FuncDecl>>>);

impl std::fmt::Display for FuncDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Decl for FuncDeclPtr {
    fn decl_name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        FuncDecl::decl_name(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn Decl> {
        Box::new(self.clone()) as Box<dyn Decl>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &dyn Decl) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncDeclPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    let mut decls: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Decl>>>>>>>> = Rc::new(RefCell::new(Some(vec![Default::default(); (3) as usize])));
    (*decls.borrow_mut().as_mut().unwrap())[(0) as usize] = Rc::new(RefCell::new(Some(Box::new(FuncDeclPtr(Rc::new(RefCell::new(Some(FuncDecl { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() }))).clone())) as Box<dyn Decl>)));
    (*decls.borrow_mut().as_mut().unwrap())[(2) as usize] = Rc::new(RefCell::new(Some(Box::new(FuncDeclPtr(Rc::new(RefCell::new(Some(FuncDecl { name: Rc::new(RefCell::new(Some("c".to_string()))), ..Default::default() }))).clone())) as Box<dyn Decl>)));

        // Read with nil check pattern.
    { let __range_holder = decls.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, d) in __range_values.iter().enumerate() {
        if { let __nil_result = (*d.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", i), format!("{}", (*(*d.borrow().as_ref().unwrap()).decl_name().borrow().as_ref().unwrap())));
    } else {
        println!("{} {}", format!("{}", i), format!("{}", "<nil>".to_string()));
    }
    } }

        // Reassign nil.
    (*decls.borrow_mut().as_mut().unwrap())[(0) as usize] = Rc::new(RefCell::new(None));
    { let __range_holder = decls.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, d) in __range_values.iter().enumerate() {
        if { let __nil_result = (*d.borrow()).is_none(); __nil_result } {
        println!("{} {}", format!("{}", i), format!("{}", "<nil>".to_string()));
    } else {
        println!("{} {}", format!("{}", i), format!("{}", (*(*d.borrow().as_ref().unwrap()).decl_name().borrow().as_ref().unwrap())));
    }
    } }
}