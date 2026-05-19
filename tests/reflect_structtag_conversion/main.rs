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

    fn num_field(&self) -> Arc<Mutex<Option<i32>>> {
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap().len() as i32)))
    }

    fn field(&self, index: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<GoReflectField>>> {
        let index = *index.lock().unwrap().as_ref().unwrap() as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
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

fn main() {
    let mut tag = Arc::new(Mutex::new(Some("json:\"name,omitempty\" db:\"full_name\"".to_string())));
    println!("{}", format!("{}", (*GoReflectStructTag { raw: Arc::new(Mutex::new(Some((*tag.lock().unwrap().as_ref().unwrap()).clone()))) }.get(Arc::new(Mutex::new(Some("json".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*GoReflectStructTag { raw: Arc::new(Mutex::new(Some((*tag.lock().unwrap().as_ref().unwrap()).clone()))) }.get(Arc::new(Mutex::new(Some("db".to_string())))).lock().unwrap().as_ref().unwrap())));
}