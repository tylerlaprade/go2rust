use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};


#[derive(Clone)]
pub struct GoLocalPtrKey<T>(pub Rc<RefCell<Option<T>>>);

impl<T> GoLocalPtrKey<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self { GoLocalPtrKey(value) }
    pub fn value(&self) -> Rc<RefCell<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoLocalPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoLocalPtrKey<T> {}
impl<T> PartialOrd for GoLocalPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoLocalPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_File;

impl std::fmt::Display for ast_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_File>")
    }
}


impl ast_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct types_Info {
    pub file_versions: Rc<RefCell<Option<BTreeMap<GoLocalPtrKey<ast_File>, Rc<RefCell<Option<String>>>>>>>,
}

impl std::fmt::Display for types_Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Info>")
    }
}


impl types_Info {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub fn version(info: Rc<RefCell<Option<types_Info>>>, file: Rc<RefCell<Option<ast_File>>>) -> Rc<RefCell<Option<String>>> {

    let mut v = Rc::new(RefCell::new(Some((*(*info.borrow().as_ref().unwrap()).file_versions.borrow().as_ref().unwrap()).get(&GoLocalPtrKey::new(file.clone())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()))));
    if (*v.borrow().as_ref().unwrap()).clone() != "" {
        return v.clone();
    }
    return Rc::new(RefCell::new(Some("".to_string())));
}

fn main() {
    let mut file = Rc::new(RefCell::new(Some(ast_File { ..Default::default() })));
    let mut info = Rc::new(RefCell::new(Some(types_Info { file_versions: Rc::new(RefCell::new(Some(BTreeMap::<GoLocalPtrKey<ast_File>, Rc<RefCell<Option<String>>>>::from([(GoLocalPtrKey::new(file.clone()), Rc::new(RefCell::new(Some("go1.22".to_string()))))])))), ..Default::default() })));
    println!("{}", (*version(info.clone(), file.clone()).borrow().as_ref().unwrap()));
}