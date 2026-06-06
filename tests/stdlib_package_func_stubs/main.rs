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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_Token(pub i32);

impl PartialEq<i32> for token_Token {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<token_Token> for i32 {
    fn eq(&self, other: &token_Token) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for token_Token {
    type Output = token_Token;
    fn bitand(self, other: Self) -> token_Token {
        token_Token(self.0 & other.0)
    }
}

impl std::ops::BitOr for token_Token {
    type Output = token_Token;
    fn bitor(self, other: Self) -> token_Token {
        token_Token(self.0 | other.0)
    }
}

impl std::fmt::Display for token_Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_Token>")
    }
}


impl token_Token {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod ast {
    use super::*;

    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

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

    pub const A_D_D: token_Token = token_Token(12);
    pub const A_D_D__A_S_S_I_G_N: token_Token = token_Token(23);
    pub const A_N_D: token_Token = token_Token(17);
    pub const A_N_D__A_S_S_I_G_N: token_Token = token_Token(28);
    pub const A_N_D__N_O_T: token_Token = token_Token(22);
    pub const A_N_D__N_O_T__A_S_S_I_G_N: token_Token = token_Token(33);
    pub const A_R_R_O_W: token_Token = token_Token(36);
    pub const A_S_S_I_G_N: token_Token = token_Token(42);
    pub const B_R_E_A_K: token_Token = token_Token(61);
    pub const C_A_S_E: token_Token = token_Token(62);
    pub const C_H_A_N: token_Token = token_Token(63);
    pub const C_H_A_R: token_Token = token_Token(8);
    pub const C_O_L_O_N: token_Token = token_Token(58);
    pub const C_O_M_M_A: token_Token = token_Token(52);
    pub const C_O_M_M_E_N_T: token_Token = token_Token(2);
    pub const C_O_N_S_T: token_Token = token_Token(64);
    pub const C_O_N_T_I_N_U_E: token_Token = token_Token(65);
    pub const D_E_C: token_Token = token_Token(38);
    pub const D_E_F_A_U_L_T: token_Token = token_Token(66);
    pub const D_E_F_E_R: token_Token = token_Token(67);
    pub const D_E_F_I_N_E: token_Token = token_Token(47);
    pub const E_L_L_I_P_S_I_S: token_Token = token_Token(48);
    pub const E_L_S_E: token_Token = token_Token(68);
    pub const E_O_F: token_Token = token_Token(1);
    pub const E_Q_L: token_Token = token_Token(39);
    pub const F_A_L_L_T_H_R_O_U_G_H: token_Token = token_Token(69);
    pub const F_L_O_A_T: token_Token = token_Token(6);
    pub const F_O_R: token_Token = token_Token(70);
    pub const F_U_N_C: token_Token = token_Token(71);
    pub const G_E_Q: token_Token = token_Token(46);
    pub const G_O: token_Token = token_Token(72);
    pub const G_O_T_O: token_Token = token_Token(73);
    pub const G_T_R: token_Token = token_Token(41);
    pub const I_D_E_N_T: token_Token = token_Token(4);
    pub const I_F: token_Token = token_Token(74);
    pub const I_L_L_E_G_A_L: token_Token = token_Token(0);
    pub const I_M_A_G: token_Token = token_Token(7);
    pub const I_M_P_O_R_T: token_Token = token_Token(75);
    pub const I_N_C: token_Token = token_Token(37);
    pub const I_N_T: token_Token = token_Token(5);
    pub const I_N_T_E_R_F_A_C_E: token_Token = token_Token(76);
    pub const L_A_N_D: token_Token = token_Token(34);
    pub const L_B_R_A_C_E: token_Token = token_Token(51);
    pub const L_B_R_A_C_K: token_Token = token_Token(50);
    pub const L_E_Q: token_Token = token_Token(45);
    pub const L_O_R: token_Token = token_Token(35);
    pub const L_P_A_R_E_N: token_Token = token_Token(49);
    pub const L_S_S: token_Token = token_Token(40);
    pub const M_A_P: token_Token = token_Token(77);
    pub const M_U_L: token_Token = token_Token(14);
    pub const M_U_L__A_S_S_I_G_N: token_Token = token_Token(25);
    pub const N_E_Q: token_Token = token_Token(44);
    pub const N_O_T: token_Token = token_Token(43);
    pub const O_R: token_Token = token_Token(18);
    pub const O_R__A_S_S_I_G_N: token_Token = token_Token(29);
    pub const P_A_C_K_A_G_E: token_Token = token_Token(78);
    pub const P_E_R_I_O_D: token_Token = token_Token(53);
    pub const Q_U_O: token_Token = token_Token(15);
    pub const Q_U_O__A_S_S_I_G_N: token_Token = token_Token(26);
    pub const R_A_N_G_E: token_Token = token_Token(79);
    pub const R_B_R_A_C_E: token_Token = token_Token(56);
    pub const R_B_R_A_C_K: token_Token = token_Token(55);
    pub const R_E_M: token_Token = token_Token(16);
    pub const R_E_M__A_S_S_I_G_N: token_Token = token_Token(27);
    pub const R_E_T_U_R_N: token_Token = token_Token(80);
    pub const R_P_A_R_E_N: token_Token = token_Token(54);
    pub const S_E_L_E_C_T: token_Token = token_Token(81);
    pub const S_E_M_I_C_O_L_O_N: token_Token = token_Token(57);
    pub const S_H_L: token_Token = token_Token(20);
    pub const S_H_L__A_S_S_I_G_N: token_Token = token_Token(31);
    pub const S_H_R: token_Token = token_Token(21);
    pub const S_H_R__A_S_S_I_G_N: token_Token = token_Token(32);
    pub const S_T_R_I_N_G: token_Token = token_Token(9);
    pub const S_T_R_U_C_T: token_Token = token_Token(82);
    pub const S_U_B: token_Token = token_Token(13);
    pub const S_U_B__A_S_S_I_G_N: token_Token = token_Token(24);
    pub const S_W_I_T_C_H: token_Token = token_Token(83);
    pub const T_I_L_D_E: token_Token = token_Token(88);
    pub const T_Y_P_E: token_Token = token_Token(84);
    pub const V_A_R: token_Token = token_Token(85);
    pub const X_O_R: token_Token = token_Token(19);
    pub const X_O_R__A_S_S_I_G_N: token_Token = token_Token(30);

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