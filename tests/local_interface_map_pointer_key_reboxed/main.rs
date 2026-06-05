use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Object: std::fmt::Display + Any {
    fn __go_clone_box_object(&self) -> Box<dyn Object>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_object(&self, other: &dyn Object) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        Object::__go_clone_box_object(self.as_ref())
    }
}

#[derive(Clone)]
pub struct GoObjectInterfaceKey(pub Rc<RefCell<Option<Box<dyn Object>>>>);

impl GoObjectInterfaceKey {
    pub fn new(value: Rc<RefCell<Option<Box<dyn Object>>>>) -> Self { GoObjectInterfaceKey(value) }
    pub fn value(&self) -> Rc<RefCell<Option<Box<dyn Object>>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
    fn identity(&self) -> (u64, String) {
        let __guard = self.0.borrow();
        match __guard.as_ref() {
            None => (0, String::new()),
            Some(__v) => {
                let mut __hasher = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&__v.as_ref().__go_as_any().type_id(), &mut __hasher);
                (std::hash::Hasher::finish(&__hasher), format!("{}", __v))
            }
        }
    }
}
impl PartialEq for GoObjectInterfaceKey {
    fn eq(&self, other: &Self) -> bool {
        let __left_guard = self.0.borrow();
        let __right_guard = other.0.borrow();
        match (__left_guard.as_ref(), __right_guard.as_ref()) {
            (None, None) => true,
            (Some(__left), Some(__right)) => __left.as_ref().__go_eq_object(__right.as_ref()),
            _ => false,
        }
    }
}
impl Eq for GoObjectInterfaceKey {}
impl PartialOrd for GoObjectInterfaceKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for GoObjectInterfaceKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other { return std::cmp::Ordering::Equal; }
        match self.identity().cmp(&other.identity()) {
            std::cmp::Ordering::Equal => self.addr().cmp(&other.addr()),
            ordering => ordering,
        }
    }
}
impl std::fmt::Debug for GoObjectInterfaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.identity().1) }
}
impl std::fmt::Display for GoObjectInterfaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.identity().1) }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Var {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Var {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct declInfo {
    pub seen: Rc<RefCell<Option<bool>>>,
}

impl declInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { seen: { let __guard = self.seen.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for declInfo {
    fn default() -> Self {
        Self { seen: Rc::new(RefCell::new(Some(false))) }
    }
}

impl std::fmt::Display for declInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.seen.borrow().as_ref().unwrap()))
    }
}


impl Var {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Object for Var {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        Var::name(self)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object> {
        Box::new(self.clone()) as Box<dyn Object>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &dyn Object) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Var>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct VarPtr(pub Rc<RefCell<Option<Var>>>);

impl std::fmt::Display for VarPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for VarPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::name(__recv)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object> {
        Box::new(self.clone()) as Box<dyn Object>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &dyn Object) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<VarPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn declare(objMap: Rc<RefCell<Option<BTreeMap<GoObjectInterfaceKey, Rc<RefCell<Option<declInfo>>>>>>>, obj: Rc<RefCell<Option<Box<dyn Object>>>>) {
    { let __map_key = GoObjectInterfaceKey::new(obj.clone()); let __map_value = Rc::new(RefCell::new(Some(declInfo { seen: Rc::new(RefCell::new(Some(true))), ..Default::default() }))); (*objMap.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
}

pub fn lookup(objMap: Rc<RefCell<Option<BTreeMap<GoObjectInterfaceKey, Rc<RefCell<Option<declInfo>>>>>>>, obj: Rc<RefCell<Option<Box<dyn Object>>>>) -> bool {
    let mut info = { let __map_holder = objMap.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(obj.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
    return (*info.borrow()).is_some() && (*(*info.borrow().as_ref().unwrap()).seen.borrow().as_ref().unwrap());
}

fn main() {
    let mut objMap = Rc::new(RefCell::new(Some(BTreeMap::<GoObjectInterfaceKey, Rc<RefCell<Option<declInfo>>>>::from([]))));
    let mut v = Rc::new(RefCell::new(Some(Var { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() })));

    declare(objMap.clone(), Rc::new(RefCell::new(Some(Box::new(VarPtr(v.clone())) as Box<dyn Object>))));
    println!("{}", format!("{}", lookup(objMap.clone(), Rc::new(RefCell::new(Some(Box::new(VarPtr(v.clone())) as Box<dyn Object>))))));

    let mut same: Rc<RefCell<Option<Box<dyn Object>>>> = Rc::new(RefCell::new(Some(Box::new(VarPtr(v.clone())) as Box<dyn Object>)));
    println!("{}", format!("{}", lookup(objMap.clone(), same.clone())));
}