use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Key: std::fmt::Display + Any {
    fn __go_clone_box_key(&self) -> Box<dyn Key>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_key(&self, other: &dyn Key) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Key> {
    fn clone(&self) -> Self {
        Key::__go_clone_box_key(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct namedKey {
    pub name: Rc<RefCell<Option<String>>>,
}

impl namedKey {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for namedKey {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for namedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct Label {
    pub key: Rc<RefCell<Option<Box<dyn Key>>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.key.clone();
        Self {
            key: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.key.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl namedKey {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl Key for namedKey {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        namedKey::name(self)
    }
    fn __go_clone_box_key(&self) -> Box<dyn Key> {
        Box::new(self.clone()) as Box<dyn Key>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_key(&self, other: &dyn Key) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<namedKey>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct namedKeyPtr(pub Rc<RefCell<Option<namedKey>>>);

impl std::fmt::Display for namedKeyPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Key for namedKeyPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        namedKey::name(__recv)
    }
    fn __go_clone_box_key(&self) -> Box<dyn Key> {
        Box::new(self.clone()) as Box<dyn Key>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_key(&self, other: &dyn Key) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<namedKeyPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Label {
    pub fn key(&self) -> Rc<RefCell<Option<Box<dyn Key>>>> {
        { let __field = self.key.clone(); __field }
    }
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(namedKey { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(namedKey { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() })));
    let mut labelA = Rc::new(RefCell::new(Some(Label { key: Rc::new(RefCell::new(Some(Box::new((*a.borrow().as_ref().unwrap()).clone()) as Box<dyn Key>))), ..Default::default() })));
    let mut labelB = Rc::new(RefCell::new(Some(Label { key: Rc::new(RefCell::new(Some(Box::new((*b.borrow().as_ref().unwrap()).clone()) as Box<dyn Key>))), ..Default::default() })));

    println!("{}", format!("{}", { let __left_holder = (*labelA.borrow().as_ref().unwrap()).key().clone(); let __left_guard = __left_holder.borrow(); let __left_opt: Option<&dyn Key> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*labelA.borrow().as_ref().unwrap()).key().clone(); let __right_guard = __right_holder.borrow(); let __right_opt: Option<&dyn Key> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_key(__right), _ => false }; __eq }));
    println!("{}", format!("{}", { let __left_holder = (*labelA.borrow().as_ref().unwrap()).key().clone(); let __left_guard = __left_holder.borrow(); let __left_opt: Option<&dyn Key> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*labelB.borrow().as_ref().unwrap()).key().clone(); let __right_guard = __right_holder.borrow(); let __right_opt: Option<&dyn Key> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_key(__right), _ => false }; __eq }));
}