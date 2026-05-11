use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    x: Rc<RefCell<Option<i32>>>,
    y: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.borrow().as_ref().unwrap()), (*self.y.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct2 {
    name: Rc<RefCell<Option<String>>>,
    age: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
struct AnonymousStruct3 {
    host: Rc<RefCell<Option<String>>>,
    port: Rc<RefCell<Option<i32>>>,
    settings: Rc<RefCell<Option<AnonymousStruct4>>>,
}

impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { host: Default::default(), port: Default::default(), settings: Rc::new(RefCell::new(Some(AnonymousStruct4::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.host.borrow().as_ref().unwrap()), (*self.port.borrow().as_ref().unwrap()), (*self.settings.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct4 {
    debug: Rc<RefCell<Option<bool>>>,
    verbose: Rc<RefCell<Option<bool>>>,
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.debug.borrow().as_ref().unwrap()), (*self.verbose.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct5 {
    i_d: Rc<RefCell<Option<i32>>>,
    value: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.i_d.borrow().as_ref().unwrap()), (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct6 {
    r#type: Rc<RefCell<Option<String>>>,
    message: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.r#type.borrow().as_ref().unwrap()), (*self.message.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct7 {
    email: Rc<RefCell<Option<String>>>,
    admin: Rc<RefCell<Option<bool>>>,
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.email.borrow().as_ref().unwrap()), (*self.admin.borrow().as_ref().unwrap()))
    }
}


fn main() {
        // Anonymous struct as variable
    let mut point: Rc<RefCell<Option<AnonymousStruct1>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = 10; *(*point.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    { let new_val = 20; *(*point.borrow().as_ref().unwrap()).y.borrow_mut() = Some(new_val); };
    print!("Point: ({}, {})\n", (*(*point.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()), (*(*point.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()));

        // Anonymous struct literal
    let mut person = Rc::new(RefCell::new(Some(AnonymousStruct2 { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))) })));
    print!("Person: {}, {} years old\n", (*(*person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*person.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));

        // Anonymous struct with nested fields
    let mut config = Rc::new(RefCell::new(Some(AnonymousStruct3 { host: Rc::new(RefCell::new(Some("localhost".to_string()))), port: Rc::new(RefCell::new(Some(8080))), settings: Rc::new(RefCell::new(Some(AnonymousStruct4::default()))) })));
    { let new_val = true; *(*(*config.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).debug.borrow_mut() = Some(new_val); };
    { let new_val = false; *(*(*config.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).verbose.borrow_mut() = Some(new_val); };
    print!("Config: {}:{} (Debug: {}, Verbose: {})\n", (*(*config.borrow().as_ref().unwrap()).host.borrow().as_ref().unwrap()), (*(*config.borrow().as_ref().unwrap()).port.borrow().as_ref().unwrap()), (*(*(*config.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).debug.borrow().as_ref().unwrap()), (*(*(*config.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).verbose.borrow().as_ref().unwrap()));

        // Array of anonymous structs
    let mut items: Rc<RefCell<Option<[AnonymousStruct5; 2]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default()))));
    { let new_val = 1; *(*items.borrow().as_ref().unwrap())[(0) as usize].clone().i_d.borrow_mut() = Some(new_val); };
    { let new_val = "first".to_string(); *(*items.borrow().as_ref().unwrap())[(0) as usize].clone().value.borrow_mut() = Some(new_val); };
    { let new_val = 2; *(*items.borrow().as_ref().unwrap())[(1) as usize].clone().i_d.borrow_mut() = Some(new_val); };
    { let new_val = "second".to_string(); *(*items.borrow().as_ref().unwrap())[(1) as usize].clone().value.borrow_mut() = Some(new_val); };
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, item) in __range_values.iter().enumerate() {
        print!("Item {}: {{ID: {}, Value: {}}}\n", i, (*item.i_d.borrow().as_ref().unwrap()), (*item.value.borrow().as_ref().unwrap()));
    } }

        // Slice of anonymous structs
    let mut events = Rc::new(RefCell::new(Some(vec![AnonymousStruct6 { r#type: Rc::new(RefCell::new(Some("info".to_string()))), message: Rc::new(RefCell::new(Some("System started".to_string()))), ..Default::default() }, AnonymousStruct6 { r#type: Rc::new(RefCell::new(Some("warning".to_string()))), message: Rc::new(RefCell::new(Some("Low memory".to_string()))), ..Default::default() }, AnonymousStruct6 { r#type: Rc::new(RefCell::new(Some("error".to_string()))), message: Rc::new(RefCell::new(Some("Connection failed".to_string()))), ..Default::default() }])));
    { let __range_holder = events.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for event in __range_values.iter() {
        print!("Event [{}]: {}\n", (*event.r#type.borrow().as_ref().unwrap()), (*event.message.borrow().as_ref().unwrap()));
    } }

        // Map with anonymous struct values
    let mut users = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<AnonymousStruct7>>>>::from([("alice".to_string(), Rc::new(RefCell::new(Some(AnonymousStruct7 { email: Rc::new(RefCell::new(Some("alice@example.com".to_string()))), admin: Rc::new(RefCell::new(Some(true))), ..Default::default() })))), ("bob".to_string(), Rc::new(RefCell::new(Some(AnonymousStruct7 { email: Rc::new(RefCell::new(Some("bob@example.com".to_string()))), admin: Rc::new(RefCell::new(Some(false))), ..Default::default() }))))]))));
    let mut userNames: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    for (name, _) in { let __range_holder = users.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = userNames.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(name.clone()); __append_target.clone() }; userNames = new_val; };
    }
    (*userNames.borrow_mut().as_mut().unwrap()).sort();
    { let __range_holder = userNames.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for name in __range_values.iter() {
        let mut user = Rc::new(RefCell::new(Some((*users.borrow().as_ref().unwrap()).get(name).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Default::default()))));
        print!("User {}: {} (admin: {})\n", name, (*(*user.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()), (*(*user.borrow().as_ref().unwrap()).admin.borrow().as_ref().unwrap()));
    } }
}