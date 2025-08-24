use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct User {
    // tags: `json:"id" db:"user_id"`
    i_d: Rc<RefCell<Option<i32>>>,
    // tags: `json:"name,omitempty" db:"full_name"`
    name: Rc<RefCell<Option<String>>>,
    // tags: `json:"email" db:"email_address" validate:"email"`
    email: Rc<RefCell<Option<String>>>,
    // tags: `json:"is_active" db:"active"`
    is_active: Rc<RefCell<Option<bool>>>,
    internal: Rc<RefCell<Option<String>>>,
}

fn main() {
    let mut u = Rc::new(RefCell::new(Some(User { i_d: Rc::new(RefCell::new(Some(1))), name: Rc::new(RefCell::new(Some("Alice".to_string()))), email: Rc::new(RefCell::new(Some("alice@example.com".to_string()))) })));
    let mut t = (*reflect.borrow_mut().as_mut().unwrap()).type_of(Rc::new(RefCell::new(Some((*u.borrow_mut().as_mut().unwrap())))));

    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow_mut().as_mut().unwrap()) < (*(*t.borrow_mut().as_mut().unwrap()).num_field().borrow().as_ref().unwrap()) {
        let mut field = (*t.borrow_mut().as_mut().unwrap()).field(Rc::new(RefCell::new(Some((*i.borrow_mut().as_mut().unwrap())))));
        print!("{}: json=%q db=%q\n", (*(*field.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*(*(*field.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).get(Rc::new(RefCell::new(Some("json".to_string())))).borrow().as_ref().unwrap()), (*(*(*(*field.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).get(Rc::new(RefCell::new(Some("db".to_string())))).borrow().as_ref().unwrap()));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}