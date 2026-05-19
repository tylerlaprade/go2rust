use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Rc<RefCell<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let raw = (*self.raw.borrow().as_ref().unwrap()).clone();
        let key = (*key.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Rc<RefCell<Option<String>>>,
    tag: Rc<RefCell<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Rc<RefCell<Option<String>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.borrow().as_ref().unwrap())
    }
}

impl GoReflectType {
    fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some((*self.name.borrow().as_ref().unwrap()).clone())))
    }

    fn num_field(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap().len() as i32)))
    }

    fn field(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<GoReflectField>>> {
        let index = *index.borrow().as_ref().unwrap() as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }
}

fn go_reflect_tag_get(raw: &str, key: &str) -> String {
    let prefix = format!("{}:\"", key);
    let Some(start) = raw.find(&prefix) else {
        return String::new();
    };
    let rest = &raw[start + prefix.len()..];
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            break;
        } else {
            value.push(ch);
        }
    }
    value
}

#[derive(Debug, Clone)]
pub struct User {
    // tags: `json:"id" db:"user_id"`
    pub i_d: Rc<RefCell<Option<i32>>>,
    // tags: `json:"name,omitempty" db:"full_name"`
    pub name: Rc<RefCell<Option<String>>>,
    // tags: `json:"email" db:"email_address" validate:"email"`
    pub email: Rc<RefCell<Option<String>>>,
    // tags: `json:"is_active" db:"active"`
    pub is_active: Rc<RefCell<Option<bool>>>,
    pub internal: Rc<RefCell<Option<String>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        Self { i_d: { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, email: { let __guard = self.email.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, is_active: { let __guard = self.is_active.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, internal: { let __guard = self.internal.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for User {
    fn default() -> Self {
        Self { i_d: Rc::new(RefCell::new(Some(0))), name: Rc::new(RefCell::new(Some(String::new()))), email: Rc::new(RefCell::new(Some(String::new()))), is_active: Rc::new(RefCell::new(Some(false))), internal: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.i_d.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()), (*self.email.borrow().as_ref().unwrap()), (*self.is_active.borrow().as_ref().unwrap()), (*self.internal.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut u = Rc::new(RefCell::new(Some(User { i_d: Rc::new(RefCell::new(Some(1))), name: Rc::new(RefCell::new(Some("Alice".to_string()))), email: Rc::new(RefCell::new(Some("alice@example.com".to_string()))), ..Default::default() })));
    let mut t = Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("main.User".to_string()))), fields: Rc::new(RefCell::new(Some(vec![GoReflectField { name: Rc::new(RefCell::new(Some("ID".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("json:\"id\" db:\"user_id\"".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("Name".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("json:\"name,omitempty\" db:\"full_name\"".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("Email".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("json:\"email\" db:\"email_address\" validate:\"email\"".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("IsActive".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("json:\"is_active\" db:\"active\"".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("internal".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("".to_string()))) }))) }]))) })));

    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < (*(*t.borrow().as_ref().unwrap()).num_field().borrow().as_ref().unwrap()) {
        let mut field = (*t.borrow().as_ref().unwrap()).field(Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()).clone()))));
        print!("{}: json={:?} db={:?}\n", (*(*field.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*(*field.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some("json".to_string())))).borrow().as_ref().unwrap()), (*(*(*field.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some("db".to_string())))).borrow().as_ref().unwrap()));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}