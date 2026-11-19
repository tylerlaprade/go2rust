use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i32>>>,
    pub y: Rc<RefCell<Option<i32>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub name: Rc<RefCell<Option<String>>>,
    pub pos: Rc<RefCell<Option<Point>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            pos: __go_clone_1_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Point::default())));
        Self {
            name: __go_default_0_0,
            pos: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pos.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl User {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }

    pub fn pos(&self) -> Rc<RefCell<Option<Point>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); drop(__selector_guard); __cloned })));
    }

    pub fn label(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

pub fn name_of(u: Rc<RefCell<Option<User>>>) -> Rc<RefCell<Option<String>>> {
    return Rc::new(RefCell::new(Some({ let __selector_holder = (*u.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

/// A method that returns a field returns a copy of it. The caller's variable
/// is its own: changing it does not change the struct.
fn main() {
    let mut u = Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("ann".to_string()))), pos: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), ..Default::default() })));

    let mut a = (*u.borrow().as_ref().unwrap()).name();
    { let new_val = Rc::new(RefCell::new(Some({ let __s = (*a.borrow().as_ref().unwrap()).clone(); __s.to_uppercase() }))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *a.borrow_mut() = __moved_val; };
    println!("{} {}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*(*u.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));

    let mut b = (*u.borrow().as_ref().unwrap()).name();
    { (*b.borrow_mut().as_mut().unwrap()).push_str(&"!".to_string()); };
    println!("{} {}", format!("{}", { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*(*u.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));

    let mut c = Rc::new(RefCell::new(Some({ let __selector_holder = (*u.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    { (*c.borrow_mut().as_mut().unwrap()).push_str(&"?".to_string()); };
    println!("{} {}", format!("{}", { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*(*u.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));

    let mut d = name_of(u.clone());
    { let new_val = "other".to_string(); *d.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", { let __v = (*d.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*(*u.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));

    let mut e = (*u.borrow().as_ref().unwrap()).label();
    { let new_val = "label".to_string(); *e.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", { let __v = (*e.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*(*u.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));

    let mut p = (*u.borrow().as_ref().unwrap()).pos();
    { let new_val = 100; *(*p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", (*p.borrow().as_ref().unwrap())), format!("{}", (*(*u.borrow().as_ref().unwrap()).pos.borrow().as_ref().unwrap()).clone()));
}