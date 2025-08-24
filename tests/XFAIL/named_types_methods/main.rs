use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
struct Celsius(Rc<RefCell<Option<f64>>>);

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}


#[derive(Debug, Clone)]
struct Fahrenheit(Rc<RefCell<Option<f64>>>);

impl Display for Fahrenheit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}


type StringAlias = Rc<RefCell<Option<String>>>;


impl Celsius {
    pub fn to_fahrenheit(&self) -> Rc<RefCell<Option<Fahrenheit>>> {
        return Rc::new(RefCell::new(Some(Fahrenheit(Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) * 9.0 / 5.0 + 32.0)))))));
    }
}

impl Fahrenheit {
    pub fn to_celsius(&self) -> Rc<RefCell<Option<Celsius>>> {
        return Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(((*self.0.borrow().as_ref().unwrap()) - 32.0) * 5.0 / 9.0)))))));
    }
}

fn main() {
    let mut temp: Rc<RefCell<Option<Celsius>>> = Rc::new(RefCell::new(Some(100)));
    print!("{}°C = {}°F\n", (*temp.borrow_mut().as_mut().unwrap()), (*(*temp.borrow_mut().as_mut().unwrap()).to_fahrenheit().borrow().as_ref().unwrap()));

    let mut f: Rc<RefCell<Option<Fahrenheit>>> = Rc::new(RefCell::new(Some(212)));
    print!("{}°F = {}°C\n", (*f.borrow_mut().as_mut().unwrap()), (*(*f.borrow_mut().as_mut().unwrap()).to_celsius().borrow().as_ref().unwrap()));

    let mut s: StringAlias = Rc::new(RefCell::new(Some("hello".to_string())));
    println!("{}", (*s.borrow_mut().as_mut().unwrap()));
}