use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_File;

impl std::fmt::Display for ast_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_File>")
    }
}


#[derive(Debug, Clone, Default)]
pub struct types_Info {
    pub file_versions: Rc<RefCell<Option<BTreeMap<Rc<RefCell<Option<ast_File>>>, Rc<RefCell<Option<String>>>>>>>,
}

impl std::fmt::Display for types_Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Info>")
    }
}


pub fn version(info: Rc<RefCell<Option<types_Info>>>, file: Rc<RefCell<Option<ast_File>>>) -> Rc<RefCell<Option<String>>> {

    let mut v = Rc::new(RefCell::new(Some((*(*info.borrow().as_ref().unwrap()).file_versions.borrow().as_ref().unwrap()).get(&file).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()))));
    if (*v.borrow().as_ref().unwrap()) != "" {
        return v.clone();
    }
    return Rc::new(RefCell::new(Some("".to_string())));
}

fn main() {
    let mut file = Rc::new(RefCell::new(Some(ast_File { ..Default::default() })));
    let mut info = Rc::new(RefCell::new(Some(types_Info { file_versions: Rc::new(RefCell::new(Some(BTreeMap::<Rc<RefCell<Option<ast_File>>>, Rc<RefCell<Option<String>>>>::from([(file.clone(), Rc::new(RefCell::new(Some("go1.22".to_string()))))])))), ..Default::default() })));
    println!("{}", (*version(info.clone(), file.clone()).borrow().as_ref().unwrap()));
}