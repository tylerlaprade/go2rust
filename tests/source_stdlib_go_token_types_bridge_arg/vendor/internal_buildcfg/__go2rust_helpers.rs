use std::any::Any;
use std::sync::{Arc, Mutex};


fn go_embedded_owner_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_register_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, owner: Arc<Mutex<Option<T>>>) {
    go_embedded_owner_registry().lock().unwrap().insert(embedded_key, Box::new(owner));
}

fn go_lookup_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, target: &str) -> Arc<Mutex<Option<T>>> {
    let registry = go_embedded_owner_registry().lock().unwrap();
    let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
    owner
        .downcast_ref::<Arc<Mutex<Option<T>>>>()
        .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
        .clone()
}

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
    fields: Arc<Mutex<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lock().unwrap().as_ref().unwrap())
    }
}

impl GoReflectType {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self.name.lock().unwrap().as_ref().unwrap()).clone())))
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
}

impl GoReflectValue {
    fn elem(&self) -> Arc<Mutex<Option<GoReflectValue>>> {
        Arc::new(Mutex::new(Some(self.clone())))
    }

    fn r#type(&self) -> Arc<Mutex<Option<GoReflectType>>> {
        self.typ.clone()
    }

    fn field(&self, index: i32) -> Arc<Mutex<Option<GoReflectValue>>> {
        let index = index as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }

    fn set_bool(&mut self, value: Arc<Mutex<Option<bool>>>) {
        let mut setter_guard = self.bool_setter.lock().unwrap();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
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
