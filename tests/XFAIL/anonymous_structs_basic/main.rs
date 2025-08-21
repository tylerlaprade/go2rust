use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    x: Arc<Mutex<Option<i32>>>,
    y: Arc<Mutex<Option<i32>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct2 {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct3 {
    host: Arc<Mutex<Option<String>>>,
    port: Arc<Mutex<Option<i32>>>,
    settings: Arc<Mutex<Option<AnonymousStruct4>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct4 {
    debug: Arc<Mutex<Option<bool>>>,
    verbose: Arc<Mutex<Option<bool>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct5 {
    i_d: Arc<Mutex<Option<i32>>>,
    value: Arc<Mutex<Option<String>>>,
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct6 {
    email: Arc<Mutex<Option<String>>>,
    admin: Arc<Mutex<Option<bool>>>,
}


fn main() {
        // Anonymous struct as variable
    let mut point: Arc<Mutex<Option<AnonymousStruct1>>>;
    { let new_val = 10; *(*point.lock().unwrap().as_mut().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 20; *(*point.lock().unwrap().as_mut().unwrap()).y.lock().unwrap() = Some(new_val); };
    print!("Point: ({}, {})\n", (*(*point.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()), (*(*point.lock().unwrap().as_ref().unwrap()).y.lock().unwrap().as_ref().unwrap()));

        // Anonymous struct literal
    let mut person = Arc::new(Mutex::new(Some(AnonymousStruct2 { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) })));
    print!("Person: {}, {} years old\n", (*(*person.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()), (*(*person.lock().unwrap().as_ref().unwrap()).age.lock().unwrap().as_ref().unwrap()));

        // Anonymous struct with nested fields
    let mut config = Arc::new(Mutex::new(Some(AnonymousStruct3 { host: Arc::new(Mutex::new(Some("localhost".to_string()))), port: Arc::new(Mutex::new(Some(8080))), settings: Default::default() })));
    { let new_val = true; *(*config.lock().unwrap().as_mut().unwrap()).settings.debug.lock().unwrap() = Some(new_val); };
    { let new_val = false; *(*config.lock().unwrap().as_mut().unwrap()).settings.verbose.lock().unwrap() = Some(new_val); };
    print!("Config: {}:{} (Debug: {}, Verbose: {})\n", (*(*config.lock().unwrap().as_ref().unwrap()).host.lock().unwrap().as_ref().unwrap()), (*(*config.lock().unwrap().as_ref().unwrap()).port.lock().unwrap().as_ref().unwrap()), (*config.lock().unwrap().as_mut().unwrap()).settings.debug, (*config.lock().unwrap().as_mut().unwrap()).settings.verbose);

        // Array of anonymous structs
    let mut items: Arc<Mutex<Option<[AnonymousStruct5; 2]>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = 1; *(*items.lock().unwrap().as_ref().unwrap())[0 as usize].clone().i_d.lock().unwrap() = Some(new_val); };
    { let new_val = "first".to_string(); *(*items.lock().unwrap().as_ref().unwrap())[0 as usize].clone().value.lock().unwrap() = Some(new_val); };
    { let new_val = 2; *(*items.lock().unwrap().as_ref().unwrap())[1 as usize].clone().i_d.lock().unwrap() = Some(new_val); };
    { let new_val = "second".to_string(); *(*items.lock().unwrap().as_ref().unwrap())[1 as usize].clone().value.lock().unwrap() = Some(new_val); };
    for (i, item) in (*items.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Item {}: {ID: {}, Value: {}}\n", i, item.i_d, item.value);
    }

        // Slice of anonymous structs
    let mut events = Arc::new(Mutex::new(Some(vec![, , ])));
    for event in &(*events.lock().unwrap().as_mut().unwrap()) {
        print!("Event [{}]: {}\n", event.r#type, event.message);
    }

        // Map with anonymous struct values
    let mut users = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<AnonymousStruct6>>>>::from([("alice".to_string(), Arc::new(Mutex::new(Some()))), ("bob".to_string(), Arc::new(Mutex::new(Some())))]))));
    for (name, user) in (*users.lock().unwrap().as_ref().unwrap()).clone() {
        print!("User {}: {} (admin: {})\n", name, user.email, user.admin);
    }
}