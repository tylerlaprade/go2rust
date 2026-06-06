use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_ChanDir(pub i32);

impl PartialEq<i32> for ast_ChanDir {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ast_ChanDir> for i32 {
    fn eq(&self, other: &ast_ChanDir) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for ast_ChanDir {
    type Output = ast_ChanDir;
    fn bitand(self, other: Self) -> ast_ChanDir {
        ast_ChanDir(self.0 & other.0)
    }
}

impl std::ops::BitOr for ast_ChanDir {
    type Output = ast_ChanDir;
    fn bitor(self, other: Self) -> ast_ChanDir {
        ast_ChanDir(self.0 | other.0)
    }
}

impl std::fmt::Display for ast_ChanDir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ChanDir>")
    }
}


impl ast_ChanDir {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct parser_Mode(pub u64);

impl PartialEq<u64> for parser_Mode {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<parser_Mode> for u64 {
    fn eq(&self, other: &parser_Mode) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for parser_Mode {
    type Output = parser_Mode;
    fn bitand(self, other: Self) -> parser_Mode {
        parser_Mode(self.0 & other.0)
    }
}

impl std::ops::BitOr for parser_Mode {
    type Output = parser_Mode;
    fn bitor(self, other: Self) -> parser_Mode {
        parser_Mode(self.0 | other.0)
    }
}

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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
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


pub mod ast {
    use super::*;
    pub const R_E_C_V: ast_ChanDir = ast_ChanDir(2);
    pub const S_E_N_D: ast_ChanDir = ast_ChanDir(1);
}


pub mod binary {
    use super::*;
    pub const MAX_VARINT_LEN64: i32 = 10;
}


pub mod parser {
    use super::*;
    pub const SKIP_OBJECT_RESOLUTION: parser_Mode = parser_Mode(64);

    pub fn parse_file<T0, T1, T2, T3>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> (Arc<Mutex<Option<ast_File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("parse_file bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod token {
    use super::*;
    pub fn new_file_set() -> Arc<Mutex<Option<token_FileSet>>> {
        panic!("new_file_set bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


fn main() {
    if false {
        let mut fset = token::new_file_set();
        { let (__tmp_0, __tmp_1) = parser::parse_file(fset.clone(), "a.go".to_string(), "package p; type A = int".to_string(), parser::SKIP_OBJECT_RESOLUTION); };
        let _ = binary::MAX_VARINT_LEN64;
        let mut dir = Arc::new(Mutex::new(Some(ast::S_E_N_D)));
        { let new_val = ast_ChanDir((((ast::S_E_N_D).0 as i32) | ((ast::R_E_C_V).0 as i32)) as i32); *dir.lock().unwrap() = Some(new_val); };
        let _ = { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    println!("{}", format!("{}", "ok".to_string()));
}