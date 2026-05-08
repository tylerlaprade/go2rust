use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Celsius(pub Rc<RefCell<Option<f64>>>);

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Celsius {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<f64> for Celsius {
    fn eq(&self, other: &f64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Celsius {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<f64> for Celsius {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Celsius> for f64 {
    fn eq(&self, other: &Celsius) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Celsius> for f64 {
    fn partial_cmp(&self, other: &Celsius) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Celsius {
    type Output = f64;
    fn add(self, other: Self) -> f64 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<f64> for Celsius {
    type Output = f64;
    fn add(self, other: f64) -> f64 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Celsius> for f64 {
    type Output = f64;
    fn add(self, other: Celsius) -> f64 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Celsius {
    type Output = f64;
    fn sub(self, other: Self) -> f64 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<f64> for Celsius {
    type Output = f64;
    fn sub(self, other: f64) -> f64 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Celsius> for f64 {
    type Output = f64;
    fn sub(self, other: Celsius) -> f64 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}


#[derive(Debug, Clone, Default)]
pub struct Fahrenheit(pub Rc<RefCell<Option<f64>>>);

impl Display for Fahrenheit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Fahrenheit {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<f64> for Fahrenheit {
    fn eq(&self, other: &f64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Fahrenheit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<f64> for Fahrenheit {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Fahrenheit> for f64 {
    fn eq(&self, other: &Fahrenheit) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Fahrenheit> for f64 {
    fn partial_cmp(&self, other: &Fahrenheit) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Fahrenheit {
    type Output = f64;
    fn add(self, other: Self) -> f64 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<f64> for Fahrenheit {
    type Output = f64;
    fn add(self, other: f64) -> f64 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Fahrenheit> for f64 {
    type Output = f64;
    fn add(self, other: Fahrenheit) -> f64 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Fahrenheit {
    type Output = f64;
    fn sub(self, other: Self) -> f64 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<f64> for Fahrenheit {
    type Output = f64;
    fn sub(self, other: f64) -> f64 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Fahrenheit> for f64 {
    type Output = f64;
    fn sub(self, other: Fahrenheit) -> f64 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}


pub type StringAlias = Rc<RefCell<Option<String>>>;


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
    let mut temp: Rc<RefCell<Option<Celsius>>> = Rc::new(RefCell::new(Some(Celsius(Rc::new(RefCell::new(Some(100.0)))))));
    print!("{}°C = {}°F\n", { let __v = (*temp.borrow().as_ref().unwrap()).clone(); __v }, (*(*temp.borrow().as_ref().unwrap()).to_fahrenheit().borrow().as_ref().unwrap()));

    let mut f: Rc<RefCell<Option<Fahrenheit>>> = Rc::new(RefCell::new(Some(Fahrenheit(Rc::new(RefCell::new(Some(212.0)))))));
    print!("{}°F = {}°C\n", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v }, (*(*f.borrow().as_ref().unwrap()).to_celsius().borrow().as_ref().unwrap()));

    let mut s: StringAlias = Rc::new(RefCell::new(Some("hello".to_string())));
    println!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v });
}