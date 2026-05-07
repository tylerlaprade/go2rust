use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
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


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct parser_Mode;

impl std::fmt::Display for parser_Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<parser_Mode>")
    }
}


impl parser_Mode {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_FileSet;

impl std::fmt::Display for token_FileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_FileSet>")
    }
}


impl token_FileSet {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Alias;

impl std::fmt::Display for types_Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Alias>")
    }
}


impl types_Alias {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic;

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Basic>")
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Config;

impl std::fmt::Display for types_Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Config>")
    }
}


impl types_Config {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn check<T0, T1, T2, T3>(&self, _arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> (Arc<Mutex<Option<types_Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<types_Package>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Info;

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


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Package;

impl std::fmt::Display for types_Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Package>")
    }
}


impl types_Package {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Type;

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}


impl types_Type {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod binary {
    use super::*;
    pub const max_varint_len64: i32 = 0;
}


pub mod parser {
    use super::*;
    pub const skip_object_resolution: parser_Mode = parser_Mode;

    pub fn parse_file<T0, T1, T2, T3>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> (Arc<Mutex<Option<ast_File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<ast_File>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


pub mod token {
    use super::*;
    pub fn new_file_set() -> Arc<Mutex<Option<token_FileSet>>> {
        Arc::new(Mutex::new(Some::<token_FileSet>(Default::default())))
    }
}


pub mod types {
    use super::*;
    pub fn typ() -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<types_Basic>>>>>>> {
        Arc::new(Mutex::new(Some::<Vec<Arc<Mutex<Option<types_Basic>>>>>(Default::default())))
    }

    pub fn unalias<T0>(_arg0: T0) -> Arc<Mutex<Option<types_Type>>> {
        Arc::new(Mutex::new(Some::<types_Type>(Default::default())))
    }
}


fn main() {
    if false {
        let mut fset = token::new_file_set();
        let (mut f, _) = parser::parse_file(fset.clone(), "a.go".to_string(), "package p; type A = int".to_string(), parser::skip_object_resolution);
        { let (__tmp_0, __tmp_1) = { let __recv = Arc::new(Mutex::new(Some(types_Config::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).check("p".to_string(), fset.clone(), Arc::new(Mutex::new(Some(vec![(*f.lock().unwrap().as_ref().unwrap()).clone()]))), Arc::new(Mutex::new(Some(types_Info::default())))); __result }; };
        let mut alias: Arc<Mutex<Option<types_Alias>>> = Arc::new(Mutex::new(None));
        let _ = types::unalias(alias.clone());
        let _ = binary::max_varint_len64;
        let _ = types::typ();
    }
    println!("{}", "ok".to_string());
}