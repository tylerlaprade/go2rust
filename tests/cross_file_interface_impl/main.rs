mod circle;
mod shape;
use circle::*;
use shape::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut shapes = Rc::new(RefCell::new(Some(vec![new_circle(Rc::new(RefCell::new(Some(2)))).clone(), new_circle(Rc::new(RefCell::new(Some(3)))).clone()])));
    println!("{}", format!("{}", total_area(shapes.clone())));
}