use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn tag(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        Node::__go_clone_box_node(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub tag: Rc<RefCell<Option<String>>>,
    pub items: Rc<RefCell<Option<Vec<i32>>>>,
}

impl Decl {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.tag.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.items.clone();
        Self {
            tag: __go_clone_0_0,
            items: __go_clone_1_0,
        }
    }
}


impl Default for Decl {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            tag: __go_default_0_0,
            items: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.tag.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.items));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Decl {
    pub fn tag(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.tag.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl Node for Decl {
    fn tag(&self) -> Rc<RefCell<Option<String>>> {
        Decl::tag(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Decl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct DeclPtr(pub Rc<RefCell<Option<Decl>>>);

impl std::fmt::Display for DeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for DeclPtr {
    fn tag(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Decl::tag(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeclPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn process(n: Rc<RefCell<Option<Box<dyn Node>>>>) {
    let (mut d, mut ok) = ({
        let val = n.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Node>::__go_as_any(any_val.as_ref()).downcast_ref::<DeclPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Decl>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Decl>)), false)
        }
    });
    if !ok {
        return;
    }
    { let new_val = { let __append_target = (*d.borrow().as_ref().unwrap()).items.clone(); let __append_value = 99; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; (*d.borrow_mut().as_mut().unwrap()).items = new_val; };
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Decl { tag: Rc::new(RefCell::new(Some("a".to_string()))), items: Rc::new(RefCell::new(Some(vec![1, 2]))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(Decl { tag: Rc::new(RefCell::new(Some("b".to_string()))), items: Rc::new(RefCell::new(Some(vec![3]))), ..Default::default() })));
    process(Rc::new(RefCell::new(Some(Box::new(DeclPtr(a.clone())) as Box<dyn Node>))));
    process(Rc::new(RefCell::new(Some(Box::new(DeclPtr(b.clone())) as Box<dyn Node>))));
    println!("{} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).clone()), format!("{}", format_slice_values(&(*(*a.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).clone())));
    println!("{} {}", format!("{}", (*(*b.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).clone()), format!("{}", format_slice_values(&(*(*b.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).clone())));
}