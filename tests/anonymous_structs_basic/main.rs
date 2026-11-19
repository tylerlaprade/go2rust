use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn main() {
        // Anonymous struct as variable
    let mut point: Rc<RefCell<Option<AnonymousStruct1>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = 10; *(*point.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    { let new_val = 20; *(*point.borrow().as_ref().unwrap()).y.borrow_mut() = Some(new_val); };
    print!("Point: ({}, {})\n", (*(*point.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()), (*(*point.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()));

        // Anonymous struct literal
    let mut person = Rc::new(RefCell::new(Some(AnonymousStruct2 { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))) })));
    print!("Person: {}, {} years old\n", (*(*person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*person.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));

        // Anonymous struct with nested fields
    let mut config = Rc::new(RefCell::new(Some(AnonymousStruct3 { host: Rc::new(RefCell::new(Some("localhost".to_string()))), port: Rc::new(RefCell::new(Some(8080))), settings: Rc::new(RefCell::new(Some(AnonymousStruct4::default()))) })));
    { let new_val = true; *(*(*config.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).debug.borrow_mut() = Some(new_val); };
    { let new_val = false; *(*(*config.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).verbose.borrow_mut() = Some(new_val); };
    print!("Config: {}:{} (Debug: {}, Verbose: {})\n", (*(*config.borrow().as_ref().unwrap()).host.borrow().as_ref().unwrap()).clone(), (*(*config.borrow().as_ref().unwrap()).port.borrow().as_ref().unwrap()), (*(*(*config.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).debug.borrow().as_ref().unwrap()), (*(*(*config.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).verbose.borrow().as_ref().unwrap()));

        // Array of anonymous structs
    let mut items: Rc<RefCell<Option<[AnonymousStruct5; 2]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default()))));
    { let new_val = 1; *(*items.borrow().as_ref().unwrap())[(0) as usize].clone().i_d.borrow_mut() = Some(new_val); };
    { let new_val = "first".to_string(); *(*items.borrow().as_ref().unwrap())[(0) as usize].clone().value.borrow_mut() = Some(new_val); };
    { let new_val = 2; *(*items.borrow().as_ref().unwrap())[(1) as usize].clone().i_d.borrow_mut() = Some(new_val); };
    { let new_val = "second".to_string(); *(*items.borrow().as_ref().unwrap())[(1) as usize].clone().value.borrow_mut() = Some(new_val); };
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for (i, item) in __range_values.iter().enumerate() {
        let item = &item.__go_value_clone();
        print!("Item {}: {{ID: {}, Value: {}}}\n", i, (*item.i_d.borrow().as_ref().unwrap()), (*item.value.borrow().as_ref().unwrap()).clone());
    } }

        // Slice of anonymous structs
    let mut events = Rc::new(RefCell::new(Some(vec![AnonymousStruct6 { r#type: Rc::new(RefCell::new(Some("info".to_string()))), message: Rc::new(RefCell::new(Some("System started".to_string()))), ..Default::default() }, AnonymousStruct6 { r#type: Rc::new(RefCell::new(Some("warning".to_string()))), message: Rc::new(RefCell::new(Some("Low memory".to_string()))), ..Default::default() }, AnonymousStruct6 { r#type: Rc::new(RefCell::new(Some("error".to_string()))), message: Rc::new(RefCell::new(Some("Connection failed".to_string()))), ..Default::default() }])));
    { let __range_holder = events.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for event in __range_values.iter() {
        let event = &event.__go_value_clone();
        print!("Event [{}]: {}\n", (*event.r#type.borrow().as_ref().unwrap()).clone(), (*event.message.borrow().as_ref().unwrap()).clone());
    } }

        // Map with anonymous struct values
    let mut users = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<AnonymousStruct7>>>>::from([("alice".to_string(), Rc::new(RefCell::new(Some(AnonymousStruct7 { email: Rc::new(RefCell::new(Some("alice@example.com".to_string()))), admin: Rc::new(RefCell::new(Some(true))), ..Default::default() })))), ("bob".to_string(), Rc::new(RefCell::new(Some(AnonymousStruct7 { email: Rc::new(RefCell::new(Some("bob@example.com".to_string()))), admin: Rc::new(RefCell::new(Some(false))), ..Default::default() }))))]))));
    let mut userNames: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    for (name, _) in { let __range_holder = users.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = userNames.clone(); let __append_value = name.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; userNames = new_val; };
    }
    { let __sort_target = userNames.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = userNames.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        let mut user = Rc::new(RefCell::new(Some({ let __map_holder = users.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(name)).map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) })));
        print!("User {}: {} (admin: {})\n", name, (*(*user.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()).clone(), (*(*user.borrow().as_ref().unwrap()).admin.borrow().as_ref().unwrap()));
    } }
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub x: Rc<RefCell<Option<i32>>>,
    pub y: Rc<RefCell<Option<i32>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct2 {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i32>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub host: Rc<RefCell<Option<String>>>,
    pub port: Rc<RefCell<Option<i32>>>,
    pub settings: Rc<RefCell<Option<AnonymousStruct4>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.host.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.port.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.settings.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            host: __go_clone_0_0,
            port: __go_clone_1_0,
            settings: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(AnonymousStruct4::default())));
        Self {
            host: __go_default_0_0,
            port: __go_default_1_0,
            settings: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.host.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.port.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.settings.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct4 {
    pub debug: Rc<RefCell<Option<bool>>>,
    pub verbose: Rc<RefCell<Option<bool>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.debug.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.verbose.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            debug: __go_clone_0_0,
            verbose: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            debug: __go_default_0_0,
            verbose: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.debug.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.verbose.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct5 {
    pub i_d: Rc<RefCell<Option<i32>>>,
    pub value: Rc<RefCell<Option<String>>>,
}
impl AnonymousStruct5 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            i_d: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            i_d: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.i_d.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct6 {
    pub r#type: Rc<RefCell<Option<String>>>,
    pub message: Rc<RefCell<Option<String>>>,
}
impl AnonymousStruct6 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.message.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            r#type: __go_clone_0_0,
            message: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            r#type: __go_default_0_0,
            message: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.message.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct7 {
    pub email: Rc<RefCell<Option<String>>>,
    pub admin: Rc<RefCell<Option<bool>>>,
}
impl AnonymousStruct7 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.email.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.admin.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            email: __go_clone_0_0,
            admin: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            email: __go_default_0_0,
            admin: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.email.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.admin.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
