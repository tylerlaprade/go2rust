#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    state: String,
}

#[derive(Debug)]
struct Employee {
    person: Person,
    address: Address,
    i_d: i32,
    salary: f64,
}

fn main() {
    let mut p1 = Person { name: "Alice".to_string(), age: 30 };
    println!("{} {}", "Person 1:".to_string(), p1);
    let mut p2 = Person { name: "Bob".to_string(), age: 25 };
    println!("{} {}", "Person 2:".to_string(), p2);
    p2.age = 26;
    println!("{} {}", "Updated Person 2:".to_string(), p2);
    let mut emp = Employee { person: Person { name: "Charlie".to_string(), age: 35 }, address: Address { street: "123 Main St".to_string(), city: "Anytown".to_string(), state: "CA".to_string() }, i_d: 1001, salary: 75000.0 };
    println!("{} {}", "Employee:".to_string(), emp);
    println!("{} {}", "Employee name:".to_string(), emp.name);
    println!("{} {}", "Employee city:".to_string(), emp.city);
}