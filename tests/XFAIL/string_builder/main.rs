use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut builder: Rc<RefCell<Option</* TODO: Unhandled type *ast.SelectorExpr */ Rc<RefCell<Option<()>>>>>>;
    (*builder.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("Hello".to_string()))));
    (*builder.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some(" ".to_string()))));
    (*builder.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("World".to_string()))));
    let mut result = (*builder.borrow_mut().as_mut().unwrap()).string();
    println!("{}", (*result.borrow_mut().as_mut().unwrap()));
}