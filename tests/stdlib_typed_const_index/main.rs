use std::cell::{RefCell};
use std::rc::{Rc};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic {
    pub __go_kind: types_BasicKind,
    pub __go_info: types_BasicInfo,
    pub __go_name: String,
}

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_name)
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_BasicInfo(pub i32);

impl PartialEq<i32> for types_BasicInfo {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<types_BasicInfo> for i32 {
    fn eq(&self, other: &types_BasicInfo) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for types_BasicInfo {
    type Output = types_BasicInfo;
    fn bitand(self, other: Self) -> types_BasicInfo {
        types_BasicInfo(self.0 & other.0)
    }
}

impl std::ops::BitOr for types_BasicInfo {
    type Output = types_BasicInfo;
    fn bitor(self, other: Self) -> types_BasicInfo {
        types_BasicInfo(self.0 | other.0)
    }
}

impl std::fmt::Display for types_BasicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_BasicInfo>")
    }
}


impl types_BasicInfo {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_BasicKind(pub i32);

impl PartialEq<i32> for types_BasicKind {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<types_BasicKind> for i32 {
    fn eq(&self, other: &types_BasicKind) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for types_BasicKind {
    type Output = types_BasicKind;
    fn bitand(self, other: Self) -> types_BasicKind {
        types_BasicKind(self.0 & other.0)
    }
}

impl std::ops::BitOr for types_BasicKind {
    type Output = types_BasicKind;
    fn bitor(self, other: Self) -> types_BasicKind {
        types_BasicKind(self.0 | other.0)
    }
}

impl std::fmt::Display for types_BasicKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_BasicKind>")
    }
}


impl types_BasicKind {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl types_Type {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
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


impl From<types_Basic> for types_Type {
    fn from(_value: types_Basic) -> Self {
        Self::__go_from(_value)
    }
}


pub mod types {
    use super::*;
    pub fn Typ() -> Rc<RefCell<Option<Vec<Rc<RefCell<Option<types_Basic>>>>>>> {
        Rc::new(RefCell::new(Some::<Vec<Rc<RefCell<Option<types_Basic>>>>>(Default::default())))
    }
}


pub fn is_invalid(t: Rc<RefCell<Option<types_Type>>>) -> bool {

    return (*t.borrow().as_ref().unwrap()).clone() == { let __arg = (*types::Typ().borrow().as_ref().unwrap())[0usize].clone(); let __converted = { let __arg_guard = __arg.borrow(); let __converted: types_Type = __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(types_Type::default); __converted }; __converted };
}

fn main() {
    if false {
        println!("{}", format!("{}", is_invalid({ let __arg = (*types::Typ().borrow().as_ref().unwrap())[0usize].clone(); let __converted = { let __arg_guard = __arg.borrow(); let __converted: Option<types_Type> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Rc::new(RefCell::new(__converted)) })));
    }
    println!("{}", format!("{}", "ok".to_string()));
}