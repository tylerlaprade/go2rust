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
        Animal::__go_clone_box_animal(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cat {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Cat {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Cat {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Dog {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Dog {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Dog {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Cat {
    pub fn sound(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), ": meow".to_string()))));
    }
}

impl Animal for Cat {
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        Cat::sound(self)
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

#[derive(Clone)]
pub struct CatPtr(pub Rc<RefCell<Option<Cat>>>);

impl std::fmt::Display for CatPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Animal for CatPtr {
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Cat::sound(__recv)
    }
    fn __go_clone_box_animal(&self) -> Box<dyn Animal> {
        Box::new(self.clone()) as Box<dyn Animal>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_animal(&self, other: &dyn Animal) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CatPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
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
        Dog::sound(self)
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

#[derive(Clone)]
pub struct DogPtr(pub Rc<RefCell<Option<Dog>>>);

impl std::fmt::Display for DogPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Animal for DogPtr {
    fn sound(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Dog::sound(__recv)
    }
    fn __go_clone_box_animal(&self) -> Box<dyn Animal> {
        Box::new(self.clone()) as Box<dyn Animal>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_animal(&self, other: &dyn Animal) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DogPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    let mut pets: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Animal>>>>>>>> = Rc::new(RefCell::new(Some(vec![Default::default(); (2) as usize])));
    (*pets.borrow_mut().as_mut().unwrap())[(0) as usize] = Rc::new(RefCell::new(Some(Box::new(Cat { name: Rc::new(RefCell::new(Some("whiskers".to_string()))), ..Default::default() }) as Box<dyn Animal>)));
    (*pets.borrow_mut().as_mut().unwrap())[(1) as usize] = Rc::new(RefCell::new(Some(Box::new(Dog { name: Rc::new(RefCell::new(Some("rex".to_string()))), ..Default::default() }) as Box<dyn Animal>)));
    { let __range_holder = pets.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        println!("{}", format!("{}", (*(*p.borrow().as_ref().unwrap()).sound().borrow().as_ref().unwrap())));
    } }
}