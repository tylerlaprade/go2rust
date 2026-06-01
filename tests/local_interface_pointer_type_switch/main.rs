use std::any::Any;
use std::cell::{RefCell};
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
        self.__go_clone_box_object()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {
    pub name: Rc<RefCell<Option<String>>>,
}

impl TypeName {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for TypeName {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Builtin {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Builtin {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Builtin {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl TypeName {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Object for TypeName {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        TypeName::name(self)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object> {
        Box::new(self.clone()) as Box<dyn Object>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &dyn Object) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeName>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TypeNamePtr(pub Rc<RefCell<Option<TypeName>>>);

impl std::fmt::Display for TypeNamePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{}", __v), None => write!(f, "<nil>") }
    }
}

impl Object for TypeNamePtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::name(__recv)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object> {
        Box::new(self.clone()) as Box<dyn Object>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &dyn Object) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeNamePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Builtin {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Object for Builtin {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        Builtin::name(self)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object> {
        Box::new(self.clone()) as Box<dyn Object>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &dyn Object) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Builtin>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BuiltinPtr(pub Rc<RefCell<Option<Builtin>>>);

impl std::fmt::Display for BuiltinPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{}", __v), None => write!(f, "<nil>") }
    }
}

impl Object for BuiltinPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::name(__recv)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object> {
        Box::new(self.clone()) as Box<dyn Object>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &dyn Object) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BuiltinPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn classify(mut obj: Rc<RefCell<Option<Box<dyn Object>>>>) -> Rc<RefCell<Option<String>>> {
    let mut obj: Rc<RefCell<Option<Box<dyn Object>>>> = obj.clone();
    {
    let _ts_subject = obj.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<TypeNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<TypeNamePtr>()).unwrap().0.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("{}{}", "type:".to_string(), (*(*obj.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<BuiltinPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<BuiltinPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("{}{}", "builtin:".to_string(), (*(*obj.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())))));;
    } else {
        let obj = obj.clone();
        drop(_ts_guard);
        panic!("unreachable");;
    }
    }
    unreachable!()
}

pub fn assert_type_name(obj: Rc<RefCell<Option<Box<dyn Object>>>>) -> Rc<RefCell<Option<String>>> {
    {
        let (mut t, mut ok) = ({
        let val = obj.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object>::__go_as_any(any_val.as_ref()).downcast_ref::<TypeNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<TypeName>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<TypeName>)), false)
        }
    });;
        if ok {
            return (*t.borrow().as_ref().unwrap()).name();;
        }
    }
    Rc::new(RefCell::new(Some("not type".to_string())))
}

fn main() {
    let mut typeName = Rc::new(RefCell::new(Some(TypeName { name: Rc::new(RefCell::new(Some("T".to_string()))), ..Default::default() })));
    let mut builtin = Rc::new(RefCell::new(Some(Builtin { name: Rc::new(RefCell::new(Some("B".to_string()))), ..Default::default() })));

    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(TypeNamePtr(typeName.clone())) as Box<dyn Object>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(BuiltinPtr(builtin.clone())) as Box<dyn Object>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*assert_type_name(Rc::new(RefCell::new(Some(Box::new(TypeNamePtr(typeName.clone())) as Box<dyn Object>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*assert_type_name(Rc::new(RefCell::new(Some(Box::new(BuiltinPtr(builtin.clone())) as Box<dyn Object>)))).borrow().as_ref().unwrap())));
}