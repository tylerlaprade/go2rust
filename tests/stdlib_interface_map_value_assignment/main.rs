use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_Pos(pub i32);

impl PartialEq<i32> for token_Pos {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<token_Pos> for i32 {
    fn eq(&self, other: &token_Pos) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for token_Pos {
    type Output = token_Pos;
    fn bitand(self, other: Self) -> token_Pos {
        token_Pos(self.0 & other.0)
    }
}

impl std::ops::BitOr for token_Pos {
    type Output = token_Pos;
    fn bitor(self, other: Self) -> token_Pos {
        token_Pos(self.0 | other.0)
    }
}

impl std::fmt::Display for token_Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_Pos>")
    }
}


impl token_Pos {
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
        write!(f, "{}", token_string_value(*self))
    }
}

fn token_string_value(tok: token_Token) -> &'static str {
    match tok.0 {
        4 => "IDENT",
        5 => "INT",
        6 => "FLOAT",
        7 => "IMAG",
        8 => "CHAR",
        9 => "STRING",
        12 => "+",
        13 => "-",
        14 => "*",
        15 => "/",
        16 => "%",
        17 => "&",
        18 => "|",
        19 => "^",
        20 => "<<",
        21 => ">>",
        22 => "&^",
        23 => "+=",
        24 => "-=",
        25 => "*=",
        26 => "/=",
        27 => "%=",
        28 => "&=",
        29 => "|=",
        30 => "^=",
        31 => "<<=",
        32 => ">>=",
        33 => "&^=",
        34 => "&&",
        35 => "||",
        36 => "<-",
        37 => "++",
        38 => "--",
        39 => "==",
        40 => "<",
        41 => ">",
        42 => "=",
        43 => "!",
        44 => "!=",
        45 => "<=",
        46 => ">=",
        47 => ":=",
        48 => "...",
        61 => "break",
        62 => "case",
        63 => "chan",
        64 => "const",
        65 => "continue",
        66 => "default",
        67 => "defer",
        68 => "else",
        69 => "fallthrough",
        70 => "for",
        71 => "func",
        72 => "go",
        73 => "goto",
        74 => "if",
        75 => "import",
        76 => "interface",
        77 => "map",
        78 => "package",
        79 => "range",
        80 => "return",
        81 => "select",
        82 => "struct",
        83 => "switch",
        84 => "type",
        85 => "var",
        88 => "~",
        _ => "ILLEGAL",
    }
}

impl token_Token {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(token_string_value(*self).to_string())))
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl types_Type {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl PartialEq for types_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for types_Type {}

impl PartialOrd for types_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for types_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_TypeName;

impl std::fmt::Display for types_TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_TypeName>")
    }
}


impl types_TypeName {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_TypeParam;

impl std::fmt::Display for types_TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_TypeParam>")
    }
}


impl types_TypeParam {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


impl From<types_TypeParam> for types_Type {
    fn from(_value: types_TypeParam) -> Self {
        Self::__go_from(_value)
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
    pub const NO_POS: token_Pos = token_Pos(0);
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
}


pub mod types {
    use super::*;
    pub fn new_type_name<T0, T1, T2, T3>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> Arc<Mutex<Option<types_TypeName>>> {
        Arc::new(Mutex::new(Some::<types_TypeName>(Default::default())))
    }

    pub fn new_type_param<T0, T1>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<types_TypeParam>>> {
        Arc::new(Mutex::new(Some::<types_TypeParam>(Default::default())))
    }
}


pub fn remember(values: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<types_Type>>>>>>>) {
    let mut tn = types::new_type_name({ let __go_arg = token::NO_POS; __go_arg }, (), "T".to_string(), ());
    let mut tp = types::new_type_param(tn.clone(), ());
    { let __map_key = "T".to_string(); let __map_value = { let __arg = tp.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<types_Type> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }; (*values.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
}

pub fn literal() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<types_Type>>>>>>> {

    let mut tn = types::new_type_name({ let __go_arg = token::NO_POS; __go_arg }, (), "U".to_string(), ());
    let mut tp = types::new_type_param(tn.clone(), ());
    return Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<types_Type>>>>::from([("U".to_string(), { let __arg = tp.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<types_Type> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) })]))));
}

fn main() {
    let mut values = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<types_Type>>>>::new())));
    remember(values.clone());
    println!("{} {}", format!("{}", (*values.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*literal().lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}