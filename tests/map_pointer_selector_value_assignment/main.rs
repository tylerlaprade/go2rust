use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Package {
    pub i_d: Rc<RefCell<Option<String>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { i_d: { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Package {
    fn default() -> Self {
        Self { i_d: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.i_d.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct loaderPackage {
    pub package: Rc<RefCell<Option<Package>>>,
}

impl loaderPackage {
    pub fn __go_value_clone(&self) -> Self {
        Self { package: self.package.clone() }
    }
}

impl std::fmt::Display for loaderPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.package.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut pkgs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<loaderPackage>>>>::from([("dep".to_string(), Rc::new(RefCell::new(Some(loaderPackage { package: Rc::new(RefCell::new(Some(Package { i_d: Rc::new(RefCell::new(Some("dep".to_string()))), ..Default::default() }))).clone(), ..Default::default() }))).clone())]))));
    let mut imports = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Package>>>>::from([]))));

    let mut imp = (*pkgs.borrow().as_ref().unwrap()).get(&"dep".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default());
    { let __map_key = "dep".to_string(); let __map_value = (*imp.borrow().as_ref().unwrap()).package.clone(); (*imports.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let new_val = "updated".to_string(); *(*(*imp.borrow().as_ref().unwrap()).package.borrow().as_ref().unwrap()).i_d.borrow_mut() = Some(new_val); };

    println!("{}", format!("{}", (*(*(*imports.borrow().as_ref().unwrap()).get(&"dep".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", { let __left = (*imports.borrow().as_ref().unwrap()).get(&"dep".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()); let __right = (*imp.borrow().as_ref().unwrap()).package.clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right); __eq }));
}