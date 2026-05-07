use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const RED: i32 = 0;
pub const GREEN: i32 = 1;
pub const BLUE: i32 = 2;
pub const YELLOW: i32 = 3;


#[derive(Debug, Clone)]
pub struct Color(pub Rc<RefCell<Option<i32>>>);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}


fn main() {
    println!("{} {}", "Red:".to_string(), RED);
    println!("{} {}", "Green:".to_string(), GREEN);
    println!("{} {}", "Blue:".to_string(), BLUE);
    println!("{} {}", "Yellow:".to_string(), YELLOW);
}