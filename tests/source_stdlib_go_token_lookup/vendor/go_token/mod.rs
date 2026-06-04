use go2rust_stdlib_stubs::*;

use crate::position::*;
use crate::serialize::*;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const I_L_L_E_G_A_L: i32 = 0;
pub const E_O_F: i32 = 1;
pub const C_O_M_M_E_N_T: i32 = 2;
pub(crate) const LITERAL_BEG: i32 = 3;
pub const I_D_E_N_T: i32 = 4;
pub const I_N_T: i32 = 5;
pub const F_L_O_A_T: i32 = 6;
pub const I_M_A_G: i32 = 7;
pub const C_H_A_R: i32 = 8;
pub const S_T_R_I_N_G: i32 = 9;
pub(crate) const LITERAL_END: i32 = 10;
pub(crate) const OPERATOR_BEG: i32 = 11;
pub const A_D_D: i32 = 12;
pub const S_U_B: i32 = 13;
pub const M_U_L: i32 = 14;
pub const Q_U_O: i32 = 15;
pub const R_E_M: i32 = 16;
pub const A_N_D: i32 = 17;
pub const O_R: i32 = 18;
pub const X_O_R: i32 = 19;
pub const S_H_L: i32 = 20;
pub const S_H_R: i32 = 21;
pub const A_N_D__N_O_T: i32 = 22;
pub const A_D_D__A_S_S_I_G_N: i32 = 23;
pub const S_U_B__A_S_S_I_G_N: i32 = 24;
pub const M_U_L__A_S_S_I_G_N: i32 = 25;
pub const Q_U_O__A_S_S_I_G_N: i32 = 26;
pub const R_E_M__A_S_S_I_G_N: i32 = 27;
pub const A_N_D__A_S_S_I_G_N: i32 = 28;
pub const O_R__A_S_S_I_G_N: i32 = 29;
pub const X_O_R__A_S_S_I_G_N: i32 = 30;
pub const S_H_L__A_S_S_I_G_N: i32 = 31;
pub const S_H_R__A_S_S_I_G_N: i32 = 32;
pub const A_N_D__N_O_T__A_S_S_I_G_N: i32 = 33;
pub const L_A_N_D: i32 = 34;
pub const L_O_R: i32 = 35;
pub const A_R_R_O_W: i32 = 36;
pub const I_N_C: i32 = 37;
pub const D_E_C: i32 = 38;
pub const E_Q_L: i32 = 39;
pub const L_S_S: i32 = 40;
pub const G_T_R: i32 = 41;
pub const A_S_S_I_G_N: i32 = 42;
pub const N_O_T: i32 = 43;
pub const N_E_Q: i32 = 44;
pub const L_E_Q: i32 = 45;
pub const G_E_Q: i32 = 46;
pub const D_E_F_I_N_E: i32 = 47;
pub const E_L_L_I_P_S_I_S: i32 = 48;
pub const L_P_A_R_E_N: i32 = 49;
pub const L_B_R_A_C_K: i32 = 50;
pub const L_B_R_A_C_E: i32 = 51;
pub const C_O_M_M_A: i32 = 52;
pub const P_E_R_I_O_D: i32 = 53;
pub const R_P_A_R_E_N: i32 = 54;
pub const R_B_R_A_C_K: i32 = 55;
pub const R_B_R_A_C_E: i32 = 56;
pub const S_E_M_I_C_O_L_O_N: i32 = 57;
pub const C_O_L_O_N: i32 = 58;
pub(crate) const OPERATOR_END: i32 = 59;
pub(crate) const KEYWORD_BEG: i32 = 60;
pub const B_R_E_A_K: i32 = 61;
pub const C_A_S_E: i32 = 62;
pub const C_H_A_N: i32 = 63;
pub const C_O_N_S_T: i32 = 64;
pub const C_O_N_T_I_N_U_E: i32 = 65;
pub const D_E_F_A_U_L_T: i32 = 66;
pub const D_E_F_E_R: i32 = 67;
pub const E_L_S_E: i32 = 68;
pub const F_A_L_L_T_H_R_O_U_G_H: i32 = 69;
pub const F_O_R: i32 = 70;
pub const F_U_N_C: i32 = 71;
pub const G_O: i32 = 72;
pub const G_O_T_O: i32 = 73;
pub const I_F: i32 = 74;
pub const I_M_P_O_R_T: i32 = 75;
pub const I_N_T_E_R_F_A_C_E: i32 = 76;
pub const M_A_P: i32 = 77;
pub const P_A_C_K_A_G_E: i32 = 78;
pub const R_A_N_G_E: i32 = 79;
pub const R_E_T_U_R_N: i32 = 80;
pub const S_E_L_E_C_T: i32 = 81;
pub const S_T_R_U_C_T: i32 = 82;
pub const S_W_I_T_C_H: i32 = 83;
pub const T_Y_P_E: i32 = 84;
pub const V_A_R: i32 = 85;
pub(crate) const KEYWORD_END: i32 = 86;
pub(crate) const ADDITIONAL_BEG: i32 = 87;
pub const T_I_L_D_E: i32 = 88;
pub(crate) const ADDITIONAL_END: i32 = 89;


