use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


#[derive(Clone, Copy)]
struct GoAnyTypeMetadata {
    pub kind: &'static str,
    pub comparable: bool,
    pub elem_kind: Option<&'static str>,
    pub elem_comparable: bool,
}

struct GoAnyMetadataBox {
    pub value: Box<dyn Any + Send + Sync>,
    pub metadata: GoAnyTypeMetadata,
}

fn go_any_type_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_key(value: &(dyn Any + Send + Sync)) -> usize {
    value as *const (dyn Any + Send + Sync) as *const () as usize
}

fn go_register_any_type<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_register_any_type_with_elem<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

fn go_box_any_with_metadata<T: Any + Send + Sync + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any + Send + Sync> {
    let metadata = GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false };
    Box::new(GoAnyMetadataBox { value: Box::new(value) as Box<dyn Any + Send + Sync>, metadata }) as Box<dyn Any + Send + Sync>
}

fn go_register_any_value_metadata(value: &(dyn Any + Send + Sync), kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_any_type_metadata(value: &(dyn Any + Send + Sync)) -> Option<GoAnyTypeMetadata> {
    if let Some(__boxed) = value.downcast_ref::<GoAnyMetadataBox>() {
        return Some(__boxed.metadata);
    }
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
}

#[derive(Debug, Clone, Default)]
pub struct alphaErr {
}

impl alphaErr {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for alphaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct betaErr {
}

impl betaErr {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for betaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct gammaErr {
}

impl gammaErr {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for gammaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct deltaErr {
}

impl deltaErr {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for deltaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


impl alphaErr {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("alpha".to_string())))
    }
}

impl StdError for alphaErr {}


impl betaErr {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("beta".to_string())))
    }
}

impl StdError for betaErr {}


impl gammaErr {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("gamma".to_string())))
    }
}

impl StdError for gammaErr {}


impl deltaErr {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("delta".to_string())))
    }
}

impl StdError for deltaErr {}


pub fn as_any(err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    Arc::new(Mutex::new(Some({
        let __err_holder = err.clone();
        let __err_guard = __err_holder.lock().unwrap();
        match __err_guard.as_ref() {
            None => panic!("nil error-to-any lowering requires nil interface representation"),
            Some(__err) => {
                if let Some(typed_val) = __err.downcast_ref::<alphaErr>() {
                    go_box_any_with_metadata(typed_val.clone(), "struct", true)
                } else if let Some(typed_val) = __err.downcast_ref::<betaErr>() {
                    go_box_any_with_metadata(typed_val.clone(), "struct", true)
                } else if let Some(typed_val) = __err.downcast_ref::<deltaErr>() {
                    go_box_any_with_metadata(typed_val.clone(), "struct", true)
                } else if let Some(typed_val) = __err.downcast_ref::<gammaErr>() {
                    go_box_any_with_metadata(typed_val.clone(), "struct", true)
                } else {
                    panic!("type info required: error-to-any for unknown dynamic error type")
                }
            }
        }
    })))
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let _ = as_any(Arc::new(Mutex::new(Some(Box::new(deltaErr {  }) as Box<dyn StdError + Send + Sync>))));
    println!("{}", format!("{}", "boxed".to_string()));
}