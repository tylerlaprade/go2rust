use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<HashMap<K, Arc<Mutex<Option<V>>>>>>>) -> String 
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
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
fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Debug)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Address {
    street: Arc<Mutex<Option<String>>>,
    city: Arc<Mutex<Option<String>>>,
    state: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Employee {
    arc<_mutex<_option<_person>>>: Arc<Mutex<Option<Person>>>,
    arc<_mutex<_option<_address>>>: Arc<Mutex<Option<Address>>>,
    i_d: Arc<Mutex<Option<i32>>>,
    salary: Arc<Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Manager {
    arc<_mutex<_option<_employee>>>: Arc<Mutex<Option<Employee>>>,
    team: Arc<Mutex<Option<Vec<String>>>>,
}

/// Anonymous struct embedding
#[derive(Debug)]
struct CompanyInfo {
    founded: Arc<Mutex<Option<i32>>>,
    c_e_o: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Company {
    name: Arc<Mutex<Option<String>>>,
    arc<_mutex<_option<_company_info>>>: Arc<Mutex<Option<CompanyInfo>>>,
}

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {}\n", (*self.name.lock().unwrap().as_mut().unwrap()));
    }

    pub fn get_info(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{} ({} years old)", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.age.lock().unwrap().as_mut().unwrap())))));
    }
}

impl Address {
    pub fn full_address(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}, {}, {}", (*self.street.lock().unwrap().as_mut().unwrap()), (*self.city.lock().unwrap().as_mut().unwrap()), (*self.state.lock().unwrap().as_mut().unwrap())))));
    }
}

impl Employee {
    pub fn work(&self) {
        print!("{} is working (ID: {})\n", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.i_d.lock().unwrap().as_mut().unwrap()));
    }
}

impl Manager {
    pub fn manage(&self) {
        print!("Manager {} is managing team: {}\n", (*self.name.lock().unwrap().as_mut().unwrap()), format_slice(&self.team.clone()));
    }
}

fn main() {
    println!("{}", "=== Basic embedded struct ===".to_string());
    let mut emp = Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) }))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("123 Main St".to_string()))), city: Arc::new(Mutex::new(Some("Anytown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))) }))), i_d: Arc::new(Mutex::new(Some(1001))), salary: Arc::new(Mutex::new(Some(75000.0))) };

    print!("Name: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).name);
    print!("Age: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).age);
    print!("Street: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).street);
    print!("ID: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).i_d);

    (*emp.lock().unwrap().as_mut().unwrap()).greet();
    println!("{} {}", "Info:".to_string(), (*(*emp.lock().unwrap().as_mut().unwrap()).get_info().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Address:".to_string(), (*(*emp.lock().unwrap().as_mut().unwrap()).full_address().lock().unwrap().as_mut().unwrap()));
    (*emp.lock().unwrap().as_mut().unwrap()).work();

    println!("{}", "\n=== Nested embedding ===".to_string());
    let mut mgr = Manager { employee: Arc::new(Mutex::new(Some(Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(35))) }))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("456 Oak Ave".to_string()))), city: Arc::new(Mutex::new(Some("Somewhere".to_string()))), state: Arc::new(Mutex::new(Some("NY".to_string()))) }))), i_d: Arc::new(Mutex::new(Some(2001))), salary: Arc::new(Mutex::new(Some(95000.0))) }))), team: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()])))))) };

    print!("Manager: {}\n", (*mgr.lock().unwrap().as_mut().unwrap()).name);
    print!("Manager ID: {}\n", (*mgr.lock().unwrap().as_mut().unwrap()).i_d);
    print!("Manager City: {}\n", (*mgr.lock().unwrap().as_mut().unwrap()).city);

    (*mgr.lock().unwrap().as_mut().unwrap()).greet();
    (*mgr.lock().unwrap().as_mut().unwrap()).work();
    (*mgr.lock().unwrap().as_mut().unwrap()).manage();

    println!("{}", "\n=== Anonymous struct embedding ===".to_string());
    let mut company = Company { name: Arc::new(Mutex::new(Some("TechCorp".to_string()))) };
    { let new_val = 2010; *(*company.lock().unwrap().as_mut().unwrap()).founded.lock().unwrap() = Some(new_val); };
    { let new_val = "John Doe".to_string(); *(*company.lock().unwrap().as_mut().unwrap()).c_e_o.lock().unwrap() = Some(new_val); };

    print!("Company: {}\n", (*company.lock().unwrap().as_mut().unwrap()).name);
    print!("Founded: {}\n", (*company.lock().unwrap().as_mut().unwrap()).founded);
    print!("CEO: {}\n", (*company.lock().unwrap().as_mut().unwrap()).c_e_o);

    println!("{}", "\n=== Method promotion ===".to_string());
    println!("{}", "Employee methods are promoted from Person and Address".to_string());
    print!("Employee can call: {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).get_info().lock().unwrap().as_mut().unwrap()));
    print!("Employee address: {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).full_address().lock().unwrap().as_mut().unwrap()));
}