use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct item {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


pub type maker = Rc<RefCell<Option<Box<dyn Fn() -> Rc<RefCell<Option<item>>>>>>>;


fn main() {
    let mut makeItem = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<item>>> {
        return Rc::new(RefCell::new(Some(item { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    }) as Box<dyn Fn() -> Rc<RefCell<Option<item>>>>)));
    let mut got = { let __f_guard = makeItem.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
    println!("{}", (*(*got.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()));
}