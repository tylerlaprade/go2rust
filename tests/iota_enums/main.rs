use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

const RED: i32 = 0;
const GREEN: i32 = 1;
const BLUE: i32 = 2;
const YELLOW: i32 = 3;


#[derive(Debug, Clone)]
struct Color(Rc<RefCell<Option<i32>>>);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}


fn main() {
    println!("{} {}", "Red:".to_string(), RED);
    println!("{} {}", "Green:".to_string(), GREEN);
    println!("{} {}", "Blue:".to_string(), BLUE);
    println!("{} {}", "Yellow:".to_string(), YELLOW);
}