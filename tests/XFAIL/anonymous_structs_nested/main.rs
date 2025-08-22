use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::{Rc};


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
    name: Rc<RefCell<Option<String>>>,
    database: Rc<RefCell<Option<AnonymousStruct1>>>,
    cache: Rc<RefCell<Option<AnonymousStruct3>>>,
}

/// Named struct with slice of anonymous structs
#[derive(Debug, Clone, Default)]
struct Dashboard {
    title: Rc<RefCell<Option<String>>>,
    widgets: Rc<RefCell<Option<Vec<AnonymousStruct4>>>>,
}

#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    host: Rc<RefCell<Option<String>>>,
    port: Rc<RefCell<Option<i32>>>,
    credentials: Rc<RefCell<Option<AnonymousStruct2>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct2 {
    user: Rc<RefCell<Option<String>>>,
    password: Rc<RefCell<Option<String>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct3 {
    enabled: Rc<RefCell<Option<bool>>>,
    t_t_l: Rc<RefCell<Option<i32>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct4 {
    i_d: Rc<RefCell<Option<i32>>>,
    r#type: Rc<RefCell<Option<String>>>,
    position: Rc<RefCell<Option<AnonymousStruct5>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct5 {
    x: Rc<RefCell<Option<i32>>>,
    y: Rc<RefCell<Option<i32>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct6 {
    version: Rc<RefCell<Option<String>>>,
    modules: Rc<RefCell<Option<HashMap<String, AnonymousStruct7>>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct7 {
    enabled: Rc<RefCell<Option<bool>>>,
    settings: Rc<RefCell<Option<AnonymousStruct8>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct8 {
    options: Rc<RefCell<Option<Vec<AnonymousStruct9>>>>,
}


fn main() {
        // Test named struct with anonymous struct field
    let mut config = Rc::new(RefCell::new(Some(Config { name: Rc::new(RefCell::new(Some("production".to_string()))) })));
    { let new_val = "db.example.com".to_string(); *(*config.borrow_mut().as_mut().unwrap()).database.host.borrow_mut() = Some(new_val); };
    { let new_val = 5432; *(*config.borrow_mut().as_mut().unwrap()).database.port.borrow_mut() = Some(new_val); };
    { let new_val = "admin".to_string(); *(*config.borrow_mut().as_mut().unwrap()).database.credentials.user.borrow_mut() = Some(new_val); };
    { let new_val = "secret".to_string(); *(*config.borrow_mut().as_mut().unwrap()).database.credentials.password.borrow_mut() = Some(new_val); };
    { let new_val = true; *(*config.borrow_mut().as_mut().unwrap()).cache.enabled.borrow_mut() = Some(new_val); };
    { let new_val = 300; *(*config.borrow_mut().as_mut().unwrap()).cache.t_t_l.borrow_mut() = Some(new_val); };

    print!("Config: {}\n", (*(*config.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("Database: {}:{} (user: {})\n", (*config.borrow_mut().as_mut().unwrap()).database.host, (*config.borrow_mut().as_mut().unwrap()).database.port, (*config.borrow_mut().as_mut().unwrap()).database.credentials.user);
    print!("Cache: enabled={}, TTL={}\n", (*config.borrow_mut().as_mut().unwrap()).cache.enabled, (*config.borrow_mut().as_mut().unwrap()).cache.t_t_l);

        // Test named struct with slice of anonymous structs
    let mut dashboard = Rc::new(RefCell::new(Some(Dashboard { title: Rc::new(RefCell::new(Some("Main Dashboard".to_string()))), widgets: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(vec![, ])))))) })));

    print!("\nDashboard: {}\n", (*(*dashboard.borrow().as_ref().unwrap()).title.borrow().as_ref().unwrap()));
    for widget in &(*dashboard.borrow().as_ref().unwrap()).widgets {
        print!("Widget {} ({}) at position ({}, {})\n", widget.i_d, widget.r#type, widget.position.x, widget.position.y);
    }

        // Deeply nested anonymous structs
    let mut system: Rc<RefCell<Option<AnonymousStruct6>>>;

    { let new_val = "1.0.0".to_string(); *(*system.borrow_mut().as_mut().unwrap()).version.borrow_mut() = Some(new_val); };
    { let new_val = Rc::new(RefCell::new(Some(HashMap::<String, Rc<RefCell<Option<AnonymousStruct7>>>>::new()))); *(*system.borrow_mut().as_mut().unwrap()).modules.borrow_mut() = Some(new_val); };

        // Add a module with settings
    let mut authModule = Rc::new(RefCell::new(Some(AnonymousStruct7 { enabled: true.clone(), settings: Default::default() })));
    { let new_val = Rc::new(RefCell::new(Some(vec![, ]))); *(*authModule.borrow_mut().as_mut().unwrap()).settings.options.borrow_mut() = Some(new_val); };
    (*(*system.borrow().as_ref().unwrap()).modules.borrow_mut().as_mut().unwrap()).insert("auth".to_string(), authModule.clone());

    print!("\nSystem version: {}\n", (*(*system.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()));
    for (name, module) in (*(*system.borrow().as_ref().unwrap()).modules.borrow().as_ref().unwrap()).clone() {
        print!("Module {}: enabled={}\n", name, module.enabled);
        for opt in &module.settings.options {
        print!("  - {}: {}\n", opt.key, format_any((opt.value).borrow().as_ref().unwrap().as_ref()));
    }
    }
}