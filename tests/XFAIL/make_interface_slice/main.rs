use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Animal: std::fmt::Display + Any {
    fn __go_clone_box_animal(&self) -> Box<dyn Animal>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_animal(&self, other: &dyn Animal) -> bool;
    fn sound(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Animal> {
    fn clone(&self) -> Self {
        self.__go_clone_box_animal()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cat {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Cat {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Cat {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Dog {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Dog {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Dog {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl Cat {
    pub fn sound(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), ": meow".to_string()))));
    }
}

impl Animal for Cat {
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), ": meow".to_string()))));
    }
    fn __go_clone_box_animal(&self) -> Box<dyn Animal> {
        Box::new(self.clone()) as Box<dyn Animal>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_animal(&self, other: &dyn Animal) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Cat>() {
            self == __other
        } else {
            false
        }
    }
}

impl Dog {
    pub fn sound(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), ": woof".to_string()))));
    }
}

impl Animal for Dog {
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), ": woof".to_string()))));
    }
    fn __go_clone_box_animal(&self) -> Box<dyn Animal> {
        Box::new(self.clone()) as Box<dyn Animal>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_animal(&self, other: &dyn Animal) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Dog>() {
            self == __other
        } else {
            false
        }
    }
}

fn main() {
    let mut pets: Rc<RefCell<Option<Vec<Box<dyn Animal>>>>> = Rc::new(RefCell::new(Some(vec![Default::default(); (2) as usize])));
    (*pets.borrow_mut().as_mut().unwrap())[(0) as usize] = Cat { name: Rc::new(RefCell::new(Some("whiskers".to_string()))), ..Default::default() };
    (*pets.borrow_mut().as_mut().unwrap())[(1) as usize] = Dog { name: Rc::new(RefCell::new(Some("rex".to_string()))), ..Default::default() };
    { let __range_holder = pets.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        println!("{}", format!("{}", (*p.sound().borrow().as_ref().unwrap())));
    } }
}