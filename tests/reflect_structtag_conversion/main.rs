use std::sync::{Arc, Mutex};


#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Arc<Mutex<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let raw = (*self.raw.lock().unwrap().as_ref().unwrap()).clone();
        let key = (*key.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Arc<Mutex<Option<String>>>,
    tag: Arc<Mutex<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Arc<Mutex<Option<String>>>,
    kind: Arc<Mutex<Option<reflect_Kind>>>,
    elem: Arc<Mutex<Option<Box<GoReflectType>>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for GoReflectType {
    fn eq(&self, other: &Self) -> bool {
        *self.name.lock().unwrap() == *other.name.lock().unwrap() &&
            *self.kind.lock().unwrap() == *other.kind.lock().unwrap()
    }
}

impl Eq for GoReflectType {}

impl GoReflectType {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self.name.lock().unwrap().as_ref().unwrap()).clone())))
    }

    fn kind(&self) -> Arc<Mutex<Option<reflect_Kind>>> {
        self.kind.clone()
    }

    fn elem(&self) -> Arc<Mutex<Option<GoReflectType>>> {
        let elem_guard = self.elem.lock().unwrap();
        let elem = elem_guard.as_ref().expect("reflect.Type.Elem requires an element type").as_ref().clone();
        Arc::new(Mutex::new(Some(elem)))
    }

    fn num_field(&self) -> i32 {
        self.fields.lock().unwrap().as_ref().unwrap().len() as i32
    }

    fn field(&self, index: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<GoReflectField>>> {
        let index = *index.lock().unwrap().as_ref().unwrap() as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }
}

type GoReflectBoolGetter = Box<dyn Fn() -> bool + Send + Sync>;
type GoReflectBoolSetter = Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync>;

#[derive(Clone)]
struct GoReflectValue {
    typ: Arc<Mutex<Option<GoReflectType>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectValue>>>>,
    bool_getter: Arc<Mutex<Option<GoReflectBoolGetter>>>,
    bool_setter: Arc<Mutex<Option<GoReflectBoolSetter>>>,
    unsupported: Option<&'static str>,
}

impl GoReflectValue {
    fn panic_if_unsupported(&self, op: &str) {
        if let Some(message) = self.unsupported {
            panic!("{}: {}", op, message);
        }
    }

    fn elem(&self) -> Arc<Mutex<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Elem");
        Arc::new(Mutex::new(Some(self.clone())))
    }

    fn r#type(&self) -> Arc<Mutex<Option<GoReflectType>>> {
        self.panic_if_unsupported("reflect.Value.Type");
        self.typ.clone()
    }

    fn kind(&self) -> Arc<Mutex<Option<reflect_Kind>>> {
        self.panic_if_unsupported("reflect.Value.Kind");
        self.typ.lock().unwrap().as_ref().unwrap().kind()
    }

    fn field(&self, index: i32) -> Arc<Mutex<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Field");
        let index = index as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }

    fn set<T>(&self, _value: T) {
        self.panic_if_unsupported("reflect.Value.Set");
        panic!("reflect.Value.Set requires typed lowering")
    }

    fn set_bool(&mut self, value: Arc<Mutex<Option<bool>>>) {
        self.panic_if_unsupported("reflect.Value.SetBool");
        let mut setter_guard = self.bool_setter.lock().unwrap();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        self.panic_if_unsupported("reflect.Value.Bool");
        let getter_guard = self.bool_getter.lock().unwrap();
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


fn main() {
    let mut tag = Arc::new(Mutex::new(Some("json:\"name,omitempty\" db:\"full_name\"".to_string())));
    println!("{}", format!("{}", (*GoReflectStructTag { raw: Arc::new(Mutex::new(Some((*tag.lock().unwrap().as_ref().unwrap()).clone()))) }.get(Arc::new(Mutex::new(Some("json".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*GoReflectStructTag { raw: Arc::new(Mutex::new(Some((*tag.lock().unwrap().as_ref().unwrap()).clone()))) }.get(Arc::new(Mutex::new(Some("db".to_string())))).lock().unwrap().as_ref().unwrap())));
}