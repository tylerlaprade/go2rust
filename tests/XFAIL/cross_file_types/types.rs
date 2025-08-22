/// Person represents a person with name and age
#[derive(Debug, Clone, Default)]
struct Person {
    name: Rc<RefCell<Option<String>>>,
    age: Rc<RefCell<Option<i32>>>,
}

/// Address represents a physical address
#[derive(Debug, Clone, Default)]
struct Address {
    street: Rc<RefCell<Option<String>>>,
    city: Rc<RefCell<Option<String>>>,
    zip: Rc<RefCell<Option<String>>>,
}

/// Employee combines Person and Address
#[derive(Debug, Clone, Default)]
struct Employee {
    person: Rc<RefCell<Option<Person>>>,
    address: Rc<RefCell<Option<Address>>>,
    i_d: Rc<RefCell<Option<i32>>>,
}