use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

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
#[derive(Debug, Clone)]
pub struct Config {
    pub name: Rc<RefCell<Option<String>>>,
    pub database: Rc<RefCell<Option<AnonymousStruct1>>>,
    pub cache: Rc<RefCell<Option<AnonymousStruct3>>>,
}


impl Default for Config {
    fn default() -> Self {
        Self { name: Default::default(), database: Rc::new(RefCell::new(Some(AnonymousStruct1::default()))), cache: Rc::new(RefCell::new(Some(AnonymousStruct3::default()))) }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.database.borrow().as_ref().unwrap()), (*self.cache.borrow().as_ref().unwrap()))
    }
}


/// Named struct with slice of anonymous structs
#[derive(Debug, Clone, Default)]
pub struct Dashboard {
    pub title: Rc<RefCell<Option<String>>>,
    pub widgets: Rc<RefCell<Option<Vec<AnonymousStruct4>>>>,
}

impl std::fmt::Display for Dashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.title.borrow().as_ref().unwrap()), format_slice(&self.widgets))
    }
}


#[derive(Debug, Clone)]
struct AnonymousStruct1 {
    host: Rc<RefCell<Option<String>>>,
    port: Rc<RefCell<Option<i32>>>,
    credentials: Rc<RefCell<Option<AnonymousStruct2>>>,
}

impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { host: Default::default(), port: Default::default(), credentials: Rc::new(RefCell::new(Some(AnonymousStruct2::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.host.borrow().as_ref().unwrap()), (*self.port.borrow().as_ref().unwrap()), (*self.credentials.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct2 {
    user: Rc<RefCell<Option<String>>>,
    password: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.user.borrow().as_ref().unwrap()), (*self.password.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct3 {
    enabled: Rc<RefCell<Option<bool>>>,
    t_t_l: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.enabled.borrow().as_ref().unwrap()), (*self.t_t_l.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
struct AnonymousStruct4 {
    i_d: Rc<RefCell<Option<i32>>>,
    r#type: Rc<RefCell<Option<String>>>,
    position: Rc<RefCell<Option<AnonymousStruct5>>>,
}

impl Default for AnonymousStruct4 {
    fn default() -> Self {
        Self { i_d: Default::default(), r#type: Default::default(), position: Rc::new(RefCell::new(Some(AnonymousStruct5::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.i_d.borrow().as_ref().unwrap()), (*self.r#type.borrow().as_ref().unwrap()), (*self.position.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct5 {
    x: Rc<RefCell<Option<i32>>>,
    y: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.borrow().as_ref().unwrap()), (*self.y.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
struct AnonymousStruct6 {
    version: Rc<RefCell<Option<String>>>,
    modules: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<AnonymousStruct7>>>>>>>,
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.version.borrow().as_ref().unwrap()), format_map(&self.modules))
    }
}


#[derive(Clone)]
struct AnonymousStruct7 {
    enabled: Rc<RefCell<Option<bool>>>,
    settings: Rc<RefCell<Option<AnonymousStruct8>>>,
}

impl Default for AnonymousStruct7 {
    fn default() -> Self {
        Self { enabled: Default::default(), settings: Rc::new(RefCell::new(Some(AnonymousStruct8::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.enabled.borrow().as_ref().unwrap()), (*self.settings.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
struct AnonymousStruct8 {
    options: Rc<RefCell<Option<Vec<AnonymousStruct9>>>>,
}

impl std::fmt::Display for AnonymousStruct8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.options))
    }
}


#[derive(Clone, Default)]
struct AnonymousStruct9 {
    key: Rc<RefCell<Option<String>>>,
    value: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl std::fmt::Display for AnonymousStruct9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.key.borrow().as_ref().unwrap()), format_any(self.value.borrow().as_ref().unwrap().as_ref()))
    }
}


fn main() {
        // Test named struct with anonymous struct field
    let mut config = Rc::new(RefCell::new(Some(Config { name: Rc::new(RefCell::new(Some("production".to_string()))), database: Rc::new(RefCell::new(Some(AnonymousStruct1::default()))), cache: Rc::new(RefCell::new(Some(AnonymousStruct3::default()))) })));
    { let new_val = "db.example.com".to_string(); *(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).host.borrow_mut() = Some(new_val); };
    { let new_val = 5432; *(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).port.borrow_mut() = Some(new_val); };
    { let new_val = "admin".to_string(); *(*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).credentials.borrow().as_ref().unwrap()).user.borrow_mut() = Some(new_val); };
    { let new_val = "secret".to_string(); *(*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).credentials.borrow().as_ref().unwrap()).password.borrow_mut() = Some(new_val); };
    { let new_val = true; *(*(*config.borrow().as_ref().unwrap()).cache.borrow().as_ref().unwrap()).enabled.borrow_mut() = Some(new_val); };
    { let new_val = 300; *(*(*config.borrow().as_ref().unwrap()).cache.borrow().as_ref().unwrap()).t_t_l.borrow_mut() = Some(new_val); };

    print!("Config: {}\n", (*(*config.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("Database: {}:{} (user: {})\n", (*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).host.borrow().as_ref().unwrap()), (*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).port.borrow().as_ref().unwrap()), (*(*(*(*config.borrow().as_ref().unwrap()).database.borrow().as_ref().unwrap()).credentials.borrow().as_ref().unwrap()).user.borrow().as_ref().unwrap()));
    print!("Cache: enabled={}, TTL={}\n", (*(*(*config.borrow().as_ref().unwrap()).cache.borrow().as_ref().unwrap()).enabled.borrow().as_ref().unwrap()), (*(*(*config.borrow().as_ref().unwrap()).cache.borrow().as_ref().unwrap()).t_t_l.borrow().as_ref().unwrap()));

        // Test named struct with slice of anonymous structs
    let mut dashboard = Rc::new(RefCell::new(Some(Dashboard { title: Rc::new(RefCell::new(Some("Main Dashboard".to_string()))), widgets: Rc::new(RefCell::new(Some(vec![AnonymousStruct4 { i_d: Rc::new(RefCell::new(Some(1))), r#type: Rc::new(RefCell::new(Some("chart".to_string()))), position: Rc::new(RefCell::new(Some(AnonymousStruct5 { x: Rc::new(RefCell::new(Some(0))), y: Rc::new(RefCell::new(Some(0))) }))), ..Default::default() }, AnonymousStruct4 { i_d: Rc::new(RefCell::new(Some(2))), r#type: Rc::new(RefCell::new(Some("table".to_string()))), position: Rc::new(RefCell::new(Some(AnonymousStruct5 { x: Rc::new(RefCell::new(Some(100))), y: Rc::new(RefCell::new(Some(0))) }))), ..Default::default() }]))), ..Default::default() })));

    print!("\nDashboard: {}\n", (*(*dashboard.borrow().as_ref().unwrap()).title.borrow().as_ref().unwrap()));
    { let __range_holder = (*dashboard.borrow().as_ref().unwrap()).widgets.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for widget in __range_values.iter() {
        print!("Widget {} ({}) at position ({}, {})\n", (*widget.i_d.borrow().as_ref().unwrap()), (*widget.r#type.borrow().as_ref().unwrap()), (*(*widget.position.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()), (*(*widget.position.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()));
    } }

        // Deeply nested anonymous structs
    let mut system: Rc<RefCell<Option<AnonymousStruct6>>> = Rc::new(RefCell::new(Some(Default::default())));

    { let new_val = "1.0.0".to_string(); *(*system.borrow().as_ref().unwrap()).version.borrow_mut() = Some(new_val); };
    { let new_val = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<AnonymousStruct7>>>>::new()))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *(*system.borrow().as_ref().unwrap()).modules.borrow_mut() = __moved_val; };

        // Add a module with settings
    let mut authModule = Rc::new(RefCell::new(Some(AnonymousStruct7 { enabled: Rc::new(RefCell::new(Some(true))), settings: Rc::new(RefCell::new(Some(AnonymousStruct8::default()))) })));
    { let new_val = Rc::new(RefCell::new(Some(vec![AnonymousStruct9 { key: Rc::new(RefCell::new(Some("timeout".to_string()))), value: Rc::new(RefCell::new(Some(Box::new(3600) as Box<dyn Any>))), ..Default::default() }, AnonymousStruct9 { key: Rc::new(RefCell::new(Some("max_attempts".to_string()))), value: Rc::new(RefCell::new(Some(Box::new(3) as Box<dyn Any>))), ..Default::default() }]))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *(*(*authModule.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).options.borrow_mut() = __moved_val; };
    (*(*system.borrow().as_ref().unwrap()).modules.borrow_mut().as_mut().unwrap()).insert("auth".to_string(), authModule.clone());

    print!("\nSystem version: {}\n", (*(*system.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()));
    for (name, module) in (*(*system.borrow().as_ref().unwrap()).modules.borrow().as_ref().unwrap()).clone() {
        print!("Module {}: enabled={}\n", name, (*(*module.borrow().as_ref().unwrap()).enabled.borrow().as_ref().unwrap()));
        { let __range_holder = (*(*module.borrow().as_ref().unwrap()).settings.borrow().as_ref().unwrap()).options.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for opt in __range_values.iter() {
        print!("  - {}: {}\n", (*opt.key.borrow().as_ref().unwrap()), format_any(opt.value.borrow().as_ref().unwrap().as_ref()));
    } }
    }
}