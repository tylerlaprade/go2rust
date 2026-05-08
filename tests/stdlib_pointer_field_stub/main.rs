use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_Ident;

impl std::fmt::Display for ast_Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Ident>")
    }
}


impl ast_Ident {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_SelectorExpr {
    pub sel: Rc<RefCell<Option<ast_Ident>>>,
}

impl std::fmt::Display for ast_SelectorExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SelectorExpr>")
    }
}


impl ast_SelectorExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub fn pick(sel: Rc<RefCell<Option<ast_SelectorExpr>>>) -> Rc<RefCell<Option<ast_Ident>>> {

    return (*sel.borrow().as_ref().unwrap()).sel.clone();
}

fn main() {
    if false {
        println!("{}", format!("&{}", (*pick(Rc::new(RefCell::new(Some(ast_SelectorExpr { ..Default::default() })))).borrow().as_ref().unwrap())));
    }
    println!("{}", "ok".to_string());
}