use std::collections::HashMap;
use std::sync::{Arc, Mutex};


fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

/// Named struct with anonymous struct field
#[derive(Debug, Clone, Default)]
struct Config {
    name: Arc<Mutex<Option<String>>>,
    database: Arc<Mutex<Option<AnonymousStruct1>>>,
    cache: Arc<Mutex<Option<AnonymousStruct3>>>,
}

/// Named struct with slice of anonymous structs
#[derive(Debug, Clone, Default)]
struct Dashboard {
    title: Arc<Mutex<Option<String>>>,
    widgets: Arc<Mutex<Option<Vec<AnonymousStruct4>>>>,
}

#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    host: Arc<Mutex<Option<String>>>,
    port: Arc<Mutex<Option<i32>>>,
    credentials: Arc<Mutex<Option<AnonymousStruct2>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct2 {
    user: Arc<Mutex<Option<String>>>,
    password: Arc<Mutex<Option<String>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct3 {
    enabled: Arc<Mutex<Option<bool>>>,
    t_t_l: Arc<Mutex<Option<i32>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct4 {
    i_d: Arc<Mutex<Option<i32>>>,
    r#type: Arc<Mutex<Option<String>>>,
    position: Arc<Mutex<Option<AnonymousStruct5>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct5 {
    x: Arc<Mutex<Option<i32>>>,
    y: Arc<Mutex<Option<i32>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct6 {
    version: Arc<Mutex<Option<String>>>,
    modules: Arc<Mutex<Option<HashMap<String, AnonymousStruct7>>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct7 {
    enabled: Arc<Mutex<Option<bool>>>,
    settings: Arc<Mutex<Option<AnonymousStruct8>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct8 {
    options: Arc<Mutex<Option<Vec<AnonymousStruct9>>>>,
}


fn main() {
        // Test named struct with anonymous struct field
    let mut config = Arc::new(Mutex::new(Some(Config { name: Arc::new(Mutex::new(Some("production".to_string()))) })));
    { let new_val = "db.example.com".to_string(); *(*config.lock().unwrap().as_mut().unwrap()).database.host.lock().unwrap() = Some(new_val); };
    { let new_val = 5432; *(*config.lock().unwrap().as_mut().unwrap()).database.port.lock().unwrap() = Some(new_val); };
    { let new_val = "admin".to_string(); *(*config.lock().unwrap().as_mut().unwrap()).database.credentials.user.lock().unwrap() = Some(new_val); };
    { let new_val = "secret".to_string(); *(*config.lock().unwrap().as_mut().unwrap()).database.credentials.password.lock().unwrap() = Some(new_val); };
    { let new_val = true; *(*config.lock().unwrap().as_mut().unwrap()).cache.enabled.lock().unwrap() = Some(new_val); };
    { let new_val = 300; *(*config.lock().unwrap().as_mut().unwrap()).cache.t_t_l.lock().unwrap() = Some(new_val); };

    print!("Config: {}\n", (*(*config.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()));
    print!("Database: {}:{} (user: {})\n", (*config.lock().unwrap().as_mut().unwrap()).database.host, (*config.lock().unwrap().as_mut().unwrap()).database.port, (*config.lock().unwrap().as_mut().unwrap()).database.credentials.user);
    print!("Cache: enabled={}, TTL={}\n", (*config.lock().unwrap().as_mut().unwrap()).cache.enabled, (*config.lock().unwrap().as_mut().unwrap()).cache.t_t_l);

        // Test named struct with slice of anonymous structs
    let mut dashboard = Arc::new(Mutex::new(Some(Dashboard { title: Arc::new(Mutex::new(Some("Main Dashboard".to_string()))), widgets: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec![, ])))))) })));

    print!("\nDashboard: {}\n", (*(*dashboard.lock().unwrap().as_ref().unwrap()).title.lock().unwrap().as_ref().unwrap()));
    for widget in &(*dashboard.lock().unwrap().as_ref().unwrap()).widgets {
        print!("Widget {} ({}) at position ({}, {})\n", widget.i_d, widget.r#type, widget.position.x, widget.position.y);
    }

        // Deeply nested anonymous structs
    let mut system: Arc<Mutex<Option<AnonymousStruct6>>>;

    { let new_val = "1.0.0".to_string(); *(*system.lock().unwrap().as_mut().unwrap()).version.lock().unwrap() = Some(new_val); };
    { let new_val = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<AnonymousStruct7>>>>::new()))); *(*system.lock().unwrap().as_mut().unwrap()).modules.lock().unwrap() = Some(new_val); };

        // Add a module with settings
    let mut authModule = Arc::new(Mutex::new(Some(AnonymousStruct7 { enabled: true.clone(), settings: Default::default() })));
    { let new_val = Arc::new(Mutex::new(Some(vec![, ]))); *(*authModule.lock().unwrap().as_mut().unwrap()).settings.options.lock().unwrap() = Some(new_val); };
    (*(*system.lock().unwrap().as_ref().unwrap()).modules.lock().unwrap().as_mut().unwrap()).insert("auth".to_string(), authModule.clone());

    print!("\nSystem version: {}\n", (*(*system.lock().unwrap().as_ref().unwrap()).version.lock().unwrap().as_ref().unwrap()));
    for (name, module) in (*(*system.lock().unwrap().as_ref().unwrap()).modules.lock().unwrap().as_ref().unwrap()).clone() {
        print!("Module {}: enabled={}\n", name, module.enabled);
        for opt in &module.settings.options {
        print!("  - {}: {}\n", opt.key, format_any((opt.value).lock().unwrap().as_ref().unwrap().as_ref()));
    }
    }
}