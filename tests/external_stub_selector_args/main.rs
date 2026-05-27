use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Checker;

impl std::fmt::Display for types_Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Checker>")
    }
}


impl types_Checker {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
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


pub mod types {
    use super::*;
    pub fn new_checker<T0, T1, T2, T3>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> Arc<Mutex<Option<types_Checker>>> {
        panic!("new_checker bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn new_package<T0, T1>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<types_Package>>> {
        panic!("new_package bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Default)]
pub struct checkerInputs {
    pub fset: Arc<Mutex<Option<token_FileSet>>>,
    pub pkg: Arc<Mutex<Option<types_Package>>>,
    pub info: Arc<Mutex<Option<types_Info>>>,
}

impl checkerInputs {
    pub fn __go_value_clone(&self) -> Self {
        Self { fset: self.fset.clone(), pkg: self.pkg.clone(), info: self.info.clone() }
    }
}

impl std::fmt::Display for checkerInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.fset.lock().unwrap().as_ref().unwrap()), (*self.pkg.lock().unwrap().as_ref().unwrap()), (*self.info.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut inputs = Arc::new(Mutex::new(Some(checkerInputs { fset: token::new_file_set().clone(), pkg: types::new_package("example.com/p".to_string(), "p".to_string()).clone(), info: Arc::new(Mutex::new(Some(types_Info { ..Default::default() }))).clone(), ..Default::default() })));
    let mut checker = types::new_checker((), { let __go_arg = (*inputs.lock().unwrap().as_ref().unwrap()).fset.clone(); __go_arg }, { let __go_arg = (*inputs.lock().unwrap().as_ref().unwrap()).pkg.clone(); __go_arg }, { let __go_arg = (*inputs.lock().unwrap().as_ref().unwrap()).info.clone(); __go_arg });
    println!("{}", format!("{}", (*checker.lock().unwrap()).is_some()));
}