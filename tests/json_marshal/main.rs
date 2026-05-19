use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


fn go_json_escape(input: &str) -> String {
    let mut escaped = String::new();
    for ch in input.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c < ' ' => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}

#[derive(Debug, Clone)]
pub struct User {
    // tags: `json:"name"`
    pub name: Rc<RefCell<Option<String>>>,
    // tags: `json:"age"`
    pub age: Rc<RefCell<Option<i32>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, age: { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for User {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))), age: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut u = Rc::new(RefCell::new(Some(User { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))), ..Default::default() })));
    let (mut data, _) = { let __json_value = (*u.borrow().as_ref().unwrap()).clone(); let mut __json_fields: Vec<String> = Vec::new(); __json_fields.push(format!("\"name\":\"{}\"", go_json_escape(__json_value.name.borrow().as_ref().unwrap()))); __json_fields.push(format!("\"age\":{}", *__json_value.age.borrow().as_ref().unwrap())); let __json = format!("{{{}}}", __json_fields.join(",")); (Rc::new(RefCell::new(Some(__json.into_bytes()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))) };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*data.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
}