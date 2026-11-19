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
    kind: Rc<RefCell<Option<reflect_Kind>>>,
    elem: Rc<RefCell<Option<Box<GoReflectType>>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.borrow().as_ref().unwrap())
    }
}

impl PartialEq for GoReflectType {
    fn eq(&self, other: &Self) -> bool {
        *self.name.borrow() == *other.name.borrow() &&
            *self.kind.borrow() == *other.kind.borrow()
    }
}

impl Eq for GoReflectType {}

impl GoReflectType {
    fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some((*self.name.borrow().as_ref().unwrap()).clone())))
    }

    fn kind(&self) -> Rc<RefCell<Option<reflect_Kind>>> {
        self.kind.clone()
    }

    fn elem(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        let elem_guard = self.elem.borrow();
        let elem = elem_guard.as_ref().expect("reflect.Type.Elem requires an element type").as_ref().clone();
        Rc::new(RefCell::new(Some(elem)))
    }

    fn num_field(&self) -> i32 {
        self.fields.borrow().as_ref().unwrap().len() as i32
    }

    fn field(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<GoReflectField>>> {
        let index = *index.borrow().as_ref().unwrap() as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }
}

type GoReflectBoolGetter = Box<dyn Fn() -> bool>;
type GoReflectBoolSetter = Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()>;

#[derive(Clone)]
struct GoReflectValue {
    typ: Rc<RefCell<Option<GoReflectType>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectValue>>>>,
    bool_getter: Rc<RefCell<Option<GoReflectBoolGetter>>>,
    bool_setter: Rc<RefCell<Option<GoReflectBoolSetter>>>,
    unsupported: Option<&'static str>,
}

impl GoReflectValue {
    fn panic_if_unsupported(&self, op: &str) {
        if let Some(message) = self.unsupported {
            panic!("{}: {}", op, message);
        }
    }

    fn elem(&self) -> Rc<RefCell<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Elem");
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn r#type(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        self.panic_if_unsupported("reflect.Value.Type");
        self.typ.clone()
    }

    fn kind(&self) -> Rc<RefCell<Option<reflect_Kind>>> {
        self.panic_if_unsupported("reflect.Value.Kind");
        self.typ.borrow().as_ref().unwrap().kind()
    }

    fn field(&self, index: i32) -> Rc<RefCell<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Field");
        let index = index as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }

    fn set<T>(&self, _value: T) {
        self.panic_if_unsupported("reflect.Value.Set");
        panic!("reflect.Value.Set requires typed lowering")
    }

    fn set_bool(&mut self, value: Rc<RefCell<Option<bool>>>) {
        self.panic_if_unsupported("reflect.Value.SetBool");
        let mut setter_guard = self.bool_setter.borrow_mut();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        self.panic_if_unsupported("reflect.Value.Bool");
        let getter_guard = self.bool_getter.borrow();
        let getter = getter_guard.as_ref().expect("reflect.Value.Bool requires a bool field");
        getter()
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct reflect_Kind(pub u64);

impl PartialEq<u64> for reflect_Kind {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<reflect_Kind> for u64 {
    fn eq(&self, other: &reflect_Kind) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for reflect_Kind {
    type Output = reflect_Kind;
    fn bitand(self, other: Self) -> reflect_Kind {
        reflect_Kind(self.0 & other.0)
    }
}

impl std::ops::BitOr for reflect_Kind {
    type Output = reflect_Kind;
    fn bitor(self, other: Self) -> reflect_Kind {
        reflect_Kind(self.0 | other.0)
    }
}

impl std::fmt::Display for reflect_Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Kind>")
    }
}


impl reflect_Kind {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
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
        let __go_clone_0_0 = { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.email.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.is_active.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.internal.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            i_d: __go_clone_0_0,
            name: __go_clone_1_0,
            email: __go_clone_2_0,
            is_active: __go_clone_3_0,
            internal: __go_clone_4_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(false)));
        let __go_default_4_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            i_d: __go_default_0_0,
            name: __go_default_1_0,
            email: __go_default_2_0,
            is_active: __go_default_3_0,
            internal: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.i_d.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.email.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.is_active.borrow().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.internal.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}


fn main() {
    let mut u = Rc::new(RefCell::new(Some(User { i_d: Rc::new(RefCell::new(Some(1))), name: Rc::new(RefCell::new(Some("Alice".to_string()))), email: Rc::new(RefCell::new(Some("alice@example.com".to_string()))), ..Default::default() })));
    let mut t = Rc::new(RefCell::new(Some(GoReflectType { name: Rc::new(RefCell::new(Some("main.User".to_string()))), kind: Rc::new(RefCell::new(Some(reflect_Kind(25)))), elem: Rc::new(RefCell::new(None::<Box<GoReflectType>>)), fields: Rc::new(RefCell::new(Some(vec![GoReflectField { name: Rc::new(RefCell::new(Some("ID".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("json:\"id\" db:\"user_id\"".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("Name".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("json:\"name,omitempty\" db:\"full_name\"".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("Email".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("json:\"email\" db:\"email_address\" validate:\"email\"".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("IsActive".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("json:\"is_active\" db:\"active\"".to_string()))) }))) }, GoReflectField { name: Rc::new(RefCell::new(Some("internal".to_string()))), tag: Rc::new(RefCell::new(Some(GoReflectStructTag { raw: Rc::new(RefCell::new(Some("".to_string()))) }))) }]))) })));

    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < (*t.borrow().as_ref().unwrap()).num_field() {
        let mut field = (*t.borrow().as_ref().unwrap()).field(Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        print!("{}: json={:?} db={:?}\n", (*(*field.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*(*field.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some("json".to_string())))).borrow().as_ref().unwrap()), (*(*(*field.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).get(Rc::new(RefCell::new(Some("db".to_string())))).borrow().as_ref().unwrap()));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}