use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Spec: std::fmt::Display + Any {
    fn __go_clone_box_spec(&self) -> Box<dyn Spec>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Spec> {
    fn clone(&self) -> Self {
        Spec::__go_clone_box_spec(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Impl {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Impl {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Impl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Impl {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl Spec for Impl {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        Impl::name(self)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec> {
        Box::new(self.clone()) as Box<dyn Spec>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Impl>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ImplPtr(pub Rc<RefCell<Option<Impl>>>);

impl std::fmt::Display for ImplPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Spec for ImplPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Impl::name(__recv)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec> {
        Box::new(self.clone()) as Box<dyn Spec>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ImplPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn get_name(s: Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>> {
    (*s.borrow().as_ref().unwrap()).name()
}

pub fn call_via_closure(a: Rc<RefCell<Option<Box<dyn Spec>>>>, b: Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>> {
    let mut f = Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<Box<dyn Spec>>>>, y: Rc<RefCell<Option<Box<dyn Spec>>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*get_name(x.clone()).borrow().as_ref().unwrap())));
            __s.push_str(&format!("{}", ",".to_string()));
            __s.push_str(&format!("{}", (*get_name(y.clone()).borrow().as_ref().unwrap())));
            __s
        })))
    }) as Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Spec>>>>, Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>>>)));
    return { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Spec>>>>, Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Spec>>>>, Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(a.clone(), b.clone()) };
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Impl { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(Impl { name: Rc::new(RefCell::new(Some("beta".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*call_via_closure(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Spec>)))).borrow().as_ref().unwrap())));
}