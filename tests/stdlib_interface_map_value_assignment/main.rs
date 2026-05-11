use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

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
        Self::default()
    }
}


pub mod token {
    use super::*;
    pub const NO_POS: token_Pos = token_Pos(0);
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
    let mut tn = types::new_type_name(token::NO_POS, (), "T".to_string(), ());
    let mut tp = types::new_type_param(tn.clone(), ());
    { let __map_key = "T".to_string(); let __map_value = { let __arg = tp.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }; (*values.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
}

pub fn literal() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<types_Type>>>>>>> {

    let mut tn = types::new_type_name(token::NO_POS, (), "U".to_string(), ());
    let mut tp = types::new_type_param(tn.clone(), ());
    return Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<types_Type>>>>::from([("U".to_string(), { let __arg = tp.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) })]))));
}

fn main() {
    let mut values = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<types_Type>>>>::new())));
    remember(values.clone());
    println!("{} {}", (*values.lock().unwrap().as_ref().unwrap()).len(), (*literal().lock().unwrap().as_ref().unwrap()).len());
}