pub const LOWEST_PREC: i32 = 0;
pub const UNARY_PREC: i32 = 6;
pub const HIGHEST_PREC: i32 = 7;


/// Token is the set of lexical tokens of the Go programming language.
#[derive(Debug, Clone, Default)]
pub struct Token(pub Arc<Mutex<Option<i32>>>);

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Token {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Token {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Token> for i32 {
    fn eq(&self, other: &Token) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Token> for i32 {
    fn partial_cmp(&self, other: &Token) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Token {
    type Output = Token;
    fn add(self, other: Self) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Token {
    type Output = Token;
    fn add(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Token> for i32 {
    type Output = Token;
    fn add(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Token {
    type Output = Token;
    fn sub(self, other: Self) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Token {
    type Output = Token;
    fn sub(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Token> for i32 {
    type Output = Token;
    fn sub(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Token {
    type Output = Token;
    fn mul(self, other: Self) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Token {
    type Output = Token;
    fn mul(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Token> for i32 {
    type Output = Token;
    fn mul(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Token {
    type Output = Token;
    fn div(self, other: Self) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Token {
    type Output = Token;
    fn div(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Token> for i32 {
    type Output = Token;
    fn div(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Token {
    type Output = Token;
    fn neg(self) -> Token {
        Token(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Token {
    type Output = Token;
    fn rem(self, other: Self) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Token {
    type Output = Token;
    fn rem(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Token> for i32 {
    type Output = Token;
    fn rem(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Token {
    type Output = Token;
    fn bitand(self, other: Self) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Token {
    type Output = Token;
    fn bitand(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Token> for i32 {
    type Output = Token;
    fn bitand(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Token {
    type Output = Token;
    fn bitor(self, other: Self) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Token {
    type Output = Token;
    fn bitor(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Token> for i32 {
    type Output = Token;
    fn bitor(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Token {
    type Output = Token;
    fn bitxor(self, other: Self) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Token {
    type Output = Token;
    fn bitxor(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Token> for i32 {
    type Output = Token;
    fn bitxor(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Token {
    type Output = Token;
    fn not(self) -> Token {
        Token(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Token {
    type Output = Token;
    fn shl(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Token {
    type Output = Token;
    fn shl(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Token {
    type Output = Token;
    fn shl(self, other: i8) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Token {
    type Output = Token;
    fn shl(self, other: i16) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Token {
    type Output = Token;
    fn shl(self, other: i64) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Token {
    type Output = Token;
    fn shl(self, other: u32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Token {
    type Output = Token;
    fn shl(self, other: u8) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Token {
    type Output = Token;
    fn shl(self, other: u16) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Token {
    type Output = Token;
    fn shl(self, other: u64) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Token {
    type Output = Token;
    fn shl(self, other: usize) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Token {
    type Output = Token;
    fn shr(self, other: Token) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Token {
    type Output = Token;
    fn shr(self, other: i32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Token {
    type Output = Token;
    fn shr(self, other: i8) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Token {
    type Output = Token;
    fn shr(self, other: i16) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Token {
    type Output = Token;
    fn shr(self, other: i64) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Token {
    type Output = Token;
    fn shr(self, other: u32) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Token {
    type Output = Token;
    fn shr(self, other: u8) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Token {
    type Output = Token;
    fn shr(self, other: u16) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Token {
    type Output = Token;
    fn shr(self, other: u64) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Token {
    type Output = Token;
    fn shr(self, other: usize) -> Token {
        Token(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Token {}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static tokens: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 89]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static keywords: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Token>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *tokens.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *keywords.lock().unwrap() = Some(BTreeMap::new());
    *tokens.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["ILLEGAL".to_string(), "EOF".to_string(), "COMMENT".to_string(), String::new(), "IDENT".to_string(), "INT".to_string(), "FLOAT".to_string(), "IMAG".to_string(), "CHAR".to_string(), "STRING".to_string(), String::new(), String::new(), "+".to_string(), "-".to_string(), "*".to_string(), "/".to_string(), "%".to_string(), "&".to_string(), "|".to_string(), "^".to_string(), "<<".to_string(), ">>".to_string(), "&^".to_string(), "+=".to_string(), "-=".to_string(), "*=".to_string(), "/=".to_string(), "%=".to_string(), "&=".to_string(), "|=".to_string(), "^=".to_string(), "<<=".to_string(), ">>=".to_string(), "&^=".to_string(), "&&".to_string(), "||".to_string(), "<-".to_string(), "++".to_string(), "--".to_string(), "==".to_string(), "<".to_string(), ">".to_string(), "=".to_string(), "!".to_string(), "!=".to_string(), "<=".to_string(), ">=".to_string(), ":=".to_string(), "...".to_string(), "(".to_string(), "[".to_string(), "{".to_string(), ",".to_string(), ".".to_string(), ")".to_string(), "]".to_string(), "}".to_string(), ";".to_string(), ":".to_string(), String::new(), String::new(), "break".to_string(), "case".to_string(), "chan".to_string(), "const".to_string(), "continue".to_string(), "default".to_string(), "defer".to_string(), "else".to_string(), "fallthrough".to_string(), "for".to_string(), "func".to_string(), "go".to_string(), "goto".to_string(), "if".to_string(), "import".to_string(), "interface".to_string(), "map".to_string(), "package".to_string(), "range".to_string(), "return".to_string(), "select".to_string(), "struct".to_string(), "switch".to_string(), "type".to_string(), "var".to_string(), String::new(), String::new(), "~".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *tokens.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *keywords.lock().unwrap() = Some(BTreeMap::new());
}


pub(crate) fn __go_init_order_0() {
    *tokens.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["ILLEGAL".to_string(), "EOF".to_string(), "COMMENT".to_string(), String::new(), "IDENT".to_string(), "INT".to_string(), "FLOAT".to_string(), "IMAG".to_string(), "CHAR".to_string(), "STRING".to_string(), String::new(), String::new(), "+".to_string(), "-".to_string(), "*".to_string(), "/".to_string(), "%".to_string(), "&".to_string(), "|".to_string(), "^".to_string(), "<<".to_string(), ">>".to_string(), "&^".to_string(), "+=".to_string(), "-=".to_string(), "*=".to_string(), "/=".to_string(), "%=".to_string(), "&=".to_string(), "|=".to_string(), "^=".to_string(), "<<=".to_string(), ">>=".to_string(), "&^=".to_string(), "&&".to_string(), "||".to_string(), "<-".to_string(), "++".to_string(), "--".to_string(), "==".to_string(), "<".to_string(), ">".to_string(), "=".to_string(), "!".to_string(), "!=".to_string(), "<=".to_string(), ">=".to_string(), ":=".to_string(), "...".to_string(), "(".to_string(), "[".to_string(), "{".to_string(), ",".to_string(), ".".to_string(), ")".to_string(), "]".to_string(), "}".to_string(), ";".to_string(), ":".to_string(), String::new(), String::new(), "break".to_string(), "case".to_string(), "chan".to_string(), "const".to_string(), "continue".to_string(), "default".to_string(), "defer".to_string(), "else".to_string(), "fallthrough".to_string(), "for".to_string(), "func".to_string(), "go".to_string(), "goto".to_string(), "if".to_string(), "import".to_string(), "interface".to_string(), "map".to_string(), "package".to_string(), "range".to_string(), "return".to_string(), "select".to_string(), "struct".to_string(), "switch".to_string(), "type".to_string(), "var".to_string(), String::new(), String::new(), "~".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl Token {
    /// String returns the string corresponding to the token tok.
    /// For operators, delimiters, and keywords the string is the actual
    /// token character sequence (e.g., for the token [ADD], the string is
    /// "+"). For all other tokens the string corresponds to the token
    /// constant name (e.g. for the token [IDENT], the string is "IDENT").
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut s = Arc::new(Mutex::new(Some("".to_string())));
        if { let __tmp_x = Token(Arc::new(Mutex::new(Some(0 as i32)))); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Token(Arc::new(Mutex::new(Some((*tokens.lock().unwrap().as_ref().unwrap()).len() as i32)))); __tmp_x < __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = tokens.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }; *s.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", "token(".to_string())); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) as i32).to_string()))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s }; *s.lock().unwrap() = Some(new_val); };
    }
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// Precedence returns the operator precedence of the binary
    /// operator op. If op is not a binary operator, the result
    /// is LowestPrecedence.
    pub fn precedence(&self) -> i32 {
        { let _switch_val = (*self.0.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (Token(Arc::new(Mutex::new(Some(L_O_R as i32))))) {
            return 1;
        } else if _switch_val == (Token(Arc::new(Mutex::new(Some(L_A_N_D as i32))))) {
            return 2;
        } else if _switch_val == (Token(Arc::new(Mutex::new(Some(E_Q_L as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(N_E_Q as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(L_S_S as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(L_E_Q as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(G_T_R as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(G_E_Q as i32))))) {
            return 3;
        } else if _switch_val == (Token(Arc::new(Mutex::new(Some(A_D_D as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(S_U_B as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(O_R as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(X_O_R as i32))))) {
            return 4;
        } else if _switch_val == (Token(Arc::new(Mutex::new(Some(M_U_L as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(Q_U_O as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(R_E_M as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(S_H_L as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(S_H_R as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(A_N_D as i32))))) || _switch_val == (Token(Arc::new(Mutex::new(Some(A_N_D__N_O_T as i32))))) {
            return 5;
        }
    }
        0
    }

    /// IsLiteral returns true for tokens corresponding to identifiers
    /// and basic type literals; it returns false otherwise.
    pub fn is_literal(&self) -> bool {
        return { let __tmp_x = Token(Arc::new(Mutex::new(Some(LITERAL_BEG as i32)))); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } && { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Token(Arc::new(Mutex::new(Some(LITERAL_END as i32)))); __tmp_x < __tmp_y };
    }

    /// IsOperator returns true for tokens corresponding to operators and
    /// delimiters; it returns false otherwise.
    pub fn is_operator(&self) -> bool {
        return ({ let __tmp_x = Token(Arc::new(Mutex::new(Some(OPERATOR_BEG as i32)))); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } && { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Token(Arc::new(Mutex::new(Some(OPERATOR_END as i32)))); __tmp_x < __tmp_y }) || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Token(Arc::new(Mutex::new(Some(T_I_L_D_E as i32)))); __tmp_x == __tmp_y };
    }

    /// IsKeyword returns true for tokens corresponding to keywords;
    /// it returns false otherwise.
    pub fn is_keyword(&self) -> bool {
        return { let __tmp_x = Token(Arc::new(Mutex::new(Some(KEYWORD_BEG as i32)))); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } && { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Token(Arc::new(Mutex::new(Some(KEYWORD_END as i32)))); __tmp_x < __tmp_y };
    }
}

fn __go_init_0() {
    { let new_val = { let __collection_holder = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Token>>>>::new()))).clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *keywords.lock().unwrap() = new_val; };
    let mut i = Arc::new(Mutex::new(Some(Token(Arc::new(Mutex::new(Some((KEYWORD_BEG as i32 + 1 as i32) as i32)))))));
    while { let __tmp_x = (*i.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Token(Arc::new(Mutex::new(Some(KEYWORD_END as i32)))); __tmp_x < __tmp_y } {
        { let __map_key = { let __seq = { let __seq_holder = tokens.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }; let __map_value = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); (*keywords.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as i32); }
    }
}

/// Lookup maps an identifier to its keyword token or [IDENT] (if not a keyword).
pub fn lookup(ident: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Token>>> {
    {
        let (mut tok, mut is_keyword) = { let __map = { let __map_holder = keywords.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*ident.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(Token(Arc::new(Mutex::new(Some(0))))))), false) } };;
        if is_keyword {
            return { let __owned = tok.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
    Arc::new(Mutex::new(Some(Token(Arc::new(Mutex::new(Some(I_D_E_N_T as i32)))))))
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}
