pub fn get_hello() -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some("Hello".to_string())));
}

pub fn get_world() -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some("World".to_string())));
}

pub fn get_magic_number() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
}