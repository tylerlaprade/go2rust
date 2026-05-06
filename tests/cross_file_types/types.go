package main

// Person represents a person with name and age
type Person struct {
	Name string
	Age  int
}

// Address represents a physical address
type Address struct {
	Street string
	City   string
	Zip    string
}

// Employee combines Person and Address
type Employee struct {
	Person  Person
	Address Address
	ID      int
}
